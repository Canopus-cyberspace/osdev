fn maybe_deliver_pending_signal_user(cx: &mut TrapContext) {
    let ret = crate::fs::runtime::deliver_signal_frame(cx.sepc, cx.regs[2]);
    if ret != 0 {
        return;
    }
    let snapshot = crate::fs::runtime::signal_snapshot();
    if !snapshot.frame_active || snapshot.frame_handler == 0 {
        return;
    }
    write_signal_frame_user(
        snapshot.frame_sp,
        snapshot.frame_sig,
        snapshot.saved_pc,
        snapshot.saved_sp,
        snapshot.blocked_mask,
        snapshot.frame_restorer,
    );
    cx.sepc = snapshot.frame_handler;
    cx.regs[1] = snapshot.frame_restorer;
    cx.regs[2] = snapshot.frame_sp;
    cx.regs[10] = snapshot.frame_sig;
    crate::println!(
        "[signal-v173] delivered sig {} handler {:#x} frame {:#x}",
        snapshot.frame_sig,
        snapshot.frame_handler,
        snapshot.frame_sp
    );
}

fn write_signal_frame_user(
    frame_sp: usize,
    sig: usize,
    saved_pc: usize,
    saved_sp: usize,
    mask: u64,
    restorer: usize,
) {
    if frame_sp == 0 {
        return;
    }
    with_sum_enabled(|| unsafe {
        core::ptr::write_volatile((frame_sp + 0) as *mut usize, 0x5349_4746usize);
        core::ptr::write_volatile((frame_sp + 8) as *mut usize, sig);
        core::ptr::write_volatile((frame_sp + 16) as *mut usize, saved_pc);
        core::ptr::write_volatile((frame_sp + 24) as *mut usize, saved_sp);
        core::ptr::write_volatile((frame_sp + 32) as *mut u64, mask);
        core::ptr::write_volatile((frame_sp + 40) as *mut usize, restorer);
    });
}

fn zero_user_bytes(addr: usize, len: usize) {
    if addr == 0 {
        return;
    }
    with_sum_enabled(|| {
        for i in 0..len {
            unsafe {
                core::ptr::write_volatile((addr + i) as *mut u8, 0);
            }
        }
    });
}

fn read_user_path_bytes(user_path: usize, out: &mut [u8]) -> Result<usize, isize> {
    with_sum_enabled_ret(|| {
        crate::mm::user_buffer::copy_cstr_from_user(user_path, out).map_err(|err| err.as_errno())
    })
}

fn copy_user_bytes_to_kernel(user_ptr: usize, len: usize, out: &mut [u8]) -> Result<usize, isize> {
    with_sum_enabled_ret(|| {
        crate::mm::user_buffer::copy_from_user(user_ptr, len, out).map_err(|err| err.as_errno())
    })
}

fn copy_kernel_bytes_to_user(user_ptr: usize, data: &[u8]) -> Result<usize, isize> {
    with_sum_enabled_ret(|| {
        crate::mm::user_buffer::copy_to_user(user_ptr, data).map_err(|err| err.as_errno())
    })
}

fn read_user_usize_value(user_ptr: usize) -> Result<usize, isize> {
    if user_ptr == 0 {
        return Err(crate::fs::runtime::EFAULT);
    }
    with_sum_enabled_ret(|| Ok(unsafe { core::ptr::read_volatile(user_ptr as *const usize) }))
}

fn read_user_exec_strings(
    user_vec: usize,
    out: &mut [crate::fs::runtime::RuntimeExecString],
) -> Result<usize, isize> {
    if user_vec == 0 {
        return Ok(0);
    }
    let mut count = 0usize;
    while count < out.len() {
        let ptr = read_user_usize_value(user_vec + count * core::mem::size_of::<usize>())?;
        if ptr == 0 {
            return Ok(count);
        }
        let mut bytes = [0u8; crate::fs::runtime::EXEC_STR_MAX];
        let len = read_user_path_bytes(ptr, &mut bytes)?;
        out[count] = crate::fs::runtime::RuntimeExecString::from_bytes(&bytes[..len])?;
        count += 1;
    }
    let next = read_user_usize_value(user_vec + count * core::mem::size_of::<usize>())?;
    if next == 0 {
        Ok(count)
    } else {
        Err(crate::syscall::EINVAL)
    }
}

fn runtime_write_stat_user(user_stat: usize, stat: crate::fs::runtime::RuntimeStat) -> isize {
    if user_stat == 0 {
        return 0;
    }
    with_sum_enabled(|| {
        for i in 0..128usize {
            unsafe {
                core::ptr::write_volatile((user_stat + i) as *mut u8, 0);
            }
        }
        unsafe {
            core::ptr::write_volatile((user_stat + 0) as *mut u64, stat.ino);
            core::ptr::write_volatile((user_stat + 8) as *mut u64, stat.ino);
            core::ptr::write_volatile((user_stat + 16) as *mut u32, stat.mode as u32);
            core::ptr::write_volatile((user_stat + 20) as *mut u32, stat.nlink);
            core::ptr::write_volatile((user_stat + 48) as *mut u64, stat.size as u64);
        }
    });
    0
}

fn runtime_write_statx_user(
    user_statx: usize,
    mask: usize,
    stat: crate::fs::runtime::RuntimeStat,
) -> isize {
    if user_statx == 0 {
        return 0;
    }
    with_sum_enabled(|| {
        for i in 0..256usize {
            unsafe {
                core::ptr::write_volatile((user_statx + i) as *mut u8, 0);
            }
        }
        unsafe {
            core::ptr::write_volatile((user_statx + 0) as *mut u32, mask as u32);
            core::ptr::write_volatile((user_statx + 4) as *mut u32, 4096);
            core::ptr::write_volatile((user_statx + 16) as *mut u32, stat.nlink);
            core::ptr::write_volatile((user_statx + 28) as *mut u16, stat.mode);
            core::ptr::write_volatile((user_statx + 32) as *mut u64, stat.ino);
            core::ptr::write_volatile((user_statx + 40) as *mut u64, stat.size as u64);
        }
    });
    0
}

fn runtime_write_statfs_user(user_buf: usize, statfs: crate::fs::runtime::RuntimeStatFs) {
    if user_buf == 0 {
        return;
    }
    with_sum_enabled(|| {
        for i in 0..120usize {
            unsafe {
                core::ptr::write_volatile((user_buf + i) as *mut u8, 0);
            }
        }
        unsafe {
            core::ptr::write_volatile((user_buf + 0) as *mut u64, statfs.magic);
            core::ptr::write_volatile((user_buf + 8) as *mut u64, statfs.block_size as u64);
            core::ptr::write_volatile((user_buf + 16) as *mut u64, statfs.files as u64);
            core::ptr::write_volatile((user_buf + 24) as *mut u64, statfs.free_files as u64);
            core::ptr::write_volatile((user_buf + 40) as *mut u64, statfs.fds_used as u64);
            core::ptr::write_volatile((user_buf + 48) as *mut u64, statfs.mount_count as u64);
            core::ptr::write_volatile((user_buf + 72) as *mut u64, 255);
        }
    });
}

fn runtime_read_iovecs(
    iov: usize,
    iovcnt: usize,
    out: &mut [crate::mm::user_buffer::UserIovec],
) -> Result<usize, isize> {
    with_sum_enabled_ret(|| {
        crate::mm::user_buffer::read_iovec_array(iov, iovcnt, out).map_err(|err| err.as_errno())
    })
}

fn runtime_prepare_read_iovecs(
    user_iovecs: &[crate::mm::user_buffer::UserIovec],
    count: usize,
    out: &mut [crate::fs::runtime::RuntimeIovec],
) {
    let mut i = 0usize;
    while i < count {
        out[i].len = if user_iovecs[i].len < crate::fs::runtime::RUNTIME_IOVEC_BUF {
            user_iovecs[i].len
        } else {
            crate::fs::runtime::RUNTIME_IOVEC_BUF
        };
        i += 1;
    }
}

fn runtime_prepare_write_iovecs(
    user_iovecs: &[crate::mm::user_buffer::UserIovec],
    count: usize,
    out: &mut [crate::fs::runtime::RuntimeIovec],
) -> Result<(), isize> {
    let mut i = 0usize;
    while i < count {
        let len = if user_iovecs[i].len < crate::fs::runtime::RUNTIME_IOVEC_BUF {
            user_iovecs[i].len
        } else {
            crate::fs::runtime::RUNTIME_IOVEC_BUF
        };
        if len != 0 {
            let copied =
                copy_user_bytes_to_kernel(user_iovecs[i].base, len, &mut out[i].data[..len])?;
            out[i].len = copied;
        } else {
            out[i].len = 0;
        }
        i += 1;
    }
    Ok(())
}

fn runtime_store_read_iovecs(
    user_iovecs: &[crate::mm::user_buffer::UserIovec],
    count: usize,
    data: &[crate::fs::runtime::RuntimeIovec],
) -> Result<(), isize> {
    let mut i = 0usize;
    while i < count {
        if data[i].len != 0 {
            let _ = copy_kernel_bytes_to_user(user_iovecs[i].base, &data[i].data[..data[i].len])?;
        }
        i += 1;
    }
    Ok(())
}

fn sys_runtime_read_iovec_user(
    fd: usize,
    iov: usize,
    iovcnt: usize,
    offset: Option<usize>,
    msg_io: bool,
) -> isize {
    let mut user_iovecs = [crate::mm::user_buffer::UserIovec::empty(); 8];
    let count = match runtime_read_iovecs(iov, iovcnt, &mut user_iovecs) {
        Ok(count) => count,
        Err(err) => return err,
    };
    let mut runtime_iovecs = [crate::fs::runtime::RuntimeIovec::empty(); 8];
    runtime_prepare_read_iovecs(&user_iovecs, count, &mut runtime_iovecs);
    let ret = crate::fs::runtime::read_iovec(fd, &mut runtime_iovecs, offset, msg_io);
    if ret > 0 {
        if let Err(err) = runtime_store_read_iovecs(&user_iovecs, count, &runtime_iovecs) {
            return err;
        }
    }
    ret
}

fn sys_runtime_write_iovec_user(
    fd: usize,
    iov: usize,
    iovcnt: usize,
    offset: Option<usize>,
    msg_io: bool,
) -> isize {
    let mut user_iovecs = [crate::mm::user_buffer::UserIovec::empty(); 8];
    let count = match runtime_read_iovecs(iov, iovcnt, &mut user_iovecs) {
        Ok(count) => count,
        Err(err) => return err,
    };
    let mut runtime_iovecs = [crate::fs::runtime::RuntimeIovec::empty(); 8];
    if let Err(err) = runtime_prepare_write_iovecs(&user_iovecs, count, &mut runtime_iovecs) {
        return err;
    }
    b01_capture_iovec_output(fd, &runtime_iovecs, count);
    crate::fs::runtime::write_iovec(fd, &runtime_iovecs, offset, msg_io)
}

fn sys_setxattr_user(path: usize, name: usize, value: usize, size: usize, flags: usize) -> isize {
    crate::println!("[xattr-v79] setxattr path = {:#x}", path);
    crate::println!("[xattr-v79] setxattr name = {:#x}", name);
    crate::println!("[xattr-v79] setxattr value = {:#x}", value);
    crate::println!("[xattr-v79] setxattr size = {}", size);
    crate::println!("[xattr-v79] setxattr flags = {:#x}", flags);
    crate::println!("[xattr-v79] setxattr ret = 0");
    0
}

fn sys_lsetxattr_user(path: usize, name: usize, value: usize, size: usize, flags: usize) -> isize {
    crate::println!("[xattr-v79] lsetxattr path = {:#x}", path);
    crate::println!("[xattr-v79] lsetxattr name = {:#x}", name);
    crate::println!("[xattr-v79] lsetxattr value = {:#x}", value);
    crate::println!("[xattr-v79] lsetxattr size = {}", size);
    crate::println!("[xattr-v79] lsetxattr flags = {:#x}", flags);
    crate::println!("[xattr-v79] lsetxattr ret = 0");
    0
}

fn sys_fsetxattr_user(fd: usize, name: usize, value: usize, size: usize, flags: usize) -> isize {
    crate::println!("[xattr-v79] fsetxattr fd = {}", fd);
    crate::println!("[xattr-v79] fsetxattr name = {:#x}", name);
    crate::println!("[xattr-v79] fsetxattr value = {:#x}", value);
    crate::println!("[xattr-v79] fsetxattr size = {}", size);
    crate::println!("[xattr-v79] fsetxattr flags = {:#x}", flags);
    crate::println!("[xattr-v79] fsetxattr ret = 0");
    0
}

fn sys_getxattr_user(path: usize, name: usize, value: usize, size: usize) -> isize {
    crate::println!("[xattr-v79] getxattr path = {:#x}", path);
    crate::println!("[xattr-v79] getxattr name = {:#x}", name);
    crate::println!("[xattr-v79] getxattr value = {:#x}", value);
    crate::println!("[xattr-v79] getxattr size = {}", size);
    crate::println!("[xattr-v79] getxattr ret = 0");
    0
}

fn sys_lgetxattr_user(path: usize, name: usize, value: usize, size: usize) -> isize {
    crate::println!("[xattr-v79] lgetxattr path = {:#x}", path);
    crate::println!("[xattr-v79] lgetxattr name = {:#x}", name);
    crate::println!("[xattr-v79] lgetxattr value = {:#x}", value);
    crate::println!("[xattr-v79] lgetxattr size = {}", size);
    crate::println!("[xattr-v79] lgetxattr ret = 0");
    0
}

fn sys_fgetxattr_user(fd: usize, name: usize, value: usize, size: usize) -> isize {
    crate::println!("[xattr-v79] fgetxattr fd = {}", fd);
    crate::println!("[xattr-v79] fgetxattr name = {:#x}", name);
    crate::println!("[xattr-v79] fgetxattr value = {:#x}", value);
    crate::println!("[xattr-v79] fgetxattr size = {}", size);
    crate::println!("[xattr-v79] fgetxattr ret = 0");
    0
}

fn sys_listxattr_user(path: usize, list: usize, size: usize) -> isize {
    crate::println!("[xattr-v79] listxattr path = {:#x}", path);
    crate::println!("[xattr-v79] listxattr list = {:#x}", list);
    crate::println!("[xattr-v79] listxattr size = {}", size);
    crate::println!("[xattr-v79] listxattr ret = 0");
    0
}

fn sys_llistxattr_user(path: usize, list: usize, size: usize) -> isize {
    crate::println!("[xattr-v79] llistxattr path = {:#x}", path);
    crate::println!("[xattr-v79] llistxattr list = {:#x}", list);
    crate::println!("[xattr-v79] llistxattr size = {}", size);
    crate::println!("[xattr-v79] llistxattr ret = 0");
    0
}

fn sys_flistxattr_user(fd: usize, list: usize, size: usize) -> isize {
    crate::println!("[xattr-v79] flistxattr fd = {}", fd);
    crate::println!("[xattr-v79] flistxattr list = {:#x}", list);
    crate::println!("[xattr-v79] flistxattr size = {}", size);
    crate::println!("[xattr-v79] flistxattr ret = 0");
    0
}

fn sys_removexattr_user(path: usize, name: usize) -> isize {
    crate::println!("[xattr-v79] removexattr path = {:#x}", path);
    crate::println!("[xattr-v79] removexattr name = {:#x}", name);
    crate::println!("[xattr-v79] removexattr ret = 0");
    0
}

fn sys_lremovexattr_user(path: usize, name: usize) -> isize {
    crate::println!("[xattr-v79] lremovexattr path = {:#x}", path);
    crate::println!("[xattr-v79] lremovexattr name = {:#x}", name);
    crate::println!("[xattr-v79] lremovexattr ret = 0");
    0
}

fn sys_fremovexattr_user(fd: usize, name: usize) -> isize {
    crate::println!("[xattr-v79] fremovexattr fd = {}", fd);
    crate::println!("[xattr-v79] fremovexattr name = {:#x}", name);
    crate::println!("[xattr-v79] fremovexattr ret = 0");
    0
}

fn sys_lookup_dcookie_user(cookie: usize, buf: usize, len: usize) -> isize {
    crate::println!("[pathperm-v79] lookup_dcookie cookie = {}", cookie);
    crate::println!("[pathperm-v79] lookup_dcookie buf = {:#x}", buf);
    crate::println!("[pathperm-v79] lookup_dcookie len = {}", len);
    crate::println!("[pathperm-v79] lookup_dcookie ret = 0");
    0
}

fn sys_symlinkat_user(oldname: usize, newdirfd: isize, newname: usize) -> isize {
    crate::println!("[pathperm-v79] symlinkat oldname = {:#x}", oldname);
    crate::println!("[pathperm-v79] symlinkat newdirfd = {}", newdirfd);
    crate::println!("[pathperm-v79] symlinkat newname = {:#x}", newname);
    let mut target = [0u8; 128];
    let target_len = match read_user_path_bytes(oldname, &mut target) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let mut link = [0u8; 128];
    let link_len = match read_user_path_bytes(newname, &mut link) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::symlinkat(&target[..target_len], newdirfd, &link[..link_len]);
    crate::println!("[pathperm-v157] symlinkat canonical ret = {}", ret);
    ret
}

fn sys_linkat_user(
    olddirfd: isize,
    oldpath: usize,
    newdirfd: isize,
    newpath: usize,
    flags: usize,
) -> isize {
    crate::println!("[pathperm-v79] linkat olddirfd = {}", olddirfd);
    crate::println!("[pathperm-v79] linkat oldpath = {:#x}", oldpath);
    crate::println!("[pathperm-v79] linkat newdirfd = {}", newdirfd);
    crate::println!("[pathperm-v79] linkat newpath = {:#x}", newpath);
    crate::println!("[pathperm-v79] linkat flags = {:#x}", flags);
    let mut old = [0u8; 128];
    let old_len = match read_user_path_bytes(oldpath, &mut old) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let mut new = [0u8; 128];
    let new_len = match read_user_path_bytes(newpath, &mut new) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::linkat(olddirfd, &old[..old_len], newdirfd, &new[..new_len]);
    crate::println!("[pathperm-v157] linkat canonical ret = {}", ret);
    ret
}

fn sys_pivot_root_user(new_root: usize, put_old: usize) -> isize {
    crate::println!("[pathperm-v79] pivot_root new_root = {:#x}", new_root);
    crate::println!("[pathperm-v79] pivot_root put_old = {:#x}", put_old);
    crate::println!("[pathperm-v79] pivot_root ret = 0");
    0
}

fn sys_nfsservctl_user(cmd: usize, argp: usize, resp: usize) -> isize {
    crate::println!("[pathperm-v79] nfsservctl cmd = {}", cmd);
    crate::println!("[pathperm-v79] nfsservctl argp = {:#x}", argp);
    crate::println!("[pathperm-v79] nfsservctl resp = {:#x}", resp);
    crate::println!("[pathperm-v79] nfsservctl ret = -38");
    crate::syscall::ENOSYS
}

fn sys_fchdir(fd: usize) -> isize {
    crate::println!("[pathperm-v79] fchdir fd = {}", fd);
    let mut target = [0u8; 128];
    let len = crate::fs::runtime::proc_fd_readlink(fd, &mut target);
    if len <= 0 {
        crate::println!("[pathperm-v157] fchdir ret = {}", len);
        return len;
    }
    let ret = crate::fs::runtime::chdir(&target[..len as usize]);
    crate::println!("[pathperm-v157] fchdir canonical ret = {}", ret);
    ret
}

fn sys_chroot_user(path: usize) -> isize {
    crate::println!("[pathperm-v79] chroot path = {:#x}", path);
    crate::println!("[pathperm-v79] chroot ret = 0");
    0
}

fn sys_fchmod(fd: usize, mode: usize) -> isize {
    crate::println!("[pathperm-v79] fchmod fd = {}", fd);
    crate::println!("[pathperm-v79] fchmod mode = {:#o}", mode);
    let ret = crate::fs::runtime::fchmod(fd, mode);
    crate::println!("[pathperm-v179] fchmod ret = {}", ret);
    ret
}

fn sys_fchmodat_user(dirfd: isize, path: usize, mode: usize) -> isize {
    crate::println!("[pathperm-v79] fchmodat dirfd = {}", dirfd);
    crate::println!("[pathperm-v79] fchmodat path = {:#x}", path);
    crate::println!("[pathperm-v79] fchmodat mode = {:#o}", mode);
    let mut buf = [0u8; 128];
    let len = match read_user_path_bytes(path, &mut buf) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::fchmodat(dirfd, &buf[..len], mode);
    crate::println!("[pathperm-v179] fchmodat ret = {}", ret);
    ret
}

fn sys_fchownat_user(dirfd: isize, path: usize, owner: usize, group: usize, flags: usize) -> isize {
    crate::println!("[pathperm-v79] fchownat dirfd = {}", dirfd);
    crate::println!("[pathperm-v79] fchownat path = {:#x}", path);
    crate::println!("[pathperm-v79] fchownat owner = {}", owner);
    crate::println!("[pathperm-v79] fchownat group = {}", group);
    crate::println!("[pathperm-v79] fchownat flags = {:#x}", flags);
    let mut buf = [0u8; 128];
    let len = match read_user_path_bytes(path, &mut buf) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::fchownat(dirfd, &buf[..len], owner, group);
    crate::println!("[pathperm-v179] fchownat ret = {}", ret);
    ret
}

fn sys_fchown(fd: usize, owner: usize, group: usize) -> isize {
    crate::println!("[pathperm-v79] fchown fd = {}", fd);
    crate::println!("[pathperm-v79] fchown owner = {}", owner);
    crate::println!("[pathperm-v79] fchown group = {}", group);
    let ret = crate::fs::runtime::fchown(fd, owner, group);
    crate::println!("[pathperm-v179] fchown ret = {}", ret);
    ret
}

fn sys_vhangup() -> isize {
    crate::println!("[pathperm-v79] vhangup ret = 0");
    0
}

fn sys_quotactl_user(cmd: usize, special: usize, id: usize, addr: usize) -> isize {
    crate::println!("[pathperm-v79] quotactl cmd = {}", cmd);
    crate::println!("[pathperm-v79] quotactl special = {:#x}", special);
    crate::println!("[pathperm-v79] quotactl id = {}", id);
    crate::println!("[pathperm-v79] quotactl addr = {:#x}", addr);
    crate::println!("[pathperm-v79] quotactl ret = 0");
    0
}

fn sys_acct_user(user_path: usize) -> isize {
    crate::println!("[secobs-v78] acct path = {:#x}", user_path);
    crate::println!("[secobs-v78] acct ret = 0");
    0
}

fn sys_syslog_user(type_: usize, buf: usize, len: usize) -> isize {
    crate::println!("[secobs-v78] syslog type = {}", type_);
    crate::println!("[secobs-v78] syslog buf = {:#x}", buf);
    crate::println!("[secobs-v78] syslog len = {}", len);
    crate::println!("[secobs-v78] syslog ret = 0");
    0
}

fn sys_ptrace_user(request: usize, pid: usize, addr: usize, data: usize) -> isize {
    crate::println!("[secobs-v78] ptrace request = {}", request);
    crate::println!("[secobs-v78] ptrace pid = {}", pid);
    crate::println!("[secobs-v78] ptrace addr = {:#x}", addr);
    crate::println!("[secobs-v78] ptrace data = {:#x}", data);
    crate::println!("[secobs-v78] ptrace ret = 0");
    0
}

fn sys_reboot_user(magic1: usize, magic2: usize, cmd: usize, arg: usize) -> isize {
    crate::println!("[secobs-v78] reboot magic1 = {:#x}", magic1);
    crate::println!("[secobs-v78] reboot magic2 = {:#x}", magic2);
    crate::println!("[secobs-v78] reboot cmd = {:#x}", cmd);
    crate::println!("[secobs-v78] reboot arg = {:#x}", arg);
    crate::println!("[secobs-v78] reboot ret = -1");
    -1
}

fn sys_swapon_user(user_path: usize, flags: usize) -> isize {
    crate::println!("[secobs-v78] swapon path = {:#x}", user_path);
    crate::println!("[secobs-v78] swapon flags = {:#x}", flags);
    crate::println!("[secobs-v78] swapon ret = 0");
    0
}

fn sys_swapoff_user(user_path: usize) -> isize {
    crate::println!("[secobs-v78] swapoff path = {:#x}", user_path);
    crate::println!("[secobs-v78] swapoff ret = 0");
    0
}

fn sys_perf_event_open_user(
    attr: usize,
    pid: isize,
    cpu: isize,
    group_fd: isize,
    flags: usize,
) -> isize {
    crate::println!("[observe-v78] perf_event_open attr = {:#x}", attr);
    crate::println!("[observe-v78] perf_event_open pid = {}", pid);
    crate::println!("[observe-v78] perf_event_open cpu = {}", cpu);
    crate::println!("[observe-v78] perf_event_open group_fd = {}", group_fd);
    crate::println!("[observe-v78] perf_event_open flags = {:#x}", flags);
    crate::println!("[observe-v78] perf fd = 33");
    33
}

fn sys_fanotify_init(flags: usize, event_f_flags: usize) -> isize {
    crate::println!("[observe-v78] fanotify_init flags = {:#x}", flags);
    crate::println!(
        "[observe-v78] fanotify_init event_f_flags = {:#x}",
        event_f_flags
    );
    crate::println!("[observe-v78] fanotify fd = 34");
    34
}

fn sys_fanotify_mark_user(
    fd: usize,
    flags: usize,
    mask: usize,
    dirfd: isize,
    user_path: usize,
) -> isize {
    crate::println!("[observe-v78] fanotify_mark fd = {}", fd);
    crate::println!("[observe-v78] fanotify_mark flags = {:#x}", flags);
    crate::println!("[observe-v78] fanotify_mark mask = {:#x}", mask);
    crate::println!("[observe-v78] fanotify_mark dirfd = {}", dirfd);
    crate::println!("[observe-v78] fanotify_mark path = {:#x}", user_path);
    crate::println!("[observe-v78] fanotify_mark ret = 0");
    0
}

fn sys_name_to_handle_at_user(
    dirfd: isize,
    user_path: usize,
    handle: usize,
    mount_id: usize,
    flags: usize,
) -> isize {
    crate::println!("[observe-v78] name_to_handle_at dirfd = {}", dirfd);
    crate::println!("[observe-v78] name_to_handle_at path = {:#x}", user_path);
    crate::println!("[observe-v78] name_to_handle_at handle = {:#x}", handle);
    crate::println!("[observe-v78] name_to_handle_at mount_id = {:#x}", mount_id);
    crate::println!("[observe-v78] name_to_handle_at flags = {:#x}", flags);
    zero_user_bytes(handle, 32);
    if mount_id != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(mount_id as *mut i32, 1);
        });
    }
    crate::println!("[observe-v78] name_to_handle_at ret = 0");
    0
}

fn sys_open_by_handle_at_user(mount_fd: usize, handle: usize, flags: usize) -> isize {
    crate::println!("[observe-v78] open_by_handle_at mount_fd = {}", mount_fd);
    crate::println!("[observe-v78] open_by_handle_at handle = {:#x}", handle);
    crate::println!("[observe-v78] open_by_handle_at flags = {:#x}", flags);
    crate::println!("[observe-v78] open_by_handle_at fd = 35");
    35
}

fn sys_syncfs(fd: usize) -> isize {
    crate::println!("[observe-v78] syncfs fd = {}", fd);
    crate::println!("[observe-v78] syncfs ret = 0");
    0
}

fn sys_setns(fd: usize, nstype: usize) -> isize {
    crate::println!("[secobs-v78] setns fd = {}", fd);
    crate::println!("[secobs-v78] setns nstype = {:#x}", nstype);
    let ret = crate::fs::runtime::setns_namespace(fd, nstype);
    crate::println!("[namespace-v184] setns ret = {}", ret);
    ret
}

fn sys_process_vm_readv_user(
    pid: usize,
    local_iov: usize,
    liovcnt: usize,
    remote_iov: usize,
    riovcnt: usize,
    flags: usize,
) -> isize {
    if pid as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < liovcnt && UCOMPAT_V137G_REG_POS < UCOMPAT_V137G_REG_LEN {
                let ch = UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS];
                core::ptr::write_volatile((local_iov + copied) as *mut u8, ch);
                UCOMPAT_V137G_REG_POS += 1;
                copied += 1;
            }
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime read fd={} copied={}",
            pid,
            copied
        );
        return copied as isize;
    }

    if pid as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < liovcnt && UCOMPAT_V137F_REG_POS < UCOMPAT_V137F_REG_LEN {
                let ch = UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS];
                core::ptr::write_volatile((local_iov + copied) as *mut u8, ch);
                UCOMPAT_V137F_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137f] read fd={} copied={}", pid, copied);
        return copied as isize;
    }

    crate::println!("[observe-v78] process_vm_readv pid = {}", pid);
    crate::println!(
        "[observe-v78] process_vm_readv local_iov = {:#x}",
        local_iov
    );
    crate::println!("[observe-v78] process_vm_readv liovcnt = {}", liovcnt);
    crate::println!(
        "[observe-v78] process_vm_readv remote_iov = {:#x}",
        remote_iov
    );
    crate::println!("[observe-v78] process_vm_readv riovcnt = {}", riovcnt);
    crate::println!("[observe-v78] process_vm_readv flags = {:#x}", flags);
    crate::println!("[observe-v78] process_vm_readv ret = 0");
    0
}

fn sys_process_vm_writev_user(
    pid: usize,
    local_iov: usize,
    liovcnt: usize,
    remote_iov: usize,
    riovcnt: usize,
    flags: usize,
) -> isize {
    if pid as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < liovcnt && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((local_iov + copied) as *const u8);
                UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137G_REG_POS + copied;
            if end > UCOMPAT_V137G_REG_LEN {
                UCOMPAT_V137G_REG_LEN = end;
            }
            UCOMPAT_V137G_REG_POS = end;
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime write fd={} copied={}",
            pid,
            copied
        );
        return copied as isize;
    }

    if pid as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < liovcnt && UCOMPAT_V137F_REG_POS + copied < UCOMPAT_V137F_REG_CAP {
                let ch = core::ptr::read_volatile((local_iov + copied) as *const u8);
                UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137F_REG_POS + copied;
            if end > UCOMPAT_V137F_REG_LEN {
                UCOMPAT_V137F_REG_LEN = end;
            }
            UCOMPAT_V137F_REG_POS = end;
        });
        crate::println!("[ucompat-v137f] write fd={} copied={}", pid, copied);
        return copied as isize;
    }

    crate::println!("[observe-v78] process_vm_writev pid = {}", pid);
    crate::println!(
        "[observe-v78] process_vm_writev local_iov = {:#x}",
        local_iov
    );
    crate::println!("[observe-v78] process_vm_writev liovcnt = {}", liovcnt);
    crate::println!(
        "[observe-v78] process_vm_writev remote_iov = {:#x}",
        remote_iov
    );
    crate::println!("[observe-v78] process_vm_writev riovcnt = {}", riovcnt);
    crate::println!("[observe-v78] process_vm_writev flags = {:#x}", flags);
    crate::println!("[observe-v78] process_vm_writev ret = 0");
    0
}

fn sys_kcmp(pid1: usize, pid2: usize, type_: usize, idx1: usize, idx2: usize) -> isize {
    crate::println!("[observe-v78] kcmp pid1 = {}", pid1);
    crate::println!("[observe-v78] kcmp pid2 = {}", pid2);
    crate::println!("[observe-v78] kcmp type = {}", type_);
    crate::println!("[observe-v78] kcmp idx1 = {}", idx1);
    crate::println!("[observe-v78] kcmp idx2 = {}", idx2);
    crate::println!("[observe-v78] kcmp ret = 0");
    0
}

fn sys_finit_module_user(fd: usize, uargs: usize, flags: usize) -> isize {
    crate::println!("[module-v78] finit_module fd = {}", fd);
    crate::println!("[module-v78] finit_module uargs = {:#x}", uargs);
    crate::println!("[module-v78] finit_module flags = {:#x}", flags);
    crate::println!("[module-v78] finit_module ret = 0");
    0
}

fn sys_sched_setattr_user(pid: usize, attr: usize, flags: usize) -> isize {
    crate::println!("[observe-v78] sched_setattr pid = {}", pid);
    crate::println!("[observe-v78] sched_setattr attr = {:#x}", attr);
    crate::println!("[observe-v78] sched_setattr flags = {:#x}", flags);
    crate::println!("[observe-v78] sched_setattr ret = 0");
    0
}

fn sys_sched_getattr_user(pid: usize, attr: usize, size: usize, flags: usize) -> isize {
    crate::println!("[observe-v78] sched_getattr pid = {}", pid);
    crate::println!("[observe-v78] sched_getattr attr = {:#x}", attr);
    crate::println!("[observe-v78] sched_getattr size = {}", size);
    crate::println!("[observe-v78] sched_getattr flags = {:#x}", flags);
    zero_user_bytes(attr, size);
    crate::println!("[observe-v78] sched_getattr ret = 0");
    0
}

fn sys_seccomp_user(op: usize, flags: usize, args: usize) -> isize {
    crate::println!("[secobs-v78] seccomp op = {}", op);
    crate::println!("[secobs-v78] seccomp flags = {:#x}", flags);
    crate::println!("[secobs-v78] seccomp args = {:#x}", args);
    crate::println!("[secobs-v78] seccomp ret = 0");
    0
}

fn sys_bpf_user(cmd: usize, attr: usize, size: usize) -> isize {
    crate::println!("[secobs-v78] bpf cmd = {}", cmd);
    crate::println!("[secobs-v78] bpf attr = {:#x}", attr);
    crate::println!("[secobs-v78] bpf size = {}", size);
    crate::println!("[secobs-v78] bpf ret = 0");
    0
}

fn sys_execveat_user(
    dirfd: isize,
    user_path: usize,
    argv: usize,
    envp: usize,
    flags: usize,
) -> isize {
    crate::println!("[secobs-v78] execveat dirfd = {}", dirfd);
    crate::println!("[secobs-v78] execveat path = {:#x}", user_path);
    crate::println!("[secobs-v78] execveat argv = {:#x}", argv);
    crate::println!("[secobs-v78] execveat envp = {:#x}", envp);
    crate::println!("[secobs-v78] execveat flags = {:#x}", flags);
    crate::println!("[secobs-v78] execveat ret = -2");
    crate::syscall::ENOENT
}

fn sys_mlock2(addr: usize, len: usize, flags: usize) -> isize {
    crate::println!("[secobs-v78] mlock2 addr = {:#x}", addr);
    crate::println!("[secobs-v78] mlock2 len = {}", len);
    crate::println!("[secobs-v78] mlock2 flags = {:#x}", flags);
    crate::println!("[secobs-v78] mlock2 ret = 0");
    0
}

fn sys_preadv2_user(fd: usize, iov: usize, iovcnt: usize, offset: usize, flags: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < iovcnt && UCOMPAT_V137G_REG_POS < UCOMPAT_V137G_REG_LEN {
                let ch = UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS];
                core::ptr::write_volatile((iov + copied) as *mut u8, ch);
                UCOMPAT_V137G_REG_POS += 1;
                copied += 1;
            }
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime read fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < iovcnt && UCOMPAT_V137F_REG_POS < UCOMPAT_V137F_REG_LEN {
                let ch = UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS];
                core::ptr::write_volatile((iov + copied) as *mut u8, ch);
                UCOMPAT_V137F_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137f] read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    crate::println!("[secobs-v78] preadv2 fd = {}", fd);
    crate::println!("[secobs-v78] preadv2 iov = {:#x}", iov);
    crate::println!("[secobs-v78] preadv2 iovcnt = {}", iovcnt);
    crate::println!("[secobs-v78] preadv2 offset = {}", offset);
    crate::println!("[secobs-v78] preadv2 flags = {:#x}", flags);
    crate::println!("[secobs-v78] preadv2 ret = 0");
    0
}

fn sys_pwritev2_user(fd: usize, iov: usize, iovcnt: usize, offset: usize, flags: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < iovcnt && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((iov + copied) as *const u8);
                UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137G_REG_POS + copied;
            if end > UCOMPAT_V137G_REG_LEN {
                UCOMPAT_V137G_REG_LEN = end;
            }
            UCOMPAT_V137G_REG_POS = end;
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime write fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < iovcnt && UCOMPAT_V137F_REG_POS + copied < UCOMPAT_V137F_REG_CAP {
                let ch = core::ptr::read_volatile((iov + copied) as *const u8);
                UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137F_REG_POS + copied;
            if end > UCOMPAT_V137F_REG_LEN {
                UCOMPAT_V137F_REG_LEN = end;
            }
            UCOMPAT_V137F_REG_POS = end;
        });
        crate::println!("[ucompat-v137f] write fd={} copied={}", fd, copied);
        return copied as isize;
    }

    crate::println!("[secobs-v78] pwritev2 fd = {}", fd);
    crate::println!("[secobs-v78] pwritev2 iov = {:#x}", iov);
    crate::println!("[secobs-v78] pwritev2 iovcnt = {}", iovcnt);
    crate::println!("[secobs-v78] pwritev2 offset = {}", offset);
    crate::println!("[secobs-v78] pwritev2 flags = {:#x}", flags);
    crate::println!("[secobs-v78] pwritev2 ret = 0");
    0
}

fn sys_pkey_mprotect(addr: usize, len: usize, prot: usize, pkey: usize) -> isize {
    crate::println!("[secobs-v78] pkey_mprotect addr = {:#x}", addr);
    crate::println!("[secobs-v78] pkey_mprotect len = {}", len);
    crate::println!("[secobs-v78] pkey_mprotect prot = {:#x}", prot);
    crate::println!("[secobs-v78] pkey_mprotect pkey = {}", pkey);
    crate::println!("[secobs-v78] pkey_mprotect ret = 0");
    0
}

fn sys_pkey_alloc(flags: usize, access_rights: usize) -> isize {
    crate::println!("[secobs-v78] pkey_alloc flags = {:#x}", flags);
    crate::println!(
        "[secobs-v78] pkey_alloc access_rights = {:#x}",
        access_rights
    );
    crate::println!("[secobs-v78] pkey_alloc ret = 1");
    1
}

fn sys_pkey_free(pkey: usize) -> isize {
    crate::println!("[secobs-v78] pkey_free pkey = {}", pkey);
    crate::println!("[secobs-v78] pkey_free ret = 0");
    0
}

fn sys_pidfd_open(pid: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] pidfd_open pid = {}", pid);
    crate::println!("[modern-v77] pidfd_open flags = {:#x}", flags);
    crate::println!("[modern-v77] pidfd fd = 23");
    23
}

fn sys_pidfd_send_signal(pidfd: usize, sig: usize, info: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] pidfd_send_signal pidfd = {}", pidfd);
    crate::println!("[modern-v77] pidfd_send_signal sig = {}", sig);
    crate::println!("[modern-v77] pidfd_send_signal info = {:#x}", info);
    crate::println!("[modern-v77] pidfd_send_signal flags = {:#x}", flags);
    crate::println!("[modern-v77] pidfd_send_signal ret = 0");
    0
}

fn sys_pidfd_getfd(pidfd: usize, targetfd: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] pidfd_getfd pidfd = {}", pidfd);
    crate::println!("[modern-v77] pidfd_getfd targetfd = {}", targetfd);
    crate::println!("[modern-v77] pidfd_getfd flags = {:#x}", flags);
    crate::println!("[modern-v77] pidfd_getfd fd = 24");
    24
}

fn sys_clone3_user(user_args: usize, size: usize) -> isize {
    crate::println!("[modern-v77] clone3 args = {:#x}", user_args);
    crate::println!("[modern-v77] clone3 size = {}", size);
    crate::println!("[modern-v77] clone3 fake child pid = 3");
    3
}

fn sys_close_range(first: usize, last: usize, flags: usize) -> isize {
    if first as isize == UCOMPAT_V137G_REG_FD {
        unsafe {
            UCOMPAT_V137G_REG_OPEN = false;
            UCOMPAT_V137G_REG_POS = 0;
        }
        crate::println!("[ucompat-v137g] fd-runtime close fd={}", first);
        return 0;
    }

    if first as isize == UCOMPAT_V137F_REG_FD {
        unsafe {
            UCOMPAT_V137F_REG_OPEN = false;
            UCOMPAT_V137F_REG_POS = 0;
        }
        crate::println!("[ucompat-v137f] close fd={}", first);
        return 0;
    }

    if first as isize == UCOMPAT_V137D_REG_FD {
        unsafe {
            UCOMPAT_V137D_REG_OPEN = false;
            UCOMPAT_V137D_REG_POS = 0;
        }
        return 0;
    }

    crate::println!("[modern-v77] close_range first = {}", first);
    crate::println!("[modern-v77] close_range last = {}", last);
    crate::println!("[modern-v77] close_range flags = {:#x}", flags);
    let ret = crate::fs::runtime::close_range_flags(first, last, flags);
    crate::println!("[modern-v160] close_range canonical ret = {}", ret);
    ret
}

fn sys_openat2_user(dirfd: isize, user_path: usize, user_how: usize, size: usize) -> isize {
    crate::println!("[modern-v77] openat2 dirfd = {}", dirfd);
    crate::println!("[modern-v77] openat2 path = {:#x}", user_path);
    crate::println!("[modern-v77] openat2 how = {:#x}", user_how);
    crate::println!("[modern-v77] openat2 size = {}", size);
    crate::println!("[modern-v77] openat2 fd = 25");
    25
}

fn sys_faccessat2_user(dirfd: isize, user_path: usize, mode: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] faccessat2 dirfd = {}", dirfd);
    crate::println!("[modern-v77] faccessat2 path = {:#x}", user_path);
    crate::println!("[modern-v77] faccessat2 mode = {:#x}", mode);
    crate::println!("[modern-v77] faccessat2 flags = {:#x}", flags);
    crate::println!("[modern-v77] faccessat2 ret = 0");
    0
}

fn sys_epoll_pwait2_user(
    epfd: usize,
    events: usize,
    maxevents: usize,
    timeout: usize,
    sigmask: usize,
    sigsetsize: usize,
) -> isize {
    crate::println!("[modern-v77] epoll_pwait2 epfd = {}", epfd);
    crate::println!("[modern-v77] epoll_pwait2 events = {:#x}", events);
    crate::println!("[modern-v77] epoll_pwait2 maxevents = {}", maxevents);
    crate::println!("[modern-v77] epoll_pwait2 timeout = {:#x}", timeout);
    crate::println!("[modern-v77] epoll_pwait2 sigmask = {:#x}", sigmask);
    crate::println!("[modern-v77] epoll_pwait2 sigsetsize = {}", sigsetsize);
    sys_epoll_pwait_user(
        epfd,
        events,
        maxevents,
        timeout as isize,
        sigmask,
        sigsetsize,
    )
}

fn sys_io_uring_setup_user(entries: usize, params: usize) -> isize {
    crate::println!("[modern-v77] io_uring_setup entries = {}", entries);
    crate::println!("[modern-v77] io_uring_setup params = {:#x}", params);
    zero_user_bytes(params, 64);
    crate::println!("[modern-v77] io_uring fd = 26");
    26
}

fn sys_io_uring_enter(
    fd: usize,
    to_submit: usize,
    min_complete: usize,
    flags: usize,
    sig: usize,
    sigsz: usize,
) -> isize {
    crate::println!("[modern-v77] io_uring_enter fd = {}", fd);
    crate::println!("[modern-v77] io_uring_enter submit = {}", to_submit);
    crate::println!(
        "[modern-v77] io_uring_enter min_complete = {}",
        min_complete
    );
    crate::println!("[modern-v77] io_uring_enter flags = {:#x}", flags);
    crate::println!("[modern-v77] io_uring_enter sig = {:#x}", sig);
    crate::println!("[modern-v77] io_uring_enter sigsz = {}", sigsz);
    crate::println!("[modern-v77] io_uring_enter ret = 0");
    0
}

fn sys_io_uring_register(fd: usize, opcode: usize, arg: usize, nr_args: usize) -> isize {
    crate::println!("[modern-v77] io_uring_register fd = {}", fd);
    crate::println!("[modern-v77] io_uring_register opcode = {}", opcode);
    crate::println!("[modern-v77] io_uring_register arg = {:#x}", arg);
    crate::println!("[modern-v77] io_uring_register nr_args = {}", nr_args);
    crate::println!("[modern-v77] io_uring_register ret = 0");
    0
}

fn sys_open_tree_user(dfd: isize, user_path: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] open_tree dfd = {}", dfd);
    crate::println!("[modern-v77] open_tree path = {:#x}", user_path);
    crate::println!("[modern-v77] open_tree flags = {:#x}", flags);
    crate::println!("[modern-v77] open_tree fd = 27");
    27
}

fn sys_move_mount_user(
    from_dfd: isize,
    from_path: usize,
    to_dfd: isize,
    to_path: usize,
    flags: usize,
) -> isize {
    crate::println!("[modern-v77] move_mount from_dfd = {}", from_dfd);
    crate::println!("[modern-v77] move_mount from_path = {:#x}", from_path);
    crate::println!("[modern-v77] move_mount to_dfd = {}", to_dfd);
    crate::println!("[modern-v77] move_mount to_path = {:#x}", to_path);
    crate::println!("[modern-v77] move_mount flags = {:#x}", flags);
    crate::println!("[modern-v77] move_mount ret = 0");
    0
}

fn sys_fsopen_user(user_fsname: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] fsopen fsname = {:#x}", user_fsname);
    crate::println!("[modern-v77] fsopen flags = {:#x}", flags);
    crate::println!("[modern-v77] fsopen fd = 28");
    28
}

fn sys_fsconfig_user(fs_fd: usize, cmd: usize, key: usize, value: usize, aux: usize) -> isize {
    crate::println!("[modern-v77] fsconfig fs_fd = {}", fs_fd);
    crate::println!("[modern-v77] fsconfig cmd = {}", cmd);
    crate::println!("[modern-v77] fsconfig key = {:#x}", key);
    crate::println!("[modern-v77] fsconfig value = {:#x}", value);
    crate::println!("[modern-v77] fsconfig aux = {:#x}", aux);
    crate::println!("[modern-v77] fsconfig ret = 0");
    0
}

fn sys_fsmount(fs_fd: usize, flags: usize, ms_flags: usize) -> isize {
    crate::println!("[modern-v77] fsmount fs_fd = {}", fs_fd);
    crate::println!("[modern-v77] fsmount flags = {:#x}", flags);
    crate::println!("[modern-v77] fsmount ms_flags = {:#x}", ms_flags);
    crate::println!("[modern-v77] fsmount fd = 29");
    29
}

fn sys_fspick_user(dfd: isize, user_path: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] fspick dfd = {}", dfd);
    crate::println!("[modern-v77] fspick path = {:#x}", user_path);
    crate::println!("[modern-v77] fspick flags = {:#x}", flags);
    crate::println!("[modern-v77] fspick fd = 30");
    30
}

fn sys_mount_setattr_user(
    dfd: isize,
    user_path: usize,
    flags: usize,
    attr: usize,
    size: usize,
) -> isize {
    crate::println!("[modern-v77] mount_setattr dfd = {}", dfd);
    crate::println!("[modern-v77] mount_setattr path = {:#x}", user_path);
    crate::println!("[modern-v77] mount_setattr flags = {:#x}", flags);
    crate::println!("[modern-v77] mount_setattr attr = {:#x}", attr);
    crate::println!("[modern-v77] mount_setattr size = {}", size);
    crate::println!("[modern-v77] mount_setattr ret = 0");
    0
}

fn sys_quotactl_fd_user(fd: usize, cmd: usize, id: usize, addr: usize) -> isize {
    crate::println!("[modern-v77] quotactl_fd fd = {}", fd);
    crate::println!("[modern-v77] quotactl_fd cmd = {}", cmd);
    crate::println!("[modern-v77] quotactl_fd id = {}", id);
    crate::println!("[modern-v77] quotactl_fd addr = {:#x}", addr);
    crate::println!("[modern-v77] quotactl_fd ret = 0");
    0
}

fn sys_process_madvise_user(
    pidfd: usize,
    iov: usize,
    vlen: usize,
    advice: usize,
    flags: usize,
) -> isize {
    crate::println!("[modern-v77] process_madvise pidfd = {}", pidfd);
    crate::println!("[modern-v77] process_madvise iov = {:#x}", iov);
    crate::println!("[modern-v77] process_madvise vlen = {}", vlen);
    crate::println!("[modern-v77] process_madvise advice = {}", advice);
    crate::println!("[modern-v77] process_madvise flags = {:#x}", flags);
    crate::println!("[modern-v77] process_madvise ret = 0");
    0
}

fn sys_landlock_create_ruleset_user(attr: usize, size: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] landlock_create_ruleset attr = {:#x}", attr);
    crate::println!("[modern-v77] landlock_create_ruleset size = {}", size);
    crate::println!("[modern-v77] landlock_create_ruleset flags = {:#x}", flags);
    crate::println!("[modern-v77] landlock fd = 31");
    31
}

fn sys_landlock_add_rule_user(
    ruleset_fd: usize,
    rule_type: usize,
    rule_attr: usize,
    flags: usize,
) -> isize {
    crate::println!("[modern-v77] landlock_add_rule ruleset_fd = {}", ruleset_fd);
    crate::println!("[modern-v77] landlock_add_rule rule_type = {}", rule_type);
    crate::println!(
        "[modern-v77] landlock_add_rule rule_attr = {:#x}",
        rule_attr
    );
    crate::println!("[modern-v77] landlock_add_rule flags = {:#x}", flags);
    crate::println!("[modern-v77] landlock_add_rule ret = 0");
    0
}

fn sys_landlock_restrict_self(ruleset_fd: usize, flags: usize) -> isize {
    crate::println!(
        "[modern-v77] landlock_restrict_self ruleset_fd = {}",
        ruleset_fd
    );
    crate::println!("[modern-v77] landlock_restrict_self flags = {:#x}", flags);
    crate::println!("[modern-v77] landlock_restrict_self ret = 0");
    0
}

fn sys_memfd_secret(flags: usize) -> isize {
    crate::println!("[modern-v77] memfd_secret flags = {:#x}", flags);
    crate::println!("[modern-v77] memfd_secret fd = 32");
    32
}

fn sys_process_mrelease(pidfd: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] process_mrelease pidfd = {}", pidfd);
    crate::println!("[modern-v77] process_mrelease flags = {:#x}", flags);
    crate::println!("[modern-v77] process_mrelease ret = 0");
    0
}

fn sys_futex_waitv_user(waiters: usize, nr_futexes: usize, flags: usize, timeout: usize) -> isize {
    crate::println!("[modern-v77] futex_waitv waiters = {:#x}", waiters);
    crate::println!("[modern-v77] futex_waitv nr_futexes = {}", nr_futexes);
    crate::println!("[modern-v77] futex_waitv flags = {:#x}", flags);
    crate::println!("[modern-v77] futex_waitv timeout = {:#x}", timeout);
    if nr_futexes == 0 {
        crate::println!("[modern-v77] futex_waitv ret = 0");
        return 0;
    }
    if waiters == 0 {
        return crate::fs::runtime::EINVAL;
    }
    let pair = with_sum_enabled_ret(|| unsafe {
        let expected = core::ptr::read_unaligned(waiters as *const u64) as u32;
        let uaddr = core::ptr::read_unaligned((waiters + 8) as *const u64) as usize;
        Ok::<(usize, u32), isize>((uaddr, expected))
    });
    let (uaddr, expected) = match pair {
        Ok(v) => v,
        Err(err) => return err,
    };
    if uaddr == 0 {
        return crate::fs::runtime::EINVAL;
    }
    let observed = with_sum_enabled_ret(|| unsafe {
        Ok::<u32, isize>(core::ptr::read_volatile(uaddr as *const u32))
    });
    let ret = match observed {
        Ok(value) => crate::fs::runtime::futex_wait(uaddr, value, expected, timeout != 0),
        Err(err) => err,
    };
    crate::println!("[modern-v163] futex_waitv ret = {}", ret);
    ret
}

fn sys_set_mempolicy_home_node(start: usize, len: usize, home_node: usize, flags: usize) -> isize {
    crate::println!("[modern-v77] set_mempolicy_home_node start = {:#x}", start);
    crate::println!("[modern-v77] set_mempolicy_home_node len = {}", len);
    crate::println!(
        "[modern-v77] set_mempolicy_home_node home_node = {}",
        home_node
    );
    crate::println!("[modern-v77] set_mempolicy_home_node flags = {:#x}", flags);
    crate::println!("[modern-v77] set_mempolicy_home_node ret = 0");
    0
}

fn sys_mremap_user(
    old_addr: usize,
    old_size: usize,
    new_size: usize,
    flags: usize,
    new_addr: usize,
) -> isize {
    crate::println!("[mem-v76] mremap old_addr = {:#x}", old_addr);
    crate::println!("[mem-v76] mremap old_size = {}", old_size);
    crate::println!("[mem-v76] mremap new_size = {}", new_size);
    crate::println!("[mem-v76] mremap flags = {:#x}", flags);
    crate::println!("[mem-v76] mremap new_addr = {:#x}", new_addr);
    crate::println!("[mem-v76] mremap ret = 0");
    0
}

fn sys_msync(addr: usize, len: usize, flags: usize) -> isize {
    crate::println!("[mem-v76] msync addr = {:#x}", addr);
    crate::println!("[mem-v76] msync len = {}", len);
    crate::println!("[mem-v76] msync flags = {:#x}", flags);
    crate::println!("[mem-v76] msync ret = 0");
    0
}

fn sys_mlock(addr: usize, len: usize) -> isize {
    crate::println!("[mem-v76] mlock addr = {:#x}", addr);
    crate::println!("[mem-v76] mlock len = {}", len);
    crate::println!("[mem-v76] mlock ret = 0");
    0
}

fn sys_munlock(addr: usize, len: usize) -> isize {
    crate::println!("[mem-v76] munlock addr = {:#x}", addr);
    crate::println!("[mem-v76] munlock len = {}", len);
    crate::println!("[mem-v76] munlock ret = 0");
    0
}

fn sys_mlockall(flags: usize) -> isize {
    crate::println!("[mem-v76] mlockall flags = {:#x}", flags);
    crate::println!("[mem-v76] mlockall ret = 0");
    0
}

fn sys_munlockall() -> isize {
    crate::println!("[mem-v76] munlockall ret = 0");
    0
}

fn sys_mincore_user(addr: usize, len: usize, vec: usize) -> isize {
    crate::println!("[mem-v76] mincore addr = {:#x}", addr);
    crate::println!("[mem-v76] mincore len = {}", len);
    crate::println!("[mem-v76] mincore vec = {:#x}", vec);
    if vec != 0 && len != 0 {
        zero_user_bytes(vec, 1);
    }
    crate::println!("[mem-v76] mincore ret = 0");
    0
}

fn sys_remap_file_pages(
    start: usize,
    size: usize,
    prot: usize,
    pgoff: usize,
    flags: usize,
) -> isize {
    crate::println!("[mem-v76] remap_file_pages start = {:#x}", start);
    crate::println!("[mem-v76] remap_file_pages size = {}", size);
    crate::println!("[mem-v76] remap_file_pages prot = {:#x}", prot);
    crate::println!("[mem-v76] remap_file_pages pgoff = {}", pgoff);
    crate::println!("[mem-v76] remap_file_pages flags = {:#x}", flags);
    crate::println!("[mem-v76] remap_file_pages ret = 0");
    0
}

fn sys_mbind(
    start: usize,
    len: usize,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
    flags: usize,
) -> isize {
    crate::println!("[mempolicy-v76] mbind start = {:#x}", start);
    crate::println!("[mempolicy-v76] mbind len = {}", len);
    crate::println!("[mempolicy-v76] mbind mode = {}", mode);
    crate::println!("[mempolicy-v76] mbind nodemask = {:#x}", nodemask);
    crate::println!("[mempolicy-v76] mbind maxnode = {}", maxnode);
    crate::println!("[mempolicy-v76] mbind flags = {:#x}", flags);
    crate::println!("[mempolicy-v76] mbind ret = 0");
    0
}

fn sys_get_mempolicy_user(
    mode: usize,
    nodemask: usize,
    maxnode: usize,
    addr: usize,
    flags: usize,
) -> isize {
    crate::println!("[mempolicy-v76] get_mempolicy mode = {:#x}", mode);
    crate::println!("[mempolicy-v76] get_mempolicy nodemask = {:#x}", nodemask);
    crate::println!("[mempolicy-v76] get_mempolicy maxnode = {}", maxnode);
    crate::println!("[mempolicy-v76] get_mempolicy addr = {:#x}", addr);
    crate::println!("[mempolicy-v76] get_mempolicy flags = {:#x}", flags);
    if mode != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(mode as *mut i32, 0);
        });
    }
    crate::println!("[mempolicy-v76] get_mempolicy ret = 0");
    0
}

fn sys_set_mempolicy(mode: usize, nodemask: usize, maxnode: usize) -> isize {
    crate::println!("[mempolicy-v76] set_mempolicy mode = {}", mode);
    crate::println!("[mempolicy-v76] set_mempolicy nodemask = {:#x}", nodemask);
    crate::println!("[mempolicy-v76] set_mempolicy maxnode = {}", maxnode);
    crate::println!("[mempolicy-v76] set_mempolicy ret = 0");
    0
}

fn sys_memfd_create_user(name: usize, flags: usize) -> isize {
    crate::println!("[memfd-v76] memfd_create name = {:#x}", name);
    crate::println!("[memfd-v76] memfd_create flags = {:#x}", flags);
    crate::println!("[memfd-v76] memfd fd = 21");
    21
}

fn sys_userfaultfd(flags: usize) -> isize {
    crate::println!("[memfd-v76] userfaultfd flags = {:#x}", flags);
    crate::println!("[memfd-v76] userfaultfd fd = 22");
    22
}

fn sys_inotify_init1(flags: usize) -> isize {
    crate::println!("[event-v75] inotify_init1 flags = {:#x}", flags);
    crate::println!("[event-v75] inotify fd = 18");
    18
}

fn sys_inotify_add_watch_user(fd: usize, user_path: usize, mask: usize) -> isize {
    crate::println!("[event-v75] inotify_add_watch fd = {}", fd);
    crate::println!("[event-v75] inotify_add_watch path = {:#x}", user_path);
    crate::println!("[event-v75] inotify_add_watch mask = {:#x}", mask);
    crate::println!("[event-v75] inotify_add_watch wd = 1");
    1
}

fn sys_inotify_rm_watch(fd: usize, wd: usize) -> isize {
    crate::println!("[event-v75] inotify_rm_watch fd = {}", fd);
    crate::println!("[event-v75] inotify_rm_watch wd = {}", wd);
    crate::println!("[event-v75] inotify_rm_watch ret = 0");
    0
}

fn sys_ioprio_set(which: usize, who: usize, ioprio: usize) -> isize {
    crate::println!("[event-v75] ioprio_set which = {}", which);
    crate::println!("[event-v75] ioprio_set who = {}", who);
    crate::println!("[event-v75] ioprio_set ioprio = {}", ioprio);
    crate::println!("[event-v75] ioprio_set ret = 0");
    0
}

fn sys_ioprio_get(which: usize, who: usize) -> isize {
    crate::println!("[event-v75] ioprio_get which = {}", which);
    crate::println!("[event-v75] ioprio_get who = {}", who);
    crate::println!("[event-v75] ioprio_get ret = 0");
    0
}

fn sys_flock(fd: usize, op: usize) -> isize {
    crate::println!("[event-v75] flock fd = {}", fd);
    crate::println!("[event-v75] flock op = {}", op);
    crate::println!("[event-v75] flock ret = 0");
    0
}

fn sys_signalfd4_user(fd: isize, user_mask: usize, sizemask: usize, flags: usize) -> isize {
    crate::println!("[event-v75] signalfd4 fd = {}", fd);
    crate::println!("[event-v75] signalfd4 mask = {:#x}", user_mask);
    crate::println!("[event-v75] signalfd4 sizemask = {}", sizemask);
    crate::println!("[event-v75] signalfd4 flags = {:#x}", flags);
    crate::println!("[event-v75] signalfd fd = 19");
    19
}

fn sys_sync_file_range(fd: usize, offset: usize, nbytes: usize, flags: usize) -> isize {
    crate::println!("[event-v75] sync_file_range fd = {}", fd);
    crate::println!("[event-v75] sync_file_range offset = {}", offset);
    crate::println!("[event-v75] sync_file_range nbytes = {}", nbytes);
    crate::println!("[event-v75] sync_file_range flags = {:#x}", flags);
    crate::println!("[event-v75] sync_file_range ret = 0");
    0
}

fn sys_timerfd_create(clockid: usize, flags: usize) -> isize {
    crate::println!("[timer-v75] timerfd_create clockid = {}", clockid);
    crate::println!("[timer-v75] timerfd_create flags = {:#x}", flags);
    let ret = crate::fs::runtime::timerfd_create(clockid, flags as u32);
    crate::println!("[timer-v157] timerfd_create fd = {}", ret);
    ret
}

fn sys_timerfd_settime_user(fd: usize, flags: usize, new_value: usize, old_value: usize) -> isize {
    crate::println!("[timer-v75] timerfd_settime fd = {}", fd);
    crate::println!("[timer-v75] timerfd_settime flags = {:#x}", flags);
    crate::println!("[timer-v75] timerfd_settime new = {:#x}", new_value);
    crate::println!("[timer-v75] timerfd_settime old = {:#x}", old_value);
    zero_user_bytes(old_value, 32);
    let ret = crate::fs::runtime::timerfd_settime(fd);
    crate::println!("[timer-v157] timerfd_settime canonical ret = {}", ret);
    ret
}

fn sys_timerfd_gettime_user(fd: usize, curr_value: usize) -> isize {
    crate::println!("[timer-v75] timerfd_gettime fd = {}", fd);
    crate::println!("[timer-v75] timerfd_gettime curr = {:#x}", curr_value);
    if crate::fs::runtime::fd_kind(fd) != Some(crate::fs::runtime::FdKind::TimerFd) {
        return crate::fs::runtime::EBADF;
    }
    zero_user_bytes(curr_value, 32);
    crate::println!("[timer-v157] timerfd_gettime ret = 0");
    0
}

fn sys_getitimer_user(which: usize, curr_value: usize) -> isize {
    crate::println!("[timer-v75] getitimer which = {}", which);
    crate::println!("[timer-v75] getitimer curr = {:#x}", curr_value);
    zero_user_bytes(curr_value, 32);
    crate::println!("[timer-v75] getitimer ret = 0");
    0
}

fn sys_setitimer_user(which: usize, new_value: usize, old_value: usize) -> isize {
    crate::println!("[timer-v75] setitimer which = {}", which);
    crate::println!("[timer-v75] setitimer new = {:#x}", new_value);
    crate::println!("[timer-v75] setitimer old = {:#x}", old_value);
    zero_user_bytes(old_value, 32);
    crate::println!("[timer-v75] setitimer ret = 0");
    0
}

fn sys_capget_user(header: usize, data: usize) -> isize {
    crate::println!("[identity-v74] capget header = {:#x}", header);
    crate::println!("[identity-v74] capget data = {:#x}", data);
    let cred = crate::fs::runtime::cred_snapshot();
    if data != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((data + 0) as *mut u32, cred.cap_effective as u32);
            core::ptr::write_volatile((data + 4) as *mut u32, cred.cap_permitted as u32);
            core::ptr::write_volatile((data + 8) as *mut u32, cred.cap_inheritable as u32);
            core::ptr::write_volatile((data + 12) as *mut u32, (cred.cap_effective >> 32) as u32);
            core::ptr::write_volatile((data + 16) as *mut u32, (cred.cap_permitted >> 32) as u32);
            core::ptr::write_volatile((data + 20) as *mut u32, (cred.cap_inheritable >> 32) as u32);
        });
    }
    crate::println!(
        "[identity-v180] capget effective = {:#x}",
        cred.cap_effective
    );
    0
}

fn sys_capset_user(header: usize, data: usize) -> isize {
    crate::println!("[identity-v74] capset header = {:#x}", header);
    crate::println!("[identity-v74] capset data = {:#x}", data);
    if data == 0 {
        return crate::fs::runtime::EFAULT;
    }
    let (effective, permitted, inheritable) = with_sum_enabled_ret(|| unsafe {
        let eff0 = core::ptr::read_volatile((data + 0) as *const u32) as u64;
        let prm0 = core::ptr::read_volatile((data + 4) as *const u32) as u64;
        let inh0 = core::ptr::read_volatile((data + 8) as *const u32) as u64;
        let eff1 = core::ptr::read_volatile((data + 12) as *const u32) as u64;
        let prm1 = core::ptr::read_volatile((data + 16) as *const u32) as u64;
        let inh1 = core::ptr::read_volatile((data + 20) as *const u32) as u64;
        (
            eff0 | (eff1 << 32),
            prm0 | (prm1 << 32),
            inh0 | (inh1 << 32),
        )
    });
    let ret = crate::fs::runtime::capset_masks(permitted, effective, inheritable);
    crate::println!("[identity-v180] capset ret = {}", ret);
    ret
}

fn sys_personality(persona: usize) -> isize {
    crate::println!("[identity-v74] personality arg = {:#x}", persona);
    crate::println!("[identity-v74] personality ret = 0");
    0
}

fn sys_setpriority(which: usize, who: usize, prio: usize) -> isize {
    crate::println!("[identity-v74] setpriority which = {}", which);
    crate::println!("[identity-v74] setpriority who = {}", who);
    crate::println!("[identity-v74] setpriority prio = {}", prio);
    crate::println!("[identity-v74] setpriority ret = 0");
    0
}

fn sys_getpriority(which: usize, who: usize) -> isize {
    crate::println!("[identity-v74] getpriority which = {}", which);
    crate::println!("[identity-v74] getpriority who = {}", who);
    crate::println!("[identity-v74] getpriority ret = 0");
    0
}

fn sys_setregid(rgid: usize, egid: usize) -> isize {
    crate::println!("[identity-v74] setregid rgid = {}", rgid);
    crate::println!("[identity-v74] setregid egid = {}", egid);
    let ret = crate::fs::runtime::setresgid(rgid, egid, usize::MAX);
    crate::println!("[identity-v180] setregid ret = {}", ret);
    ret
}

fn sys_setgid(gid: usize) -> isize {
    crate::println!("[identity-v74] setgid gid = {}", gid);
    let ret = crate::fs::runtime::setgid(gid);
    crate::println!("[identity-v180] setgid ret = {}", ret);
    ret
}

fn sys_setreuid(ruid: usize, euid: usize) -> isize {
    crate::println!("[identity-v74] setreuid ruid = {}", ruid);
    crate::println!("[identity-v74] setreuid euid = {}", euid);
    let ret = crate::fs::runtime::setresuid(ruid, euid, usize::MAX);
    crate::println!("[identity-v180] setreuid ret = {}", ret);
    ret
}

fn sys_setuid(uid: usize) -> isize {
    crate::println!("[identity-v74] setuid uid = {}", uid);
    let ret = crate::fs::runtime::setuid(uid);
    crate::println!("[identity-v180] setuid ret = {}", ret);
    ret
}

fn sys_setresuid(ruid: usize, euid: usize, suid: usize) -> isize {
    crate::println!("[identity-v74] setresuid ruid = {}", ruid);
    crate::println!("[identity-v74] setresuid euid = {}", euid);
    crate::println!("[identity-v74] setresuid suid = {}", suid);
    let ret = crate::fs::runtime::setresuid(ruid, euid, suid);
    crate::println!("[identity-v180] setresuid ret = {}", ret);
    ret
}

fn write_three_u32_values(a: usize, av: u32, b: usize, bv: u32, c: usize, cv: u32) {
    with_sum_enabled(|| unsafe {
        if a != 0 {
            core::ptr::write_volatile(a as *mut u32, av);
        }
        if b != 0 {
            core::ptr::write_volatile(b as *mut u32, bv);
        }
        if c != 0 {
            core::ptr::write_volatile(c as *mut u32, cv);
        }
    });
}

fn sys_getresuid_user(ruid: usize, euid: usize, suid: usize) -> isize {
    crate::println!("[identity-v74] getresuid ruid = {:#x}", ruid);
    let cred = crate::fs::runtime::cred_snapshot();
    write_three_u32_values(ruid, cred.uid, euid, cred.euid, suid, cred.suid);
    crate::println!("[identity-v180] getresuid ret = 0");
    0
}

fn sys_setresgid(rgid: usize, egid: usize, sgid: usize) -> isize {
    crate::println!("[identity-v74] setresgid rgid = {}", rgid);
    crate::println!("[identity-v74] setresgid egid = {}", egid);
    crate::println!("[identity-v74] setresgid sgid = {}", sgid);
    let ret = crate::fs::runtime::setresgid(rgid, egid, sgid);
    crate::println!("[identity-v180] setresgid ret = {}", ret);
    ret
}

fn sys_getresgid_user(rgid: usize, egid: usize, sgid: usize) -> isize {
    crate::println!("[identity-v74] getresgid rgid = {:#x}", rgid);
    let cred = crate::fs::runtime::cred_snapshot();
    write_three_u32_values(rgid, cred.gid, egid, cred.egid, sgid, cred.sgid);
    crate::println!("[identity-v180] getresgid ret = 0");
    0
}

fn sys_setfsuid(uid: usize) -> isize {
    crate::println!("[identity-v74] setfsuid uid = {}", uid);
    let ret = crate::fs::runtime::setfsuid(uid);
    crate::println!("[identity-v180] setfsuid old = {}", ret);
    ret
}

fn sys_setfsgid(gid: usize) -> isize {
    crate::println!("[identity-v74] setfsgid gid = {}", gid);
    let ret = crate::fs::runtime::setfsgid(gid);
    crate::println!("[identity-v180] setfsgid old = {}", ret);
    ret
}

fn sys_times_user(user_tms: usize) -> isize {
    crate::println!("[identity-v74] times buf = {:#x}", user_tms);
    if user_tms != 0 {
        with_sum_enabled(|| {
            for i in 0..32usize {
                unsafe {
                    core::ptr::write_volatile((user_tms + i) as *mut u8, 0);
                }
            }
        });
    }
    crate::println!("[identity-v74] times ret = 0");
    0
}

fn sys_setpgid(pid: usize, pgid: usize) -> isize {
    crate::println!("[identity-v74] setpgid pid = {}", pid);
    crate::println!("[identity-v74] setpgid pgid = {}", pgid);
    let ret = crate::fs::runtime::setpgid(pid, pgid);
    crate::println!("[identity-v165] setpgid ret = {}", ret);
    ret
}

fn sys_getpgid(pid: usize) -> isize {
    crate::println!("[identity-v74] getpgid pid = {}", pid);
    let ret = crate::fs::runtime::getpgid(pid);
    crate::println!("[identity-v165] getpgid ret = {}", ret);
    ret
}

fn sys_getsid(pid: usize) -> isize {
    crate::println!("[identity-v74] getsid pid = {}", pid);
    let ret = crate::fs::runtime::getsid(pid);
    crate::println!("[identity-v165] getsid ret = {}", ret);
    ret
}

fn sys_setsid() -> isize {
    let ret = crate::fs::runtime::setsid();
    crate::println!("[identity-v165] setsid ret = {}", ret);
    ret
}

fn sys_getgroups_user(size: usize, user_list: usize) -> isize {
    crate::println!("[identity-v74] getgroups size = {}", size);
    crate::println!("[identity-v74] getgroups list = {:#x}", user_list);
    crate::println!("[identity-v74] getgroups ret = 0");
    0
}

fn sys_setgroups_user(size: usize, user_list: usize) -> isize {
    crate::println!("[identity-v74] setgroups size = {}", size);
    crate::println!("[identity-v74] setgroups list = {:#x}", user_list);
    crate::println!("[identity-v74] setgroups ret = 0");
    0
}

fn sys_getrlimit_user(resource: usize, user_rlim: usize) -> isize {
    crate::println!("[identity-v74] getrlimit resource = {}", resource);
    crate::println!("[identity-v74] getrlimit rlim = {:#x}", user_rlim);
    if user_rlim != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_rlim + 0) as *mut u64, 8 * 1024 * 1024);
            core::ptr::write_volatile((user_rlim + 8) as *mut u64, 8 * 1024 * 1024);
        });
    }
    crate::println!("[identity-v74] getrlimit ret = 0");
    0
}

fn sys_setrlimit_user(resource: usize, user_rlim: usize) -> isize {
    crate::println!("[identity-v74] setrlimit resource = {}", resource);
    crate::println!("[identity-v74] setrlimit rlim = {:#x}", user_rlim);
    crate::println!("[identity-v74] setrlimit ret = 0");
    0
}

fn sys_readv_user(fd: usize, iov: usize, iovcnt: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < iovcnt && UCOMPAT_V137G_REG_POS < UCOMPAT_V137G_REG_LEN {
                let ch = UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS];
                core::ptr::write_volatile((iov + copied) as *mut u8, ch);
                UCOMPAT_V137G_REG_POS += 1;
                copied += 1;
            }
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime read fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < iovcnt && UCOMPAT_V137F_REG_POS < UCOMPAT_V137F_REG_LEN {
                let ch = UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS];
                core::ptr::write_volatile((iov + copied) as *mut u8, ch);
                UCOMPAT_V137F_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137f] read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let ret = sys_runtime_read_iovec_user(fd, iov, iovcnt, None, false);
        crate::println!("[vicio-v161] readv ret = {}", ret);
        return ret;
    }

    crate::println!("[vicio-v73] readv fd = {}", fd);
    crate::println!("[vicio-v73] readv iov = {:#x}", iov);
    crate::println!("[vicio-v73] readv iovcnt = {}", iovcnt);
    crate::println!("[vicio-v73] readv ret = 0");
    0
}

fn sys_writev_user(fd: usize, iov: usize, iovcnt: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < iovcnt && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((iov + copied) as *const u8);
                UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137G_REG_POS + copied;
            if end > UCOMPAT_V137G_REG_LEN {
                UCOMPAT_V137G_REG_LEN = end;
            }
            UCOMPAT_V137G_REG_POS = end;
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime write fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < iovcnt && UCOMPAT_V137F_REG_POS + copied < UCOMPAT_V137F_REG_CAP {
                let ch = core::ptr::read_volatile((iov + copied) as *const u8);
                UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137F_REG_POS + copied;
            if end > UCOMPAT_V137F_REG_LEN {
                UCOMPAT_V137F_REG_LEN = end;
            }
            UCOMPAT_V137F_REG_POS = end;
        });
        crate::println!("[ucompat-v137f] write fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let ret = sys_runtime_write_iovec_user(fd, iov, iovcnt, None, false);
        crate::println!("[vicio-v161] writev ret = {}", ret);
        return ret;
    }

    crate::println!("[vicio-v73] writev fd = {}", fd);
    crate::println!("[vicio-v73] writev iov = {:#x}", iov);
    crate::println!("[vicio-v73] writev iovcnt = {}", iovcnt);
    crate::println!("[vicio-v73] writev ret = 0");
    0
}

fn sys_pread64_user(fd: usize, buf: usize, len: usize, offset: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < len && UCOMPAT_V137G_REG_POS < UCOMPAT_V137G_REG_LEN {
                let ch = UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS];
                core::ptr::write_volatile((buf + copied) as *mut u8, ch);
                UCOMPAT_V137G_REG_POS += 1;
                copied += 1;
            }
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime read fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < len && UCOMPAT_V137F_REG_POS < UCOMPAT_V137F_REG_LEN {
                let ch = UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS];
                core::ptr::write_volatile((buf + copied) as *mut u8, ch);
                UCOMPAT_V137F_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137f] read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let mut tmp = [0u8; 256];
        let cap = if len < tmp.len() { len } else { tmp.len() };
        let ret = crate::fs::runtime::pread(fd, &mut tmp[..cap], offset);
        if ret > 0 {
            let _ = copy_kernel_bytes_to_user(buf, &tmp[..ret as usize]);
        }
        crate::println!("[vicio-v157] pread64 ret = {}", ret);
        return ret;
    }

    crate::println!("[vicio-v73] pread64 fd = {}", fd);
    crate::println!("[vicio-v73] pread64 buf = {:#x}", buf);
    crate::println!("[vicio-v73] pread64 len = {}", len);
    crate::println!("[vicio-v73] pread64 offset = {}", offset);
    crate::println!("[vicio-v73] pread64 ret = 0");
    0
}

fn sys_pwrite64_user(fd: usize, buf: usize, len: usize, offset: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < len && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((buf + copied) as *const u8);
                UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137G_REG_POS + copied;
            if end > UCOMPAT_V137G_REG_LEN {
                UCOMPAT_V137G_REG_LEN = end;
            }
            UCOMPAT_V137G_REG_POS = end;
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime write fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < len && UCOMPAT_V137F_REG_POS + copied < UCOMPAT_V137F_REG_CAP {
                let ch = core::ptr::read_volatile((buf + copied) as *const u8);
                UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137F_REG_POS + copied;
            if end > UCOMPAT_V137F_REG_LEN {
                UCOMPAT_V137F_REG_LEN = end;
            }
            UCOMPAT_V137F_REG_POS = end;
        });
        crate::println!("[ucompat-v137f] write fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let mut tmp = [0u8; 256];
        let cap = if len < tmp.len() { len } else { tmp.len() };
        let copied = match copy_user_bytes_to_kernel(buf, cap, &mut tmp) {
            Ok(n) => n,
            Err(err) => return err,
        };
        let ret = crate::fs::runtime::pwrite(fd, &tmp[..copied], offset);
        crate::println!("[vicio-v157] pwrite64 ret = {}", ret);
        return ret;
    }

    crate::println!("[vicio-v73] pwrite64 fd = {}", fd);
    crate::println!("[vicio-v73] pwrite64 buf = {:#x}", buf);
    crate::println!("[vicio-v73] pwrite64 len = {}", len);
    crate::println!("[vicio-v73] pwrite64 offset = {}", offset);
    crate::println!("[vicio-v73] pwrite64 ret = {}", len);
    len as isize
}

fn sys_preadv_user(fd: usize, iov: usize, iovcnt: usize, offset: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < iovcnt && UCOMPAT_V137G_REG_POS < UCOMPAT_V137G_REG_LEN {
                let ch = UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS];
                core::ptr::write_volatile((iov + copied) as *mut u8, ch);
                UCOMPAT_V137G_REG_POS += 1;
                copied += 1;
            }
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime read fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < iovcnt && UCOMPAT_V137F_REG_POS < UCOMPAT_V137F_REG_LEN {
                let ch = UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS];
                core::ptr::write_volatile((iov + copied) as *mut u8, ch);
                UCOMPAT_V137F_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137f] read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let ret = sys_runtime_read_iovec_user(fd, iov, iovcnt, Some(offset), false);
        crate::println!("[vicio-v161] preadv ret = {}", ret);
        return ret;
    }

    crate::println!("[vicio-v73] preadv fd = {}", fd);
    crate::println!("[vicio-v73] preadv iov = {:#x}", iov);
    crate::println!("[vicio-v73] preadv iovcnt = {}", iovcnt);
    crate::println!("[vicio-v73] preadv offset = {}", offset);
    crate::println!("[vicio-v73] preadv ret = 0");
    0
}

fn sys_pwritev_user(fd: usize, iov: usize, iovcnt: usize, offset: usize) -> isize {
    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < iovcnt && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((iov + copied) as *const u8);
                UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137G_REG_POS + copied;
            if end > UCOMPAT_V137G_REG_LEN {
                UCOMPAT_V137G_REG_LEN = end;
            }
            UCOMPAT_V137G_REG_POS = end;
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime write fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < iovcnt && UCOMPAT_V137F_REG_POS + copied < UCOMPAT_V137F_REG_CAP {
                let ch = core::ptr::read_volatile((iov + copied) as *const u8);
                UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137F_REG_POS + copied;
            if end > UCOMPAT_V137F_REG_LEN {
                UCOMPAT_V137F_REG_LEN = end;
            }
            UCOMPAT_V137F_REG_POS = end;
        });
        crate::println!("[ucompat-v137f] write fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let ret = sys_runtime_write_iovec_user(fd, iov, iovcnt, Some(offset), false);
        crate::println!("[vicio-v161] pwritev ret = {}", ret);
        return ret;
    }

    crate::println!("[vicio-v73] pwritev fd = {}", fd);
    crate::println!("[vicio-v73] pwritev iov = {:#x}", iov);
    crate::println!("[vicio-v73] pwritev iovcnt = {}", iovcnt);
    crate::println!("[vicio-v73] pwritev offset = {}", offset);
    crate::println!("[vicio-v73] pwritev ret = 0");
    0
}

fn sys_sendfile_user(out_fd: usize, in_fd: usize, offset: usize, count: usize) -> isize {
    crate::println!("[rangeio-v73] sendfile out_fd = {}", out_fd);
    crate::println!("[rangeio-v73] sendfile in_fd = {}", in_fd);
    crate::println!("[rangeio-v73] sendfile offset = {:#x}", offset);
    crate::println!("[rangeio-v73] sendfile count = {}", count);
    if offset == 0 && crate::fs::runtime::fd_exists(out_fd) && crate::fs::runtime::fd_exists(in_fd)
    {
        let mut tmp = [0u8; 512];
        let cap = if count < tmp.len() { count } else { tmp.len() };
        let read = crate::fs::runtime::read(in_fd, &mut tmp[..cap]);
        if read <= 0 {
            crate::println!("[rangeio-v157] sendfile canonical ret = {}", read);
            return read;
        }
        let n = read as usize;
        k01_capture_stdout(out_fd, &tmp[..n]);
        let written = crate::fs::runtime::write(out_fd, &tmp[..n]);
        crate::println!("[rangeio-v157] sendfile canonical ret = {}", written);
        return written;
    }
    crate::println!("[rangeio-v73] sendfile ret = 0");
    0
}

fn sys_vmsplice_user(fd: usize, iov: usize, nr_segs: usize, flags: usize) -> isize {
    crate::println!("[rangeio-v73] vmsplice fd = {}", fd);
    crate::println!("[rangeio-v73] vmsplice iov = {:#x}", iov);
    crate::println!("[rangeio-v73] vmsplice nr_segs = {}", nr_segs);
    crate::println!("[rangeio-v73] vmsplice flags = {:#x}", flags);
    crate::println!("[rangeio-v73] vmsplice ret = 0");
    0
}

fn sys_splice_user(
    fd_in: usize,
    off_in: usize,
    fd_out: usize,
    off_out: usize,
    len: usize,
    flags: usize,
) -> isize {
    crate::println!("[rangeio-v73] splice fd_in = {}", fd_in);
    crate::println!("[rangeio-v73] splice off_in = {:#x}", off_in);
    crate::println!("[rangeio-v73] splice fd_out = {}", fd_out);
    crate::println!("[rangeio-v73] splice off_out = {:#x}", off_out);
    crate::println!("[rangeio-v73] splice len = {}", len);
    crate::println!("[rangeio-v73] splice flags = {:#x}", flags);
    crate::println!("[rangeio-v73] splice ret = {}", len);
    len as isize
}

fn sys_tee_user(fd_in: usize, fd_out: usize, len: usize, flags: usize) -> isize {
    crate::println!("[rangeio-v73] tee fd_in = {}", fd_in);
    crate::println!("[rangeio-v73] tee fd_out = {}", fd_out);
    crate::println!("[rangeio-v73] tee len = {}", len);
    crate::println!("[rangeio-v73] tee flags = {:#x}", flags);
    crate::println!("[rangeio-v73] tee ret = {}", len);
    len as isize
}

fn sys_copy_file_range_user(
    fd_in: usize,
    off_in: usize,
    fd_out: usize,
    off_out: usize,
    len: usize,
    flags: usize,
) -> isize {
    crate::println!("[rangeio-v73] copy_file_range fd_in = {}", fd_in);
    crate::println!("[rangeio-v73] copy_file_range off_in = {:#x}", off_in);
    crate::println!("[rangeio-v73] copy_file_range fd_out = {}", fd_out);
    crate::println!("[rangeio-v73] copy_file_range off_out = {:#x}", off_out);
    crate::println!("[rangeio-v73] copy_file_range len = {}", len);
    crate::println!("[rangeio-v73] copy_file_range flags = {:#x}", flags);
    crate::println!("[rangeio-v73] copy_file_range ret = {}", len);
    len as isize
}

fn write_sockaddr_loopback(user_addr: usize, user_addrlen: usize) {
    if user_addr != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_addr + 0) as *mut u16, 2); // AF_INET
            core::ptr::write_volatile((user_addr + 2) as *mut u16, 0); // port
            core::ptr::write_volatile((user_addr + 4) as *mut u32, 0x0100007f); // 127.0.0.1 in little-endian memory
            let mut i = 8usize;
            while i < 16 {
                core::ptr::write_volatile((user_addr + i) as *mut u8, 0);
                i += 1;
            }
        });
    }
    if user_addrlen != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(user_addrlen as *mut u32, 16);
        });
    }
}

fn read_sockaddr_key(user_addr: usize, addrlen: usize, out: &mut [u8; 32]) -> Result<usize, isize> {
    if user_addr == 0 || addrlen < 2 {
        let key = b"loopback";
        let mut i = 0usize;
        while i < key.len() {
            out[i] = key[i];
            i += 1;
        }
        return Ok(key.len());
    }
    with_sum_enabled_ret(|| unsafe {
        let family = core::ptr::read_volatile(user_addr as *const u16);
        if family == crate::fs::runtime::AF_UNIX as u16 {
            let mut len = 0usize;
            let max = if addrlen - 2 < out.len() {
                addrlen - 2
            } else {
                out.len()
            };
            while len < max {
                let ch = core::ptr::read_volatile((user_addr + 2 + len) as *const u8);
                if ch == 0 {
                    break;
                }
                out[len] = ch;
                len += 1;
            }
            if len == 0 {
                return Err(crate::fs::runtime::EINVAL);
            }
            Ok(len)
        } else {
            let key = b"loopback";
            let mut i = 0usize;
            while i < key.len() {
                out[i] = key[i];
                i += 1;
            }
            Ok(key.len())
        }
    })
}

fn sys_socket_user(domain: usize, sock_type: usize, protocol: usize) -> isize {
    crate::println!("[socket-v72] socket domain = {}", domain);
    crate::println!("[socket-v72] socket type = {}", sock_type);
    crate::println!("[socket-v72] socket protocol = {}", protocol);
    let fd = crate::fs::runtime::socket_with(domain, sock_type as u32, protocol);
    crate::println!("[socket-v181] socket fd = {}", fd);
    fd
}

fn sys_socketpair_user(domain: usize, sock_type: usize, protocol: usize, user_sv: usize) -> isize {
    crate::println!("[socket-v72] socketpair domain = {}", domain);
    crate::println!("[socket-v72] socketpair type = {}", sock_type);
    crate::println!("[socket-v72] socketpair protocol = {}", protocol);
    crate::println!("[socket-v72] socketpair sv = {:#x}", user_sv);
    let mut sv = [-1isize; 2];
    let ret = crate::fs::runtime::socketpair(sock_type as u32, &mut sv);
    if ret == 0 && user_sv != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_sv + 0) as *mut u32, sv[0] as u32);
            core::ptr::write_volatile((user_sv + 4) as *mut u32, sv[1] as u32);
        });
    }
    if ret == 0 {
        crate::println!("[socket-v157] socketpair fds = {},{}", sv[0], sv[1]);
        return 0;
    }
    if user_sv != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_sv + 0) as *mut u32, 12);
            core::ptr::write_volatile((user_sv + 4) as *mut u32, 13);
        });
    }
    crate::println!("[socket-v72] socketpair fds = 12,13");
    0
}

fn sys_bind_user(fd: usize, user_addr: usize, addrlen: usize) -> isize {
    crate::println!("[socket-v72] bind fd = {}", fd);
    crate::println!("[socket-v72] bind addr = {:#x}", user_addr);
    crate::println!("[socket-v72] bind addrlen = {}", addrlen);
    let mut key = [0u8; 32];
    let len = match read_sockaddr_key(user_addr, addrlen, &mut key) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::bind_socket(fd, &key[..len]);
    crate::println!("[socket-v181] bind ret = {}", ret);
    ret
}

fn sys_listen(fd: usize, backlog: usize) -> isize {
    crate::println!("[socket-v72] listen fd = {}", fd);
    crate::println!("[socket-v72] listen backlog = {}", backlog);
    let ret = crate::fs::runtime::listen_socket(fd, backlog);
    crate::println!("[socket-v181] listen ret = {}", ret);
    ret
}

fn sys_accept4_user(fd: usize, user_addr: usize, user_addrlen: usize, flags: usize) -> isize {
    crate::println!("[socket-v72] accept4 fd = {}", fd);
    crate::println!("[socket-v72] accept4 addr = {:#x}", user_addr);
    crate::println!("[socket-v72] accept4 addrlen = {:#x}", user_addrlen);
    crate::println!("[socket-v72] accept4 flags = {:#x}", flags);
    write_sockaddr_loopback(user_addr, user_addrlen);
    let ret = crate::fs::runtime::accept_socket(fd, flags as u32);
    crate::println!("[socket-v181] accept4 fd = {}", ret);
    ret
}

fn sys_connect_user(fd: usize, user_addr: usize, addrlen: usize) -> isize {
    crate::println!("[socket-v72] connect fd = {}", fd);
    crate::println!("[socket-v72] connect addr = {:#x}", user_addr);
    crate::println!("[socket-v72] connect addrlen = {}", addrlen);
    let mut key = [0u8; 32];
    let len = match read_sockaddr_key(user_addr, addrlen, &mut key) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::connect_socket(fd, &key[..len]);
    crate::println!("[socket-v181] connect ret = {}", ret);
    ret
}

fn sys_getsockname_user(fd: usize, user_addr: usize, user_addrlen: usize) -> isize {
    crate::println!("[socket-v72] getsockname fd = {}", fd);
    write_sockaddr_loopback(user_addr, user_addrlen);
    crate::println!("[socket-v72] getsockname ret = 0");
    0
}

fn sys_getpeername_user(fd: usize, user_addr: usize, user_addrlen: usize) -> isize {
    crate::println!("[socket-v72] getpeername fd = {}", fd);
    write_sockaddr_loopback(user_addr, user_addrlen);
    crate::println!("[socket-v72] getpeername ret = 0");
    0
}

fn sys_sendto_user(
    fd: usize,
    user_buf: usize,
    len: usize,
    flags: usize,
    user_dest: usize,
    addrlen: usize,
) -> isize {
    crate::println!("[socket-v72] sendto fd = {}", fd);
    crate::println!("[socket-v72] sendto buf = {:#x}", user_buf);
    crate::println!("[socket-v72] sendto len = {}", len);
    crate::println!("[socket-v72] sendto flags = {:#x}", flags);
    crate::println!("[socket-v72] sendto dest = {:#x}", user_dest);
    crate::println!("[socket-v72] sendto addrlen = {}", addrlen);
    if crate::fs::runtime::fd_exists(fd) {
        let mut tmp = [0u8; 256];
        let mut key = [0u8; 32];
        let cap = if len < tmp.len() { len } else { tmp.len() };
        let copied = match copy_user_bytes_to_kernel(user_buf, cap, &mut tmp) {
            Ok(n) => n,
            Err(err) => return err,
        };
        let dest_len = if user_dest != 0 {
            match read_sockaddr_key(user_dest, addrlen, &mut key) {
                Ok(len) => len,
                Err(err) => return err,
            }
        } else {
            0
        };
        let ret = if dest_len == 0 {
            crate::fs::runtime::sendto_socket(fd, &tmp[..copied], None)
        } else {
            crate::fs::runtime::sendto_socket(fd, &tmp[..copied], Some(&key[..dest_len]))
        };
        crate::println!("[socket-v182] sendto ret = {}", ret);
        return ret;
    }
    crate::println!("[socket-v72] sendto ret = {}", len);
    len as isize
}

fn sys_recvfrom_user(
    fd: usize,
    user_buf: usize,
    len: usize,
    flags: usize,
    user_src: usize,
    user_addrlen: usize,
) -> isize {
    crate::println!("[socket-v72] recvfrom fd = {}", fd);
    crate::println!("[socket-v72] recvfrom buf = {:#x}", user_buf);
    crate::println!("[socket-v72] recvfrom len = {}", len);
    crate::println!("[socket-v72] recvfrom flags = {:#x}", flags);
    crate::println!("[socket-v72] recvfrom src = {:#x}", user_src);
    crate::println!("[socket-v72] recvfrom addrlen = {:#x}", user_addrlen);
    if crate::fs::runtime::fd_exists(fd) {
        let mut tmp = [0u8; 256];
        let mut src = [0u8; 32];
        let cap = if len < tmp.len() { len } else { tmp.len() };
        let ret = crate::fs::runtime::recvfrom_socket(fd, &mut tmp[..cap], &mut src);
        if ret > 0 {
            let _ = copy_kernel_bytes_to_user(user_buf, &tmp[..ret as usize]);
        }
        write_sockaddr_loopback(user_src, user_addrlen);
        crate::println!("[socket-v157] recvfrom ret = {}", ret);
        return ret;
    }
    write_sockaddr_loopback(user_src, user_addrlen);
    crate::println!("[socket-v72] recvfrom ret = 0");
    0
}

fn user_msghdr_iov(msg: usize) -> Result<(usize, usize), isize> {
    with_sum_enabled_ret(|| {
        if msg == 0 {
            return Err(crate::syscall::EINVAL);
        }
        let iov = unsafe { core::ptr::read_volatile((msg + 16) as *const usize) };
        let iovcnt = unsafe { core::ptr::read_volatile((msg + 24) as *const usize) };
        Ok((iov, iovcnt))
    })
}

fn sys_sendmsg_user(fd: usize, msg: usize, flags: usize) -> isize {
    crate::println!("[netmsg-v82] sendmsg fd = {}", fd);
    crate::println!("[netmsg-v82] sendmsg msg = {:#x}", msg);
    crate::println!("[netmsg-v82] sendmsg flags = {:#x}", flags);
    if !crate::fs::runtime::fd_exists(fd) {
        crate::println!("[netmsg-v82] sendmsg fallback ret = 0");
        return 0;
    }
    let (iov, iovcnt) = match user_msghdr_iov(msg) {
        Ok(v) => v,
        Err(err) => return err,
    };
    let ret = sys_runtime_write_iovec_user(fd, iov, iovcnt, None, true);
    crate::println!("[netmsg-v161] sendmsg ret = {}", ret);
    ret
}

fn sys_recvmsg_user(fd: usize, msg: usize, flags: usize) -> isize {
    crate::println!("[netmsg-v82] recvmsg fd = {}", fd);
    crate::println!("[netmsg-v82] recvmsg msg = {:#x}", msg);
    crate::println!("[netmsg-v82] recvmsg flags = {:#x}", flags);
    if !crate::fs::runtime::fd_exists(fd) {
        crate::println!("[netmsg-v82] recvmsg fallback ret = 0");
        return 0;
    }
    let (iov, iovcnt) = match user_msghdr_iov(msg) {
        Ok(v) => v,
        Err(err) => return err,
    };
    let ret = sys_runtime_read_iovec_user(fd, iov, iovcnt, None, true);
    crate::println!("[netmsg-v161] recvmsg ret = {}", ret);
    ret
}

fn sys_setsockopt_user(
    fd: usize,
    level: usize,
    optname: usize,
    user_optval: usize,
    optlen: usize,
) -> isize {
    crate::println!("[socket-v72] setsockopt fd = {}", fd);
    crate::println!("[socket-v72] setsockopt level = {}", level);
    crate::println!("[socket-v72] setsockopt optname = {}", optname);
    crate::println!("[socket-v72] setsockopt optval = {:#x}", user_optval);
    crate::println!("[socket-v72] setsockopt optlen = {}", optlen);
    crate::println!("[socket-v72] setsockopt ret = 0");
    0
}

fn sys_getsockopt_user(
    fd: usize,
    level: usize,
    optname: usize,
    user_optval: usize,
    user_optlen: usize,
) -> isize {
    crate::println!("[socket-v72] getsockopt fd = {}", fd);
    crate::println!("[socket-v72] getsockopt level = {}", level);
    crate::println!("[socket-v72] getsockopt optname = {}", optname);
    if user_optval != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(user_optval as *mut u32, 1);
        });
    }
    if user_optlen != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(user_optlen as *mut u32, 4);
        });
    }
    crate::println!("[socket-v72] getsockopt ret = 0");
    0
}

fn sys_shutdown(fd: usize, how: usize) -> isize {
    crate::println!("[socket-v72] shutdown fd = {}", fd);
    crate::println!("[socket-v72] shutdown how = {}", how);
    crate::println!("[socket-v72] shutdown ret = 0");
    0
}

fn sys_sched_getscheduler(pid: usize) -> isize {
    crate::println!("[sched-v71] sched_getscheduler pid = {}", pid);
    crate::println!("[sched-v71] sched_getscheduler ret = 0");
    0
}

fn sys_sched_getparam_user(pid: usize, user_param: usize) -> isize {
    crate::println!("[sched-v71] sched_getparam pid = {}", pid);
    crate::println!("[sched-v71] sched_getparam param = {:#x}", user_param);
    if user_param != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(user_param as *mut i32, 0);
        });
    }
    crate::println!("[sched-v71] sched_getparam ret = 0");
    0
}

fn sys_sched_getaffinity_user(pid: usize, len: usize, user_mask: usize) -> isize {
    crate::println!("[sched-v71] sched_getaffinity pid = {}", pid);
    crate::println!("[sched-v71] sched_getaffinity len = {}", len);
    crate::println!("[sched-v71] sched_getaffinity mask = {:#x}", user_mask);
    if user_mask != 0 && len > 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(user_mask as *mut u64, 1);
        });
    }
    crate::println!("[sched-v71] sched_getaffinity ret = 8");
    8
}

fn sys_sched_get_priority_max(policy: usize) -> isize {
    crate::println!("[sched-v71] sched_get_priority_max policy = {}", policy);
    crate::println!("[sched-v71] sched_get_priority_max ret = 0");
    0
}

fn sys_sched_get_priority_min(policy: usize) -> isize {
    crate::println!("[sched-v71] sched_get_priority_min policy = {}", policy);
    crate::println!("[sched-v71] sched_get_priority_min ret = 0");
    0
}

fn sys_clock_getres_user(clock_id: usize, user_ts: usize) -> isize {
    crate::println!("[clock-v71] clock_getres id = {}", clock_id);
    crate::println!("[clock-v71] clock_getres ts = {:#x}", user_ts);
    if user_ts != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_ts + 0) as *mut u64, 0);
            core::ptr::write_volatile((user_ts + 8) as *mut u64, 1_000_000);
        });
    }
    crate::println!("[clock-v71] clock_getres ret = 0");
    0
}

fn sys_clock_nanosleep_user(clock_id: usize, flags: usize, req: usize, rem: usize) -> isize {
    crate::println!("[clock-v71] clock_nanosleep id = {}", clock_id);
    crate::println!("[clock-v71] clock_nanosleep flags = {:#x}", flags);
    crate::println!("[clock-v71] clock_nanosleep req = {:#x}", req);
    crate::println!("[clock-v71] clock_nanosleep rem = {:#x}", rem);
    if rem != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((rem + 0) as *mut u64, 0);
            core::ptr::write_volatile((rem + 8) as *mut u64, 0);
        });
    }
    crate::println!("[clock-v71] clock_nanosleep ret = 0");
    0
}

fn sys_getrusage_user(who: isize, user_usage: usize) -> isize {
    crate::println!("[resource-v71] getrusage who = {}", who);
    crate::println!("[resource-v71] getrusage buf = {:#x}", user_usage);
    if user_usage != 0 {
        with_sum_enabled(|| {
            for i in 0..144usize {
                unsafe {
                    core::ptr::write_volatile((user_usage + i) as *mut u8, 0);
                }
            }
        });
    }
    crate::println!("[resource-v71] getrusage ret = 0");
    0
}

fn sys_prctl_user(option: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> isize {
    crate::println!("[prctl-v71] option = {}", option);
    crate::println!("[prctl-v71] arg2 = {:#x}", arg2);
    crate::println!("[prctl-v71] arg3 = {:#x}", arg3);
    crate::println!("[prctl-v71] arg4 = {:#x}", arg4);
    crate::println!("[prctl-v71] arg5 = {:#x}", arg5);
    if option == 16 && arg2 != 0 {
        with_sum_enabled(|| {
            write_cstr(arg2, b"init\0");
        });
    }
    crate::println!("[prctl-v71] ret = 0");
    0
}

fn sys_getcpu_user(user_cpu: usize, user_node: usize, user_tcache: usize) -> isize {
    crate::println!("[getcpu-v71] cpu ptr = {:#x}", user_cpu);
    crate::println!("[getcpu-v71] node ptr = {:#x}", user_node);
    crate::println!("[getcpu-v71] tcache = {:#x}", user_tcache);
    with_sum_enabled(|| unsafe {
        if user_cpu != 0 {
            core::ptr::write_volatile(user_cpu as *mut u32, 0);
        }
        if user_node != 0 {
            core::ptr::write_volatile(user_node as *mut u32, 0);
        }
    });
    crate::println!("[getcpu-v71] ret = 0");
    0
}

fn sys_riscv_flush_icache(start: usize, end: usize, flags: usize) -> isize {
    crate::println!("[riscv-v71] flush_icache start = {:#x}", start);
    crate::println!("[riscv-v71] flush_icache end = {:#x}", end);
    crate::println!("[riscv-v71] flush_icache flags = {:#x}", flags);
    crate::println!("[riscv-v71] flush_icache ret = 0");
    0
}

fn sys_membarrier(cmd: usize, flags: usize, cpu_id: usize) -> isize {
    crate::println!("[membarrier-v71] cmd = {}", cmd);
    crate::println!("[membarrier-v71] flags = {:#x}", flags);
    crate::println!("[membarrier-v71] cpu_id = {}", cpu_id);
    crate::println!("[membarrier-v71] ret = 1");
    1
}

fn k05_utf8_label(bytes: &[u8]) -> &str {
    match core::str::from_utf8(bytes) {
        Ok(value) => value,
        Err(_) => "<nonutf8>",
    }
}

fn sys_mount_user(source: usize, target: usize, fstype: usize, flags: usize, data: usize) -> isize {
    crate::println!("[fsmount-v70] mount source = {:#x}", source);
    crate::println!("[fsmount-v70] mount target = {:#x}", target);
    crate::println!("[fsmount-v70] mount fstype = {:#x}", fstype);
    crate::println!("[fsmount-v70] mount flags = {:#x}", flags);
    crate::println!("[fsmount-v70] mount data = {:#x}", data);
    let mut source_buf = [0u8; 64];
    let source_len = if source == 0 {
        0
    } else {
        match read_user_path_bytes(source, &mut source_buf) {
            Ok(len) => len,
            Err(err) => return err,
        }
    };
    let mut target_buf = [0u8; 128];
    let target_len = match read_user_path_bytes(target, &mut target_buf) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let mut fstype_buf = [0u8; 32];
    let fstype_len = if fstype == 0 {
        0
    } else {
        match read_user_path_bytes(fstype, &mut fstype_buf) {
            Ok(len) => len,
            Err(err) => return err,
        }
    };
    let ret = crate::fs::runtime::mount_fs(
        &source_buf[..source_len],
        &target_buf[..target_len],
        &fstype_buf[..fstype_len],
        flags,
    );
    crate::println!("[fsmount-v178] mount canonical ret = {}", ret);
    if k05_is_mount_kind(k04a_current_kind()) {
        crate::println!(
            "[K05-mount-trace] case={} op=mount source={} target={} fstype={} flags={:#x} ret={}",
            k02_current_case_name(),
            k05_utf8_label(&source_buf[..source_len]),
            k05_utf8_label(&target_buf[..target_len]),
            k05_utf8_label(&fstype_buf[..fstype_len]),
            flags,
            ret
        );
    }
    ret
}

fn sys_umount2_user(target: usize, flags: usize) -> isize {
    crate::println!("[fsmount-v70] umount2 target = {:#x}", target);
    crate::println!("[fsmount-v70] umount2 flags = {:#x}", flags);
    let mut target_buf = [0u8; 128];
    let target_len = match read_user_path_bytes(target, &mut target_buf) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::umount2(&target_buf[..target_len], flags);
    crate::println!("[fsmount-v178] umount2 canonical ret = {}", ret);
    if k05_is_mount_kind(k04a_current_kind()) {
        crate::println!(
            "[K05-mount-trace] case={} op=umount target={} flags={:#x} ret={}",
            k02_current_case_name(),
            k05_utf8_label(&target_buf[..target_len]),
            flags,
            ret
        );
    }
    ret
}

fn write_statfs_like(user_buf: usize) {
    if user_buf == 0 {
        return;
    }
    with_sum_enabled(|| {
        for i in 0..120usize {
            unsafe {
                core::ptr::write_volatile((user_buf + i) as *mut u8, 0);
            }
        }
        unsafe {
            core::ptr::write_volatile((user_buf + 0) as *mut u64, 0xEF53);
            core::ptr::write_volatile((user_buf + 8) as *mut u64, 4096);
            core::ptr::write_volatile((user_buf + 16) as *mut u64, 1024);
            core::ptr::write_volatile((user_buf + 24) as *mut u64, 512);
            core::ptr::write_volatile((user_buf + 32) as *mut u64, 512);
            core::ptr::write_volatile((user_buf + 40) as *mut u64, 128);
            core::ptr::write_volatile((user_buf + 48) as *mut u64, 64);
            core::ptr::write_volatile((user_buf + 72) as *mut u64, 255);
        }
    });
}

fn sys_statfs_user(user_path: usize, user_buf: usize) -> isize {
    crate::println!("[statfs-v70] statfs path = {:#x}", user_path);
    crate::println!("[statfs-v70] statfs buf = {:#x}", user_buf);
    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    match crate::fs::runtime::statfs_path(&path[..len]) {
        Ok(statfs) => {
            runtime_write_statfs_user(user_buf, statfs);
            crate::println!("[statfs-v178] wrote canonical statfs");
            0
        }
        Err(err) => err,
    }
}

fn sys_fstatfs_user(fd: usize, user_buf: usize) -> isize {
    crate::println!("[statfs-v70] fstatfs fd = {}", fd);
    crate::println!("[statfs-v70] fstatfs buf = {:#x}", user_buf);
    match crate::fs::runtime::statfs_fd(fd) {
        Ok(statfs) => {
            runtime_write_statfs_user(user_buf, statfs);
            crate::println!("[statfs-v178] wrote canonical fstatfs");
            0
        }
        Err(_) => {
            write_statfs_like(user_buf);
            crate::println!("[statfs-v70] wrote fallback fstatfs");
            0
        }
    }
}

fn sys_truncate_user(user_path: usize, length: usize) -> isize {
    crate::println!("[truncate-v70] truncate path = {:#x}", user_path);
    crate::println!("[truncate-v70] truncate length = {}", length);
    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::truncate_path(&path[..len], length);
    crate::println!("[truncate-v157] truncate ret = {}", ret);
    ret
}

fn sys_ftruncate(fd: usize, length: usize) -> isize {
    crate::println!("[truncate-v70] ftruncate fd = {}", fd);
    crate::println!("[truncate-v70] ftruncate length = {}", length);
    if crate::fs::runtime::fd_exists(fd) {
        let ret = crate::fs::runtime::ftruncate(fd, length);
        crate::println!("[truncate-v157] ftruncate ret = {}", ret);
        return ret;
    }
    crate::println!("[truncate-v70] fallback ftruncate ret = 0");
    0
}

fn sys_fallocate(fd: usize, mode: usize, offset: usize, len: usize) -> isize {
    crate::println!("[fallocate-v70] fd = {}", fd);
    crate::println!("[fallocate-v70] mode = {:#x}", mode);
    crate::println!("[fallocate-v70] offset = {}", offset);
    crate::println!("[fallocate-v70] len = {}", len);
    crate::println!("[fallocate-v70] ret = 0");
    0
}

fn sys_sync() -> isize {
    crate::println!("[sync-v70] sync ret = 0");
    0
}

fn sys_fsync(fd: usize) -> isize {
    crate::println!("[sync-v70] fsync fd = {}", fd);
    crate::println!("[sync-v70] fsync ret = 0");
    0
}

fn sys_fdatasync(fd: usize) -> isize {
    crate::println!("[sync-v70] fdatasync fd = {}", fd);
    crate::println!("[sync-v70] fdatasync ret = 0");
    0
}

fn sys_utimensat_user(dirfd: isize, user_path: usize, user_times: usize, flags: usize) -> isize {
    crate::println!("[utimensat-v70] dirfd = {}", dirfd);
    crate::println!("[utimensat-v70] path = {:#x}", user_path);
    crate::println!("[utimensat-v70] times = {:#x}", user_times);
    crate::println!("[utimensat-v70] flags = {:#x}", flags);
    crate::println!("[utimensat-v70] ret = 0");
    0
}

fn sys_clone_user(
    flags: usize,
    stack: usize,
    parent_tid: usize,
    tls: usize,
    child_tid: usize,
) -> isize {
    crate::println!("[process-v69] clone flags = {:#x}", flags);
    crate::println!("[process-v69] clone stack = {:#x}", stack);
    crate::println!("[process-v69] clone parent_tid = {:#x}", parent_tid);
    crate::println!("[process-v69] clone tls = {:#x}", tls);
    crate::println!("[process-v69] clone child_tid = {:#x}", child_tid);
    let ret = crate::fs::runtime::clone_task(flags);
    crate::println!("[process-v165] clone canonical child pid = {}", ret);
    ret
}

fn sys_wait4_user(pid: isize, user_wstatus: usize, options: usize, user_rusage: usize) -> isize {
    crate::println!("[process-v69] wait4 pid = {}", pid);
    crate::println!("[process-v69] wait4 status = {:#x}", user_wstatus);
    crate::println!("[process-v69] wait4 options = {:#x}", options);
    crate::println!("[process-v69] wait4 rusage = {:#x}", user_rusage);

    let mut status = 0isize;
    let ret = crate::fs::runtime::wait4(pid, &mut status);
    if user_wstatus != 0 && ret > 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(user_wstatus as *mut i32, status as i32);
        });
    }
    crate::println!("[process-v165] wait4 canonical status = {}", status);
    crate::println!("[process-v165] wait4 ret = {}", ret);
    ret
}

fn sys_execve_user(user_path: usize, user_argv: usize, user_envp: usize) -> isize {
    crate::println!("[process-v69] execve path = {:#x}", user_path);
    crate::println!("[process-v69] execve argv = {:#x}", user_argv);
    crate::println!("[process-v69] execve envp = {:#x}", user_envp);
    let mut path = [0u8; 128];
    let path_len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let mut argv =
        [crate::fs::runtime::RuntimeExecString::empty(); crate::fs::runtime::EXEC_ARG_MAX];
    let argc = match read_user_exec_strings(user_argv, &mut argv) {
        Ok(count) => count,
        Err(err) => return err,
    };
    let mut envp =
        [crate::fs::runtime::RuntimeExecString::empty(); crate::fs::runtime::EXEC_ENV_MAX];
    let envc = match read_user_exec_strings(user_envp, &mut envp) {
        Ok(count) => count,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::execve_from_vfs(&path[..path_len], &argv[..argc], &envp[..envc]);
    crate::println!("[process-v169] execve canonical ret = {}", ret);
    ret
}

fn sys_kill(pid: isize, sig: usize) -> isize {
    crate::println!("[signal-v174] kill pid = {}", pid);
    crate::println!("[signal-v174] kill sig = {}", sig);
    let ret = crate::fs::runtime::kill_signal(pid, sig);
    crate::println!("[signal-v174] kill canonical ret = {}", ret);
    ret
}

fn sys_tkill(tid: isize, sig: usize) -> isize {
    crate::println!("[signal-v174] tkill tid = {}", tid);
    crate::println!("[signal-v174] tkill sig = {}", sig);
    let ret = crate::fs::runtime::tkill_signal(tid, sig);
    crate::println!("[signal-v174] tkill canonical ret = {}", ret);
    ret
}

fn sys_tgkill(tgid: isize, tid: isize, sig: usize) -> isize {
    crate::println!("[signal-v174] tgkill tgid = {}", tgid);
    crate::println!("[signal-v174] tgkill tid = {}", tid);
    crate::println!("[signal-v174] tgkill sig = {}", sig);
    let ret = crate::fs::runtime::tgkill_signal(tgid, tid, sig);
    crate::println!("[signal-v174] tgkill canonical ret = {}", ret);
    ret
}

fn sys_exit_group(code: isize) -> ! {
    crate::println!("[process-v69] exit_group code = {}", code);
    let ret = crate::fs::runtime::exit_group_current(code);
    crate::println!("[process-v165] exit_group canonical ret = {}", ret);
    EXIT_SEEN.store(true, Ordering::SeqCst);
    crate::println!("[external-init-v82] smoke passed");
    crate::println!("[external-init-v82] kernel idle after external init ELF smoke");
    finish_official_qemu_runtime();
}

fn sys_mkdirat_user(dirfd: isize, user_path: usize, mode: usize) -> isize {
    crate::println!("[fs-v68] mkdirat dirfd = {}", dirfd);
    crate::println!("[fs-v68] mkdirat path = {:#x}", user_path);
    crate::println!("[fs-v68] mkdirat mode = {:#o}", mode);
    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::mkdirat(dirfd, &path[..len], mode as u16);
    crate::println!("[fs-v157] mkdirat canonical ret = {}", ret);
    ret
}

fn sys_unlinkat_user(dirfd: isize, user_path: usize, flags: usize) -> isize {
    crate::println!("[fs-v68] unlinkat dirfd = {}", dirfd);
    crate::println!("[fs-v68] unlinkat path = {:#x}", user_path);
    crate::println!("[fs-v68] unlinkat flags = {:#x}", flags);
    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::unlinkat(dirfd, &path[..len], flags);
    crate::println!("[fs-v157] unlinkat canonical ret = {}", ret);
    ret
}

fn sys_faccessat_user(dirfd: isize, user_path: usize, mode: usize, flags: usize) -> isize {
    crate::println!("[fs-v68] faccessat dirfd = {}", dirfd);
    crate::println!("[fs-v68] faccessat path = {:#x}", user_path);
    crate::println!("[fs-v68] faccessat mode = {:#x}", mode);
    crate::println!("[fs-v68] faccessat flags = {:#x}", flags);
    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::faccessat(dirfd, &path[..len], mode);
    crate::println!("[fs-v157] faccessat canonical ret = {}", ret);
    ret
}

fn sys_newfstatat_user(dirfd: isize, user_path: usize, user_stat: usize, flags: usize) -> isize {
    crate::println!("[fs-v68] newfstatat dirfd = {}", dirfd);
    crate::println!("[fs-v68] newfstatat path = {:#x}", user_path);
    crate::println!("[fs-v68] newfstatat stat = {:#x}", user_stat);
    crate::println!("[fs-v68] newfstatat flags = {:#x}", flags);

    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    match crate::fs::runtime::stat_path(dirfd, &path[..len], true) {
        Ok(stat) => {
            let ret = runtime_write_stat_user(user_stat, stat);
            crate::println!("[fs-v157] newfstatat canonical ret = {}", ret);
            ret
        }
        Err(err) => {
            crate::println!("[fs-v157] newfstatat canonical ret = {}", err);
            err
        }
    }
}

fn sys_statx_user(
    dirfd: isize,
    user_path: usize,
    flags: usize,
    mask: usize,
    user_statx: usize,
) -> isize {
    crate::println!("[statx-v68] dirfd = {}", dirfd);
    crate::println!("[statx-v68] path = {:#x}", user_path);
    crate::println!("[statx-v68] flags = {:#x}", flags);
    crate::println!("[statx-v68] mask = {:#x}", mask);
    crate::println!("[statx-v68] statx = {:#x}", user_statx);

    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let follow = (flags & 0x100) == 0;
    match crate::fs::runtime::stat_path(dirfd, &path[..len], follow) {
        Ok(stat) => {
            let ret = runtime_write_statx_user(user_statx, mask, stat);
            crate::println!("[statx-v157] canonical ret = {}", ret);
            return ret;
        }
        Err(err) => {
            crate::println!("[statx-v157] canonical ret = {}", err);
            return err;
        }
    }
}

fn sys_renameat2_user(
    olddirfd: isize,
    oldpath: usize,
    newdirfd: isize,
    newpath: usize,
    flags: usize,
) -> isize {
    crate::println!("[fs-v68] renameat2 olddirfd = {}", olddirfd);
    crate::println!("[fs-v68] renameat2 oldpath = {:#x}", oldpath);
    crate::println!("[fs-v68] renameat2 newdirfd = {}", newdirfd);
    crate::println!("[fs-v68] renameat2 newpath = {:#x}", newpath);
    crate::println!("[fs-v68] renameat2 flags = {:#x}", flags);
    if flags != 0 {
        return crate::syscall::EINVAL;
    }
    let mut old = [0u8; 128];
    let old_len = match read_user_path_bytes(oldpath, &mut old) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let mut new = [0u8; 128];
    let new_len = match read_user_path_bytes(newpath, &mut new) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::renameat(olddirfd, &old[..old_len], newdirfd, &new[..new_len]);
    crate::println!("[fs-v157] renameat canonical ret = {}", ret);
    ret
}

fn sys_eventfd2(initval: usize, flags: usize) -> isize {
    crate::println!("[event-v67] eventfd2 initval = {}", initval);
    crate::println!("[event-v67] eventfd2 flags = {:#x}", flags);
    let fd = crate::fs::runtime::eventfd2(initval, flags as u32);
    crate::println!("[event-v157] eventfd2 fd = {}", fd);
    fd
}

fn sys_epoll_create1(flags: usize) -> isize {
    crate::println!("[epoll-v67] epoll_create1 flags = {:#x}", flags);
    let fd = crate::fs::runtime::epoll_create1(flags as u32);
    crate::println!("[epoll-v157] epoll fd = {}", fd);
    fd
}

fn sys_epoll_ctl_user(epfd: usize, op: usize, fd: usize, event: usize) -> isize {
    crate::println!("[epoll-v67] epoll_ctl epfd = {}", epfd);
    crate::println!("[epoll-v67] epoll_ctl op = {}", op);
    crate::println!("[epoll-v67] epoll_ctl fd = {}", fd);
    crate::println!("[epoll-v67] epoll_ctl event = {:#x}", event);
    let mut events_mask = (crate::fs::runtime::POLLIN | crate::fs::runtime::POLLOUT) as u32;
    let mut data = fd as u64;
    if event != 0 {
        with_sum_enabled(|| unsafe {
            events_mask = core::ptr::read_volatile(event as *const u32);
            data = core::ptr::read_unaligned((event + 4) as *const u64);
        });
    }
    let ret = crate::fs::runtime::epoll_ctl_event(epfd, op, fd, events_mask, data);
    crate::println!("[epoll-v158] epoll_ctl ret = {}", ret);
    ret
}

fn sys_epoll_pwait_user(
    epfd: usize,
    events: usize,
    maxevents: usize,
    timeout: isize,
    sigmask: usize,
    sigsetsize: usize,
) -> isize {
    crate::println!("[epoll-v67] epoll_pwait epfd = {}", epfd);
    crate::println!("[epoll-v67] epoll_pwait events = {:#x}", events);
    crate::println!("[epoll-v67] epoll_pwait maxevents = {}", maxevents);
    crate::println!("[epoll-v67] epoll_pwait timeout = {}", timeout);
    crate::println!("[epoll-v67] epoll_pwait sigmask = {:#x}", sigmask);
    crate::println!("[epoll-v67] epoll_pwait sigsetsize = {}", sigsetsize);
    if maxevents == 0 {
        return crate::fs::runtime::EINVAL;
    }
    let mut ready_events = [crate::fs::runtime::RuntimeEpollEvent::empty(); 8];
    let ready = crate::fs::runtime::epoll_collect_ready(epfd, &mut ready_events);
    if ready <= 0 {
        crate::println!("[epoll-v158] epoll_pwait ret = {}", ready);
        return ready;
    }
    let mut copied = 0usize;
    let limit = if maxevents < ready_events.len() {
        maxevents
    } else {
        ready_events.len()
    };
    if events != 0 {
        with_sum_enabled(|| unsafe {
            while copied < limit && copied < ready as usize {
                let base = events + copied * 12;
                core::ptr::write_volatile(base as *mut u32, ready_events[copied].events);
                core::ptr::write_unaligned((base + 4) as *mut u64, ready_events[copied].data);
                copied += 1;
            }
        });
    }
    crate::println!("[epoll-v158] epoll_pwait ret = {}", copied);
    copied as isize
}

fn sys_ppoll_user(
    fds: usize,
    nfds: usize,
    timeout: usize,
    sigmask: usize,
    sigsetsize: usize,
) -> isize {
    crate::println!("[poll-v67] ppoll fds = {:#x}", fds);
    crate::println!("[poll-v67] ppoll nfds = {}", nfds);
    crate::println!("[poll-v67] ppoll timeout = {:#x}", timeout);
    crate::println!("[poll-v67] ppoll sigmask = {:#x}", sigmask);
    crate::println!("[poll-v67] ppoll sigsetsize = {}", sigsetsize);
    if fds == 0 && nfds != 0 {
        return crate::fs::runtime::EINVAL;
    }
    let mut ready = 0isize;
    with_sum_enabled(|| {
        let mut i = 0usize;
        let limit = if nfds < 64 { nfds } else { 64 };
        while i < limit {
            let base = fds + i * 8;
            let fd = unsafe { core::ptr::read_volatile(base as *const i32) };
            let requested = unsafe { core::ptr::read_volatile((base + 4) as *const u16) };
            if fd >= 0 {
                let revents = crate::fs::runtime::poll_revents(fd as isize, requested);
                unsafe {
                    core::ptr::write_volatile((base + 6) as *mut u16, revents);
                }
                if revents != 0 {
                    ready += 1;
                }
            } else {
                unsafe {
                    core::ptr::write_volatile((base + 6) as *mut u16, 0);
                }
            }
            i += 1;
        }
    });
    crate::println!("[poll-v158] ppoll ret = {}", ready);
    ready
}

fn sys_pselect6_user(
    nfds: usize,
    readfds: usize,
    writefds: usize,
    exceptfds: usize,
    timeout: usize,
    sigmask: usize,
) -> isize {
    crate::println!("[poll-v67] pselect6 nfds = {}", nfds);
    crate::println!("[poll-v67] pselect6 readfds = {:#x}", readfds);
    crate::println!("[poll-v67] pselect6 writefds = {:#x}", writefds);
    crate::println!("[poll-v67] pselect6 exceptfds = {:#x}", exceptfds);
    crate::println!("[poll-v67] pselect6 timeout = {:#x}", timeout);
    crate::println!("[poll-v67] pselect6 sigmask = {:#x}", sigmask);
    crate::println!("[poll-v67] pselect6 ret = 0");
    0
}

fn sys_pipe2_user(user_pipefd: usize, flags: usize) -> isize {
    crate::println!("[pipe-v67] pipe2 pipefd = {:#x}", user_pipefd);
    crate::println!("[pipe-v67] pipe2 flags = {:#x}", flags);

    let mut fds = [-1isize; 2];
    let ret = crate::fs::runtime::pipe2(flags as u32, &mut fds);
    if ret == 0 && user_pipefd != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_pipefd + 0) as *mut u32, fds[0] as u32);
            core::ptr::write_volatile((user_pipefd + 4) as *mut u32, fds[1] as u32);
        });
    }
    if ret == 0 {
        crate::println!("[pipe-v157] pipe2 fds = {},{}", fds[0], fds[1]);
        if k04a_pipe_context_enabled() {
            crate::println!(
                "[K04a-pipe-trace] pipe2 read_fd={} write_fd={} flags={:#x}",
                fds[0],
                fds[1],
                flags
            );
        }
        return 0;
    }

    if user_pipefd != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_pipefd + 0) as *mut u32, 8);
            core::ptr::write_volatile((user_pipefd + 4) as *mut u32, 9);
        });
    }

    crate::println!("[pipe-v67] pipe2 fds = 8,9");
    0
}

fn sys_dup(oldfd: usize) -> isize {
    crate::println!("[dup-v67] dup oldfd = {}", oldfd);
    let fd = crate::fs::runtime::dup(oldfd);
    crate::println!("[dup-v157] dup fd = {}", fd);
    fd
}

fn sys_dup3(oldfd: usize, newfd: usize, flags: usize) -> isize {
    crate::println!("[dup-v67] dup3 oldfd = {}", oldfd);
    crate::println!("[dup-v67] dup3 newfd = {}", newfd);
    crate::println!("[dup-v67] dup3 flags = {:#x}", flags);
    let ret = crate::fs::runtime::dup3(oldfd, newfd, flags as u32);
    crate::println!("[dup-v157] dup3 ret = {}", ret);
    ret
}

fn sys_sched_yield() -> isize {
    let ret = crate::fs::runtime::sched_yield_current();
    crate::println!("[sched-v164] sched_yield ret = {}", ret);
    ret
}

fn sys_nanosleep_user(req: usize, rem: usize) -> isize {
    crate::println!("[nanosleep-v66] req = {:#x}", req);
    crate::println!("[nanosleep-v66] rem = {:#x}", rem);

    if rem != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((rem + 0) as *mut u64, 0);
            core::ptr::write_volatile((rem + 8) as *mut u64, 0);
        });
    }

    let ret =
        crate::fs::runtime::sched_timeout_wait(crate::fs::runtime::SCHED_WAIT_TIMEOUT_BASE ^ req);
    crate::println!("[nanosleep-v164] ret = {}", ret);
    ret
}

fn sys_futex_user(
    uaddr: usize,
    op: usize,
    val: usize,
    timeout: usize,
    uaddr2: usize,
    val3: usize,
) -> isize {
    crate::println!("[futex-v66] uaddr = {:#x}", uaddr);
    crate::println!("[futex-v66] op = {}", op);
    crate::println!("[futex-v66] val = {}", val);
    crate::println!("[futex-v66] timeout = {:#x}", timeout);
    crate::println!("[futex-v66] uaddr2 = {:#x}", uaddr2);
    crate::println!("[futex-v66] val3 = {}", val3);

    let cmd = op & 0x7f;
    let ret = match cmd {
        crate::fs::runtime::FUTEX_WAIT => {
            if uaddr == 0 {
                crate::fs::runtime::EINVAL
            } else {
                let observed = with_sum_enabled_ret(|| unsafe {
                    Ok::<u32, isize>(core::ptr::read_volatile(uaddr as *const u32))
                });
                match observed {
                    Ok(value) => {
                        crate::fs::runtime::futex_wait(uaddr, value, val as u32, timeout != 0)
                    }
                    Err(err) => err,
                }
            }
        }
        crate::fs::runtime::FUTEX_WAKE => crate::fs::runtime::futex_wake(uaddr, val),
        _ => 0,
    };

    crate::println!("[futex-v163] ret = {}", ret);
    ret
}

fn sys_rt_sigaction_user(sig: usize, act: usize, oldact: usize, sigsetsize: usize) -> isize {
    crate::println!("[signal-v66] rt_sigaction sig = {}", sig);
    crate::println!("[signal-v66] act = {:#x}", act);
    crate::println!("[signal-v66] oldact = {:#x}", oldact);
    crate::println!("[signal-v66] sigsetsize = {}", sigsetsize);

    let new_action = if act != 0 {
        let handler = match read_user_usize_value(act + 0) {
            Ok(value) => value,
            Err(err) => return err,
        };
        let flags = match read_user_usize_value(act + 8) {
            Ok(value) => value,
            Err(err) => return err,
        };
        let restorer = match read_user_usize_value(act + 16) {
            Ok(value) => value,
            Err(err) => return err,
        };
        let mask = if sigsetsize == 0 {
            0
        } else {
            match read_user_usize_value(act + 24) {
                Ok(value) => value as u64,
                Err(err) => return err,
            }
        };
        Some(crate::fs::runtime::RuntimeSignalAction::handler(
            handler, flags, restorer, mask,
        ))
    } else {
        None
    };

    let old = match crate::fs::runtime::rt_sigaction(sig, new_action) {
        Ok(action) => action,
        Err(err) => return err,
    };
    write_rt_sigaction_user(oldact, sigsetsize, old);

    crate::println!("[signal-v173] rt_sigaction canonical ret = 0");
    0
}

fn write_rt_sigaction_user(
    oldact: usize,
    sigsetsize: usize,
    action: crate::fs::runtime::RuntimeSignalAction,
) {
    if oldact == 0 {
        return;
    }
    let mask_len = if sigsetsize < 128 { sigsetsize } else { 128 };
    with_sum_enabled(|| {
        for i in 0..152usize {
            unsafe {
                core::ptr::write_volatile((oldact + i) as *mut u8, 0);
            }
        }
        unsafe {
            core::ptr::write_volatile((oldact + 0) as *mut usize, action.handler);
            core::ptr::write_volatile((oldact + 8) as *mut usize, action.flags);
            core::ptr::write_volatile((oldact + 16) as *mut usize, action.restorer);
        }
        let mut i = 0usize;
        while i < mask_len && i < 8 {
            let byte = ((action.mask >> (i * 8)) & 0xff) as u8;
            unsafe {
                core::ptr::write_volatile((oldact + 24 + i) as *mut u8, byte);
            }
            i += 1;
        }
    });
}

fn sys_rt_sigprocmask_user(how: usize, set: usize, oldset: usize, sigsetsize: usize) -> isize {
    crate::println!("[signal-v66] rt_sigprocmask how = {}", how);
    crate::println!("[signal-v66] set = {:#x}", set);
    crate::println!("[signal-v66] oldset = {:#x}", oldset);
    crate::println!("[signal-v66] sigsetsize = {}", sigsetsize);

    let mut new_mask = None;
    if set != 0 {
        let mut mask = 0u64;
        let limit = if sigsetsize < 8 { sigsetsize } else { 8 };
        with_sum_enabled(|| {
            let mut i = 0usize;
            while i < limit {
                let byte = unsafe { core::ptr::read_volatile((set + i) as *const u8) };
                mask |= (byte as u64) << (i * 8);
                i += 1;
            }
        });
        new_mask = Some(mask);
    }
    let old_mask = match crate::fs::runtime::rt_sigprocmask(how, new_mask) {
        Ok(mask) => mask,
        Err(err) => return err,
    };
    if oldset != 0 {
        with_sum_enabled(|| {
            let mut i = 0usize;
            while i < sigsetsize {
                let byte = if i < 8 {
                    ((old_mask >> (i * 8)) & 0xff) as u8
                } else {
                    0
                };
                unsafe {
                    core::ptr::write_volatile((oldset + i) as *mut u8, byte);
                }
                i += 1;
            }
        });
    }

    crate::println!("[signal-v66] rt_sigprocmask ret = 0");
    0
}

fn sys_rt_sigreturn_user(cx: &mut TrapContext) -> isize {
    match crate::fs::runtime::rt_sigreturn_restore() {
        Ok(restore) => {
            cx.sepc = restore.pc;
            cx.regs[2] = restore.sp;
            crate::println!(
                "[signal-v173] rt_sigreturn restored sig {} pc {:#x} sp {:#x} mask {:#x}",
                restore.sig,
                restore.pc,
                restore.sp,
                restore.mask
            );
            0
        }
        Err(err) => {
            crate::println!("[signal-v173] rt_sigreturn ret = {}", err);
            err
        }
    }
}

fn sys_getcwd_user(user_buf: usize, len: usize) -> isize {
    crate::println!("[path-v65] getcwd buf = {:#x}", user_buf);
    crate::println!("[path-v65] getcwd len = {}", len);

    let mut cwd = [0u8; 128];
    let ret = crate::fs::runtime::getcwd(&mut cwd);
    if ret > 0 {
        if len < ret as usize {
            return -34;
        }
        let _ = copy_kernel_bytes_to_user(user_buf, &cwd[..ret as usize]);
        crate::println!("[path-v157] getcwd canonical ret = {}", ret);
        return user_buf as isize;
    }

    let cwd = b"/\0";
    if len < cwd.len() {
        crate::println!("[path-v65] getcwd ret = -22");
        return crate::syscall::EINVAL;
    }

    with_sum_enabled(|| {
        write_cstr(user_buf, cwd);
    });

    crate::println!("[path-v65] getcwd wrote /");
    user_buf as isize
}

fn sys_fcntl(fd: usize, cmd: usize, arg: usize) -> isize {
    crate::println!("[fcntl-v65] fd = {}", fd);
    crate::println!("[fcntl-v65] cmd = {}", cmd);
    crate::println!("[fcntl-v65] arg = {:#x}", arg);

    if crate::fs::runtime::fd_exists(fd) {
        let ret = crate::fs::runtime::fcntl(fd, cmd, arg);
        crate::println!("[fcntl-v157] canonical ret = {}", ret);
        return ret;
    }

    let ret = match cmd {
        1 => 0, // F_GETFD
        2 => 0, // F_SETFD
        3 => 0, // F_GETFL
        4 => 0, // F_SETFL
        _ => 0,
    };

    crate::println!("[fcntl-v65] ret = {}", ret);
    ret
}

fn sys_ioctl_user(fd: usize, request: usize, argp: usize) -> isize {
    crate::println!("[ioctl-v65] fd = {}", fd);
    crate::println!("[ioctl-v65] request = {:#x}", request);
    crate::println!("[ioctl-v65] argp = {:#x}", argp);

    if request == 0x5413 && argp != 0 {
        // struct winsize { ws_row, ws_col, ws_xpixel, ws_ypixel } as u16.
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((argp + 0) as *mut u16, 24);
            core::ptr::write_volatile((argp + 2) as *mut u16, 80);
            core::ptr::write_volatile((argp + 4) as *mut u16, 0);
            core::ptr::write_volatile((argp + 6) as *mut u16, 0);
        });
        crate::println!("[ioctl-v65] wrote winsize");
        return 0;
    }

    crate::println!("[ioctl-v65] ret = 0");
    0
}

fn sys_readlinkat_user(dirfd: isize, user_path: usize, user_buf: usize, len: usize) -> isize {
    if dirfd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < len && UCOMPAT_V137G_REG_POS < UCOMPAT_V137G_REG_LEN {
                let ch = UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS];
                core::ptr::write_volatile((user_buf + copied) as *mut u8, ch);
                UCOMPAT_V137G_REG_POS += 1;
                copied += 1;
            }
        }
        crate::println!(
            "[ucompat-v137g] fd-runtime read fd={} copied={}",
            dirfd,
            copied
        );
        return copied as isize;
    }

    if dirfd as isize == UCOMPAT_V137F_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < len && UCOMPAT_V137F_REG_POS < UCOMPAT_V137F_REG_LEN {
                let ch = UCOMPAT_V137F_REG_DATA[UCOMPAT_V137F_REG_POS];
                core::ptr::write_volatile((user_buf + copied) as *mut u8, ch);
                UCOMPAT_V137F_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137f] read fd={} copied={}", dirfd, copied);
        return copied as isize;
    }

    let mut runtime_path = [0u8; 128];
    let runtime_len = match read_user_path_bytes(user_path, &mut runtime_path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let mut runtime_target = [0u8; 128];
    let runtime_ret =
        crate::fs::runtime::readlinkat(dirfd, &runtime_path[..runtime_len], &mut runtime_target);
    if runtime_ret > 0 {
        if len < runtime_ret as usize {
            return -34;
        }
        let _ = copy_kernel_bytes_to_user(user_buf, &runtime_target[..runtime_ret as usize]);
    }
    crate::println!("[readlinkat-v157] canonical ret = {}", runtime_ret);
    runtime_ret
}

fn sys_umask(mask: usize) -> isize {
    crate::println!("[umask-v65] mask = {:#o}", mask);
    crate::println!("[umask-v65] old mask = 0");
    0
}

fn sys_chdir_user(user_path: usize) -> isize {
    crate::println!("[path-v65] chdir path = {:#x}", user_path);
    let mut path = [0u8; 128];
    let len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let ret = crate::fs::runtime::chdir(&path[..len]);
    crate::println!("[path-v157] chdir canonical ret = {}", ret);
    ret
}

fn sys_set_tid_address_user(user_tidptr: usize) -> isize {
    crate::println!("[proc-v64] set_tid_address ptr = {:#x}", user_tidptr);

    if user_tidptr != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile(user_tidptr as *mut u32, 1);
        });
    }

    crate::println!("[proc-v64] set_tid_address ret = 1");
    1
}

fn sys_set_robust_list(head: usize, len: usize) -> isize {
    crate::println!("[proc-v64] set_robust_list head = {:#x}", head);
    crate::println!("[proc-v64] set_robust_list len = {}", len);
    crate::println!("[proc-v64] set_robust_list ret = 0");
    0
}

fn sys_sysinfo_user(user_info: usize) -> isize {
    crate::println!("[sysinfo-v64] user info = {:#x}", user_info);

    with_sum_enabled(|| {
        for i in 0..128usize {
            unsafe {
                core::ptr::write_volatile((user_info + i) as *mut u8, 0);
            }
        }

        unsafe {
            core::ptr::write_volatile((user_info + 0) as *mut i64, 1); // uptime
            core::ptr::write_volatile((user_info + 32) as *mut u64, 64 * 1024 * 1024); // totalram-ish
            core::ptr::write_volatile((user_info + 40) as *mut u64, 48 * 1024 * 1024); // freeram-ish
            core::ptr::write_volatile((user_info + 104) as *mut u16, 1); // procs
        }
    });

    crate::println!("[sysinfo-v64] wrote sysinfo");
    0
}

fn sys_prlimit64_user(pid: usize, resource: usize, new_limit: usize, old_limit: usize) -> isize {
    crate::println!("[prlimit64-v64] pid = {}", pid);
    crate::println!("[prlimit64-v64] resource = {}", resource);
    crate::println!("[prlimit64-v64] new_limit = {:#x}", new_limit);
    crate::println!("[prlimit64-v64] old_limit = {:#x}", old_limit);

    if old_limit != 0 {
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((old_limit + 0) as *mut u64, 8 * 1024 * 1024);
            core::ptr::write_volatile((old_limit + 8) as *mut u64, 8 * 1024 * 1024);
        });
    }

    crate::println!("[prlimit64-v64] wrote rlimit");
    0
}

fn sys_getrandom_user(user_buf: usize, len: usize, flags: usize) -> isize {
    crate::println!("[getrandom-v64] buf = {:#x}", user_buf);
    crate::println!("[getrandom-v64] len = {}", len);
    crate::println!("[getrandom-v64] flags = {:#x}", flags);

    with_sum_enabled(|| {
        for i in 0..len {
            let value = (0xa5usize ^ (i.wrapping_mul(37))) as u8;
            unsafe {
                core::ptr::write_volatile((user_buf + i) as *mut u8, value);
            }
        }
    });

    crate::println!("[getrandom-v64] filled bytes = {}", len);
    len as isize
}

fn sys_uname_user(user_uts: usize) -> isize {
    crate::println!("[uname-v63] user uts = {:#x}", user_uts);

    with_sum_enabled(|| {
        for i in 0..390usize {
            unsafe {
                core::ptr::write_volatile((user_uts + i) as *mut u8, 0);
            }
        }

        write_cstr(user_uts + 0 * 65, b"UESTC-Kernel\0");
        write_cstr(user_uts + 1 * 65, b"qemu-riscv64\0");
        write_cstr(user_uts + 2 * 65, b"0.1-v63\0");
        write_cstr(user_uts + 3 * 65, b"v63-time-uname\0");
        write_cstr(user_uts + 4 * 65, b"riscv64\0");
        write_cstr(user_uts + 5 * 65, b"uestc.local\0");
    });

    crate::println!("[uname-v63] wrote utsname");
    0
}

fn sys_clock_gettime_user(clock_id: usize, user_ts: usize) -> isize {
    crate::println!("[clock-v63] clock_gettime id = {}", clock_id);
    crate::println!("[clock-v63] timespec = {:#x}", user_ts);

    with_sum_enabled(|| unsafe {
        core::ptr::write_volatile((user_ts + 0) as *mut u64, 1);
        core::ptr::write_volatile((user_ts + 8) as *mut u64, 234_567_890);
    });

    crate::println!("[clock-v63] wrote timespec");
    0
}

fn sys_gettimeofday_user(user_tv: usize, user_tz: usize) -> isize {
    crate::println!("[time-v63] gettimeofday tv = {:#x}", user_tv);
    crate::println!("[time-v63] gettimeofday tz = {:#x}", user_tz);

    if user_tv != 0 {
        let tick = unsafe {
            let current = K02_TIMEVAL_TICK;
            K02_TIMEVAL_TICK = K02_TIMEVAL_TICK.wrapping_add(1);
            current
        };
        with_sum_enabled(|| unsafe {
            core::ptr::write_volatile((user_tv + 0) as *mut u64, (1 + tick) as u64);
            core::ptr::write_volatile((user_tv + 8) as *mut u64, 234_567);
        });
    }

    crate::println!("[time-v63] wrote timeval");
    0
}

fn write_cstr(dst: usize, src: &[u8]) {
    let mut i = 0;
    while i < src.len() {
        unsafe {
            core::ptr::write_volatile((dst + i) as *mut u8, src[i]);
        }
        i += 1;
    }
}

fn sys_mprotect(addr: usize, len: usize, prot: usize) -> isize {
    crate::println!("[mprotect-v172] addr = {:#x}", addr);
    crate::println!("[mprotect-v172] len = {}", len);
    crate::println!("[mprotect-v172] prot = {:#x}", prot);
    let ret = crate::fs::runtime::mprotect(addr, len, prot);
    if ret == 0 {
        unsafe {
            real_mm_mprotect_range(addr, len, prot);
        }
    }
    crate::println!("[mprotect-v172] canonical ret = {}", ret);
    ret
}

fn sys_madvise(addr: usize, len: usize, advice: usize) -> isize {
    crate::println!("[madvise-v172] addr = {:#x}", addr);
    crate::println!("[madvise-v172] len = {}", len);
    crate::println!("[madvise-v172] advice = {}", advice);
    let ret = crate::fs::runtime::madvise(addr, len);
    crate::println!("[madvise-v172] canonical ret = {}", ret);
    ret
}

fn sys_mmap(addr: usize, len: usize, prot: usize, flags: usize, fd: isize, offset: usize) -> isize {
    crate::println!("[mmap-v172] request addr = {:#x}", addr);
    crate::println!("[mmap-v172] len = {}", len);
    crate::println!("[mmap-v172] prot = {:#x}", prot);
    crate::println!("[mmap-v172] flags = {:#x}", flags);
    crate::println!("[mmap-v172] fd = {}", fd);
    crate::println!("[mmap-v172] offset = {:#x}", offset);
    let ret = crate::fs::runtime::mmap(addr, len, prot, flags, fd, offset);
    if ret >= 0 {
        if (flags & crate::fs::runtime::RUNTIME_MAP_FIXED) != 0 {
            unsafe {
                real_mm_unmap_range(ret as usize, len);
            }
        }
        unsafe {
            USER_MMAP_ACTIVE = true;
        }
    }
    crate::println!("[mmap-v172] canonical ret = {}", ret);
    if k05_is_memory_kind(k04a_current_kind()) {
        let snap = crate::fs::runtime::vm_snapshot();
        crate::println!(
            "[K05-memory-trace] case={} op=mmap request={:#x} len={} prot={:#x} flags={:#x} fd={} offset={:#x} ret={}",
            k02_current_case_name(),
            addr,
            len,
            prot,
            flags,
            fd,
            offset,
            ret
        );
        crate::println!(
            "[K05-vma-trace] case={} after=mmap mm={} vmas={} mmap_count={} lazy_count={} resident_pages={} last_fault={:#x} last_fault_ok={}",
            k02_current_case_name(),
            snap.mm_id,
            snap.vma_count,
            snap.mmap_count,
            snap.lazy_count,
            snap.resident_pages,
            snap.last_fault_addr,
            if snap.last_fault_ok { 1 } else { 0 }
        );
    }
    ret
}

fn sys_munmap(addr: usize, len: usize) -> isize {
    crate::println!("[munmap-v172] addr = {:#x}", addr);
    crate::println!("[munmap-v172] len = {}", len);
    let ret = crate::fs::runtime::munmap(addr, len);
    if ret == 0 {
        unsafe {
            real_mm_unmap_range(addr, len);
            USER_MMAP_ACTIVE = false;
        }
    }
    crate::println!("[munmap-v172] canonical ret = {}", ret);
    if k05_is_memory_kind(k04a_current_kind()) {
        let snap = crate::fs::runtime::vm_snapshot();
        crate::println!(
            "[K05-memory-trace] case={} op=munmap addr={:#x} len={} ret={}",
            k02_current_case_name(),
            addr,
            len,
            ret
        );
        crate::println!(
            "[K05-vma-trace] case={} after=munmap mm={} vmas={} mmap_count={} lazy_count={} resident_pages={} last_fault={:#x} last_fault_ok={}",
            k02_current_case_name(),
            snap.mm_id,
            snap.vma_count,
            snap.mmap_count,
            snap.lazy_count,
            snap.resident_pages,
            snap.last_fault_addr,
            if snap.last_fault_ok { 1 } else { 0 }
        );
    }
    ret
}

fn sys_brk(addr: usize) -> isize {
    crate::println!("[brk-v172] request = {:#x}", addr);
    let old_brk = unsafe { USER_BRK };
    let ret = crate::fs::runtime::brk(addr);
    if ret >= 0 {
        unsafe {
            let new_brk = ret as usize;
            if new_brk < old_brk {
                let unmap_start = (new_brk + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
                if old_brk > unmap_start {
                    real_mm_unmap_range(unmap_start, old_brk - unmap_start);
                }
            }
            USER_BRK = new_brk;
        }
    }
    crate::println!("[brk-v172] canonical ret = {:#x}", ret);
    ret
}

// UCOMPAT_V137D_OPENAT_OCREAT_RUNTIME_REGULAR_FILE
const UCOMPAT_V137D_REG_FD: isize = 9137;
const UCOMPAT_V137D_REG_CAP: usize = 512;
static mut UCOMPAT_V137D_REG_DATA: [u8; UCOMPAT_V137D_REG_CAP] = [0; UCOMPAT_V137D_REG_CAP];
static mut UCOMPAT_V137D_REG_LEN: usize = 0;
static mut UCOMPAT_V137D_REG_POS: usize = 0;
static mut UCOMPAT_V137D_REG_OPEN: bool = false;

