use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::contract::{BoundaryMode, BspServices, UserMmuState};
use crate::core::fs::{
    builtin_dir_entry_at, builtin_ioctl, builtin_open, builtin_read_at, builtin_stat,
    builtin_stat_open, builtin_write, resolve_path, seek_offset, FdError, FdTable, MountedRootfs,
    OpenFileDescription, OpenOptions, PipeTable, SyscallVfs, VfsDirEntry, VfsError, VfsIoctl,
    VfsMountTable, VfsPath, VfsPathBuffer, VfsRuntimeError, VfsStat, WritableOverlay,
};
use crate::core::loader::{executable_interpreter, prepare_executable_image, AuxEntry};
use crate::core::mm::{
    MemoryFoundation, UserMemoryMapper, UserMemoryReader, UserMemoryWriter, PAGE_SIZE,
};
use crate::core::syscall::{
    dispatch_single_with_runtime, dispatch_with_runtime, ExecRequest, SyscallError, SyscallFrame,
    SyscallOutcome,
};
use crate::core::task::{single_pid, single_set_user_memory_state, PendingUserEntry, Process};
use crate::kernel::exec::executable_abi;
use crate::official::user_output::{write_user_fd, UserOutputError};

static ACTIVE_ROOTFS: AtomicUsize = AtomicUsize::new(0);
static ACTIVE_MEMORY: AtomicUsize = AtomicUsize::new(0);
static mut ACTIVE_FD_TABLE: FdTable = FdTable::new();
static mut ACTIVE_OVERLAY: WritableOverlay = WritableOverlay::new();
static mut ACTIVE_MOUNTS: VfsMountTable = VfsMountTable::new();
static mut ACTIVE_PIPES: PipeTable = PipeTable::new();
static mut ACTIVE_CWD: VfsPathBuffer = VfsPathBuffer::root();
static mut ACTIVE_EXEC_PATH: VfsPathBuffer = VfsPathBuffer::root();
static mut ACTIVE_BSP: Option<BspServices> = None;

const SYSCALL_EXEC_BUFFER_SIZE: usize = 2 * 1024 * 1024;
const SYSCALL_EXEC_ARG_COUNT: usize = 8;
const SYSCALL_INTERPRETER_ARG_COUNT: usize = SYSCALL_EXEC_ARG_COUNT + 3;
const USER_MMAP_BASE: usize = 0x1_0000_0000;
const AT_PAGESZ: usize = 6;

#[derive(Clone, Copy)]
pub struct ActiveRuntimeSnapshot {
    cwd: VfsPathBuffer,
    exec_path: VfsPathBuffer,
    fd_table: FdTable,
}

impl ActiveRuntimeSnapshot {
    pub const fn empty() -> Self {
        Self {
            cwd: VfsPathBuffer::root(),
            exec_path: VfsPathBuffer::root(),
            fd_table: FdTable::new(),
        }
    }
}

#[repr(align(4096))]
struct SyscallExecBuffer {
    bytes: [u8; SYSCALL_EXEC_BUFFER_SIZE],
}

static mut SYSCALL_EXEC_BUFFER: SyscallExecBuffer = SyscallExecBuffer {
    bytes: [0; SYSCALL_EXEC_BUFFER_SIZE],
};

pub fn install_rootfs(
    bsp: BspServices,
    memory: &mut MemoryFoundation,
    rootfs: &mut MountedRootfs,
    cwd: VfsPathBuffer,
    exec_path: VfsPathBuffer,
) {
    active_fd_table_mut().reset();
    install_stdio_fds();
    active_overlay_mut().reset();
    active_mounts_mut().reset();
    active_pipe_table_mut().reset();
    set_active_cwd(cwd);
    set_active_exec_path(exec_path);
    unsafe {
        core::ptr::addr_of_mut!(ACTIVE_BSP).write(Some(bsp));
    }
    ACTIVE_MEMORY.store(memory as *mut MemoryFoundation as usize, Ordering::Release);
    ACTIVE_ROOTFS.store(rootfs as *mut MountedRootfs as usize, Ordering::Release);
}

pub fn clear_rootfs() {
    ACTIVE_ROOTFS.store(0, Ordering::Release);
    ACTIVE_MEMORY.store(0, Ordering::Release);
    unsafe {
        core::ptr::addr_of_mut!(ACTIVE_BSP).write(None);
    }
    active_fd_table_mut().reset();
    active_overlay_mut().reset();
    active_mounts_mut().reset();
    active_pipe_table_mut().reset();
}

pub fn active_runtime_snapshot() -> ActiveRuntimeSnapshot {
    ActiveRuntimeSnapshot {
        cwd: active_cwd(),
        exec_path: active_exec_path(),
        fd_table: active_fd_table(),
    }
}

pub fn restore_active_runtime_snapshot(snapshot: ActiveRuntimeSnapshot) {
    set_active_cwd(snapshot.cwd);
    set_active_exec_path(snapshot.exec_path);
    set_active_fd_table(snapshot.fd_table);
}

pub fn dispatch_with_memory<R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper>(
    frame: SyscallFrame,
    process: &mut Process,
    memory: &R,
) -> SyscallOutcome {
    let vfs = ActiveSyscallVfs;
    dispatch_with_runtime(frame, process, memory, &vfs)
}

pub fn dispatch_single_with_memory<R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper>(
    frame: SyscallFrame,
    memory: &R,
) -> SyscallOutcome {
    let vfs = ActiveSyscallVfs;
    dispatch_single_with_runtime(frame, memory, &vfs)
}

pub fn exec_from_syscall(request: ExecRequest) -> Result<PendingUserEntry, isize> {
    let bsp = active_bsp()?;
    let memory = active_memory_mut()?;
    let rootfs = active_rootfs_mut().map_err(map_runtime_errno)?;
    let resolved = resolve_from_active_cwd(-100, request.path()).map_err(map_runtime_errno)?;
    let file = rootfs
        .open(
            resolved
                .as_path()
                .map_err(VfsRuntimeError::Vfs)
                .map_err(map_runtime_errno)?,
        )
        .map_err(VfsRuntimeError::Vfs)
        .map_err(map_runtime_errno)?;
    if !file.identity().is_regular() {
        return Err(SyscallError::Invalid.errno());
    }

    let file_buffer = syscall_exec_buffer();
    let read = rootfs
        .read_all(&file, file_buffer)
        .map_err(VfsRuntimeError::Vfs)
        .map_err(map_runtime_errno)?;
    let bytes = read.bytes();
    if let Some(script) = Shebang::parse(&file_buffer[..bytes]) {
        return exec_shebang_script(rootfs, bsp, memory, resolved, request, script);
    }
    if !file.identity().executable() {
        return Err(SyscallError::ExecFormat.errno());
    }
    let mut argv = [&[][..]; SYSCALL_EXEC_ARG_COUNT];
    let mut argc = 0usize;
    while argc < request.arg_count() && argc < argv.len() {
        if let Some(arg) = request.arg(argc) {
            argv[argc] = arg;
        }
        argc += 1;
    }

    match load_exec_image(bsp, memory, &file_buffer[..bytes], &argv[..argc]) {
        Ok(pending) => {
            set_active_exec_path(resolved);
            Ok(pending)
        }
        Err(errno) if errno == SyscallError::ExecFormat.errno() => {
            let abi = executable_abi(bsp.snapshot().boot().architecture());
            match executable_interpreter(Some(&file_buffer[..bytes]), abi)
                .map_err(|_| SyscallError::ExecFormat.errno())?
            {
                Some(interpreter) => exec_interpreted_elf(
                    rootfs,
                    bsp,
                    memory,
                    resolved,
                    &argv[..argc],
                    interpreter.path(),
                ),
                None => Err(errno),
            }
        }
        Err(errno) => Err(errno),
    }
}

fn exec_interpreted_elf(
    rootfs: &mut MountedRootfs,
    bsp: BspServices,
    memory: &mut MemoryFoundation,
    executable_path: VfsPathBuffer,
    executable_argv: &[&[u8]],
    interpreter_path: &[u8],
) -> Result<PendingUserEntry, isize> {
    let mut interpreter_storage = [0u8; 256];
    let interpreter_len = copy_exec_component(interpreter_path, &mut interpreter_storage)?;
    let interpreter_path = VfsPathBuffer::from_absolute(&interpreter_storage[..interpreter_len])
        .map_err(VfsRuntimeError::Vfs)
        .map_err(map_runtime_errno)?;
    let interpreter = rootfs
        .open(
            interpreter_path
                .as_path()
                .map_err(VfsRuntimeError::Vfs)
                .map_err(map_runtime_errno)?,
        )
        .map_err(VfsRuntimeError::Vfs)
        .map_err(map_runtime_errno)?;
    if !interpreter.identity().is_regular() || !interpreter.identity().executable() {
        return Err(SyscallError::ExecFormat.errno());
    }

    let mut argv = [&[][..]; SYSCALL_INTERPRETER_ARG_COUNT];
    let mut argc = 0usize;
    argv[argc] = interpreter_path.bytes();
    argc += 1;
    argv[argc] = executable_path.bytes();
    argc += 1;
    let mut source_arg = 1usize;
    while source_arg < executable_argv.len() && argc < argv.len() {
        argv[argc] = executable_argv[source_arg];
        argc += 1;
        source_arg += 1;
    }

    let file_buffer = syscall_exec_buffer();
    let read = rootfs
        .read_all(&interpreter, file_buffer)
        .map_err(VfsRuntimeError::Vfs)
        .map_err(map_runtime_errno)?;
    let pending = load_exec_image(bsp, memory, &file_buffer[..read.bytes()], &argv[..argc])?;
    set_active_exec_path(executable_path);
    Ok(pending)
}

fn exec_shebang_script(
    rootfs: &mut MountedRootfs,
    bsp: BspServices,
    memory: &mut MemoryFoundation,
    script_path: VfsPathBuffer,
    request: ExecRequest,
    script: Shebang<'_>,
) -> Result<PendingUserEntry, isize> {
    let mut interpreter_storage = [0u8; 256];
    let interpreter_len = copy_exec_component(script.interpreter, &mut interpreter_storage)?;
    let interpreter_path = VfsPathBuffer::from_absolute(&interpreter_storage[..interpreter_len])
        .map_err(VfsRuntimeError::Vfs)
        .map_err(map_runtime_errno)?;

    let interpreter_open = rootfs.open(
        interpreter_path
            .as_path()
            .map_err(VfsRuntimeError::Vfs)
            .map_err(map_runtime_errno)?,
    );
    let mut fallback_name = [0u8; 64];
    let mut fallback_name_len = 0usize;
    let (exec_path, interpreter, use_applet_name) = match interpreter_open {
        Ok(file) => (interpreter_path, file, false),
        Err(VfsError::PathNotFound) if basename(script.interpreter) == b"sh" => {
            let active = active_exec_path();
            fallback_name_len =
                copy_exec_component(basename(script.interpreter), &mut fallback_name)?;
            let file = rootfs
                .open(
                    active
                        .as_path()
                        .map_err(VfsRuntimeError::Vfs)
                        .map_err(map_runtime_errno)?,
                )
                .map_err(VfsRuntimeError::Vfs)
                .map_err(map_runtime_errno)?;
            (active, file, true)
        }
        Err(error) => {
            return Err(map_runtime_errno(VfsRuntimeError::Vfs(error)));
        }
    };
    if !interpreter.identity().is_regular() || !interpreter.identity().executable() {
        return Err(SyscallError::ExecFormat.errno());
    }

    let mut argv = [&[][..]; SYSCALL_INTERPRETER_ARG_COUNT];
    let mut argc = 0usize;
    argv[argc] = exec_path.bytes();
    argc += 1;
    if use_applet_name {
        argv[argc] = &fallback_name[..fallback_name_len];
        argc += 1;
    } else if let Some(argument) = script.argument {
        argv[argc] = argument;
        argc += 1;
    }
    argv[argc] = script_path.bytes();
    argc += 1;

    let mut source_arg = 1usize;
    while source_arg < request.arg_count() && argc < argv.len() {
        if let Some(arg) = request.arg(source_arg) {
            argv[argc] = arg;
            argc += 1;
        }
        source_arg += 1;
    }

    let file_buffer = syscall_exec_buffer();
    let read = rootfs
        .read_all(&interpreter, file_buffer)
        .map_err(VfsRuntimeError::Vfs)
        .map_err(map_runtime_errno)?;
    let pending = load_exec_image(bsp, memory, &file_buffer[..read.bytes()], &argv[..argc])?;
    set_active_exec_path(exec_path);
    Ok(pending)
}

fn load_exec_image(
    bsp: BspServices,
    memory: &mut MemoryFoundation,
    image_bytes: &[u8],
    argv: &[&[u8]],
) -> Result<PendingUserEntry, isize> {
    let auxv = [AuxEntry::new(AT_PAGESZ, PAGE_SIZE)];
    let load = prepare_executable_image(
        Some(image_bytes),
        memory.kernel_globals(),
        executable_abi(bsp.snapshot().boot().architecture()),
        argv,
        &[],
        &auxv,
    )
    .map_err(|_| SyscallError::ExecFormat.errno())?;

    let user_mmu = bsp.prepare_user_mmu(
        memory.frames_mut(),
        load.address_space_load(),
        BoundaryMode::ApplyUnsafe,
    );
    let address_space = match user_mmu {
        UserMmuState::Applied(address_space) => address_space,
        UserMmuState::Planned(_)
        | UserMmuState::Prepared(_)
        | UserMmuState::NotReady(_)
        | UserMmuState::Unsupported(_) => return Err(SyscallError::NoMemory.errno()),
    };
    let image = load
        .complete(address_space)
        .map_err(|_| SyscallError::Invalid.errno())?;
    let mut process = Process::new(single_pid());
    let pending = process
        .commit_exec(image)
        .map_err(|_| SyscallError::Invalid.errno())?;
    single_set_user_memory_state(process.heap_base(), process.program_break(), USER_MMAP_BASE);
    Ok(pending)
}

#[derive(Clone, Copy)]
struct Shebang<'a> {
    interpreter: &'a [u8],
    argument: Option<&'a [u8]>,
}

impl<'a> Shebang<'a> {
    fn parse(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() < 3 || bytes[0] != b'#' || bytes[1] != b'!' {
            return None;
        }
        let mut cursor = 2usize;
        while cursor < bytes.len() && is_shebang_space(bytes[cursor]) {
            cursor += 1;
        }
        let start = cursor;
        while cursor < bytes.len()
            && !is_shebang_space(bytes[cursor])
            && !is_line_end(bytes[cursor])
        {
            cursor += 1;
        }
        if start == cursor {
            return None;
        }
        let interpreter = &bytes[start..cursor];
        while cursor < bytes.len() && is_shebang_space(bytes[cursor]) {
            cursor += 1;
        }
        let arg_start = cursor;
        while cursor < bytes.len() && !is_line_end(bytes[cursor]) {
            cursor += 1;
        }
        let argument = if arg_start == cursor {
            None
        } else {
            Some(&bytes[arg_start..cursor])
        };
        Some(Self {
            interpreter,
            argument,
        })
    }
}

const fn is_shebang_space(byte: u8) -> bool {
    byte == b' ' || byte == b'\t'
}

const fn is_line_end(byte: u8) -> bool {
    byte == b'\n' || byte == b'\r' || byte == 0
}

fn basename(path: &[u8]) -> &[u8] {
    let mut start = 0usize;
    let mut index = 0usize;
    while index < path.len() {
        if path[index] == b'/' {
            start = index + 1;
        }
        index += 1;
    }
    &path[start..]
}

fn copy_exec_component(source: &[u8], out: &mut [u8]) -> Result<usize, isize> {
    if source.is_empty() || source.len() > out.len() {
        return Err(SyscallError::ExecFormat.errno());
    }
    out[..source.len()].copy_from_slice(source);
    Ok(source.len())
}

#[derive(Clone, Copy)]
struct ActiveSyscallVfs;

impl SyscallVfs for ActiveSyscallVfs {
    fn stat_path(&self, path: VfsPath<'_>) -> Result<VfsStat, VfsError> {
        if let Some(stat) = builtin_stat(path, active_mounts()) {
            return Ok(stat);
        }
        if let Some(stat) = active_overlay().stat(path)? {
            return Ok(stat);
        }
        active_rootfs_mut()
            .map_err(runtime_to_vfs_error)?
            .stat(path)
    }

    fn stat_path_at(&self, dirfd: isize, path: &[u8]) -> Result<VfsStat, VfsRuntimeError> {
        let resolved = resolve_from_active_cwd(dirfd, path)?;
        self.stat_path(resolved.as_path().map_err(VfsRuntimeError::Vfs)?)
            .map_err(VfsRuntimeError::Vfs)
    }

    fn open_path(
        &self,
        path: VfsPath<'_>,
        options: OpenOptions,
        mode: u16,
    ) -> Result<usize, VfsRuntimeError> {
        let description = match active_overlay_mut()
            .open(path, options, mode)
            .map_err(VfsRuntimeError::Vfs)?
        {
            Some(description) => description,
            None => {
                match builtin_open(path, options, active_mounts()).map_err(VfsRuntimeError::Vfs)? {
                    Some(description) => description,
                    None => active_rootfs_mut()?
                        .open_with_options(path, options)
                        .map_err(VfsRuntimeError::Vfs)?,
                }
            }
        };
        active_fd_table_mut()
            .insert(description, options.close_on_exec())
            .map_err(VfsRuntimeError::Fd)
    }

    fn open_path_at(
        &self,
        dirfd: isize,
        path: &[u8],
        options: OpenOptions,
        mode: u16,
    ) -> Result<usize, VfsRuntimeError> {
        let resolved = resolve_from_active_cwd(dirfd, path)?;
        self.open_path(
            resolved.as_path().map_err(VfsRuntimeError::Vfs)?,
            options,
            mode,
        )
    }

    fn change_dir(&self, path: &[u8]) -> Result<(), VfsRuntimeError> {
        let resolved = resolve_from_active_cwd(-100, path)?;
        let stat = self
            .stat_path(resolved.as_path().map_err(VfsRuntimeError::Vfs)?)
            .map_err(VfsRuntimeError::Vfs)?;
        if !stat.kind().is_directory() {
            return Err(VfsRuntimeError::Vfs(VfsError::DirectoryExpected));
        }
        set_active_cwd(resolved);
        Ok(())
    }

    fn mkdir_at(&self, dirfd: isize, path: &[u8], mode: u16) -> Result<(), VfsRuntimeError> {
        let resolved = resolve_from_active_cwd(dirfd, path)?;
        let resolved_path = resolved.as_path().map_err(VfsRuntimeError::Vfs)?;
        if active_overlay()
            .stat(resolved_path)
            .map_err(VfsRuntimeError::Vfs)?
            .is_some()
        {
            return Err(VfsRuntimeError::Vfs(VfsError::AlreadyExists));
        }
        match active_rootfs_mut()?.stat(resolved_path) {
            Ok(_) => return Err(VfsRuntimeError::Vfs(VfsError::AlreadyExists)),
            Err(VfsError::PathNotFound) => {}
            Err(error) => return Err(VfsRuntimeError::Vfs(error)),
        }

        let parent =
            VfsPathBuffer::parent_of_absolute(resolved.bytes()).map_err(VfsRuntimeError::Vfs)?;
        let parent_stat = self
            .stat_path(parent.as_path().map_err(VfsRuntimeError::Vfs)?)
            .map_err(VfsRuntimeError::Vfs)?;
        if !parent_stat.kind().is_directory() {
            return Err(VfsRuntimeError::Vfs(VfsError::DirectoryExpected));
        }

        active_overlay_mut()
            .mkdir(resolved_path, mode)
            .map_err(VfsRuntimeError::Vfs)
    }

    fn unlink_at(&self, dirfd: isize, path: &[u8], flags: usize) -> Result<(), VfsRuntimeError> {
        const AT_REMOVEDIR: usize = 0x200;
        if flags != 0 && flags != AT_REMOVEDIR {
            return Err(VfsRuntimeError::Vfs(VfsError::UnsupportedPath));
        }
        let resolved = resolve_from_active_cwd(dirfd, path)?;
        let resolved_path = resolved.as_path().map_err(VfsRuntimeError::Vfs)?;
        if active_overlay()
            .stat(resolved_path)
            .map_err(VfsRuntimeError::Vfs)?
            .is_some()
        {
            let result = if flags == AT_REMOVEDIR {
                active_overlay_mut().rmdir(resolved_path)
            } else {
                active_overlay_mut().unlink(resolved_path)
            };
            return result.map_err(VfsRuntimeError::Vfs);
        }
        match active_rootfs_mut()?.stat(resolved_path) {
            Ok(_) => Err(VfsRuntimeError::Vfs(VfsError::UnsupportedPath)),
            Err(error) => Err(VfsRuntimeError::Vfs(error)),
        }
    }

    fn rename_at(
        &self,
        old_dirfd: isize,
        old_path: &[u8],
        new_dirfd: isize,
        new_path: &[u8],
        flags: usize,
    ) -> Result<(), VfsRuntimeError> {
        if flags != 0 {
            return Err(VfsRuntimeError::Vfs(VfsError::UnsupportedPath));
        }
        let old_resolved = resolve_from_active_cwd(old_dirfd, old_path)?;
        let new_resolved = resolve_from_active_cwd(new_dirfd, new_path)?;
        let old_vfs_path = old_resolved.as_path().map_err(VfsRuntimeError::Vfs)?;
        let new_vfs_path = new_resolved.as_path().map_err(VfsRuntimeError::Vfs)?;
        let source_is_overlay = active_overlay()
            .stat(old_vfs_path)
            .map_err(VfsRuntimeError::Vfs)?
            .is_some();
        if !source_is_overlay {
            match active_rootfs_mut()?.stat(old_vfs_path) {
                Ok(_) => return Err(VfsRuntimeError::Vfs(VfsError::UnsupportedPath)),
                Err(error) => return Err(VfsRuntimeError::Vfs(error)),
            }
        }
        let parent = VfsPathBuffer::parent_of_absolute(new_resolved.bytes())
            .map_err(VfsRuntimeError::Vfs)?;
        let parent_stat = self
            .stat_path(parent.as_path().map_err(VfsRuntimeError::Vfs)?)
            .map_err(VfsRuntimeError::Vfs)?;
        if !parent_stat.kind().is_directory() {
            return Err(VfsRuntimeError::Vfs(VfsError::DirectoryExpected));
        }
        match active_rootfs_mut()?.stat(new_vfs_path) {
            Ok(_) => return Err(VfsRuntimeError::Vfs(VfsError::AlreadyExists)),
            Err(VfsError::PathNotFound) => {}
            Err(error) => return Err(VfsRuntimeError::Vfs(error)),
        }
        active_overlay_mut()
            .rename(old_vfs_path, new_vfs_path)
            .map_err(VfsRuntimeError::Vfs)
    }

    fn mount(
        &self,
        source: &[u8],
        target: &[u8],
        filesystem: &[u8],
        flags: usize,
    ) -> Result<(), VfsRuntimeError> {
        let resolved = resolve_from_active_cwd(-100, target)?;
        let resolved_path = resolved.as_path().map_err(VfsRuntimeError::Vfs)?;
        let stat = self
            .stat_path(resolved_path)
            .map_err(VfsRuntimeError::Vfs)?;
        if !stat.kind().is_directory() {
            return Err(VfsRuntimeError::Vfs(VfsError::DirectoryExpected));
        }
        active_mounts_mut()
            .mount(resolved_path, source, filesystem, flags)
            .map_err(VfsRuntimeError::Vfs)
    }

    fn unmount(&self, target: &[u8], flags: usize) -> Result<(), VfsRuntimeError> {
        if flags != 0 {
            return Err(VfsRuntimeError::Vfs(VfsError::UnsupportedPath));
        }
        let resolved = resolve_from_active_cwd(-100, target)?;
        let resolved_path = resolved.as_path().map_err(VfsRuntimeError::Vfs)?;
        active_mounts_mut()
            .unmount(resolved_path)
            .map_err(VfsRuntimeError::Vfs)
    }

    fn pipe(&self) -> Result<(usize, usize), VfsRuntimeError> {
        let (pipe_id, reader, writer) = active_pipe_table_mut()
            .create()
            .map_err(VfsRuntimeError::Fd)?;
        let read_fd = match active_fd_table_mut().insert(reader, false) {
            Ok(fd) => fd,
            Err(error) => {
                let _ = active_pipe_table_mut().release(pipe_id);
                return Err(VfsRuntimeError::Fd(error));
            }
        };
        let write_fd = match active_fd_table_mut().insert(writer, false) {
            Ok(fd) => fd,
            Err(error) => {
                let _ = active_fd_table_mut().close(read_fd);
                let _ = active_pipe_table_mut().release(pipe_id);
                return Err(VfsRuntimeError::Fd(error));
            }
        };
        Ok((read_fd, write_fd))
    }

    fn owns_stdio_fds(&self) -> bool {
        true
    }

    fn getcwd(&self, out: &mut [u8]) -> Result<usize, VfsRuntimeError> {
        let cwd = active_cwd();
        let bytes = cwd.bytes();
        let len = bytes
            .len()
            .checked_add(1)
            .ok_or(VfsRuntimeError::Vfs(VfsError::InvalidPath))?;
        if len > out.len() {
            return Err(VfsRuntimeError::Vfs(VfsError::InvalidPath));
        }
        out[..bytes.len()].copy_from_slice(bytes);
        out[bytes.len()] = 0;
        Ok(len)
    }

    fn close_fd(&self, fd: usize) -> Result<(), FdError> {
        active_fd_table_mut().close(fd)
    }

    fn fstat_fd(&self, fd: usize) -> Result<VfsStat, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if description.is_pipe() {
            return Ok(VfsStat::new(description.identity(), 4096));
        }
        if let Some(stat) = builtin_stat_open(description, active_mounts()) {
            return Ok(stat);
        }
        if let Some(stat) = active_overlay().stat_open(description) {
            return Ok(stat);
        }
        Ok(active_rootfs_mut()?.stat_open(description))
    }

    fn read_fd(&self, fd: usize, out: &mut [u8]) -> Result<usize, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        self.read_description_at(description, description.offset(), out)
    }

    fn read_fd_at(&self, fd: usize, offset: u64, out: &mut [u8]) -> Result<usize, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        self.read_description_at(description, offset, out)
    }

    fn write_fd(&self, fd: usize, bytes: &[u8]) -> Result<usize, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if let Some(user_fd) = description.user_output_fd() {
            return write_user_fd(user_fd, bytes).map_err(map_user_output_to_runtime);
        }
        if !description.writable() {
            return Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor));
        }
        if let Some(result) = builtin_write(description, bytes) {
            return result.map_err(VfsRuntimeError::Vfs);
        }
        if description.is_pipe() {
            return active_pipe_table_mut()
                .write(description, bytes)
                .map_err(VfsRuntimeError::Fd);
        }
        if active_overlay().owns_identity(description.identity()) {
            let offset = if description.append() {
                active_overlay()
                    .file_len(description)
                    .map_err(VfsRuntimeError::Vfs)?
            } else {
                description.offset()
            };
            let written = active_overlay_mut()
                .write_at(description, offset, bytes)
                .map_err(VfsRuntimeError::Vfs)?;
            active_fd_table_mut()
                .set_offset(fd, offset + written as u64)
                .map_err(VfsRuntimeError::Fd)?;
            return Ok(written);
        }
        Err(VfsRuntimeError::Vfs(VfsError::UnsupportedPath))
    }

    fn ioctl_fd(&self, fd: usize, request: usize) -> Result<VfsIoctl, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if let Some(result) = builtin_ioctl(description, request) {
            return result.map_err(VfsRuntimeError::Vfs);
        }
        Err(VfsRuntimeError::Vfs(VfsError::UnsupportedPath))
    }

    fn fd_readable(&self, fd: usize) -> Result<bool, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if !description.readable() {
            return Ok(false);
        }
        if description.is_pipe() {
            return active_pipe_table_mut()
                .readable(description)
                .map_err(VfsRuntimeError::Fd);
        }
        Ok(true)
    }

    fn fd_writable(&self, fd: usize) -> Result<bool, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if !description.writable() {
            return Ok(false);
        }
        if description.is_pipe() {
            return active_pipe_table_mut()
                .writable(description)
                .map_err(VfsRuntimeError::Fd);
        }
        Ok(true)
    }

    fn lseek_fd(&self, fd: usize, offset: i64, whence: usize) -> Result<u64, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if description.is_pipe() {
            return Ok(0);
        }
        let next = seek_offset(
            description.offset(),
            description.identity().byte_len(),
            offset,
            whence,
        )
        .map_err(VfsRuntimeError::Fd)?;
        active_fd_table_mut()
            .set_offset(fd, next)
            .map_err(VfsRuntimeError::Fd)?;
        Ok(next)
    }

    fn dir_entry_at_fd(&self, fd: usize) -> Result<Option<VfsDirEntry>, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if !description.is_directory() {
            return Err(VfsRuntimeError::Fd(FdError::NotDirectory));
        }
        if let Some(result) = builtin_dir_entry_at(description, description.offset()) {
            return result.map_err(VfsRuntimeError::Vfs);
        }
        if active_overlay().owns_identity(description.identity()) {
            return active_overlay()
                .dir_entry_at(description, description.offset())
                .map_err(VfsRuntimeError::Vfs);
        }
        active_rootfs_mut()?
            .dir_entry_at(description, description.offset())
            .map_err(VfsRuntimeError::Vfs)
    }

    fn set_fd_offset(&self, fd: usize, offset: u64) -> Result<(), FdError> {
        active_fd_table_mut().set_offset(fd, offset)
    }

    fn set_fd_close_on_exec(&self, fd: usize, close_on_exec: bool) -> Result<(), FdError> {
        active_fd_table_mut().set_close_on_exec(fd, close_on_exec)
    }

    fn fd_close_on_exec(&self, fd: usize) -> Result<bool, FdError> {
        active_fd_table_mut().close_on_exec(fd)
    }

    fn fd_status_flags(&self, fd: usize) -> Result<usize, FdError> {
        active_fd_table_mut().status_flags(fd)
    }

    fn duplicate_fd_min(
        &self,
        old_fd: usize,
        min_fd: usize,
        close_on_exec: bool,
    ) -> Result<usize, FdError> {
        if is_user_output_fd(old_fd) && !active_fd_table_mut().contains(old_fd) {
            return active_fd_table_mut()
                .insert(OpenFileDescription::user_output(old_fd), close_on_exec);
        }
        active_fd_table_mut().duplicate_min(old_fd, min_fd, close_on_exec)
    }

    fn duplicate_fd_to(
        &self,
        old_fd: usize,
        new_fd: usize,
        close_on_exec: bool,
    ) -> Result<usize, FdError> {
        if is_user_output_fd(old_fd) && !active_fd_table_mut().contains(old_fd) {
            return active_fd_table_mut().insert_at(
                new_fd,
                OpenFileDescription::user_output(old_fd),
                close_on_exec,
            );
        }
        active_fd_table_mut().duplicate_to(old_fd, new_fd, close_on_exec)
    }
}

impl ActiveSyscallVfs {
    fn read_description_at(
        &self,
        description: OpenFileDescription,
        offset: u64,
        out: &mut [u8],
    ) -> Result<usize, VfsRuntimeError> {
        if !description.readable() {
            return Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor));
        }
        if let Some(result) = builtin_read_at(description, active_mounts(), offset, out) {
            return result.map_err(VfsRuntimeError::Vfs);
        }
        if active_overlay().owns_identity(description.identity()) {
            return active_overlay()
                .read_at(description, offset, out)
                .map_err(VfsRuntimeError::Vfs);
        }
        if description.is_pipe() {
            return active_pipe_table_mut()
                .read(description, out)
                .map_err(VfsRuntimeError::Fd);
        }
        if !description.is_regular() {
            return Err(VfsRuntimeError::Fd(FdError::NotRegularFile));
        }
        active_rootfs_mut()?
            .read_at(&description, offset, out)
            .map(|read| read.bytes())
            .map_err(VfsRuntimeError::Vfs)
    }
}

fn active_rootfs_mut() -> Result<&'static mut MountedRootfs, VfsRuntimeError> {
    let raw = ACTIVE_ROOTFS.load(Ordering::Acquire);
    if raw == 0 {
        return Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing));
    }

    Ok(unsafe { &mut *(raw as *mut MountedRootfs) })
}

fn active_memory_mut() -> Result<&'static mut MemoryFoundation, isize> {
    let raw = ACTIVE_MEMORY.load(Ordering::Acquire);
    if raw == 0 {
        return Err(SyscallError::NoDevice.errno());
    }

    Ok(unsafe { &mut *(raw as *mut MemoryFoundation) })
}

fn active_bsp() -> Result<BspServices, isize> {
    match unsafe { core::ptr::addr_of!(ACTIVE_BSP).read() } {
        Some(bsp) => Ok(bsp),
        None => Err(SyscallError::NoDevice.errno()),
    }
}

fn active_fd_table_mut() -> &'static mut FdTable {
    unsafe { &mut *core::ptr::addr_of_mut!(ACTIVE_FD_TABLE) }
}

fn active_mounts() -> &'static VfsMountTable {
    unsafe { &*core::ptr::addr_of!(ACTIVE_MOUNTS) }
}

fn install_stdio_fds() {
    let _ = active_fd_table_mut().insert_at(0, OpenFileDescription::user_input(), false);
    let _ = active_fd_table_mut().insert_at(1, OpenFileDescription::user_output(1), false);
    let _ = active_fd_table_mut().insert_at(2, OpenFileDescription::user_output(2), false);
}

fn active_fd_table() -> FdTable {
    unsafe { core::ptr::addr_of!(ACTIVE_FD_TABLE).read() }
}

fn set_active_fd_table(table: FdTable) {
    unsafe {
        core::ptr::addr_of_mut!(ACTIVE_FD_TABLE).write(table);
    }
}

fn active_overlay() -> &'static WritableOverlay {
    unsafe { &*core::ptr::addr_of!(ACTIVE_OVERLAY) }
}

fn active_overlay_mut() -> &'static mut WritableOverlay {
    unsafe { &mut *core::ptr::addr_of_mut!(ACTIVE_OVERLAY) }
}

fn active_mounts_mut() -> &'static mut VfsMountTable {
    unsafe { &mut *core::ptr::addr_of_mut!(ACTIVE_MOUNTS) }
}

fn active_pipe_table_mut() -> &'static mut PipeTable {
    unsafe { &mut *core::ptr::addr_of_mut!(ACTIVE_PIPES) }
}

fn set_active_cwd(cwd: VfsPathBuffer) {
    unsafe {
        core::ptr::addr_of_mut!(ACTIVE_CWD).write(cwd);
    }
}

fn active_cwd() -> VfsPathBuffer {
    unsafe { core::ptr::addr_of!(ACTIVE_CWD).read() }
}

fn set_active_exec_path(path: VfsPathBuffer) {
    unsafe {
        core::ptr::addr_of_mut!(ACTIVE_EXEC_PATH).write(path);
    }
}

fn active_exec_path() -> VfsPathBuffer {
    unsafe { core::ptr::addr_of!(ACTIVE_EXEC_PATH).read() }
}

const fn is_user_output_fd(fd: usize) -> bool {
    fd == 1 || fd == 2
}

fn map_user_output_to_runtime(error: UserOutputError) -> VfsRuntimeError {
    match error {
        UserOutputError::UnsupportedFd => VfsRuntimeError::Fd(FdError::BadFileDescriptor),
        UserOutputError::SinkMissing => VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing),
    }
}

fn resolve_from_active_cwd(dirfd: isize, path: &[u8]) -> Result<VfsPathBuffer, VfsRuntimeError> {
    if path.first() != Some(&b'/') && dirfd != -100 {
        let description = active_fd_table_mut()
            .get(dirfd as usize)
            .map_err(VfsRuntimeError::Fd)?;
        if !description.is_directory() {
            return Err(VfsRuntimeError::Fd(FdError::NotDirectory));
        }
    }
    let cwd = active_cwd();
    resolve_path(cwd.as_path().map_err(VfsRuntimeError::Vfs)?, path).map_err(VfsRuntimeError::Vfs)
}

fn runtime_to_vfs_error(error: VfsRuntimeError) -> VfsError {
    match error {
        VfsRuntimeError::Vfs(error) => error,
        VfsRuntimeError::Fd(_) => VfsError::RootfsSourceMissing,
    }
}

fn map_runtime_errno(error: VfsRuntimeError) -> isize {
    match error {
        VfsRuntimeError::Fd(FdError::BadFileDescriptor) => SyscallError::BadFileDescriptor.errno(),
        VfsRuntimeError::Fd(FdError::InvalidOffset) => SyscallError::Invalid.errno(),
        VfsRuntimeError::Fd(FdError::NotDirectory) => SyscallError::NotDirectory.errno(),
        VfsRuntimeError::Fd(FdError::NotRegularFile) => SyscallError::IsDirectory.errno(),
        VfsRuntimeError::Fd(FdError::TableFull) => SyscallError::TooManyOpenFiles.errno(),
        VfsRuntimeError::Vfs(VfsError::AlreadyExists) => SyscallError::Exists.errno(),
        VfsRuntimeError::Vfs(VfsError::PathNotFound) => SyscallError::NoEntry.errno(),
        VfsRuntimeError::Vfs(VfsError::DirectoryExpected) => SyscallError::NotDirectory.errno(),
        VfsRuntimeError::Vfs(VfsError::NotRegularFile) => SyscallError::IsDirectory.errno(),
        VfsRuntimeError::Vfs(VfsError::NoSpace) => SyscallError::NoSpace.errno(),
        VfsRuntimeError::Vfs(VfsError::PermissionDenied) => SyscallError::PermissionDenied.errno(),
        VfsRuntimeError::Vfs(VfsError::UnsupportedPath | VfsError::UnsupportedRootfs) => {
            SyscallError::NotSupported.errno()
        }
        VfsRuntimeError::Vfs(
            VfsError::Block(_)
            | VfsError::EmptyFile
            | VfsError::FileTooLarge
            | VfsError::InvalidPath
            | VfsError::MetadataCorrupt
            | VfsError::NotExecutable
            | VfsError::RootfsSourceMissing,
        ) => SyscallError::Invalid.errno(),
    }
}

fn syscall_exec_buffer() -> &'static mut [u8] {
    unsafe { &mut (*core::ptr::addr_of_mut!(SYSCALL_EXEC_BUFFER)).bytes }
}
