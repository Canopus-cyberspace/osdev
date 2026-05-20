const REAL_UMODE_BASE: usize = 0x4000_0000;
const REAL_UMODE_CODE_OFF: usize = 0x100;
const REAL_UMODE_MSG_OFF: usize = 0x2c0;
const REAL_UMODE_ELF_CAP: usize = 2048;
const REAL_UMODE_STACK_AUXC: usize = 3;
const REAL_UMODE_AT_NULL: usize = 0;
const REAL_UMODE_AT_PAGESZ: usize = 6;
const REAL_UMODE_AT_ENTRY: usize = 9;

const REAL_UMODE_PHASE_IDLE: usize = 0;
const REAL_UMODE_PHASE_V191: usize = 1;
const REAL_UMODE_PHASE_V192_A: usize = 2;
const REAL_UMODE_PHASE_V192_B: usize = 3;
const REAL_UMODE_PHASE_V192_C: usize = 4;
const REAL_UMODE_PHASE_V193_CHILD: usize = 5;
const REAL_UMODE_PHASE_V194_ABI: usize = 6;
const REAL_UMODE_PHASE_V197_LAZY: usize = 7;
const REAL_UMODE_PHASE_V198_RO: usize = 8;
const REAL_UMODE_PHASE_V198_UNMAP: usize = 9;
const REAL_UMODE_PHASE_V200_STRESS: usize = 10;
const REAL_UMODE_PHASE_DONE: usize = 11;
const REAL_UMODE_PHASE_K01_WRITE: usize = 12;
const REAL_UMODE_PHASE_B01_BUSYBOX: usize = 13;

const K01_OFFICIAL_WRITE_PATH: &[u8] = b"/musl/basic/write";
const K01_EXPECTED_STDOUT: &[u8] = b"========== START test_write ==========\nHello operating system contest.\n========== END test_write ==========\n";
const K01_STDOUT_CAP: usize = K02_STDOUT_CAP;
const K01_OFFICIAL_ELF_CAP: usize = 2 * 1024 * 1024;
const K02_STDOUT_CAP: usize = 2048;
const K02_SYSCALL_TRACE_CAP: usize = 64;
const B01_STDOUT_CAP: usize = 8192;
const B01_SYSCALL_TRACE_CAP: usize = 1024;
const B01_BUSYBOX_CASE_COUNT: usize = 39;
const B01_BUSYBOX_ARG_SLOTS: usize = 6;
const B01_BUSYBOX_PATH: &[u8] = b"/musl/busybox";
const B01_BUSYBOX_EMPTY_ARG: &[u8] = b"";
const B01_BUSYBOX_ENV: [&[u8]; 2] = [b"PATH=/musl:/bin:/usr/bin:.", b"HOME=/musl"];
const B01_BUSYBOX_SHA256: &str =
    "bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de";
const B01_REALRUN_KIND_EMPTY: usize = 0;
const B01_REALRUN_KIND_ECHO: usize = 1;
const B01_REALRUN_KIND_PWD: usize = 2;
const B01_REALRUN_KIND_LS: usize = 3;
const B01_REALRUN_KIND_CAT_HELLO: usize = 4;
const B01_REALRUN_KIND_STAT_TEST: usize = 5;
const B01_REALRUN_KIND_FIND_BUSYBOX_CMD: usize = 6;
const B01_REALRUN_KIND_WC_TEST: usize = 7;
const B01_REALRUN_KIND_CONTAINS_HELLO: usize = 8;
const B01_REALRUN_KIND_CUT_C3: usize = 9;
const B01_REALRUN_KIND_OD_TEST: usize = 10;
const B01_REALRUN_KIND_HEXDUMP_TEST: usize = 11;
const B01_REALRUN_KIND_MD5_TEST: usize = 12;
const B01_REALRUN_KIND_BASENAME: usize = 13;
const B01_REALRUN_KIND_DIRNAME: usize = 14;
const B01_REALRUN_KIND_NONEMPTY: usize = 15;
const B01_REALRUN_KIND_WHICH_LS: usize = 16;
const B01_REALRUN_KIND_ANY_STDOUT: usize = 17;
const K02_REALRUN_CASE_COUNT: usize = 31;
const K02_REALRUN_KIND_WRITE: usize = 0;
const K02_REALRUN_KIND_GETPID: usize = 1;
const K02_REALRUN_KIND_GETPPID: usize = 2;
const K02_REALRUN_KIND_UNAME: usize = 3;
const K02_REALRUN_KIND_GETCWD: usize = 4;
const K02_REALRUN_KIND_BRK: usize = 5;
const K02_REALRUN_KIND_GETTIMEOFDAY: usize = 6;
const K02_REALRUN_KIND_TIMES: usize = 7;
const K02_REALRUN_KIND_SLEEP: usize = 8;
const K03_REALRUN_KIND_CLOSE: usize = 9;
const K03_REALRUN_KIND_DUP: usize = 10;
const K03_REALRUN_KIND_DUP2: usize = 11;
const K03_REALRUN_KIND_OPEN: usize = 12;
const K03_REALRUN_KIND_READ: usize = 13;
const K03_REALRUN_KIND_OPENAT: usize = 14;
const K03_REALRUN_KIND_FSTAT: usize = 15;
const K03_REALRUN_KIND_GETDENTS: usize = 16;
const K03_REALRUN_KIND_CHDIR: usize = 17;
const K03_REALRUN_KIND_MKDIR: usize = 18;
const K03_REALRUN_KIND_UNLINK: usize = 19;
const K04A_REALRUN_KIND_PIPE: usize = 20;
const K04A_REALRUN_KIND_YIELD: usize = 21;
const K04A_REALRUN_KIND_WAIT: usize = 22;
const K04A_REALRUN_KIND_WAITPID: usize = 23;
const K04B_REALRUN_KIND_FORK: usize = 24;
const K04B_REALRUN_KIND_CLONE: usize = 25;
const K04B_REALRUN_KIND_EXECVE: usize = 26;
const K05_REALRUN_KIND_MMAP: usize = 27;
const K05_REALRUN_KIND_MOUNT: usize = 28;
const K05_REALRUN_KIND_MUNMAP: usize = 29;
const K05_REALRUN_KIND_UMOUNT: usize = 30;
const K04B_TEST_ECHO_SHA256: &str =
    "a290dd0987d8500b3affde391a27d7de35f97659b04c8e7c5f48b96e62a653aa";
const REAL_UMODE_V191_PATH: &[u8] = b"/umode/v191.elf";
const REAL_UMODE_V192_A_PATH: &[u8] = b"/umode/v192a.elf";
const REAL_UMODE_V192_B_PATH: &[u8] = b"/umode/v192b.elf";
const REAL_UMODE_V192_C_PATH: &[u8] = b"/umode/v192c.elf";
const REAL_UMODE_V193_PATH: &[u8] = b"/umode/v193child.elf";
const REAL_UMODE_V194_PATH: &[u8] = b"/umode/v194abi.elf";
const REAL_UMODE_BAD_PATH: &[u8] = b"/umode/v194bad.elf";
const REAL_UMODE_V197_PATH: &[u8] = b"/umode/v197lazy.elf";
const REAL_UMODE_V198_RO_PATH: &[u8] = b"/umode/v198ro.elf";
const REAL_UMODE_V198_UNMAP_PATH: &[u8] = b"/umode/v198unmap.elf";
const REAL_UMODE_V200_PATH: &[u8] = b"/umode/v200stress.elf";
static mut REAL_UMODE_PHASE: usize = REAL_UMODE_PHASE_IDLE;
static mut REAL_UMODE_V192_PASS_COUNT: usize = 0;
static mut REAL_UMODE_CHILD_PID: usize = 0;
static mut REAL_UMODE_CLOEXEC_FD: isize = -1;
static mut K01_OFFICIAL_ELF: [u8; K01_OFFICIAL_ELF_CAP] = [0; K01_OFFICIAL_ELF_CAP];
static mut K02_CURRENT_CASE: usize = 0;
static mut K02_STDOUT: [[u8; K02_STDOUT_CAP]; K02_REALRUN_CASE_COUNT] =
    [[0; K02_STDOUT_CAP]; K02_REALRUN_CASE_COUNT];
static mut K02_STDOUT_LEN: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_STDOUT_OVERFLOW: [bool; K02_REALRUN_CASE_COUNT] = [false; K02_REALRUN_CASE_COUNT];
static mut K02_EXIT_CODE: [isize; K02_REALRUN_CASE_COUNT] = [isize::MIN; K02_REALRUN_CASE_COUNT];
static mut K02_VERIFIED: [bool; K02_REALRUN_CASE_COUNT] = [false; K02_REALRUN_CASE_COUNT];
static mut K02_ENTERED_UMODE: [bool; K02_REALRUN_CASE_COUNT] = [false; K02_REALRUN_CASE_COUNT];
static mut K02_INODE: [u32; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_FILE_SIZE: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_ENTRY: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_LOAD_BASE: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_LOAD_PAGES: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_LOAD_SEGMENTS: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_SYSCALL_TRACE: [[usize; K02_SYSCALL_TRACE_CAP]; K02_REALRUN_CASE_COUNT] =
    [[0; K02_SYSCALL_TRACE_CAP]; K02_REALRUN_CASE_COUNT];
static mut K02_SYSCALL_TRACE_LEN: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_PAGE_FAULT_COUNT: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_LAST_PAGE_FAULT: [usize; K02_REALRUN_CASE_COUNT] = [0; K02_REALRUN_CASE_COUNT];
static mut K02_TIMEVAL_TICK: usize = 0;
static mut B01_CURRENT_CASE: usize = 0;
static mut B01_STDOUT: [[u8; B01_STDOUT_CAP]; B01_BUSYBOX_CASE_COUNT] =
    [[0; B01_STDOUT_CAP]; B01_BUSYBOX_CASE_COUNT];
static mut B01_STDOUT_LEN: [usize; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_STDOUT_OVERFLOW: [bool; B01_BUSYBOX_CASE_COUNT] = [false; B01_BUSYBOX_CASE_COUNT];
static mut B01_STDERR: [[u8; B01_STDOUT_CAP]; B01_BUSYBOX_CASE_COUNT] =
    [[0; B01_STDOUT_CAP]; B01_BUSYBOX_CASE_COUNT];
static mut B01_STDERR_LEN: [usize; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_STDERR_OVERFLOW: [bool; B01_BUSYBOX_CASE_COUNT] = [false; B01_BUSYBOX_CASE_COUNT];
static mut B01_EXIT_CODE: [isize; B01_BUSYBOX_CASE_COUNT] =
    [isize::MIN; B01_BUSYBOX_CASE_COUNT];
static mut B01_VERIFIED: [bool; B01_BUSYBOX_CASE_COUNT] = [false; B01_BUSYBOX_CASE_COUNT];
static mut B01_ENTERED_UMODE: [bool; B01_BUSYBOX_CASE_COUNT] = [false; B01_BUSYBOX_CASE_COUNT];
static mut B01_INODE: [u32; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_FILE_SIZE: [usize; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_ENTRY: [usize; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_LOAD_BASE: [usize; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_LOAD_PAGES: [usize; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_LOAD_SEGMENTS: [usize; B01_BUSYBOX_CASE_COUNT] = [0; B01_BUSYBOX_CASE_COUNT];
static mut B01_SYSCALL_TRACE: [[usize; B01_SYSCALL_TRACE_CAP]; B01_BUSYBOX_CASE_COUNT] =
    [[0; B01_SYSCALL_TRACE_CAP]; B01_BUSYBOX_CASE_COUNT];
static mut B01_SYSCALL_TRACE_LEN: [usize; B01_BUSYBOX_CASE_COUNT] =
    [0; B01_BUSYBOX_CASE_COUNT];

const K04A_FORK_CONTEXTS: usize = 4;
const K04A_PARENT_SLOT: isize = -1;
const EMPTY_TRAP_CONTEXT: TrapContext = TrapContext {
    regs: [0; 32],
    sstatus: 0,
    sepc: 0,
};
static mut K04A_CHILD_CTX: [TrapContext; K04A_FORK_CONTEXTS] =
    [EMPTY_TRAP_CONTEXT; K04A_FORK_CONTEXTS];
static mut K04A_CHILD_ACTIVE: [bool; K04A_FORK_CONTEXTS] = [false; K04A_FORK_CONTEXTS];
static mut K04A_CHILD_EXITED: [bool; K04A_FORK_CONTEXTS] = [false; K04A_FORK_CONTEXTS];
static mut K04A_CHILD_PID: [usize; K04A_FORK_CONTEXTS] = [0; K04A_FORK_CONTEXTS];
static mut K04A_PARENT_CTX: TrapContext = EMPTY_TRAP_CONTEXT;
static mut K04A_PARENT_BLOCKED: bool = false;
static mut K04A_CURRENT_SLOT: isize = K04A_PARENT_SLOT;

#[derive(Copy, Clone)]
struct RealUmodeLoad {
    entry: usize,
    load_start: usize,
    file_size: usize,
    stack_pointer: usize,
}

#[derive(Copy, Clone)]
struct K02RealRunCaseSpec {
    test_name: &'static str,
    program_name: &'static [u8],
    program_path: &'static [u8],
    test_marker: &'static [u8],
    required_pattern: &'static [u8],
    elf_sha256: &'static str,
    expected_kind: usize,
}

#[derive(Copy, Clone)]
struct B01BusyboxCaseSpec {
    case_name: &'static str,
    command_label: &'static str,
    argv: [&'static [u8]; B01_BUSYBOX_ARG_SLOTS],
    argc: usize,
    expected_kind: usize,
    expected_exit: isize,
}

pub struct RealRunResult {
    pub program_path: &'static str,
    pub elf_sha256: &'static str,
    pub entry_pc: usize,
    pub loaded_segments: usize,
    pub argv: &'static str,
    pub envp: &'static str,
    pub auxv: &'static str,
    pub entered_umode: bool,
    pub syscall_trace: &'static str,
    pub page_fault_trace: &'static str,
    pub stdout: &'static str,
    pub stderr: &'static str,
    pub exit_code: isize,
    pub final_task_state: &'static str,
}

const K02_REALRUN_CASES: [K02RealRunCaseSpec; K02_REALRUN_CASE_COUNT] = [
    K02RealRunCaseSpec {
        test_name: "test_write",
        program_name: b"write",
        program_path: b"/musl/basic/write",
        test_marker: b"test_write",
        required_pattern: b"Hello operating system contest.\n",
        elf_sha256: "aa0a6577d698fe1e3fa361f19d5f4abf0e944cb639e3cda3f4da380352883fb9",
        expected_kind: K02_REALRUN_KIND_WRITE,
    },
    K02RealRunCaseSpec {
        test_name: "test_getpid",
        program_name: b"getpid",
        program_path: b"/musl/basic/getpid",
        test_marker: b"test_getpid",
        required_pattern: b"getpid success.",
        elf_sha256: "1ccbca7716ca6f636c2515acded448b565c73897021a11eb55089ce98e928c38",
        expected_kind: K02_REALRUN_KIND_GETPID,
    },
    K02RealRunCaseSpec {
        test_name: "test_getppid",
        program_name: b"getppid",
        program_path: b"/musl/basic/getppid",
        test_marker: b"test_getppid",
        required_pattern: b"getppid success.",
        elf_sha256: "43b2854c296de4220e9434ce18215c8a1cbd1b916c9e8c916870abb83b6fe595",
        expected_kind: K02_REALRUN_KIND_GETPPID,
    },
    K02RealRunCaseSpec {
        test_name: "test_uname",
        program_name: b"uname",
        program_path: b"/musl/basic/uname",
        test_marker: b"test_uname",
        required_pattern: b"Uname: %s %s %s %s %s %s",
        elf_sha256: "b4e88c0c4d86b894908967e35bc7735244d0553b48d594f6ab47d7508841e075",
        expected_kind: K02_REALRUN_KIND_UNAME,
    },
    K02RealRunCaseSpec {
        test_name: "test_getcwd",
        program_name: b"getcwd",
        program_path: b"/musl/basic/getcwd",
        test_marker: b"test_getcwd",
        required_pattern: b"getcwd: %s successfully!",
        elf_sha256: "1455ccfb491b3718f01153011505a6c01bb171d68b64af36885c5213352d3c02",
        expected_kind: K02_REALRUN_KIND_GETCWD,
    },
    K02RealRunCaseSpec {
        test_name: "test_brk",
        program_name: b"brk",
        program_path: b"/musl/basic/brk",
        test_marker: b"test_brk",
        required_pattern: b"Before alloc,heap pos:",
        elf_sha256: "3756dce8d8734a564300ca4404e6b80136f3d369672c03afaffab63da1fae086",
        expected_kind: K02_REALRUN_KIND_BRK,
    },
    K02RealRunCaseSpec {
        test_name: "test_gettimeofday",
        program_name: b"gettimeofday",
        program_path: b"/musl/basic/gettimeofday",
        test_marker: b"test_gettimeofday",
        required_pattern: b"gettimeofday success.",
        elf_sha256: "bcd04cded0d8c078fe9d8160e836beea053860326cfcbf791ac4d069920388ac",
        expected_kind: K02_REALRUN_KIND_GETTIMEOFDAY,
    },
    K02RealRunCaseSpec {
        test_name: "test_times",
        program_name: b"times",
        program_path: b"/musl/basic/times",
        test_marker: b"test_times",
        required_pattern: b"mytimes success",
        elf_sha256: "ee23dcbba06f63f2f28ce47f4d154c9b36df919bd961050f7f2f4f5642c66647",
        expected_kind: K02_REALRUN_KIND_TIMES,
    },
    K02RealRunCaseSpec {
        test_name: "test_sleep",
        program_name: b"sleep",
        program_path: b"/musl/basic/sleep",
        test_marker: b"test_sleep",
        required_pattern: b"sleep success.",
        elf_sha256: "36da514075b584fde0533a27315c3db6a81712e7d5476805967e7d7c0e9273eb",
        expected_kind: K02_REALRUN_KIND_SLEEP,
    },
    K02RealRunCaseSpec {
        test_name: "test_close",
        program_name: b"close",
        program_path: b"/musl/basic/close",
        test_marker: b"test_close",
        required_pattern: b"  close %d success.",
        elf_sha256: "e39fdcd43045f1fe143a960878886e5c3858c7641c6be3d8b4271385da9ba08d",
        expected_kind: K03_REALRUN_KIND_CLOSE,
    },
    K02RealRunCaseSpec {
        test_name: "test_dup",
        program_name: b"dup",
        program_path: b"/musl/basic/dup",
        test_marker: b"test_dup",
        required_pattern: b"  new fd is %d.",
        elf_sha256: "e7e03fe1ee3b2b88b0d37682b8f95370152a92d35672f4e046878f909613da55",
        expected_kind: K03_REALRUN_KIND_DUP,
    },
    K02RealRunCaseSpec {
        test_name: "test_dup2",
        program_name: b"dup2",
        program_path: b"/musl/basic/dup2",
        test_marker: b"test_dup2",
        required_pattern: b"  from fd 100",
        elf_sha256: "cfabb21987882957ded46f9b6fd1d8945a68e995f5cd333f580eee4970b0e26f",
        expected_kind: K03_REALRUN_KIND_DUP2,
    },
    K02RealRunCaseSpec {
        test_name: "test_open",
        program_name: b"open",
        program_path: b"/musl/basic/open",
        test_marker: b"test_open",
        required_pattern: b"./text.txt",
        elf_sha256: "97fb9c699b73213b7397ef5ba3c9acda213f2fa348b3c1324375dc2e31d5b2bc",
        expected_kind: K03_REALRUN_KIND_OPEN,
    },
    K02RealRunCaseSpec {
        test_name: "test_read",
        program_name: b"read",
        program_path: b"/musl/basic/read",
        test_marker: b"test_read",
        required_pattern: b"./text.txt",
        elf_sha256: "12a96731b1fd0ecd52068b53aad639c99d10f35196428aae2c7fbc2fc7f66e72",
        expected_kind: K03_REALRUN_KIND_READ,
    },
    K02RealRunCaseSpec {
        test_name: "test_openat",
        program_name: b"openat",
        program_path: b"/musl/basic/openat",
        test_marker: b"test_openat",
        required_pattern: b"openat success.",
        elf_sha256: "7fa639078c6f9355c436c260a42b0bc412efd74e1e7df74808e68f22a80a2f48",
        expected_kind: K03_REALRUN_KIND_OPENAT,
    },
    K02RealRunCaseSpec {
        test_name: "test_fstat",
        program_name: b"fstat",
        program_path: b"/musl/basic/fstat",
        test_marker: b"test_fstat",
        required_pattern: b"fstat ret: %d",
        elf_sha256: "8cfb851a75a671f4814acb608262e27d01ec83baa57628bb206dd7dacaf48ed8",
        expected_kind: K03_REALRUN_KIND_FSTAT,
    },
    K02RealRunCaseSpec {
        test_name: "test_getdents",
        program_name: b"getdents",
        program_path: b"/musl/basic/getdents",
        test_marker: b"test_getdents",
        required_pattern: b"getdents success.",
        elf_sha256: "60729cb4dd0d079097d05bf2e2c3fd6b75d2dcfcc3968d7c8eb2caa6393952fe",
        expected_kind: K03_REALRUN_KIND_GETDENTS,
    },
    K02RealRunCaseSpec {
        test_name: "test_chdir",
        program_name: b"chdir",
        program_path: b"/musl/basic/chdir",
        test_marker: b"test_chdir",
        required_pattern: b"chdir ret: %d",
        elf_sha256: "39a1b66fd4f58f7683be099068fe74da98a8ba1188f1484d172634dcd5c71d82",
        expected_kind: K03_REALRUN_KIND_CHDIR,
    },
    K02RealRunCaseSpec {
        test_name: "test_mkdir",
        program_name: b"mkdir_",
        program_path: b"/musl/basic/mkdir_",
        test_marker: b"test_mkdir",
        required_pattern: b"  mkdir success.",
        elf_sha256: "f532c781e21d403f05e090b92be9645d7ceb373dfeae49515e5ae7a91f04bc15",
        expected_kind: K03_REALRUN_KIND_MKDIR,
    },
    K02RealRunCaseSpec {
        test_name: "test_unlink",
        program_name: b"unlink",
        program_path: b"/musl/basic/unlink",
        test_marker: b"test_unlink",
        required_pattern: b"  unlink success!",
        elf_sha256: "dbf54b203a28c5dc531ede43bee4153813589b25462bbd9c500dc28229ccc710",
        expected_kind: K03_REALRUN_KIND_UNLINK,
    },
    K02RealRunCaseSpec {
        test_name: "test_pipe",
        program_name: b"pipe",
        program_path: b"/musl/basic/pipe",
        test_marker: b"test_pipe",
        required_pattern: b"  Write to pipe successfully.",
        elf_sha256: "e1f3d28c6e640aa897637c69e5c89d01b44d7d1fc4f897d2053c7e812600e19a",
        expected_kind: K04A_REALRUN_KIND_PIPE,
    },
    K02RealRunCaseSpec {
        test_name: "test_yield",
        program_name: b"yield",
        program_path: b"/musl/basic/yield",
        test_marker: b"test_yield",
        required_pattern: b"sched_yield",
        elf_sha256: "0ab9f0744d5755c9d077deda0c703d0b07df905fc311c0298d9e533223e352a1",
        expected_kind: K04A_REALRUN_KIND_YIELD,
    },
    K02RealRunCaseSpec {
        test_name: "test_wait",
        program_name: b"wait",
        program_path: b"/musl/basic/wait",
        test_marker: b"test_wait",
        required_pattern: b"wait child success.",
        elf_sha256: "0853594952004e4f07b109cd5d9a7b7be951002fe3caadd02756d188f2c323ad",
        expected_kind: K04A_REALRUN_KIND_WAIT,
    },
    K02RealRunCaseSpec {
        test_name: "test_waitpid",
        program_name: b"waitpid",
        program_path: b"/musl/basic/waitpid",
        test_marker: b"test_waitpid",
        required_pattern: b"waitpid successfully.",
        elf_sha256: "7d304b041b8113bbdc9939fb89f72629a9dc80b75527c3ae1e1c1d77f9e15569",
        expected_kind: K04A_REALRUN_KIND_WAITPID,
    },
    K02RealRunCaseSpec {
        test_name: "test_fork",
        program_name: b"fork",
        program_path: b"/musl/basic/fork",
        test_marker: b"test_fork",
        required_pattern: b"  child process.",
        elf_sha256: "e649f07b31204f7f7a916f625dee73005b07a77b74f48765ecc5c541995d27ad",
        expected_kind: K04B_REALRUN_KIND_FORK,
    },
    K02RealRunCaseSpec {
        test_name: "test_clone",
        program_name: b"clone",
        program_path: b"/musl/basic/clone",
        test_marker: b"test_clone",
        required_pattern: b"clone process successfully.",
        elf_sha256: "307cf13a2ee35eb2d3566c630205459a25b05c2e8f6dd28b027e13d30e1f8ee2",
        expected_kind: K04B_REALRUN_KIND_CLONE,
    },
    K02RealRunCaseSpec {
        test_name: "test_execve",
        program_name: b"execve",
        program_path: b"/musl/basic/execve",
        test_marker: b"test_execve",
        required_pattern: b"test_echo",
        elf_sha256: "66fd950624b50163fcdb728de44b2ad9838dfb77d35e5f30a87c8e89d9653325",
        expected_kind: K04B_REALRUN_KIND_EXECVE,
    },
    K02RealRunCaseSpec {
        test_name: "test_mmap",
        program_name: b"mmap",
        program_path: b"/musl/basic/mmap",
        test_marker: b"test_mmap",
        required_pattern: b"mmap content: %s",
        elf_sha256: "cd3ed8bd069653962d06f07830b6b6a1b36ccddeb20c6a3eb7d57d86f951f5f9",
        expected_kind: K05_REALRUN_KIND_MMAP,
    },
    K02RealRunCaseSpec {
        test_name: "test_mount",
        program_name: b"mount",
        program_path: b"/musl/basic/mount",
        test_marker: b"test_mount",
        required_pattern: b"mount successfully",
        elf_sha256: "f47e7e127ec5d2b3ce3542c46e79952f7f3182df0370e8941533993bd81a7a73",
        expected_kind: K05_REALRUN_KIND_MOUNT,
    },
    K02RealRunCaseSpec {
        test_name: "test_munmap",
        program_name: b"munmap",
        program_path: b"/musl/basic/munmap",
        test_marker: b"test_munmap",
        required_pattern: b"munmap successfully!",
        elf_sha256: "607b91c6d000724034e279a1bdddf520d0fd663a3c9c1c7e203a911d7ed17bda",
        expected_kind: K05_REALRUN_KIND_MUNMAP,
    },
    K02RealRunCaseSpec {
        test_name: "test_umount",
        program_name: b"umount",
        program_path: b"/musl/basic/umount",
        test_marker: b"test_umount",
        required_pattern: b"umount success.",
        elf_sha256: "e0129c46e2746ad63e15174c368e83e0ede9540d7323a630e0d1b28ba9090834",
        expected_kind: K05_REALRUN_KIND_UMOUNT,
    },
];

const B01_BUSYBOX_CASES: [B01BusyboxCaseSpec; B01_BUSYBOX_CASE_COUNT] = [
    B01BusyboxCaseSpec {
        case_name: "busybox_true",
        command_label: "busybox true",
        argv: [B01_BUSYBOX_PATH, b"true", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_echo_independent",
        command_label: "busybox echo #### independent command test",
        argv: [B01_BUSYBOX_PATH, b"echo", b"#### independent command test", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_ECHO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_pwd",
        command_label: "busybox pwd",
        argv: [B01_BUSYBOX_PATH, b"pwd", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_PWD,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_ls",
        command_label: "busybox ls",
        argv: [B01_BUSYBOX_PATH, b"ls", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_LS,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_cat_test_txt",
        command_label: "busybox cat test.txt",
        argv: [B01_BUSYBOX_PATH, b"cat", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_CAT_HELLO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_touch_test_txt",
        command_label: "busybox touch test.txt",
        argv: [B01_BUSYBOX_PATH, b"touch", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_rm_test_txt",
        command_label: "busybox rm test.txt",
        argv: [B01_BUSYBOX_PATH, b"rm", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_mkdir_test_dir",
        command_label: "busybox mkdir test_dir",
        argv: [B01_BUSYBOX_PATH, b"mkdir", b"test_dir", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_rmdir_test",
        command_label: "busybox rmdir test",
        argv: [B01_BUSYBOX_PATH, b"rmdir", b"test", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_mv_test_dir_test",
        command_label: "busybox mv test_dir test",
        argv: [B01_BUSYBOX_PATH, b"mv", b"test_dir", b"test", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 4,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_cp_busybox_cmd_bak",
        command_label: "busybox cp busybox_cmd.txt busybox_cmd.bak",
        argv: [B01_BUSYBOX_PATH, b"cp", b"busybox_cmd.txt", b"busybox_cmd.bak", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 4,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_rm_busybox_cmd_bak",
        command_label: "busybox rm busybox_cmd.bak",
        argv: [B01_BUSYBOX_PATH, b"rm", b"busybox_cmd.bak", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_stat_test_txt",
        command_label: "busybox stat test.txt",
        argv: [B01_BUSYBOX_PATH, b"stat", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_STAT_TEST,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_find_busybox_cmd",
        command_label: "busybox find -name busybox_cmd.txt",
        argv: [B01_BUSYBOX_PATH, b"find", b"-name", b"busybox_cmd.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 4,
        expected_kind: B01_REALRUN_KIND_FIND_BUSYBOX_CMD,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_wc_test_txt",
        command_label: "busybox wc test.txt",
        argv: [B01_BUSYBOX_PATH, b"wc", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_WC_TEST,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_head_test_txt",
        command_label: "busybox head test.txt",
        argv: [B01_BUSYBOX_PATH, b"head", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_CAT_HELLO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_tail_test_txt",
        command_label: "busybox tail test.txt",
        argv: [B01_BUSYBOX_PATH, b"tail", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_CAT_HELLO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_sort_test_txt",
        command_label: "busybox sort test.txt",
        argv: [B01_BUSYBOX_PATH, b"sort", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_CAT_HELLO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_uniq_test_txt",
        command_label: "busybox uniq test.txt",
        argv: [B01_BUSYBOX_PATH, b"uniq", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_CAT_HELLO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_grep_hello_cmd",
        command_label: "busybox grep hello busybox_cmd.txt",
        argv: [B01_BUSYBOX_PATH, b"grep", b"hello", b"busybox_cmd.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 4,
        expected_kind: B01_REALRUN_KIND_CONTAINS_HELLO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_cut_c3_test_txt",
        command_label: "busybox cut -c 3 test.txt",
        argv: [B01_BUSYBOX_PATH, b"cut", b"-c", b"3", b"test.txt", B01_BUSYBOX_EMPTY_ARG],
        argc: 5,
        expected_kind: B01_REALRUN_KIND_CUT_C3,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_od_test_txt",
        command_label: "busybox od test.txt",
        argv: [B01_BUSYBOX_PATH, b"od", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_OD_TEST,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_hexdump_c_test_txt",
        command_label: "busybox hexdump -C test.txt",
        argv: [B01_BUSYBOX_PATH, b"hexdump", b"-C", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 4,
        expected_kind: B01_REALRUN_KIND_HEXDUMP_TEST,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_md5sum_test_txt",
        command_label: "busybox md5sum test.txt",
        argv: [B01_BUSYBOX_PATH, b"md5sum", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_MD5_TEST,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_strings_test_txt",
        command_label: "busybox strings test.txt",
        argv: [B01_BUSYBOX_PATH, b"strings", b"test.txt", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_CAT_HELLO,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_basename_aaa_bbb",
        command_label: "busybox basename /aaa/bbb",
        argv: [B01_BUSYBOX_PATH, b"basename", b"/aaa/bbb", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_BASENAME,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_dirname_aaa_bbb",
        command_label: "busybox dirname /aaa/bbb",
        argv: [B01_BUSYBOX_PATH, b"dirname", b"/aaa/bbb", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_DIRNAME,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_date",
        command_label: "busybox date",
        argv: [B01_BUSYBOX_PATH, b"date", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_cal",
        command_label: "busybox cal",
        argv: [B01_BUSYBOX_PATH, b"cal", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_df",
        command_label: "busybox df",
        argv: [B01_BUSYBOX_PATH, b"df", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_du",
        command_label: "busybox du",
        argv: [B01_BUSYBOX_PATH, b"du", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_dmesg",
        command_label: "busybox dmesg",
        argv: [B01_BUSYBOX_PATH, b"dmesg", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_ANY_STDOUT,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_ps",
        command_label: "busybox ps",
        argv: [B01_BUSYBOX_PATH, b"ps", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_free",
        command_label: "busybox free",
        argv: [B01_BUSYBOX_PATH, b"free", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_uptime",
        command_label: "busybox uptime",
        argv: [B01_BUSYBOX_PATH, b"uptime", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_uname",
        command_label: "busybox uname",
        argv: [B01_BUSYBOX_PATH, b"uname", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_NONEMPTY,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_which_ls",
        command_label: "busybox which ls",
        argv: [B01_BUSYBOX_PATH, b"which", b"ls", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_WHICH_LS,
        expected_exit: 0,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_false",
        command_label: "busybox false",
        argv: [B01_BUSYBOX_PATH, b"false", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 2,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 1,
    },
    B01BusyboxCaseSpec {
        case_name: "busybox_sleep_1",
        command_label: "busybox sleep 1",
        argv: [B01_BUSYBOX_PATH, b"sleep", b"1", B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG, B01_BUSYBOX_EMPTY_ARG],
        argc: 3,
        expected_kind: B01_REALRUN_KIND_EMPTY,
        expected_exit: 0,
    },
];
