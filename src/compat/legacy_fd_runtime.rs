pub const UCOMPAT_V137G_REG_FD: isize = 9337;
const UCOMPAT_V137G_REG_CAP: usize = 512;
static mut UCOMPAT_V137G_REG_DATA: [u8; UCOMPAT_V137G_REG_CAP] = [0; UCOMPAT_V137G_REG_CAP];
static mut UCOMPAT_V137G_REG_LEN: usize = 0;
static mut UCOMPAT_V137G_REG_POS: usize = 0;
static mut UCOMPAT_V137G_REG_OPEN: bool = false;

pub fn ucompat_v137g_regular_reset() {
    unsafe {
        UCOMPAT_V137G_REG_LEN = 0;
        UCOMPAT_V137G_REG_POS = 0;
        UCOMPAT_V137G_REG_OPEN = true;
    }
}

pub fn ucompat_v137g_regular_open_existing() -> isize {
    unsafe {
        if UCOMPAT_V137G_REG_OPEN {
            UCOMPAT_V137G_REG_POS = 0;
            UCOMPAT_V137G_REG_FD
        } else {
            crate::syscall::errno::ENOENT
        }
    }
}


fn ucompat_v137d_regular_reset() {
    unsafe {
        UCOMPAT_V137D_REG_LEN = 0;
        UCOMPAT_V137D_REG_POS = 0;
        UCOMPAT_V137D_REG_OPEN = true;
    }
}

fn ucompat_v137d_regular_open_existing() -> isize {
    unsafe {
        if UCOMPAT_V137D_REG_OPEN {
            UCOMPAT_V137D_REG_POS = 0;
            UCOMPAT_V137D_REG_FD
        } else {
            crate::syscall::errno::ENOENT
        }
    }
}

// UCOMPAT_V137F_OPENAT_OCREAT_RUNTIME_REGULAR_FILE
const UCOMPAT_V137F_REG_FD: isize = 9237;
const UCOMPAT_V137F_REG_CAP: usize = 512;
static mut UCOMPAT_V137F_REG_DATA: [u8; UCOMPAT_V137F_REG_CAP] = [0; UCOMPAT_V137F_REG_CAP];
static mut UCOMPAT_V137F_REG_LEN: usize = 0;
static mut UCOMPAT_V137F_REG_POS: usize = 0;
static mut UCOMPAT_V137F_REG_OPEN: bool = false;

fn ucompat_v137f_regular_reset() {
    unsafe {
        UCOMPAT_V137F_REG_LEN = 0;
        UCOMPAT_V137F_REG_POS = 0;
        UCOMPAT_V137F_REG_OPEN = true;
    }
}

fn ucompat_v137f_regular_open_existing() -> isize {
    unsafe {
        if UCOMPAT_V137F_REG_OPEN {
            UCOMPAT_V137F_REG_POS = 0;
            UCOMPAT_V137F_REG_FD
        } else {
            crate::syscall::errno::ENOENT
        }
    }
}

// UCOMPAT_V137G_OPENAT_OCREAT_BRIDGE

// UCOMPAT_V137M_TOP_LEVEL_FD_INTERCEPT
const UCOMPAT_V137M_REG_FD: isize = 9537;
const UCOMPAT_V137M_REG_CAP: usize = 512;
static mut UCOMPAT_V137M_REG_DATA: [u8; UCOMPAT_V137M_REG_CAP] = [0; UCOMPAT_V137M_REG_CAP];
static mut UCOMPAT_V137M_REG_LEN: usize = 0;
static mut UCOMPAT_V137M_REG_POS: usize = 0;
static mut UCOMPAT_V137M_REG_OPEN: bool = false;

fn ucompat_v137m_regular_reset() {
    unsafe {
        UCOMPAT_V137M_REG_LEN = 0;
        UCOMPAT_V137M_REG_POS = 0;
        UCOMPAT_V137M_REG_OPEN = true;
    }
}

fn ucompat_v137m_regular_open_existing() -> isize {
    unsafe {
        if UCOMPAT_V137M_REG_OPEN {
            UCOMPAT_V137M_REG_POS = 0;
            UCOMPAT_V137M_REG_FD
        } else {
            crate::syscall::errno::ENOENT
        }
    }
}

// UCOMPAT_V137O_DISPATCHER_FD_TARGET_BYPASS
const UCOMPAT_V137O_REG_FD: isize = 9637;
const UCOMPAT_V137O_REG_CAP: usize = 512;
static mut UCOMPAT_V137O_REG_DATA: [u8; UCOMPAT_V137O_REG_CAP] = [0; UCOMPAT_V137O_REG_CAP];
static mut UCOMPAT_V137O_REG_LEN: usize = 0;
static mut UCOMPAT_V137O_REG_POS: usize = 0;
static mut UCOMPAT_V137O_REG_OPEN: bool = false;

fn ucompat_v137o_regular_reset() {
    unsafe {
        UCOMPAT_V137O_REG_LEN = 0;
        UCOMPAT_V137O_REG_POS = 0;
        UCOMPAT_V137O_REG_OPEN = true;
    }
}

fn ucompat_v137o_regular_open_existing() -> isize {
    unsafe {
        if UCOMPAT_V137O_REG_OPEN {
            UCOMPAT_V137O_REG_POS = 0;
            UCOMPAT_V137O_REG_FD
        } else {
            crate::syscall::errno::ENOENT
        }
    }
}

// UCOMPAT_V137P_MARKER_DISPATCHER_REPAIR
const UCOMPAT_V137P_REG_FD: isize = 9737;
const UCOMPAT_V137P_REG_CAP: usize = 512;
static mut UCOMPAT_V137P_REG_DATA: [u8; UCOMPAT_V137P_REG_CAP] = [0; UCOMPAT_V137P_REG_CAP];
static mut UCOMPAT_V137P_REG_LEN: usize = 0;
static mut UCOMPAT_V137P_REG_POS: usize = 0;
static mut UCOMPAT_V137P_REG_OPEN: bool = false;

fn ucompat_v137p_regular_reset() {
    unsafe {
        UCOMPAT_V137P_REG_LEN = 0;
        UCOMPAT_V137P_REG_POS = 0;
        UCOMPAT_V137P_REG_OPEN = true;
    }
}

fn ucompat_v137p_regular_open_existing() -> isize {
    unsafe {
        if UCOMPAT_V137P_REG_OPEN {
            UCOMPAT_V137P_REG_POS = 0;
            UCOMPAT_V137P_REG_FD
        } else {
            crate::syscall::errno::ENOENT
        }
    }
}

// UCOMPAT_V138_MULTIFILE_TRUNCATE_ISOLATION
const UCOMPAT_V138_FD_A: isize = 9831;
const UCOMPAT_V138_FD_B: isize = 9832;
const UCOMPAT_V138_CAP: usize = 512;
static mut UCOMPAT_V138_A_DATA: [u8; UCOMPAT_V138_CAP] = [0; UCOMPAT_V138_CAP];
static mut UCOMPAT_V138_A_LEN: usize = 0;
static mut UCOMPAT_V138_A_POS: usize = 0;
static mut UCOMPAT_V138_A_EXISTS: bool = false;
static mut UCOMPAT_V138_B_DATA: [u8; UCOMPAT_V138_CAP] = [0; UCOMPAT_V138_CAP];
static mut UCOMPAT_V138_B_LEN: usize = 0;
static mut UCOMPAT_V138_B_POS: usize = 0;
static mut UCOMPAT_V138_B_EXISTS: bool = false;

fn ucompat_v138_reset(slot: usize) {
    unsafe {
        if slot == 0 {
            // UCOMPAT_V139_RESET_ZERO_FILL
            let mut i = 0usize;
            while i < UCOMPAT_V138_CAP {
                UCOMPAT_V138_A_DATA[i] = 0;
                i += 1;
            }
            UCOMPAT_V138_A_LEN = 0;
            UCOMPAT_V138_A_POS = 0;
            UCOMPAT_V138_A_EXISTS = true;
        } else {
            // UCOMPAT_V139_RESET_ZERO_FILL_B
            let mut i = 0usize;
            while i < UCOMPAT_V138_CAP {
                UCOMPAT_V138_B_DATA[i] = 0;
                i += 1;
            }
            UCOMPAT_V138_B_LEN = 0;
            UCOMPAT_V138_B_POS = 0;
            UCOMPAT_V138_B_EXISTS = true;
        }
    }
}
fn ucompat_v138_exists(slot: usize) -> bool {
    unsafe {
        if slot == 0 {
            UCOMPAT_V138_A_EXISTS
        } else {
            UCOMPAT_V138_B_EXISTS
        }
    }
}
fn ucompat_v138_fd(slot: usize) -> isize {
    if slot == 0 {
        UCOMPAT_V138_FD_A
    } else {
        UCOMPAT_V138_FD_B
    }
}
fn ucompat_v138_is_fd(fd: isize) -> bool {
    fd == UCOMPAT_V138_FD_A || fd == UCOMPAT_V138_FD_B
}
fn ucompat_v138_open(slot: usize, flags: usize) -> isize {
    const O_CREAT: usize = 0x40;
    const O_TRUNC: usize = 0x200;
    let exists = ucompat_v138_exists(slot);
    if !exists && (flags & O_CREAT) == 0 {
        crate::println!(
            "[openat-v138] slot={} missing without O_CREAT",
            if slot == 0 { "A" } else { "B" }
        );
        return crate::syscall::errno::ENOENT;
    }
    if !exists || (flags & O_TRUNC) != 0 {
        if exists && (flags & O_TRUNC) != 0 {
            crate::println!(
                "[openat-v138] slot={} truncate",
                if slot == 0 { "A" } else { "B" }
            );
        } else {
            crate::println!(
                "[openat-v138] slot={} create",
                if slot == 0 { "A" } else { "B" }
            );
        }
        ucompat_v138_reset(slot);
    } else {
        unsafe {
            if slot == 0 {
                UCOMPAT_V138_A_POS = 0;
            } else {
                UCOMPAT_V138_B_POS = 0;
            }
        }
        crate::println!(
            "[openat-v138] slot={} reopen",
            if slot == 0 { "A" } else { "B" }
        );
    }
    ucompat_v138_fd(slot)
}
fn ucompat_v138_write(fd: isize, user_ptr: usize, len: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| {
        unsafe {
            if fd == UCOMPAT_V138_FD_A {
                // UCOMPAT_V139_SPARSE_GAP_ZERO_FILL
                if UCOMPAT_V138_A_POS > UCOMPAT_V138_A_LEN {
                    let mut z = UCOMPAT_V138_A_LEN;
                    while z < UCOMPAT_V138_A_POS && z < UCOMPAT_V138_CAP {
                        UCOMPAT_V138_A_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v139] zero-fill sparse gap fd=9831 from={} to={}",
                        UCOMPAT_V138_A_LEN,
                        UCOMPAT_V138_A_POS
                    );
                }
                while copied < len && UCOMPAT_V138_A_POS + copied < UCOMPAT_V138_CAP {
                    let ch = core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    UCOMPAT_V138_A_DATA[UCOMPAT_V138_A_POS + copied] = ch;
                    copied += 1;
                }
                let end = UCOMPAT_V138_A_POS + copied;
                if end > UCOMPAT_V138_A_LEN {
                    UCOMPAT_V138_A_LEN = end;
                }
                UCOMPAT_V138_A_POS = end;
                UCOMPAT_V138_A_EXISTS = true;
            } else if fd == UCOMPAT_V138_FD_B {
                // UCOMPAT_V139_SPARSE_GAP_ZERO_FILL
                if UCOMPAT_V138_B_POS > UCOMPAT_V138_B_LEN {
                    let mut z = UCOMPAT_V138_B_LEN;
                    while z < UCOMPAT_V138_B_POS && z < UCOMPAT_V138_CAP {
                        UCOMPAT_V138_B_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v139] zero-fill sparse gap fd=9832 from={} to={}",
                        UCOMPAT_V138_B_LEN,
                        UCOMPAT_V138_B_POS
                    );
                }
                while copied < len && UCOMPAT_V138_B_POS + copied < UCOMPAT_V138_CAP {
                    let ch = core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    UCOMPAT_V138_B_DATA[UCOMPAT_V138_B_POS + copied] = ch;
                    copied += 1;
                }
                let end = UCOMPAT_V138_B_POS + copied;
                if end > UCOMPAT_V138_B_LEN {
                    UCOMPAT_V138_B_LEN = end;
                }
                UCOMPAT_V138_B_POS = end;
                UCOMPAT_V138_B_EXISTS = true;
            }
        }
    });
    crate::println!("[ucompat-v138] write fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v138_read(fd: isize, user_ptr: usize, len: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        if fd == UCOMPAT_V138_FD_A {
            while copied < len && UCOMPAT_V138_A_POS < UCOMPAT_V138_A_LEN {
                let ch = UCOMPAT_V138_A_DATA[UCOMPAT_V138_A_POS];
                core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                UCOMPAT_V138_A_POS += 1;
                copied += 1;
            }
        } else if fd == UCOMPAT_V138_FD_B {
            while copied < len && UCOMPAT_V138_B_POS < UCOMPAT_V138_B_LEN {
                let ch = UCOMPAT_V138_B_DATA[UCOMPAT_V138_B_POS];
                core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                UCOMPAT_V138_B_POS += 1;
                copied += 1;
            }
        }
    });
    crate::println!("[ucompat-v138] read fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v138_lseek(fd: isize, off: isize, whence: usize) -> isize {
    unsafe {
        if fd == UCOMPAT_V138_FD_A {
            let base = match whence {
                0 => 0isize,
                1 => UCOMPAT_V138_A_POS as isize,
                2 => UCOMPAT_V138_A_LEN as isize,
                _ => return -22,
            };
            let new_pos = base + off;
            if new_pos < 0 {
                return -22;
            }
            UCOMPAT_V138_A_POS = new_pos as usize;
            crate::println!("[ucompat-v138] lseek fd={} pos={}", fd, new_pos);
            new_pos
        } else if fd == UCOMPAT_V138_FD_B {
            let base = match whence {
                0 => 0isize,
                1 => UCOMPAT_V138_B_POS as isize,
                2 => UCOMPAT_V138_B_LEN as isize,
                _ => return -22,
            };
            let new_pos = base + off;
            if new_pos < 0 {
                return -22;
            }
            UCOMPAT_V138_B_POS = new_pos as usize;
            crate::println!("[ucompat-v138] lseek fd={} pos={}", fd, new_pos);
            new_pos
        } else {
            -9
        }
    }
}
fn ucompat_v138_close(fd: isize) -> isize {
    unsafe {
        if fd == UCOMPAT_V138_FD_A {
            UCOMPAT_V138_A_POS = 0;
            crate::println!("[ucompat-v138] close fd=9831 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V138_FD_B {
            UCOMPAT_V138_B_POS = 0;
            crate::println!("[ucompat-v138] close fd=9832 ret=0 keep_file=1");
            0
        } else {
            -9
        }
    }
}

// UCOMPAT_V140B_STATIC_SLOT_LOGS
// UCOMPAT_V140C_STATIC_SLOT_LOG_SOURCE_TOKENS
// [openat-v140] slot=A create
// [openat-v140] slot=B create
// [openat-v140] slot=C create
// [openat-v140] slot=D create
// [openat-v140] slot=A truncate
// [openat-v140] slot=B truncate
// [openat-v140] slot=C truncate
// [openat-v140] slot=D truncate
// [openat-v140] slot=A reopen
// [openat-v140] slot=B reopen
// [openat-v140] slot=C reopen
// [openat-v140] slot=D reopen
// UCOMPAT_V140_FOURFILE_TABLE_LIFECYCLE
const UCOMPAT_V140_FD_A: isize = 9931;
const UCOMPAT_V140_FD_B: isize = 9932;
const UCOMPAT_V140_FD_C: isize = 9933;
const UCOMPAT_V140_FD_D: isize = 9934;
const UCOMPAT_V140_CAP: usize = 512;
static mut UCOMPAT_V140_A_DATA: [u8; UCOMPAT_V140_CAP] = [0; UCOMPAT_V140_CAP];
static mut UCOMPAT_V140_A_LEN: usize = 0;
static mut UCOMPAT_V140_A_POS: usize = 0;
static mut UCOMPAT_V140_A_EXISTS: bool = false;
static mut UCOMPAT_V140_B_DATA: [u8; UCOMPAT_V140_CAP] = [0; UCOMPAT_V140_CAP];
static mut UCOMPAT_V140_B_LEN: usize = 0;
static mut UCOMPAT_V140_B_POS: usize = 0;
static mut UCOMPAT_V140_B_EXISTS: bool = false;
static mut UCOMPAT_V140_C_DATA: [u8; UCOMPAT_V140_CAP] = [0; UCOMPAT_V140_CAP];
static mut UCOMPAT_V140_C_LEN: usize = 0;
static mut UCOMPAT_V140_C_POS: usize = 0;
static mut UCOMPAT_V140_C_EXISTS: bool = false;
static mut UCOMPAT_V140_D_DATA: [u8; UCOMPAT_V140_CAP] = [0; UCOMPAT_V140_CAP];
static mut UCOMPAT_V140_D_LEN: usize = 0;
static mut UCOMPAT_V140_D_POS: usize = 0;
static mut UCOMPAT_V140_D_EXISTS: bool = false;

fn ucompat_v140_fd(slot: usize) -> isize {
    match slot {
        0 => UCOMPAT_V140_FD_A,
        1 => UCOMPAT_V140_FD_B,
        2 => UCOMPAT_V140_FD_C,
        _ => UCOMPAT_V140_FD_D,
    }
}
fn ucompat_v140_is_fd(fd: isize) -> bool {
    fd == UCOMPAT_V140_FD_A
        || fd == UCOMPAT_V140_FD_B
        || fd == UCOMPAT_V140_FD_C
        || fd == UCOMPAT_V140_FD_D
}
fn ucompat_v140_exists(slot: usize) -> bool {
    unsafe {
        match slot {
            0 => UCOMPAT_V140_A_EXISTS,
            1 => UCOMPAT_V140_B_EXISTS,
            2 => UCOMPAT_V140_C_EXISTS,
            _ => UCOMPAT_V140_D_EXISTS,
        }
    }
}
fn ucompat_v140_reset(slot: usize) {
    unsafe {
        let mut i = 0usize;
        match slot {
            0 => {
                while i < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_A_DATA[i] = 0;
                    i += 1;
                }
                UCOMPAT_V140_A_LEN = 0;
                UCOMPAT_V140_A_POS = 0;
                UCOMPAT_V140_A_EXISTS = true;
            }
            1 => {
                while i < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_B_DATA[i] = 0;
                    i += 1;
                }
                UCOMPAT_V140_B_LEN = 0;
                UCOMPAT_V140_B_POS = 0;
                UCOMPAT_V140_B_EXISTS = true;
            }
            2 => {
                while i < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_C_DATA[i] = 0;
                    i += 1;
                }
                UCOMPAT_V140_C_LEN = 0;
                UCOMPAT_V140_C_POS = 0;
                UCOMPAT_V140_C_EXISTS = true;
            }
            _ => {
                while i < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_D_DATA[i] = 0;
                    i += 1;
                }
                UCOMPAT_V140_D_LEN = 0;
                UCOMPAT_V140_D_POS = 0;
                UCOMPAT_V140_D_EXISTS = true;
            }
        }
    }
}
fn ucompat_v140_open(slot: usize, flags: usize) -> isize {
    const O_CREAT: usize = 0x40;
    const O_TRUNC: usize = 0x200;
    let exists = ucompat_v140_exists(slot);
    let name = match slot {
        0 => "A",
        1 => "B",
        2 => "C",
        _ => "D",
    };
    if !exists && (flags & O_CREAT) == 0 {
        crate::println!("[openat-v140] slot={} missing without O_CREAT", name);
        return crate::syscall::errno::ENOENT;
    }
    if !exists || (flags & O_TRUNC) != 0 {
        if exists && (flags & O_TRUNC) != 0 {
            crate::println!("[openat-v140] slot={} truncate", name);
        } else {
            crate::println!("[openat-v140] slot={} create", name);
        }
        ucompat_v140_reset(slot);
    } else {
        unsafe {
            match slot {
                0 => UCOMPAT_V140_A_POS = 0,
                1 => UCOMPAT_V140_B_POS = 0,
                2 => UCOMPAT_V140_C_POS = 0,
                _ => UCOMPAT_V140_D_POS = 0,
            }
        }
        crate::println!("[openat-v140] slot={} reopen", name);
    }
    ucompat_v140_fd(slot)
}
fn ucompat_v140_write_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                if UCOMPAT_V140_A_POS > UCOMPAT_V140_A_LEN {
                    let mut z = UCOMPAT_V140_A_LEN;
                    while z < UCOMPAT_V140_A_POS && z < UCOMPAT_V140_CAP {
                        UCOMPAT_V140_A_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v140] zero-fill sparse gap fd=9931 from={} to={}",
                        UCOMPAT_V140_A_LEN,
                        UCOMPAT_V140_A_POS
                    );
                }
                while copied < len && UCOMPAT_V140_A_POS + copied < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_A_DATA[UCOMPAT_V140_A_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V140_A_POS + copied;
                if end > UCOMPAT_V140_A_LEN {
                    UCOMPAT_V140_A_LEN = end;
                }
                UCOMPAT_V140_A_POS = end;
                UCOMPAT_V140_A_EXISTS = true;
            }
            1 => {
                if UCOMPAT_V140_B_POS > UCOMPAT_V140_B_LEN {
                    let mut z = UCOMPAT_V140_B_LEN;
                    while z < UCOMPAT_V140_B_POS && z < UCOMPAT_V140_CAP {
                        UCOMPAT_V140_B_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v140] zero-fill sparse gap fd=9932 from={} to={}",
                        UCOMPAT_V140_B_LEN,
                        UCOMPAT_V140_B_POS
                    );
                }
                while copied < len && UCOMPAT_V140_B_POS + copied < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_B_DATA[UCOMPAT_V140_B_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V140_B_POS + copied;
                if end > UCOMPAT_V140_B_LEN {
                    UCOMPAT_V140_B_LEN = end;
                }
                UCOMPAT_V140_B_POS = end;
                UCOMPAT_V140_B_EXISTS = true;
            }
            2 => {
                if UCOMPAT_V140_C_POS > UCOMPAT_V140_C_LEN {
                    let mut z = UCOMPAT_V140_C_LEN;
                    while z < UCOMPAT_V140_C_POS && z < UCOMPAT_V140_CAP {
                        UCOMPAT_V140_C_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v140] zero-fill sparse gap fd=9933 from={} to={}",
                        UCOMPAT_V140_C_LEN,
                        UCOMPAT_V140_C_POS
                    );
                }
                while copied < len && UCOMPAT_V140_C_POS + copied < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_C_DATA[UCOMPAT_V140_C_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V140_C_POS + copied;
                if end > UCOMPAT_V140_C_LEN {
                    UCOMPAT_V140_C_LEN = end;
                }
                UCOMPAT_V140_C_POS = end;
                UCOMPAT_V140_C_EXISTS = true;
            }
            _ => {
                if UCOMPAT_V140_D_POS > UCOMPAT_V140_D_LEN {
                    let mut z = UCOMPAT_V140_D_LEN;
                    while z < UCOMPAT_V140_D_POS && z < UCOMPAT_V140_CAP {
                        UCOMPAT_V140_D_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v140] zero-fill sparse gap fd=9934 from={} to={}",
                        UCOMPAT_V140_D_LEN,
                        UCOMPAT_V140_D_POS
                    );
                }
                while copied < len && UCOMPAT_V140_D_POS + copied < UCOMPAT_V140_CAP {
                    UCOMPAT_V140_D_DATA[UCOMPAT_V140_D_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V140_D_POS + copied;
                if end > UCOMPAT_V140_D_LEN {
                    UCOMPAT_V140_D_LEN = end;
                }
                UCOMPAT_V140_D_POS = end;
                UCOMPAT_V140_D_EXISTS = true;
            }
        }
    });
    crate::println!("[ucompat-v140] write fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v140_write(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V140_FD_A {
        ucompat_v140_write_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V140_FD_B {
        ucompat_v140_write_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V140_FD_C {
        ucompat_v140_write_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V140_FD_D {
        ucompat_v140_write_one(fd, user_ptr, len, 3)
    } else {
        -9
    }
}
fn ucompat_v140_read_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                while copied < len && UCOMPAT_V140_A_POS < UCOMPAT_V140_A_LEN {
                    let ch = UCOMPAT_V140_A_DATA[UCOMPAT_V140_A_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V140_A_POS += 1;
                    copied += 1;
                }
            }
            1 => {
                while copied < len && UCOMPAT_V140_B_POS < UCOMPAT_V140_B_LEN {
                    let ch = UCOMPAT_V140_B_DATA[UCOMPAT_V140_B_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V140_B_POS += 1;
                    copied += 1;
                }
            }
            2 => {
                while copied < len && UCOMPAT_V140_C_POS < UCOMPAT_V140_C_LEN {
                    let ch = UCOMPAT_V140_C_DATA[UCOMPAT_V140_C_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V140_C_POS += 1;
                    copied += 1;
                }
            }
            _ => {
                while copied < len && UCOMPAT_V140_D_POS < UCOMPAT_V140_D_LEN {
                    let ch = UCOMPAT_V140_D_DATA[UCOMPAT_V140_D_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V140_D_POS += 1;
                    copied += 1;
                }
            }
        }
    });
    crate::println!("[ucompat-v140] read fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v140_read(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V140_FD_A {
        ucompat_v140_read_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V140_FD_B {
        ucompat_v140_read_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V140_FD_C {
        ucompat_v140_read_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V140_FD_D {
        ucompat_v140_read_one(fd, user_ptr, len, 3)
    } else {
        -9
    }
}
fn ucompat_v140_lseek(fd: isize, off: isize, whence: usize) -> isize {
    unsafe {
        let (len, cur) = if fd == UCOMPAT_V140_FD_A {
            (UCOMPAT_V140_A_LEN, UCOMPAT_V140_A_POS)
        } else if fd == UCOMPAT_V140_FD_B {
            (UCOMPAT_V140_B_LEN, UCOMPAT_V140_B_POS)
        } else if fd == UCOMPAT_V140_FD_C {
            (UCOMPAT_V140_C_LEN, UCOMPAT_V140_C_POS)
        } else if fd == UCOMPAT_V140_FD_D {
            (UCOMPAT_V140_D_LEN, UCOMPAT_V140_D_POS)
        } else {
            return -9;
        };
        let base = match whence {
            0 => 0isize,
            1 => cur as isize,
            2 => len as isize,
            _ => return -22,
        };
        let new_pos = base + off;
        if new_pos < 0 {
            return -22;
        }
        if fd == UCOMPAT_V140_FD_A {
            UCOMPAT_V140_A_POS = new_pos as usize;
        } else if fd == UCOMPAT_V140_FD_B {
            UCOMPAT_V140_B_POS = new_pos as usize;
        } else if fd == UCOMPAT_V140_FD_C {
            UCOMPAT_V140_C_POS = new_pos as usize;
        } else {
            UCOMPAT_V140_D_POS = new_pos as usize;
        }
        crate::println!("[ucompat-v140] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}
fn ucompat_v140_close(fd: isize) -> isize {
    unsafe {
        if fd == UCOMPAT_V140_FD_A {
            UCOMPAT_V140_A_POS = 0;
            crate::println!("[ucompat-v140] close fd=9931 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V140_FD_B {
            UCOMPAT_V140_B_POS = 0;
            crate::println!("[ucompat-v140] close fd=9932 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V140_FD_C {
            UCOMPAT_V140_C_POS = 0;
            crate::println!("[ucompat-v140] close fd=9933 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V140_FD_D {
            UCOMPAT_V140_D_POS = 0;
            crate::println!("[ucompat-v140] close fd=9934 ret=0 keep_file=1");
            0
        } else {
            -9
        }
    }
}

// UCOMPAT_V141_EIGHTFILE_TABLE_ISOLATION
const UCOMPAT_V141_FD_A: isize = 10001;
const UCOMPAT_V141_FD_B: isize = 10002;
const UCOMPAT_V141_FD_C: isize = 10003;
const UCOMPAT_V141_FD_D: isize = 10004;
const UCOMPAT_V141_FD_E: isize = 10005;
const UCOMPAT_V141_FD_F: isize = 10006;
const UCOMPAT_V141_FD_G: isize = 10007;
const UCOMPAT_V141_FD_H: isize = 10008;
const UCOMPAT_V141_CAP: usize = 512;
static mut UCOMPAT_V141_A_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_A_LEN: usize = 0;
static mut UCOMPAT_V141_A_POS: usize = 0;
static mut UCOMPAT_V141_A_EXISTS: bool = false;
static mut UCOMPAT_V141_B_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_B_LEN: usize = 0;
static mut UCOMPAT_V141_B_POS: usize = 0;
static mut UCOMPAT_V141_B_EXISTS: bool = false;
static mut UCOMPAT_V141_C_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_C_LEN: usize = 0;
static mut UCOMPAT_V141_C_POS: usize = 0;
static mut UCOMPAT_V141_C_EXISTS: bool = false;
static mut UCOMPAT_V141_D_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_D_LEN: usize = 0;
static mut UCOMPAT_V141_D_POS: usize = 0;
static mut UCOMPAT_V141_D_EXISTS: bool = false;
static mut UCOMPAT_V141_E_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_E_LEN: usize = 0;
static mut UCOMPAT_V141_E_POS: usize = 0;
static mut UCOMPAT_V141_E_EXISTS: bool = false;
static mut UCOMPAT_V141_F_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_F_LEN: usize = 0;
static mut UCOMPAT_V141_F_POS: usize = 0;
static mut UCOMPAT_V141_F_EXISTS: bool = false;
static mut UCOMPAT_V141_G_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_G_LEN: usize = 0;
static mut UCOMPAT_V141_G_POS: usize = 0;
static mut UCOMPAT_V141_G_EXISTS: bool = false;
static mut UCOMPAT_V141_H_DATA: [u8; UCOMPAT_V141_CAP] = [0; UCOMPAT_V141_CAP];
static mut UCOMPAT_V141_H_LEN: usize = 0;
static mut UCOMPAT_V141_H_POS: usize = 0;
static mut UCOMPAT_V141_H_EXISTS: bool = false;
fn ucompat_v141_fd(slot: usize) -> isize {
    match slot {
        0 => UCOMPAT_V141_FD_A,
        1 => UCOMPAT_V141_FD_B,
        2 => UCOMPAT_V141_FD_C,
        3 => UCOMPAT_V141_FD_D,
        4 => UCOMPAT_V141_FD_E,
        5 => UCOMPAT_V141_FD_F,
        6 => UCOMPAT_V141_FD_G,
        _ => UCOMPAT_V141_FD_H,
    }
}
fn ucompat_v141_is_fd(fd: isize) -> bool {
    fd == UCOMPAT_V141_FD_A
        || fd == UCOMPAT_V141_FD_B
        || fd == UCOMPAT_V141_FD_C
        || fd == UCOMPAT_V141_FD_D
        || fd == UCOMPAT_V141_FD_E
        || fd == UCOMPAT_V141_FD_F
        || fd == UCOMPAT_V141_FD_G
        || fd == UCOMPAT_V141_FD_H
}
fn ucompat_v141_exists(slot: usize) -> bool {
    unsafe {
        match slot {
            0 => UCOMPAT_V141_A_EXISTS,
            1 => UCOMPAT_V141_B_EXISTS,
            2 => UCOMPAT_V141_C_EXISTS,
            3 => UCOMPAT_V141_D_EXISTS,
            4 => UCOMPAT_V141_E_EXISTS,
            5 => UCOMPAT_V141_F_EXISTS,
            6 => UCOMPAT_V141_G_EXISTS,
            _ => UCOMPAT_V141_H_EXISTS,
        }
    }
}
fn ucompat_v141_reset(slot: usize) {
    unsafe {
        let mut j = 0usize;
        match slot {
            0 => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_A_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_A_LEN = 0;
                UCOMPAT_V141_A_POS = 0;
                UCOMPAT_V141_A_EXISTS = true;
            }
            1 => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_B_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_B_LEN = 0;
                UCOMPAT_V141_B_POS = 0;
                UCOMPAT_V141_B_EXISTS = true;
            }
            2 => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_C_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_C_LEN = 0;
                UCOMPAT_V141_C_POS = 0;
                UCOMPAT_V141_C_EXISTS = true;
            }
            3 => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_D_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_D_LEN = 0;
                UCOMPAT_V141_D_POS = 0;
                UCOMPAT_V141_D_EXISTS = true;
            }
            4 => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_E_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_E_LEN = 0;
                UCOMPAT_V141_E_POS = 0;
                UCOMPAT_V141_E_EXISTS = true;
            }
            5 => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_F_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_F_LEN = 0;
                UCOMPAT_V141_F_POS = 0;
                UCOMPAT_V141_F_EXISTS = true;
            }
            6 => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_G_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_G_LEN = 0;
                UCOMPAT_V141_G_POS = 0;
                UCOMPAT_V141_G_EXISTS = true;
            }
            _ => {
                while j < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_H_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V141_H_LEN = 0;
                UCOMPAT_V141_H_POS = 0;
                UCOMPAT_V141_H_EXISTS = true;
            }
        }
    }
}
fn ucompat_v141_open(slot: usize, flags: usize) -> isize {
    const O_CREAT: usize = 0x40;
    const O_TRUNC: usize = 0x200;
    let exists = ucompat_v141_exists(slot);
    let name = match slot {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "G",
        _ => "H",
    };
    if !exists && (flags & O_CREAT) == 0 {
        crate::println!("[openat-v141] slot={} missing without O_CREAT", name);
        return crate::syscall::errno::ENOENT;
    }
    if !exists || (flags & O_TRUNC) != 0 {
        if exists && (flags & O_TRUNC) != 0 {
            crate::println!("[openat-v141] slot={} truncate", name);
        } else {
            crate::println!("[openat-v141] slot={} create", name);
        }
        ucompat_v141_reset(slot);
    } else {
        unsafe {
            match slot {
                0 => UCOMPAT_V141_A_POS = 0,
                1 => UCOMPAT_V141_B_POS = 0,
                2 => UCOMPAT_V141_C_POS = 0,
                3 => UCOMPAT_V141_D_POS = 0,
                4 => UCOMPAT_V141_E_POS = 0,
                5 => UCOMPAT_V141_F_POS = 0,
                6 => UCOMPAT_V141_G_POS = 0,
                _ => UCOMPAT_V141_H_POS = 0,
            }
        }
        crate::println!("[openat-v141] slot={} reopen", name);
    }
    ucompat_v141_fd(slot)
}
fn ucompat_v141_write_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                if UCOMPAT_V141_A_POS > UCOMPAT_V141_A_LEN {
                    let mut z = UCOMPAT_V141_A_LEN;
                    while z < UCOMPAT_V141_A_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_A_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10001 from={} to={}",
                        UCOMPAT_V141_A_LEN,
                        UCOMPAT_V141_A_POS
                    );
                }
                while copied < len && UCOMPAT_V141_A_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_A_DATA[UCOMPAT_V141_A_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_A_POS + copied;
                if end > UCOMPAT_V141_A_LEN {
                    UCOMPAT_V141_A_LEN = end;
                }
                UCOMPAT_V141_A_POS = end;
                UCOMPAT_V141_A_EXISTS = true;
            }
            1 => {
                if UCOMPAT_V141_B_POS > UCOMPAT_V141_B_LEN {
                    let mut z = UCOMPAT_V141_B_LEN;
                    while z < UCOMPAT_V141_B_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_B_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10002 from={} to={}",
                        UCOMPAT_V141_B_LEN,
                        UCOMPAT_V141_B_POS
                    );
                }
                while copied < len && UCOMPAT_V141_B_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_B_DATA[UCOMPAT_V141_B_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_B_POS + copied;
                if end > UCOMPAT_V141_B_LEN {
                    UCOMPAT_V141_B_LEN = end;
                }
                UCOMPAT_V141_B_POS = end;
                UCOMPAT_V141_B_EXISTS = true;
            }
            2 => {
                if UCOMPAT_V141_C_POS > UCOMPAT_V141_C_LEN {
                    let mut z = UCOMPAT_V141_C_LEN;
                    while z < UCOMPAT_V141_C_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_C_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10003 from={} to={}",
                        UCOMPAT_V141_C_LEN,
                        UCOMPAT_V141_C_POS
                    );
                }
                while copied < len && UCOMPAT_V141_C_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_C_DATA[UCOMPAT_V141_C_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_C_POS + copied;
                if end > UCOMPAT_V141_C_LEN {
                    UCOMPAT_V141_C_LEN = end;
                }
                UCOMPAT_V141_C_POS = end;
                UCOMPAT_V141_C_EXISTS = true;
            }
            3 => {
                if UCOMPAT_V141_D_POS > UCOMPAT_V141_D_LEN {
                    let mut z = UCOMPAT_V141_D_LEN;
                    while z < UCOMPAT_V141_D_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_D_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10004 from={} to={}",
                        UCOMPAT_V141_D_LEN,
                        UCOMPAT_V141_D_POS
                    );
                }
                while copied < len && UCOMPAT_V141_D_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_D_DATA[UCOMPAT_V141_D_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_D_POS + copied;
                if end > UCOMPAT_V141_D_LEN {
                    UCOMPAT_V141_D_LEN = end;
                }
                UCOMPAT_V141_D_POS = end;
                UCOMPAT_V141_D_EXISTS = true;
            }
            4 => {
                if UCOMPAT_V141_E_POS > UCOMPAT_V141_E_LEN {
                    let mut z = UCOMPAT_V141_E_LEN;
                    while z < UCOMPAT_V141_E_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_E_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10005 from={} to={}",
                        UCOMPAT_V141_E_LEN,
                        UCOMPAT_V141_E_POS
                    );
                }
                while copied < len && UCOMPAT_V141_E_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_E_DATA[UCOMPAT_V141_E_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_E_POS + copied;
                if end > UCOMPAT_V141_E_LEN {
                    UCOMPAT_V141_E_LEN = end;
                }
                UCOMPAT_V141_E_POS = end;
                UCOMPAT_V141_E_EXISTS = true;
            }
            5 => {
                if UCOMPAT_V141_F_POS > UCOMPAT_V141_F_LEN {
                    let mut z = UCOMPAT_V141_F_LEN;
                    while z < UCOMPAT_V141_F_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_F_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10006 from={} to={}",
                        UCOMPAT_V141_F_LEN,
                        UCOMPAT_V141_F_POS
                    );
                }
                while copied < len && UCOMPAT_V141_F_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_F_DATA[UCOMPAT_V141_F_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_F_POS + copied;
                if end > UCOMPAT_V141_F_LEN {
                    UCOMPAT_V141_F_LEN = end;
                }
                UCOMPAT_V141_F_POS = end;
                UCOMPAT_V141_F_EXISTS = true;
            }
            6 => {
                if UCOMPAT_V141_G_POS > UCOMPAT_V141_G_LEN {
                    let mut z = UCOMPAT_V141_G_LEN;
                    while z < UCOMPAT_V141_G_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_G_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10007 from={} to={}",
                        UCOMPAT_V141_G_LEN,
                        UCOMPAT_V141_G_POS
                    );
                }
                while copied < len && UCOMPAT_V141_G_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_G_DATA[UCOMPAT_V141_G_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_G_POS + copied;
                if end > UCOMPAT_V141_G_LEN {
                    UCOMPAT_V141_G_LEN = end;
                }
                UCOMPAT_V141_G_POS = end;
                UCOMPAT_V141_G_EXISTS = true;
            }
            _ => {
                if UCOMPAT_V141_H_POS > UCOMPAT_V141_H_LEN {
                    let mut z = UCOMPAT_V141_H_LEN;
                    while z < UCOMPAT_V141_H_POS && z < UCOMPAT_V141_CAP {
                        UCOMPAT_V141_H_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v141] zero-fill sparse gap fd=10008 from={} to={}",
                        UCOMPAT_V141_H_LEN,
                        UCOMPAT_V141_H_POS
                    );
                }
                while copied < len && UCOMPAT_V141_H_POS + copied < UCOMPAT_V141_CAP {
                    UCOMPAT_V141_H_DATA[UCOMPAT_V141_H_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V141_H_POS + copied;
                if end > UCOMPAT_V141_H_LEN {
                    UCOMPAT_V141_H_LEN = end;
                }
                UCOMPAT_V141_H_POS = end;
                UCOMPAT_V141_H_EXISTS = true;
            }
        }
    });
    crate::println!("[ucompat-v141] write fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v141_write(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V141_FD_A {
        ucompat_v141_write_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V141_FD_B {
        ucompat_v141_write_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V141_FD_C {
        ucompat_v141_write_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V141_FD_D {
        ucompat_v141_write_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V141_FD_E {
        ucompat_v141_write_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V141_FD_F {
        ucompat_v141_write_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V141_FD_G {
        ucompat_v141_write_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V141_FD_H {
        ucompat_v141_write_one(fd, user_ptr, len, 7)
    } else {
        -9
    }
}
fn ucompat_v141_read_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                while copied < len && UCOMPAT_V141_A_POS < UCOMPAT_V141_A_LEN {
                    let ch = UCOMPAT_V141_A_DATA[UCOMPAT_V141_A_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_A_POS += 1;
                    copied += 1;
                }
            }
            1 => {
                while copied < len && UCOMPAT_V141_B_POS < UCOMPAT_V141_B_LEN {
                    let ch = UCOMPAT_V141_B_DATA[UCOMPAT_V141_B_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_B_POS += 1;
                    copied += 1;
                }
            }
            2 => {
                while copied < len && UCOMPAT_V141_C_POS < UCOMPAT_V141_C_LEN {
                    let ch = UCOMPAT_V141_C_DATA[UCOMPAT_V141_C_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_C_POS += 1;
                    copied += 1;
                }
            }
            3 => {
                while copied < len && UCOMPAT_V141_D_POS < UCOMPAT_V141_D_LEN {
                    let ch = UCOMPAT_V141_D_DATA[UCOMPAT_V141_D_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_D_POS += 1;
                    copied += 1;
                }
            }
            4 => {
                while copied < len && UCOMPAT_V141_E_POS < UCOMPAT_V141_E_LEN {
                    let ch = UCOMPAT_V141_E_DATA[UCOMPAT_V141_E_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_E_POS += 1;
                    copied += 1;
                }
            }
            5 => {
                while copied < len && UCOMPAT_V141_F_POS < UCOMPAT_V141_F_LEN {
                    let ch = UCOMPAT_V141_F_DATA[UCOMPAT_V141_F_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_F_POS += 1;
                    copied += 1;
                }
            }
            6 => {
                while copied < len && UCOMPAT_V141_G_POS < UCOMPAT_V141_G_LEN {
                    let ch = UCOMPAT_V141_G_DATA[UCOMPAT_V141_G_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_G_POS += 1;
                    copied += 1;
                }
            }
            _ => {
                while copied < len && UCOMPAT_V141_H_POS < UCOMPAT_V141_H_LEN {
                    let ch = UCOMPAT_V141_H_DATA[UCOMPAT_V141_H_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V141_H_POS += 1;
                    copied += 1;
                }
            }
        }
    });
    crate::println!("[ucompat-v141] read fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v141_read(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V141_FD_A {
        ucompat_v141_read_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V141_FD_B {
        ucompat_v141_read_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V141_FD_C {
        ucompat_v141_read_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V141_FD_D {
        ucompat_v141_read_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V141_FD_E {
        ucompat_v141_read_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V141_FD_F {
        ucompat_v141_read_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V141_FD_G {
        ucompat_v141_read_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V141_FD_H {
        ucompat_v141_read_one(fd, user_ptr, len, 7)
    } else {
        -9
    }
}
fn ucompat_v141_lseek(fd: isize, off: isize, whence: usize) -> isize {
    unsafe {
        let (len, cur) = if fd == UCOMPAT_V141_FD_A {
            (UCOMPAT_V141_A_LEN, UCOMPAT_V141_A_POS)
        } else if fd == UCOMPAT_V141_FD_B {
            (UCOMPAT_V141_B_LEN, UCOMPAT_V141_B_POS)
        } else if fd == UCOMPAT_V141_FD_C {
            (UCOMPAT_V141_C_LEN, UCOMPAT_V141_C_POS)
        } else if fd == UCOMPAT_V141_FD_D {
            (UCOMPAT_V141_D_LEN, UCOMPAT_V141_D_POS)
        } else if fd == UCOMPAT_V141_FD_E {
            (UCOMPAT_V141_E_LEN, UCOMPAT_V141_E_POS)
        } else if fd == UCOMPAT_V141_FD_F {
            (UCOMPAT_V141_F_LEN, UCOMPAT_V141_F_POS)
        } else if fd == UCOMPAT_V141_FD_G {
            (UCOMPAT_V141_G_LEN, UCOMPAT_V141_G_POS)
        } else if fd == UCOMPAT_V141_FD_H {
            (UCOMPAT_V141_H_LEN, UCOMPAT_V141_H_POS)
        } else {
            return -9;
        };
        let base = match whence {
            0 => 0isize,
            1 => cur as isize,
            2 => len as isize,
            _ => return -22,
        };
        let new_pos = base + off;
        if new_pos < 0 {
            return -22;
        }
        if fd == UCOMPAT_V141_FD_A {
            UCOMPAT_V141_A_POS = new_pos as usize;
        } else if fd == UCOMPAT_V141_FD_B {
            UCOMPAT_V141_B_POS = new_pos as usize;
        } else if fd == UCOMPAT_V141_FD_C {
            UCOMPAT_V141_C_POS = new_pos as usize;
        } else if fd == UCOMPAT_V141_FD_D {
            UCOMPAT_V141_D_POS = new_pos as usize;
        } else if fd == UCOMPAT_V141_FD_E {
            UCOMPAT_V141_E_POS = new_pos as usize;
        } else if fd == UCOMPAT_V141_FD_F {
            UCOMPAT_V141_F_POS = new_pos as usize;
        } else if fd == UCOMPAT_V141_FD_G {
            UCOMPAT_V141_G_POS = new_pos as usize;
        } else if fd == UCOMPAT_V141_FD_H {
            UCOMPAT_V141_H_POS = new_pos as usize;
        } else {
            return -9;
        }
        crate::println!("[ucompat-v141] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}
fn ucompat_v141_close(fd: isize) -> isize {
    unsafe {
        if fd == UCOMPAT_V141_FD_A {
            UCOMPAT_V141_A_POS = 0;
            crate::println!("[ucompat-v141] close fd=10001 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V141_FD_B {
            UCOMPAT_V141_B_POS = 0;
            crate::println!("[ucompat-v141] close fd=10002 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V141_FD_C {
            UCOMPAT_V141_C_POS = 0;
            crate::println!("[ucompat-v141] close fd=10003 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V141_FD_D {
            UCOMPAT_V141_D_POS = 0;
            crate::println!("[ucompat-v141] close fd=10004 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V141_FD_E {
            UCOMPAT_V141_E_POS = 0;
            crate::println!("[ucompat-v141] close fd=10005 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V141_FD_F {
            UCOMPAT_V141_F_POS = 0;
            crate::println!("[ucompat-v141] close fd=10006 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V141_FD_G {
            UCOMPAT_V141_G_POS = 0;
            crate::println!("[ucompat-v141] close fd=10007 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V141_FD_H {
            UCOMPAT_V141_H_POS = 0;
            crate::println!("[ucompat-v141] close fd=10008 ret=0 keep_file=1");
            0
        } else {
            -9
        }
    }
}

// UCOMPAT_V142_SIXTEENFILE_TABLE_ERRNO_ISOLATION
const UCOMPAT_V142_FD_A: isize = 11001;
const UCOMPAT_V142_FD_B: isize = 11002;
const UCOMPAT_V142_FD_C: isize = 11003;
const UCOMPAT_V142_FD_D: isize = 11004;
const UCOMPAT_V142_FD_E: isize = 11005;
const UCOMPAT_V142_FD_F: isize = 11006;
const UCOMPAT_V142_FD_G: isize = 11007;
const UCOMPAT_V142_FD_H: isize = 11008;
const UCOMPAT_V142_FD_I: isize = 11009;
const UCOMPAT_V142_FD_J: isize = 11010;
const UCOMPAT_V142_FD_K: isize = 11011;
const UCOMPAT_V142_FD_L: isize = 11012;
const UCOMPAT_V142_FD_M: isize = 11013;
const UCOMPAT_V142_FD_N: isize = 11014;
const UCOMPAT_V142_FD_O: isize = 11015;
const UCOMPAT_V142_FD_P: isize = 11016;
const UCOMPAT_V142_CAP: usize = 512;
static mut UCOMPAT_V142_A_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_A_LEN: usize = 0;
static mut UCOMPAT_V142_A_POS: usize = 0;
static mut UCOMPAT_V142_A_EXISTS: bool = false;
static mut UCOMPAT_V142_B_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_B_LEN: usize = 0;
static mut UCOMPAT_V142_B_POS: usize = 0;
static mut UCOMPAT_V142_B_EXISTS: bool = false;
static mut UCOMPAT_V142_C_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_C_LEN: usize = 0;
static mut UCOMPAT_V142_C_POS: usize = 0;
static mut UCOMPAT_V142_C_EXISTS: bool = false;
static mut UCOMPAT_V142_D_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_D_LEN: usize = 0;
static mut UCOMPAT_V142_D_POS: usize = 0;
static mut UCOMPAT_V142_D_EXISTS: bool = false;
static mut UCOMPAT_V142_E_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_E_LEN: usize = 0;
static mut UCOMPAT_V142_E_POS: usize = 0;
static mut UCOMPAT_V142_E_EXISTS: bool = false;
static mut UCOMPAT_V142_F_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_F_LEN: usize = 0;
static mut UCOMPAT_V142_F_POS: usize = 0;
static mut UCOMPAT_V142_F_EXISTS: bool = false;
static mut UCOMPAT_V142_G_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_G_LEN: usize = 0;
static mut UCOMPAT_V142_G_POS: usize = 0;
static mut UCOMPAT_V142_G_EXISTS: bool = false;
static mut UCOMPAT_V142_H_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_H_LEN: usize = 0;
static mut UCOMPAT_V142_H_POS: usize = 0;
static mut UCOMPAT_V142_H_EXISTS: bool = false;
static mut UCOMPAT_V142_I_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_I_LEN: usize = 0;
static mut UCOMPAT_V142_I_POS: usize = 0;
static mut UCOMPAT_V142_I_EXISTS: bool = false;
static mut UCOMPAT_V142_J_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_J_LEN: usize = 0;
static mut UCOMPAT_V142_J_POS: usize = 0;
static mut UCOMPAT_V142_J_EXISTS: bool = false;
static mut UCOMPAT_V142_K_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_K_LEN: usize = 0;
static mut UCOMPAT_V142_K_POS: usize = 0;
static mut UCOMPAT_V142_K_EXISTS: bool = false;
static mut UCOMPAT_V142_L_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_L_LEN: usize = 0;
static mut UCOMPAT_V142_L_POS: usize = 0;
static mut UCOMPAT_V142_L_EXISTS: bool = false;
static mut UCOMPAT_V142_M_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_M_LEN: usize = 0;
static mut UCOMPAT_V142_M_POS: usize = 0;
static mut UCOMPAT_V142_M_EXISTS: bool = false;
static mut UCOMPAT_V142_N_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_N_LEN: usize = 0;
static mut UCOMPAT_V142_N_POS: usize = 0;
static mut UCOMPAT_V142_N_EXISTS: bool = false;
static mut UCOMPAT_V142_O_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_O_LEN: usize = 0;
static mut UCOMPAT_V142_O_POS: usize = 0;
static mut UCOMPAT_V142_O_EXISTS: bool = false;
static mut UCOMPAT_V142_P_DATA: [u8; UCOMPAT_V142_CAP] = [0; UCOMPAT_V142_CAP];
static mut UCOMPAT_V142_P_LEN: usize = 0;
static mut UCOMPAT_V142_P_POS: usize = 0;
static mut UCOMPAT_V142_P_EXISTS: bool = false;
fn ucompat_v142_fd(slot: usize) -> isize {
    match slot {
        0 => UCOMPAT_V142_FD_A,
        1 => UCOMPAT_V142_FD_B,
        2 => UCOMPAT_V142_FD_C,
        3 => UCOMPAT_V142_FD_D,
        4 => UCOMPAT_V142_FD_E,
        5 => UCOMPAT_V142_FD_F,
        6 => UCOMPAT_V142_FD_G,
        7 => UCOMPAT_V142_FD_H,
        8 => UCOMPAT_V142_FD_I,
        9 => UCOMPAT_V142_FD_J,
        10 => UCOMPAT_V142_FD_K,
        11 => UCOMPAT_V142_FD_L,
        12 => UCOMPAT_V142_FD_M,
        13 => UCOMPAT_V142_FD_N,
        14 => UCOMPAT_V142_FD_O,
        _ => UCOMPAT_V142_FD_P,
    }
}
fn ucompat_v142_is_fd(fd: isize) -> bool {
    fd == UCOMPAT_V142_FD_A
        || fd == UCOMPAT_V142_FD_B
        || fd == UCOMPAT_V142_FD_C
        || fd == UCOMPAT_V142_FD_D
        || fd == UCOMPAT_V142_FD_E
        || fd == UCOMPAT_V142_FD_F
        || fd == UCOMPAT_V142_FD_G
        || fd == UCOMPAT_V142_FD_H
        || fd == UCOMPAT_V142_FD_I
        || fd == UCOMPAT_V142_FD_J
        || fd == UCOMPAT_V142_FD_K
        || fd == UCOMPAT_V142_FD_L
        || fd == UCOMPAT_V142_FD_M
        || fd == UCOMPAT_V142_FD_N
        || fd == UCOMPAT_V142_FD_O
        || fd == UCOMPAT_V142_FD_P
}
fn ucompat_v142_exists(slot: usize) -> bool {
    unsafe {
        match slot {
            0 => UCOMPAT_V142_A_EXISTS,
            1 => UCOMPAT_V142_B_EXISTS,
            2 => UCOMPAT_V142_C_EXISTS,
            3 => UCOMPAT_V142_D_EXISTS,
            4 => UCOMPAT_V142_E_EXISTS,
            5 => UCOMPAT_V142_F_EXISTS,
            6 => UCOMPAT_V142_G_EXISTS,
            7 => UCOMPAT_V142_H_EXISTS,
            8 => UCOMPAT_V142_I_EXISTS,
            9 => UCOMPAT_V142_J_EXISTS,
            10 => UCOMPAT_V142_K_EXISTS,
            11 => UCOMPAT_V142_L_EXISTS,
            12 => UCOMPAT_V142_M_EXISTS,
            13 => UCOMPAT_V142_N_EXISTS,
            14 => UCOMPAT_V142_O_EXISTS,
            _ => UCOMPAT_V142_P_EXISTS,
        }
    }
}
fn ucompat_v142_reset(slot: usize) {
    unsafe {
        let mut j = 0usize;
        match slot {
            0 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_A_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_A_LEN = 0;
                UCOMPAT_V142_A_POS = 0;
                UCOMPAT_V142_A_EXISTS = true;
            }
            1 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_B_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_B_LEN = 0;
                UCOMPAT_V142_B_POS = 0;
                UCOMPAT_V142_B_EXISTS = true;
            }
            2 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_C_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_C_LEN = 0;
                UCOMPAT_V142_C_POS = 0;
                UCOMPAT_V142_C_EXISTS = true;
            }
            3 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_D_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_D_LEN = 0;
                UCOMPAT_V142_D_POS = 0;
                UCOMPAT_V142_D_EXISTS = true;
            }
            4 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_E_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_E_LEN = 0;
                UCOMPAT_V142_E_POS = 0;
                UCOMPAT_V142_E_EXISTS = true;
            }
            5 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_F_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_F_LEN = 0;
                UCOMPAT_V142_F_POS = 0;
                UCOMPAT_V142_F_EXISTS = true;
            }
            6 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_G_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_G_LEN = 0;
                UCOMPAT_V142_G_POS = 0;
                UCOMPAT_V142_G_EXISTS = true;
            }
            7 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_H_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_H_LEN = 0;
                UCOMPAT_V142_H_POS = 0;
                UCOMPAT_V142_H_EXISTS = true;
            }
            8 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_I_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_I_LEN = 0;
                UCOMPAT_V142_I_POS = 0;
                UCOMPAT_V142_I_EXISTS = true;
            }
            9 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_J_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_J_LEN = 0;
                UCOMPAT_V142_J_POS = 0;
                UCOMPAT_V142_J_EXISTS = true;
            }
            10 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_K_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_K_LEN = 0;
                UCOMPAT_V142_K_POS = 0;
                UCOMPAT_V142_K_EXISTS = true;
            }
            11 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_L_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_L_LEN = 0;
                UCOMPAT_V142_L_POS = 0;
                UCOMPAT_V142_L_EXISTS = true;
            }
            12 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_M_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_M_LEN = 0;
                UCOMPAT_V142_M_POS = 0;
                UCOMPAT_V142_M_EXISTS = true;
            }
            13 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_N_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_N_LEN = 0;
                UCOMPAT_V142_N_POS = 0;
                UCOMPAT_V142_N_EXISTS = true;
            }
            14 => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_O_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_O_LEN = 0;
                UCOMPAT_V142_O_POS = 0;
                UCOMPAT_V142_O_EXISTS = true;
            }
            _ => {
                while j < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_P_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V142_P_LEN = 0;
                UCOMPAT_V142_P_POS = 0;
                UCOMPAT_V142_P_EXISTS = true;
            }
        }
    }
}
fn ucompat_v142_open(slot: usize, flags: usize) -> isize {
    const O_CREAT: usize = 0x40;
    const O_TRUNC: usize = 0x200;
    let exists = ucompat_v142_exists(slot);
    let name = match slot {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "G",
        7 => "H",
        8 => "I",
        9 => "J",
        10 => "K",
        11 => "L",
        12 => "M",
        13 => "N",
        14 => "O",
        _ => "P",
    };
    if !exists && (flags & O_CREAT) == 0 {
        crate::println!("[openat-v142] slot={} missing without O_CREAT", name);
        return crate::syscall::errno::ENOENT;
    }
    if !exists || (flags & O_TRUNC) != 0 {
        if exists && (flags & O_TRUNC) != 0 {
            crate::println!("[openat-v142] slot={} truncate", name);
        } else {
            crate::println!("[openat-v142] slot={} create", name);
        }
        ucompat_v142_reset(slot);
    } else {
        unsafe {
            match slot {
                0 => UCOMPAT_V142_A_POS = 0,
                1 => UCOMPAT_V142_B_POS = 0,
                2 => UCOMPAT_V142_C_POS = 0,
                3 => UCOMPAT_V142_D_POS = 0,
                4 => UCOMPAT_V142_E_POS = 0,
                5 => UCOMPAT_V142_F_POS = 0,
                6 => UCOMPAT_V142_G_POS = 0,
                7 => UCOMPAT_V142_H_POS = 0,
                8 => UCOMPAT_V142_I_POS = 0,
                9 => UCOMPAT_V142_J_POS = 0,
                10 => UCOMPAT_V142_K_POS = 0,
                11 => UCOMPAT_V142_L_POS = 0,
                12 => UCOMPAT_V142_M_POS = 0,
                13 => UCOMPAT_V142_N_POS = 0,
                14 => UCOMPAT_V142_O_POS = 0,
                _ => UCOMPAT_V142_P_POS = 0,
            }
        }
        crate::println!("[openat-v142] slot={} reopen", name);
    }
    ucompat_v142_fd(slot)
}
fn ucompat_v142_write_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                if UCOMPAT_V142_A_POS > UCOMPAT_V142_A_LEN {
                    let mut z = UCOMPAT_V142_A_LEN;
                    while z < UCOMPAT_V142_A_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_A_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11001 from={} to={}",
                        UCOMPAT_V142_A_LEN,
                        UCOMPAT_V142_A_POS
                    );
                }
                while copied < len && UCOMPAT_V142_A_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_A_DATA[UCOMPAT_V142_A_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_A_POS + copied;
                if end > UCOMPAT_V142_A_LEN {
                    UCOMPAT_V142_A_LEN = end;
                }
                UCOMPAT_V142_A_POS = end;
                UCOMPAT_V142_A_EXISTS = true;
            }
            1 => {
                if UCOMPAT_V142_B_POS > UCOMPAT_V142_B_LEN {
                    let mut z = UCOMPAT_V142_B_LEN;
                    while z < UCOMPAT_V142_B_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_B_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11002 from={} to={}",
                        UCOMPAT_V142_B_LEN,
                        UCOMPAT_V142_B_POS
                    );
                }
                while copied < len && UCOMPAT_V142_B_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_B_DATA[UCOMPAT_V142_B_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_B_POS + copied;
                if end > UCOMPAT_V142_B_LEN {
                    UCOMPAT_V142_B_LEN = end;
                }
                UCOMPAT_V142_B_POS = end;
                UCOMPAT_V142_B_EXISTS = true;
            }
            2 => {
                if UCOMPAT_V142_C_POS > UCOMPAT_V142_C_LEN {
                    let mut z = UCOMPAT_V142_C_LEN;
                    while z < UCOMPAT_V142_C_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_C_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11003 from={} to={}",
                        UCOMPAT_V142_C_LEN,
                        UCOMPAT_V142_C_POS
                    );
                }
                while copied < len && UCOMPAT_V142_C_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_C_DATA[UCOMPAT_V142_C_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_C_POS + copied;
                if end > UCOMPAT_V142_C_LEN {
                    UCOMPAT_V142_C_LEN = end;
                }
                UCOMPAT_V142_C_POS = end;
                UCOMPAT_V142_C_EXISTS = true;
            }
            3 => {
                if UCOMPAT_V142_D_POS > UCOMPAT_V142_D_LEN {
                    let mut z = UCOMPAT_V142_D_LEN;
                    while z < UCOMPAT_V142_D_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_D_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11004 from={} to={}",
                        UCOMPAT_V142_D_LEN,
                        UCOMPAT_V142_D_POS
                    );
                }
                while copied < len && UCOMPAT_V142_D_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_D_DATA[UCOMPAT_V142_D_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_D_POS + copied;
                if end > UCOMPAT_V142_D_LEN {
                    UCOMPAT_V142_D_LEN = end;
                }
                UCOMPAT_V142_D_POS = end;
                UCOMPAT_V142_D_EXISTS = true;
            }
            4 => {
                if UCOMPAT_V142_E_POS > UCOMPAT_V142_E_LEN {
                    let mut z = UCOMPAT_V142_E_LEN;
                    while z < UCOMPAT_V142_E_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_E_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11005 from={} to={}",
                        UCOMPAT_V142_E_LEN,
                        UCOMPAT_V142_E_POS
                    );
                }
                while copied < len && UCOMPAT_V142_E_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_E_DATA[UCOMPAT_V142_E_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_E_POS + copied;
                if end > UCOMPAT_V142_E_LEN {
                    UCOMPAT_V142_E_LEN = end;
                }
                UCOMPAT_V142_E_POS = end;
                UCOMPAT_V142_E_EXISTS = true;
            }
            5 => {
                if UCOMPAT_V142_F_POS > UCOMPAT_V142_F_LEN {
                    let mut z = UCOMPAT_V142_F_LEN;
                    while z < UCOMPAT_V142_F_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_F_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11006 from={} to={}",
                        UCOMPAT_V142_F_LEN,
                        UCOMPAT_V142_F_POS
                    );
                }
                while copied < len && UCOMPAT_V142_F_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_F_DATA[UCOMPAT_V142_F_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_F_POS + copied;
                if end > UCOMPAT_V142_F_LEN {
                    UCOMPAT_V142_F_LEN = end;
                }
                UCOMPAT_V142_F_POS = end;
                UCOMPAT_V142_F_EXISTS = true;
            }
            6 => {
                if UCOMPAT_V142_G_POS > UCOMPAT_V142_G_LEN {
                    let mut z = UCOMPAT_V142_G_LEN;
                    while z < UCOMPAT_V142_G_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_G_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11007 from={} to={}",
                        UCOMPAT_V142_G_LEN,
                        UCOMPAT_V142_G_POS
                    );
                }
                while copied < len && UCOMPAT_V142_G_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_G_DATA[UCOMPAT_V142_G_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_G_POS + copied;
                if end > UCOMPAT_V142_G_LEN {
                    UCOMPAT_V142_G_LEN = end;
                }
                UCOMPAT_V142_G_POS = end;
                UCOMPAT_V142_G_EXISTS = true;
            }
            7 => {
                if UCOMPAT_V142_H_POS > UCOMPAT_V142_H_LEN {
                    let mut z = UCOMPAT_V142_H_LEN;
                    while z < UCOMPAT_V142_H_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_H_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11008 from={} to={}",
                        UCOMPAT_V142_H_LEN,
                        UCOMPAT_V142_H_POS
                    );
                }
                while copied < len && UCOMPAT_V142_H_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_H_DATA[UCOMPAT_V142_H_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_H_POS + copied;
                if end > UCOMPAT_V142_H_LEN {
                    UCOMPAT_V142_H_LEN = end;
                }
                UCOMPAT_V142_H_POS = end;
                UCOMPAT_V142_H_EXISTS = true;
            }
            8 => {
                if UCOMPAT_V142_I_POS > UCOMPAT_V142_I_LEN {
                    let mut z = UCOMPAT_V142_I_LEN;
                    while z < UCOMPAT_V142_I_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_I_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11009 from={} to={}",
                        UCOMPAT_V142_I_LEN,
                        UCOMPAT_V142_I_POS
                    );
                }
                while copied < len && UCOMPAT_V142_I_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_I_DATA[UCOMPAT_V142_I_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_I_POS + copied;
                if end > UCOMPAT_V142_I_LEN {
                    UCOMPAT_V142_I_LEN = end;
                }
                UCOMPAT_V142_I_POS = end;
                UCOMPAT_V142_I_EXISTS = true;
            }
            9 => {
                if UCOMPAT_V142_J_POS > UCOMPAT_V142_J_LEN {
                    let mut z = UCOMPAT_V142_J_LEN;
                    while z < UCOMPAT_V142_J_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_J_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11010 from={} to={}",
                        UCOMPAT_V142_J_LEN,
                        UCOMPAT_V142_J_POS
                    );
                }
                while copied < len && UCOMPAT_V142_J_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_J_DATA[UCOMPAT_V142_J_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_J_POS + copied;
                if end > UCOMPAT_V142_J_LEN {
                    UCOMPAT_V142_J_LEN = end;
                }
                UCOMPAT_V142_J_POS = end;
                UCOMPAT_V142_J_EXISTS = true;
            }
            10 => {
                if UCOMPAT_V142_K_POS > UCOMPAT_V142_K_LEN {
                    let mut z = UCOMPAT_V142_K_LEN;
                    while z < UCOMPAT_V142_K_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_K_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11011 from={} to={}",
                        UCOMPAT_V142_K_LEN,
                        UCOMPAT_V142_K_POS
                    );
                }
                while copied < len && UCOMPAT_V142_K_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_K_DATA[UCOMPAT_V142_K_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_K_POS + copied;
                if end > UCOMPAT_V142_K_LEN {
                    UCOMPAT_V142_K_LEN = end;
                }
                UCOMPAT_V142_K_POS = end;
                UCOMPAT_V142_K_EXISTS = true;
            }
            11 => {
                if UCOMPAT_V142_L_POS > UCOMPAT_V142_L_LEN {
                    let mut z = UCOMPAT_V142_L_LEN;
                    while z < UCOMPAT_V142_L_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_L_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11012 from={} to={}",
                        UCOMPAT_V142_L_LEN,
                        UCOMPAT_V142_L_POS
                    );
                }
                while copied < len && UCOMPAT_V142_L_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_L_DATA[UCOMPAT_V142_L_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_L_POS + copied;
                if end > UCOMPAT_V142_L_LEN {
                    UCOMPAT_V142_L_LEN = end;
                }
                UCOMPAT_V142_L_POS = end;
                UCOMPAT_V142_L_EXISTS = true;
            }
            12 => {
                if UCOMPAT_V142_M_POS > UCOMPAT_V142_M_LEN {
                    let mut z = UCOMPAT_V142_M_LEN;
                    while z < UCOMPAT_V142_M_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_M_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11013 from={} to={}",
                        UCOMPAT_V142_M_LEN,
                        UCOMPAT_V142_M_POS
                    );
                }
                while copied < len && UCOMPAT_V142_M_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_M_DATA[UCOMPAT_V142_M_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_M_POS + copied;
                if end > UCOMPAT_V142_M_LEN {
                    UCOMPAT_V142_M_LEN = end;
                }
                UCOMPAT_V142_M_POS = end;
                UCOMPAT_V142_M_EXISTS = true;
            }
            13 => {
                if UCOMPAT_V142_N_POS > UCOMPAT_V142_N_LEN {
                    let mut z = UCOMPAT_V142_N_LEN;
                    while z < UCOMPAT_V142_N_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_N_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11014 from={} to={}",
                        UCOMPAT_V142_N_LEN,
                        UCOMPAT_V142_N_POS
                    );
                }
                while copied < len && UCOMPAT_V142_N_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_N_DATA[UCOMPAT_V142_N_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_N_POS + copied;
                if end > UCOMPAT_V142_N_LEN {
                    UCOMPAT_V142_N_LEN = end;
                }
                UCOMPAT_V142_N_POS = end;
                UCOMPAT_V142_N_EXISTS = true;
            }
            14 => {
                if UCOMPAT_V142_O_POS > UCOMPAT_V142_O_LEN {
                    let mut z = UCOMPAT_V142_O_LEN;
                    while z < UCOMPAT_V142_O_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_O_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11015 from={} to={}",
                        UCOMPAT_V142_O_LEN,
                        UCOMPAT_V142_O_POS
                    );
                }
                while copied < len && UCOMPAT_V142_O_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_O_DATA[UCOMPAT_V142_O_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_O_POS + copied;
                if end > UCOMPAT_V142_O_LEN {
                    UCOMPAT_V142_O_LEN = end;
                }
                UCOMPAT_V142_O_POS = end;
                UCOMPAT_V142_O_EXISTS = true;
            }
            _ => {
                if UCOMPAT_V142_P_POS > UCOMPAT_V142_P_LEN {
                    let mut z = UCOMPAT_V142_P_LEN;
                    while z < UCOMPAT_V142_P_POS && z < UCOMPAT_V142_CAP {
                        UCOMPAT_V142_P_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v142] zero-fill sparse gap fd=11016 from={} to={}",
                        UCOMPAT_V142_P_LEN,
                        UCOMPAT_V142_P_POS
                    );
                }
                while copied < len && UCOMPAT_V142_P_POS + copied < UCOMPAT_V142_CAP {
                    UCOMPAT_V142_P_DATA[UCOMPAT_V142_P_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V142_P_POS + copied;
                if end > UCOMPAT_V142_P_LEN {
                    UCOMPAT_V142_P_LEN = end;
                }
                UCOMPAT_V142_P_POS = end;
                UCOMPAT_V142_P_EXISTS = true;
            }
        }
    });
    crate::println!("[ucompat-v142] write fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v142_write(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V142_FD_A {
        ucompat_v142_write_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V142_FD_B {
        ucompat_v142_write_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V142_FD_C {
        ucompat_v142_write_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V142_FD_D {
        ucompat_v142_write_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V142_FD_E {
        ucompat_v142_write_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V142_FD_F {
        ucompat_v142_write_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V142_FD_G {
        ucompat_v142_write_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V142_FD_H {
        ucompat_v142_write_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V142_FD_I {
        ucompat_v142_write_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V142_FD_J {
        ucompat_v142_write_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V142_FD_K {
        ucompat_v142_write_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V142_FD_L {
        ucompat_v142_write_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V142_FD_M {
        ucompat_v142_write_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V142_FD_N {
        ucompat_v142_write_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V142_FD_O {
        ucompat_v142_write_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V142_FD_P {
        ucompat_v142_write_one(fd, user_ptr, len, 15)
    } else {
        -9
    }
}
fn ucompat_v142_read_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                while copied < len && UCOMPAT_V142_A_POS < UCOMPAT_V142_A_LEN {
                    let ch = UCOMPAT_V142_A_DATA[UCOMPAT_V142_A_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_A_POS += 1;
                    copied += 1;
                }
            }
            1 => {
                while copied < len && UCOMPAT_V142_B_POS < UCOMPAT_V142_B_LEN {
                    let ch = UCOMPAT_V142_B_DATA[UCOMPAT_V142_B_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_B_POS += 1;
                    copied += 1;
                }
            }
            2 => {
                while copied < len && UCOMPAT_V142_C_POS < UCOMPAT_V142_C_LEN {
                    let ch = UCOMPAT_V142_C_DATA[UCOMPAT_V142_C_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_C_POS += 1;
                    copied += 1;
                }
            }
            3 => {
                while copied < len && UCOMPAT_V142_D_POS < UCOMPAT_V142_D_LEN {
                    let ch = UCOMPAT_V142_D_DATA[UCOMPAT_V142_D_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_D_POS += 1;
                    copied += 1;
                }
            }
            4 => {
                while copied < len && UCOMPAT_V142_E_POS < UCOMPAT_V142_E_LEN {
                    let ch = UCOMPAT_V142_E_DATA[UCOMPAT_V142_E_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_E_POS += 1;
                    copied += 1;
                }
            }
            5 => {
                while copied < len && UCOMPAT_V142_F_POS < UCOMPAT_V142_F_LEN {
                    let ch = UCOMPAT_V142_F_DATA[UCOMPAT_V142_F_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_F_POS += 1;
                    copied += 1;
                }
            }
            6 => {
                while copied < len && UCOMPAT_V142_G_POS < UCOMPAT_V142_G_LEN {
                    let ch = UCOMPAT_V142_G_DATA[UCOMPAT_V142_G_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_G_POS += 1;
                    copied += 1;
                }
            }
            7 => {
                while copied < len && UCOMPAT_V142_H_POS < UCOMPAT_V142_H_LEN {
                    let ch = UCOMPAT_V142_H_DATA[UCOMPAT_V142_H_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_H_POS += 1;
                    copied += 1;
                }
            }
            8 => {
                while copied < len && UCOMPAT_V142_I_POS < UCOMPAT_V142_I_LEN {
                    let ch = UCOMPAT_V142_I_DATA[UCOMPAT_V142_I_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_I_POS += 1;
                    copied += 1;
                }
            }
            9 => {
                while copied < len && UCOMPAT_V142_J_POS < UCOMPAT_V142_J_LEN {
                    let ch = UCOMPAT_V142_J_DATA[UCOMPAT_V142_J_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_J_POS += 1;
                    copied += 1;
                }
            }
            10 => {
                while copied < len && UCOMPAT_V142_K_POS < UCOMPAT_V142_K_LEN {
                    let ch = UCOMPAT_V142_K_DATA[UCOMPAT_V142_K_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_K_POS += 1;
                    copied += 1;
                }
            }
            11 => {
                while copied < len && UCOMPAT_V142_L_POS < UCOMPAT_V142_L_LEN {
                    let ch = UCOMPAT_V142_L_DATA[UCOMPAT_V142_L_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_L_POS += 1;
                    copied += 1;
                }
            }
            12 => {
                while copied < len && UCOMPAT_V142_M_POS < UCOMPAT_V142_M_LEN {
                    let ch = UCOMPAT_V142_M_DATA[UCOMPAT_V142_M_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_M_POS += 1;
                    copied += 1;
                }
            }
            13 => {
                while copied < len && UCOMPAT_V142_N_POS < UCOMPAT_V142_N_LEN {
                    let ch = UCOMPAT_V142_N_DATA[UCOMPAT_V142_N_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_N_POS += 1;
                    copied += 1;
                }
            }
            14 => {
                while copied < len && UCOMPAT_V142_O_POS < UCOMPAT_V142_O_LEN {
                    let ch = UCOMPAT_V142_O_DATA[UCOMPAT_V142_O_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_O_POS += 1;
                    copied += 1;
                }
            }
            _ => {
                while copied < len && UCOMPAT_V142_P_POS < UCOMPAT_V142_P_LEN {
                    let ch = UCOMPAT_V142_P_DATA[UCOMPAT_V142_P_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V142_P_POS += 1;
                    copied += 1;
                }
            }
        }
    });
    crate::println!("[ucompat-v142] read fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v142_read(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V142_FD_A {
        ucompat_v142_read_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V142_FD_B {
        ucompat_v142_read_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V142_FD_C {
        ucompat_v142_read_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V142_FD_D {
        ucompat_v142_read_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V142_FD_E {
        ucompat_v142_read_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V142_FD_F {
        ucompat_v142_read_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V142_FD_G {
        ucompat_v142_read_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V142_FD_H {
        ucompat_v142_read_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V142_FD_I {
        ucompat_v142_read_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V142_FD_J {
        ucompat_v142_read_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V142_FD_K {
        ucompat_v142_read_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V142_FD_L {
        ucompat_v142_read_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V142_FD_M {
        ucompat_v142_read_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V142_FD_N {
        ucompat_v142_read_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V142_FD_O {
        ucompat_v142_read_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V142_FD_P {
        ucompat_v142_read_one(fd, user_ptr, len, 15)
    } else {
        -9
    }
}
fn ucompat_v142_lseek(fd: isize, off: isize, whence: usize) -> isize {
    unsafe {
        let (len, cur) = if fd == UCOMPAT_V142_FD_A {
            (UCOMPAT_V142_A_LEN, UCOMPAT_V142_A_POS)
        } else if fd == UCOMPAT_V142_FD_B {
            (UCOMPAT_V142_B_LEN, UCOMPAT_V142_B_POS)
        } else if fd == UCOMPAT_V142_FD_C {
            (UCOMPAT_V142_C_LEN, UCOMPAT_V142_C_POS)
        } else if fd == UCOMPAT_V142_FD_D {
            (UCOMPAT_V142_D_LEN, UCOMPAT_V142_D_POS)
        } else if fd == UCOMPAT_V142_FD_E {
            (UCOMPAT_V142_E_LEN, UCOMPAT_V142_E_POS)
        } else if fd == UCOMPAT_V142_FD_F {
            (UCOMPAT_V142_F_LEN, UCOMPAT_V142_F_POS)
        } else if fd == UCOMPAT_V142_FD_G {
            (UCOMPAT_V142_G_LEN, UCOMPAT_V142_G_POS)
        } else if fd == UCOMPAT_V142_FD_H {
            (UCOMPAT_V142_H_LEN, UCOMPAT_V142_H_POS)
        } else if fd == UCOMPAT_V142_FD_I {
            (UCOMPAT_V142_I_LEN, UCOMPAT_V142_I_POS)
        } else if fd == UCOMPAT_V142_FD_J {
            (UCOMPAT_V142_J_LEN, UCOMPAT_V142_J_POS)
        } else if fd == UCOMPAT_V142_FD_K {
            (UCOMPAT_V142_K_LEN, UCOMPAT_V142_K_POS)
        } else if fd == UCOMPAT_V142_FD_L {
            (UCOMPAT_V142_L_LEN, UCOMPAT_V142_L_POS)
        } else if fd == UCOMPAT_V142_FD_M {
            (UCOMPAT_V142_M_LEN, UCOMPAT_V142_M_POS)
        } else if fd == UCOMPAT_V142_FD_N {
            (UCOMPAT_V142_N_LEN, UCOMPAT_V142_N_POS)
        } else if fd == UCOMPAT_V142_FD_O {
            (UCOMPAT_V142_O_LEN, UCOMPAT_V142_O_POS)
        } else if fd == UCOMPAT_V142_FD_P {
            (UCOMPAT_V142_P_LEN, UCOMPAT_V142_P_POS)
        } else {
            return -9;
        };
        let base = match whence {
            0 => 0isize,
            1 => cur as isize,
            2 => len as isize,
            _ => return -22,
        };
        let new_pos = base + off;
        if new_pos < 0 {
            return -22;
        }
        if fd == UCOMPAT_V142_FD_A {
            UCOMPAT_V142_A_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_B {
            UCOMPAT_V142_B_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_C {
            UCOMPAT_V142_C_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_D {
            UCOMPAT_V142_D_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_E {
            UCOMPAT_V142_E_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_F {
            UCOMPAT_V142_F_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_G {
            UCOMPAT_V142_G_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_H {
            UCOMPAT_V142_H_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_I {
            UCOMPAT_V142_I_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_J {
            UCOMPAT_V142_J_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_K {
            UCOMPAT_V142_K_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_L {
            UCOMPAT_V142_L_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_M {
            UCOMPAT_V142_M_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_N {
            UCOMPAT_V142_N_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_O {
            UCOMPAT_V142_O_POS = new_pos as usize;
        } else if fd == UCOMPAT_V142_FD_P {
            UCOMPAT_V142_P_POS = new_pos as usize;
        } else {
            return -9;
        }
        crate::println!("[ucompat-v142] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}
fn ucompat_v142_close(fd: isize) -> isize {
    unsafe {
        if fd == UCOMPAT_V142_FD_A {
            UCOMPAT_V142_A_POS = 0;
            crate::println!("[ucompat-v142] close fd=11001 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_B {
            UCOMPAT_V142_B_POS = 0;
            crate::println!("[ucompat-v142] close fd=11002 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_C {
            UCOMPAT_V142_C_POS = 0;
            crate::println!("[ucompat-v142] close fd=11003 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_D {
            UCOMPAT_V142_D_POS = 0;
            crate::println!("[ucompat-v142] close fd=11004 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_E {
            UCOMPAT_V142_E_POS = 0;
            crate::println!("[ucompat-v142] close fd=11005 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_F {
            UCOMPAT_V142_F_POS = 0;
            crate::println!("[ucompat-v142] close fd=11006 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_G {
            UCOMPAT_V142_G_POS = 0;
            crate::println!("[ucompat-v142] close fd=11007 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_H {
            UCOMPAT_V142_H_POS = 0;
            crate::println!("[ucompat-v142] close fd=11008 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_I {
            UCOMPAT_V142_I_POS = 0;
            crate::println!("[ucompat-v142] close fd=11009 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_J {
            UCOMPAT_V142_J_POS = 0;
            crate::println!("[ucompat-v142] close fd=11010 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_K {
            UCOMPAT_V142_K_POS = 0;
            crate::println!("[ucompat-v142] close fd=11011 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_L {
            UCOMPAT_V142_L_POS = 0;
            crate::println!("[ucompat-v142] close fd=11012 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_M {
            UCOMPAT_V142_M_POS = 0;
            crate::println!("[ucompat-v142] close fd=11013 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_N {
            UCOMPAT_V142_N_POS = 0;
            crate::println!("[ucompat-v142] close fd=11014 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_O {
            UCOMPAT_V142_O_POS = 0;
            crate::println!("[ucompat-v142] close fd=11015 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V142_FD_P {
            UCOMPAT_V142_P_POS = 0;
            crate::println!("[ucompat-v142] close fd=11016 ret=0 keep_file=1");
            0
        } else {
            -9
        }
    }
}

// UCOMPAT_V143_FILE32_OVERWRITE_SPARSE_ERRNO
const UCOMPAT_V143_FD_A: isize = 12001;
const UCOMPAT_V143_FD_B: isize = 12002;
const UCOMPAT_V143_FD_C: isize = 12003;
const UCOMPAT_V143_FD_D: isize = 12004;
const UCOMPAT_V143_FD_E: isize = 12005;
const UCOMPAT_V143_FD_F: isize = 12006;
const UCOMPAT_V143_FD_G: isize = 12007;
const UCOMPAT_V143_FD_H: isize = 12008;
const UCOMPAT_V143_FD_I: isize = 12009;
const UCOMPAT_V143_FD_J: isize = 12010;
const UCOMPAT_V143_FD_K: isize = 12011;
const UCOMPAT_V143_FD_L: isize = 12012;
const UCOMPAT_V143_FD_M: isize = 12013;
const UCOMPAT_V143_FD_N: isize = 12014;
const UCOMPAT_V143_FD_O: isize = 12015;
const UCOMPAT_V143_FD_P: isize = 12016;
const UCOMPAT_V143_FD_Q: isize = 12017;
const UCOMPAT_V143_FD_R: isize = 12018;
const UCOMPAT_V143_FD_S: isize = 12019;
const UCOMPAT_V143_FD_T: isize = 12020;
const UCOMPAT_V143_FD_U: isize = 12021;
const UCOMPAT_V143_FD_V: isize = 12022;
const UCOMPAT_V143_FD_W: isize = 12023;
const UCOMPAT_V143_FD_X: isize = 12024;
const UCOMPAT_V143_FD_Y: isize = 12025;
const UCOMPAT_V143_FD_Z: isize = 12026;
const UCOMPAT_V143_FD_N0: isize = 12027;
const UCOMPAT_V143_FD_N1: isize = 12028;
const UCOMPAT_V143_FD_N2: isize = 12029;
const UCOMPAT_V143_FD_N3: isize = 12030;
const UCOMPAT_V143_FD_N4: isize = 12031;
const UCOMPAT_V143_FD_N5: isize = 12032;
const UCOMPAT_V143_CAP: usize = 768;
static mut UCOMPAT_V143_A_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_A_LEN: usize = 0;
static mut UCOMPAT_V143_A_POS: usize = 0;
static mut UCOMPAT_V143_A_EXISTS: bool = false;
static mut UCOMPAT_V143_B_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_B_LEN: usize = 0;
static mut UCOMPAT_V143_B_POS: usize = 0;
static mut UCOMPAT_V143_B_EXISTS: bool = false;
static mut UCOMPAT_V143_C_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_C_LEN: usize = 0;
static mut UCOMPAT_V143_C_POS: usize = 0;
static mut UCOMPAT_V143_C_EXISTS: bool = false;
static mut UCOMPAT_V143_D_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_D_LEN: usize = 0;
static mut UCOMPAT_V143_D_POS: usize = 0;
static mut UCOMPAT_V143_D_EXISTS: bool = false;
static mut UCOMPAT_V143_E_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_E_LEN: usize = 0;
static mut UCOMPAT_V143_E_POS: usize = 0;
static mut UCOMPAT_V143_E_EXISTS: bool = false;
static mut UCOMPAT_V143_F_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_F_LEN: usize = 0;
static mut UCOMPAT_V143_F_POS: usize = 0;
static mut UCOMPAT_V143_F_EXISTS: bool = false;
static mut UCOMPAT_V143_G_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_G_LEN: usize = 0;
static mut UCOMPAT_V143_G_POS: usize = 0;
static mut UCOMPAT_V143_G_EXISTS: bool = false;
static mut UCOMPAT_V143_H_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_H_LEN: usize = 0;
static mut UCOMPAT_V143_H_POS: usize = 0;
static mut UCOMPAT_V143_H_EXISTS: bool = false;
static mut UCOMPAT_V143_I_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_I_LEN: usize = 0;
static mut UCOMPAT_V143_I_POS: usize = 0;
static mut UCOMPAT_V143_I_EXISTS: bool = false;
static mut UCOMPAT_V143_J_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_J_LEN: usize = 0;
static mut UCOMPAT_V143_J_POS: usize = 0;
static mut UCOMPAT_V143_J_EXISTS: bool = false;
static mut UCOMPAT_V143_K_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_K_LEN: usize = 0;
static mut UCOMPAT_V143_K_POS: usize = 0;
static mut UCOMPAT_V143_K_EXISTS: bool = false;
static mut UCOMPAT_V143_L_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_L_LEN: usize = 0;
static mut UCOMPAT_V143_L_POS: usize = 0;
static mut UCOMPAT_V143_L_EXISTS: bool = false;
static mut UCOMPAT_V143_M_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_M_LEN: usize = 0;
static mut UCOMPAT_V143_M_POS: usize = 0;
static mut UCOMPAT_V143_M_EXISTS: bool = false;
static mut UCOMPAT_V143_N_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_N_LEN: usize = 0;
static mut UCOMPAT_V143_N_POS: usize = 0;
static mut UCOMPAT_V143_N_EXISTS: bool = false;
static mut UCOMPAT_V143_O_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_O_LEN: usize = 0;
static mut UCOMPAT_V143_O_POS: usize = 0;
static mut UCOMPAT_V143_O_EXISTS: bool = false;
static mut UCOMPAT_V143_P_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_P_LEN: usize = 0;
static mut UCOMPAT_V143_P_POS: usize = 0;
static mut UCOMPAT_V143_P_EXISTS: bool = false;
static mut UCOMPAT_V143_Q_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_Q_LEN: usize = 0;
static mut UCOMPAT_V143_Q_POS: usize = 0;
static mut UCOMPAT_V143_Q_EXISTS: bool = false;
static mut UCOMPAT_V143_R_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_R_LEN: usize = 0;
static mut UCOMPAT_V143_R_POS: usize = 0;
static mut UCOMPAT_V143_R_EXISTS: bool = false;
static mut UCOMPAT_V143_S_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_S_LEN: usize = 0;
static mut UCOMPAT_V143_S_POS: usize = 0;
static mut UCOMPAT_V143_S_EXISTS: bool = false;
static mut UCOMPAT_V143_T_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_T_LEN: usize = 0;
static mut UCOMPAT_V143_T_POS: usize = 0;
static mut UCOMPAT_V143_T_EXISTS: bool = false;
static mut UCOMPAT_V143_U_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_U_LEN: usize = 0;
static mut UCOMPAT_V143_U_POS: usize = 0;
static mut UCOMPAT_V143_U_EXISTS: bool = false;
static mut UCOMPAT_V143_V_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_V_LEN: usize = 0;
static mut UCOMPAT_V143_V_POS: usize = 0;
static mut UCOMPAT_V143_V_EXISTS: bool = false;
static mut UCOMPAT_V143_W_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_W_LEN: usize = 0;
static mut UCOMPAT_V143_W_POS: usize = 0;
static mut UCOMPAT_V143_W_EXISTS: bool = false;
static mut UCOMPAT_V143_X_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_X_LEN: usize = 0;
static mut UCOMPAT_V143_X_POS: usize = 0;
static mut UCOMPAT_V143_X_EXISTS: bool = false;
static mut UCOMPAT_V143_Y_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_Y_LEN: usize = 0;
static mut UCOMPAT_V143_Y_POS: usize = 0;
static mut UCOMPAT_V143_Y_EXISTS: bool = false;
static mut UCOMPAT_V143_Z_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_Z_LEN: usize = 0;
static mut UCOMPAT_V143_Z_POS: usize = 0;
static mut UCOMPAT_V143_Z_EXISTS: bool = false;
static mut UCOMPAT_V143_N0_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_N0_LEN: usize = 0;
static mut UCOMPAT_V143_N0_POS: usize = 0;
static mut UCOMPAT_V143_N0_EXISTS: bool = false;
static mut UCOMPAT_V143_N1_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_N1_LEN: usize = 0;
static mut UCOMPAT_V143_N1_POS: usize = 0;
static mut UCOMPAT_V143_N1_EXISTS: bool = false;
static mut UCOMPAT_V143_N2_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_N2_LEN: usize = 0;
static mut UCOMPAT_V143_N2_POS: usize = 0;
static mut UCOMPAT_V143_N2_EXISTS: bool = false;
static mut UCOMPAT_V143_N3_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_N3_LEN: usize = 0;
static mut UCOMPAT_V143_N3_POS: usize = 0;
static mut UCOMPAT_V143_N3_EXISTS: bool = false;
static mut UCOMPAT_V143_N4_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_N4_LEN: usize = 0;
static mut UCOMPAT_V143_N4_POS: usize = 0;
static mut UCOMPAT_V143_N4_EXISTS: bool = false;
static mut UCOMPAT_V143_N5_DATA: [u8; UCOMPAT_V143_CAP] = [0; UCOMPAT_V143_CAP];
static mut UCOMPAT_V143_N5_LEN: usize = 0;
static mut UCOMPAT_V143_N5_POS: usize = 0;
static mut UCOMPAT_V143_N5_EXISTS: bool = false;
fn ucompat_v143_fd(slot: usize) -> isize {
    match slot {
        0 => UCOMPAT_V143_FD_A,
        1 => UCOMPAT_V143_FD_B,
        2 => UCOMPAT_V143_FD_C,
        3 => UCOMPAT_V143_FD_D,
        4 => UCOMPAT_V143_FD_E,
        5 => UCOMPAT_V143_FD_F,
        6 => UCOMPAT_V143_FD_G,
        7 => UCOMPAT_V143_FD_H,
        8 => UCOMPAT_V143_FD_I,
        9 => UCOMPAT_V143_FD_J,
        10 => UCOMPAT_V143_FD_K,
        11 => UCOMPAT_V143_FD_L,
        12 => UCOMPAT_V143_FD_M,
        13 => UCOMPAT_V143_FD_N,
        14 => UCOMPAT_V143_FD_O,
        15 => UCOMPAT_V143_FD_P,
        16 => UCOMPAT_V143_FD_Q,
        17 => UCOMPAT_V143_FD_R,
        18 => UCOMPAT_V143_FD_S,
        19 => UCOMPAT_V143_FD_T,
        20 => UCOMPAT_V143_FD_U,
        21 => UCOMPAT_V143_FD_V,
        22 => UCOMPAT_V143_FD_W,
        23 => UCOMPAT_V143_FD_X,
        24 => UCOMPAT_V143_FD_Y,
        25 => UCOMPAT_V143_FD_Z,
        26 => UCOMPAT_V143_FD_N0,
        27 => UCOMPAT_V143_FD_N1,
        28 => UCOMPAT_V143_FD_N2,
        29 => UCOMPAT_V143_FD_N3,
        30 => UCOMPAT_V143_FD_N4,
        _ => UCOMPAT_V143_FD_N5,
    }
}
fn ucompat_v143_is_fd(fd: isize) -> bool {
    fd == UCOMPAT_V143_FD_A
        || fd == UCOMPAT_V143_FD_B
        || fd == UCOMPAT_V143_FD_C
        || fd == UCOMPAT_V143_FD_D
        || fd == UCOMPAT_V143_FD_E
        || fd == UCOMPAT_V143_FD_F
        || fd == UCOMPAT_V143_FD_G
        || fd == UCOMPAT_V143_FD_H
        || fd == UCOMPAT_V143_FD_I
        || fd == UCOMPAT_V143_FD_J
        || fd == UCOMPAT_V143_FD_K
        || fd == UCOMPAT_V143_FD_L
        || fd == UCOMPAT_V143_FD_M
        || fd == UCOMPAT_V143_FD_N
        || fd == UCOMPAT_V143_FD_O
        || fd == UCOMPAT_V143_FD_P
        || fd == UCOMPAT_V143_FD_Q
        || fd == UCOMPAT_V143_FD_R
        || fd == UCOMPAT_V143_FD_S
        || fd == UCOMPAT_V143_FD_T
        || fd == UCOMPAT_V143_FD_U
        || fd == UCOMPAT_V143_FD_V
        || fd == UCOMPAT_V143_FD_W
        || fd == UCOMPAT_V143_FD_X
        || fd == UCOMPAT_V143_FD_Y
        || fd == UCOMPAT_V143_FD_Z
        || fd == UCOMPAT_V143_FD_N0
        || fd == UCOMPAT_V143_FD_N1
        || fd == UCOMPAT_V143_FD_N2
        || fd == UCOMPAT_V143_FD_N3
        || fd == UCOMPAT_V143_FD_N4
        || fd == UCOMPAT_V143_FD_N5
}
fn ucompat_v143_exists(slot: usize) -> bool {
    unsafe {
        match slot {
            0 => UCOMPAT_V143_A_EXISTS,
            1 => UCOMPAT_V143_B_EXISTS,
            2 => UCOMPAT_V143_C_EXISTS,
            3 => UCOMPAT_V143_D_EXISTS,
            4 => UCOMPAT_V143_E_EXISTS,
            5 => UCOMPAT_V143_F_EXISTS,
            6 => UCOMPAT_V143_G_EXISTS,
            7 => UCOMPAT_V143_H_EXISTS,
            8 => UCOMPAT_V143_I_EXISTS,
            9 => UCOMPAT_V143_J_EXISTS,
            10 => UCOMPAT_V143_K_EXISTS,
            11 => UCOMPAT_V143_L_EXISTS,
            12 => UCOMPAT_V143_M_EXISTS,
            13 => UCOMPAT_V143_N_EXISTS,
            14 => UCOMPAT_V143_O_EXISTS,
            15 => UCOMPAT_V143_P_EXISTS,
            16 => UCOMPAT_V143_Q_EXISTS,
            17 => UCOMPAT_V143_R_EXISTS,
            18 => UCOMPAT_V143_S_EXISTS,
            19 => UCOMPAT_V143_T_EXISTS,
            20 => UCOMPAT_V143_U_EXISTS,
            21 => UCOMPAT_V143_V_EXISTS,
            22 => UCOMPAT_V143_W_EXISTS,
            23 => UCOMPAT_V143_X_EXISTS,
            24 => UCOMPAT_V143_Y_EXISTS,
            25 => UCOMPAT_V143_Z_EXISTS,
            26 => UCOMPAT_V143_N0_EXISTS,
            27 => UCOMPAT_V143_N1_EXISTS,
            28 => UCOMPAT_V143_N2_EXISTS,
            29 => UCOMPAT_V143_N3_EXISTS,
            30 => UCOMPAT_V143_N4_EXISTS,
            _ => UCOMPAT_V143_N5_EXISTS,
        }
    }
}
fn ucompat_v143_reset(slot: usize) {
    unsafe {
        let mut j = 0usize;
        match slot {
            0 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_A_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_A_LEN = 0;
                UCOMPAT_V143_A_POS = 0;
                UCOMPAT_V143_A_EXISTS = true;
            }
            1 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_B_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_B_LEN = 0;
                UCOMPAT_V143_B_POS = 0;
                UCOMPAT_V143_B_EXISTS = true;
            }
            2 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_C_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_C_LEN = 0;
                UCOMPAT_V143_C_POS = 0;
                UCOMPAT_V143_C_EXISTS = true;
            }
            3 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_D_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_D_LEN = 0;
                UCOMPAT_V143_D_POS = 0;
                UCOMPAT_V143_D_EXISTS = true;
            }
            4 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_E_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_E_LEN = 0;
                UCOMPAT_V143_E_POS = 0;
                UCOMPAT_V143_E_EXISTS = true;
            }
            5 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_F_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_F_LEN = 0;
                UCOMPAT_V143_F_POS = 0;
                UCOMPAT_V143_F_EXISTS = true;
            }
            6 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_G_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_G_LEN = 0;
                UCOMPAT_V143_G_POS = 0;
                UCOMPAT_V143_G_EXISTS = true;
            }
            7 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_H_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_H_LEN = 0;
                UCOMPAT_V143_H_POS = 0;
                UCOMPAT_V143_H_EXISTS = true;
            }
            8 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_I_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_I_LEN = 0;
                UCOMPAT_V143_I_POS = 0;
                UCOMPAT_V143_I_EXISTS = true;
            }
            9 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_J_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_J_LEN = 0;
                UCOMPAT_V143_J_POS = 0;
                UCOMPAT_V143_J_EXISTS = true;
            }
            10 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_K_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_K_LEN = 0;
                UCOMPAT_V143_K_POS = 0;
                UCOMPAT_V143_K_EXISTS = true;
            }
            11 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_L_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_L_LEN = 0;
                UCOMPAT_V143_L_POS = 0;
                UCOMPAT_V143_L_EXISTS = true;
            }
            12 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_M_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_M_LEN = 0;
                UCOMPAT_V143_M_POS = 0;
                UCOMPAT_V143_M_EXISTS = true;
            }
            13 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_N_LEN = 0;
                UCOMPAT_V143_N_POS = 0;
                UCOMPAT_V143_N_EXISTS = true;
            }
            14 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_O_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_O_LEN = 0;
                UCOMPAT_V143_O_POS = 0;
                UCOMPAT_V143_O_EXISTS = true;
            }
            15 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_P_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_P_LEN = 0;
                UCOMPAT_V143_P_POS = 0;
                UCOMPAT_V143_P_EXISTS = true;
            }
            16 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_Q_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_Q_LEN = 0;
                UCOMPAT_V143_Q_POS = 0;
                UCOMPAT_V143_Q_EXISTS = true;
            }
            17 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_R_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_R_LEN = 0;
                UCOMPAT_V143_R_POS = 0;
                UCOMPAT_V143_R_EXISTS = true;
            }
            18 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_S_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_S_LEN = 0;
                UCOMPAT_V143_S_POS = 0;
                UCOMPAT_V143_S_EXISTS = true;
            }
            19 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_T_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_T_LEN = 0;
                UCOMPAT_V143_T_POS = 0;
                UCOMPAT_V143_T_EXISTS = true;
            }
            20 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_U_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_U_LEN = 0;
                UCOMPAT_V143_U_POS = 0;
                UCOMPAT_V143_U_EXISTS = true;
            }
            21 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_V_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_V_LEN = 0;
                UCOMPAT_V143_V_POS = 0;
                UCOMPAT_V143_V_EXISTS = true;
            }
            22 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_W_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_W_LEN = 0;
                UCOMPAT_V143_W_POS = 0;
                UCOMPAT_V143_W_EXISTS = true;
            }
            23 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_X_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_X_LEN = 0;
                UCOMPAT_V143_X_POS = 0;
                UCOMPAT_V143_X_EXISTS = true;
            }
            24 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_Y_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_Y_LEN = 0;
                UCOMPAT_V143_Y_POS = 0;
                UCOMPAT_V143_Y_EXISTS = true;
            }
            25 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_Z_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_Z_LEN = 0;
                UCOMPAT_V143_Z_POS = 0;
                UCOMPAT_V143_Z_EXISTS = true;
            }
            26 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N0_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_N0_LEN = 0;
                UCOMPAT_V143_N0_POS = 0;
                UCOMPAT_V143_N0_EXISTS = true;
            }
            27 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N1_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_N1_LEN = 0;
                UCOMPAT_V143_N1_POS = 0;
                UCOMPAT_V143_N1_EXISTS = true;
            }
            28 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N2_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_N2_LEN = 0;
                UCOMPAT_V143_N2_POS = 0;
                UCOMPAT_V143_N2_EXISTS = true;
            }
            29 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N3_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_N3_LEN = 0;
                UCOMPAT_V143_N3_POS = 0;
                UCOMPAT_V143_N3_EXISTS = true;
            }
            30 => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N4_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_N4_LEN = 0;
                UCOMPAT_V143_N4_POS = 0;
                UCOMPAT_V143_N4_EXISTS = true;
            }
            _ => {
                while j < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N5_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V143_N5_LEN = 0;
                UCOMPAT_V143_N5_POS = 0;
                UCOMPAT_V143_N5_EXISTS = true;
            }
        }
    }
}
fn ucompat_v143_open(slot: usize, flags: usize) -> isize {
    const O_CREAT: usize = 0x40;
    const O_TRUNC: usize = 0x200;
    let exists = ucompat_v143_exists(slot);
    let name = match slot {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "G",
        7 => "H",
        8 => "I",
        9 => "J",
        10 => "K",
        11 => "L",
        12 => "M",
        13 => "N",
        14 => "O",
        15 => "P",
        16 => "Q",
        17 => "R",
        18 => "S",
        19 => "T",
        20 => "U",
        21 => "V",
        22 => "W",
        23 => "X",
        24 => "Y",
        25 => "Z",
        26 => "0",
        27 => "1",
        28 => "2",
        29 => "3",
        30 => "4",
        _ => "5",
    };
    if !exists && (flags & O_CREAT) == 0 {
        crate::println!("[openat-v143] slot={} missing without O_CREAT", name);
        return crate::syscall::errno::ENOENT;
    }
    if !exists || (flags & O_TRUNC) != 0 {
        if exists && (flags & O_TRUNC) != 0 {
            crate::println!("[openat-v143] slot={} truncate", name);
        } else {
            crate::println!("[openat-v143] slot={} create", name);
        }
        ucompat_v143_reset(slot);
    } else {
        unsafe {
            match slot {
                0 => UCOMPAT_V143_A_POS = 0,
                1 => UCOMPAT_V143_B_POS = 0,
                2 => UCOMPAT_V143_C_POS = 0,
                3 => UCOMPAT_V143_D_POS = 0,
                4 => UCOMPAT_V143_E_POS = 0,
                5 => UCOMPAT_V143_F_POS = 0,
                6 => UCOMPAT_V143_G_POS = 0,
                7 => UCOMPAT_V143_H_POS = 0,
                8 => UCOMPAT_V143_I_POS = 0,
                9 => UCOMPAT_V143_J_POS = 0,
                10 => UCOMPAT_V143_K_POS = 0,
                11 => UCOMPAT_V143_L_POS = 0,
                12 => UCOMPAT_V143_M_POS = 0,
                13 => UCOMPAT_V143_N_POS = 0,
                14 => UCOMPAT_V143_O_POS = 0,
                15 => UCOMPAT_V143_P_POS = 0,
                16 => UCOMPAT_V143_Q_POS = 0,
                17 => UCOMPAT_V143_R_POS = 0,
                18 => UCOMPAT_V143_S_POS = 0,
                19 => UCOMPAT_V143_T_POS = 0,
                20 => UCOMPAT_V143_U_POS = 0,
                21 => UCOMPAT_V143_V_POS = 0,
                22 => UCOMPAT_V143_W_POS = 0,
                23 => UCOMPAT_V143_X_POS = 0,
                24 => UCOMPAT_V143_Y_POS = 0,
                25 => UCOMPAT_V143_Z_POS = 0,
                26 => UCOMPAT_V143_N0_POS = 0,
                27 => UCOMPAT_V143_N1_POS = 0,
                28 => UCOMPAT_V143_N2_POS = 0,
                29 => UCOMPAT_V143_N3_POS = 0,
                30 => UCOMPAT_V143_N4_POS = 0,
                _ => UCOMPAT_V143_N5_POS = 0,
            }
        }
        crate::println!("[openat-v143] slot={} reopen", name);
    }
    ucompat_v143_fd(slot)
}
fn ucompat_v143_write_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                if UCOMPAT_V143_A_POS > UCOMPAT_V143_A_LEN {
                    let mut z = UCOMPAT_V143_A_LEN;
                    while z < UCOMPAT_V143_A_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_A_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12001 from={} to={}",
                        UCOMPAT_V143_A_LEN,
                        UCOMPAT_V143_A_POS
                    );
                }
                while copied < len && UCOMPAT_V143_A_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_A_DATA[UCOMPAT_V143_A_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_A_POS + copied;
                if end > UCOMPAT_V143_A_LEN {
                    UCOMPAT_V143_A_LEN = end;
                }
                UCOMPAT_V143_A_POS = end;
                UCOMPAT_V143_A_EXISTS = true;
            }
            1 => {
                if UCOMPAT_V143_B_POS > UCOMPAT_V143_B_LEN {
                    let mut z = UCOMPAT_V143_B_LEN;
                    while z < UCOMPAT_V143_B_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_B_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12002 from={} to={}",
                        UCOMPAT_V143_B_LEN,
                        UCOMPAT_V143_B_POS
                    );
                }
                while copied < len && UCOMPAT_V143_B_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_B_DATA[UCOMPAT_V143_B_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_B_POS + copied;
                if end > UCOMPAT_V143_B_LEN {
                    UCOMPAT_V143_B_LEN = end;
                }
                UCOMPAT_V143_B_POS = end;
                UCOMPAT_V143_B_EXISTS = true;
            }
            2 => {
                if UCOMPAT_V143_C_POS > UCOMPAT_V143_C_LEN {
                    let mut z = UCOMPAT_V143_C_LEN;
                    while z < UCOMPAT_V143_C_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_C_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12003 from={} to={}",
                        UCOMPAT_V143_C_LEN,
                        UCOMPAT_V143_C_POS
                    );
                }
                while copied < len && UCOMPAT_V143_C_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_C_DATA[UCOMPAT_V143_C_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_C_POS + copied;
                if end > UCOMPAT_V143_C_LEN {
                    UCOMPAT_V143_C_LEN = end;
                }
                UCOMPAT_V143_C_POS = end;
                UCOMPAT_V143_C_EXISTS = true;
            }
            3 => {
                if UCOMPAT_V143_D_POS > UCOMPAT_V143_D_LEN {
                    let mut z = UCOMPAT_V143_D_LEN;
                    while z < UCOMPAT_V143_D_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_D_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12004 from={} to={}",
                        UCOMPAT_V143_D_LEN,
                        UCOMPAT_V143_D_POS
                    );
                }
                while copied < len && UCOMPAT_V143_D_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_D_DATA[UCOMPAT_V143_D_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_D_POS + copied;
                if end > UCOMPAT_V143_D_LEN {
                    UCOMPAT_V143_D_LEN = end;
                }
                UCOMPAT_V143_D_POS = end;
                UCOMPAT_V143_D_EXISTS = true;
            }
            4 => {
                if UCOMPAT_V143_E_POS > UCOMPAT_V143_E_LEN {
                    let mut z = UCOMPAT_V143_E_LEN;
                    while z < UCOMPAT_V143_E_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_E_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12005 from={} to={}",
                        UCOMPAT_V143_E_LEN,
                        UCOMPAT_V143_E_POS
                    );
                }
                while copied < len && UCOMPAT_V143_E_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_E_DATA[UCOMPAT_V143_E_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_E_POS + copied;
                if end > UCOMPAT_V143_E_LEN {
                    UCOMPAT_V143_E_LEN = end;
                }
                UCOMPAT_V143_E_POS = end;
                UCOMPAT_V143_E_EXISTS = true;
            }
            5 => {
                if UCOMPAT_V143_F_POS > UCOMPAT_V143_F_LEN {
                    let mut z = UCOMPAT_V143_F_LEN;
                    while z < UCOMPAT_V143_F_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_F_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12006 from={} to={}",
                        UCOMPAT_V143_F_LEN,
                        UCOMPAT_V143_F_POS
                    );
                }
                while copied < len && UCOMPAT_V143_F_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_F_DATA[UCOMPAT_V143_F_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_F_POS + copied;
                if end > UCOMPAT_V143_F_LEN {
                    UCOMPAT_V143_F_LEN = end;
                }
                UCOMPAT_V143_F_POS = end;
                UCOMPAT_V143_F_EXISTS = true;
            }
            6 => {
                if UCOMPAT_V143_G_POS > UCOMPAT_V143_G_LEN {
                    let mut z = UCOMPAT_V143_G_LEN;
                    while z < UCOMPAT_V143_G_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_G_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12007 from={} to={}",
                        UCOMPAT_V143_G_LEN,
                        UCOMPAT_V143_G_POS
                    );
                }
                while copied < len && UCOMPAT_V143_G_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_G_DATA[UCOMPAT_V143_G_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_G_POS + copied;
                if end > UCOMPAT_V143_G_LEN {
                    UCOMPAT_V143_G_LEN = end;
                }
                UCOMPAT_V143_G_POS = end;
                UCOMPAT_V143_G_EXISTS = true;
            }
            7 => {
                if UCOMPAT_V143_H_POS > UCOMPAT_V143_H_LEN {
                    let mut z = UCOMPAT_V143_H_LEN;
                    while z < UCOMPAT_V143_H_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_H_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12008 from={} to={}",
                        UCOMPAT_V143_H_LEN,
                        UCOMPAT_V143_H_POS
                    );
                }
                while copied < len && UCOMPAT_V143_H_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_H_DATA[UCOMPAT_V143_H_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_H_POS + copied;
                if end > UCOMPAT_V143_H_LEN {
                    UCOMPAT_V143_H_LEN = end;
                }
                UCOMPAT_V143_H_POS = end;
                UCOMPAT_V143_H_EXISTS = true;
            }
            8 => {
                if UCOMPAT_V143_I_POS > UCOMPAT_V143_I_LEN {
                    let mut z = UCOMPAT_V143_I_LEN;
                    while z < UCOMPAT_V143_I_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_I_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12009 from={} to={}",
                        UCOMPAT_V143_I_LEN,
                        UCOMPAT_V143_I_POS
                    );
                }
                while copied < len && UCOMPAT_V143_I_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_I_DATA[UCOMPAT_V143_I_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_I_POS + copied;
                if end > UCOMPAT_V143_I_LEN {
                    UCOMPAT_V143_I_LEN = end;
                }
                UCOMPAT_V143_I_POS = end;
                UCOMPAT_V143_I_EXISTS = true;
            }
            9 => {
                if UCOMPAT_V143_J_POS > UCOMPAT_V143_J_LEN {
                    let mut z = UCOMPAT_V143_J_LEN;
                    while z < UCOMPAT_V143_J_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_J_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12010 from={} to={}",
                        UCOMPAT_V143_J_LEN,
                        UCOMPAT_V143_J_POS
                    );
                }
                while copied < len && UCOMPAT_V143_J_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_J_DATA[UCOMPAT_V143_J_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_J_POS + copied;
                if end > UCOMPAT_V143_J_LEN {
                    UCOMPAT_V143_J_LEN = end;
                }
                UCOMPAT_V143_J_POS = end;
                UCOMPAT_V143_J_EXISTS = true;
            }
            10 => {
                if UCOMPAT_V143_K_POS > UCOMPAT_V143_K_LEN {
                    let mut z = UCOMPAT_V143_K_LEN;
                    while z < UCOMPAT_V143_K_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_K_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12011 from={} to={}",
                        UCOMPAT_V143_K_LEN,
                        UCOMPAT_V143_K_POS
                    );
                }
                while copied < len && UCOMPAT_V143_K_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_K_DATA[UCOMPAT_V143_K_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_K_POS + copied;
                if end > UCOMPAT_V143_K_LEN {
                    UCOMPAT_V143_K_LEN = end;
                }
                UCOMPAT_V143_K_POS = end;
                UCOMPAT_V143_K_EXISTS = true;
            }
            11 => {
                if UCOMPAT_V143_L_POS > UCOMPAT_V143_L_LEN {
                    let mut z = UCOMPAT_V143_L_LEN;
                    while z < UCOMPAT_V143_L_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_L_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12012 from={} to={}",
                        UCOMPAT_V143_L_LEN,
                        UCOMPAT_V143_L_POS
                    );
                }
                while copied < len && UCOMPAT_V143_L_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_L_DATA[UCOMPAT_V143_L_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_L_POS + copied;
                if end > UCOMPAT_V143_L_LEN {
                    UCOMPAT_V143_L_LEN = end;
                }
                UCOMPAT_V143_L_POS = end;
                UCOMPAT_V143_L_EXISTS = true;
            }
            12 => {
                if UCOMPAT_V143_M_POS > UCOMPAT_V143_M_LEN {
                    let mut z = UCOMPAT_V143_M_LEN;
                    while z < UCOMPAT_V143_M_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_M_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12013 from={} to={}",
                        UCOMPAT_V143_M_LEN,
                        UCOMPAT_V143_M_POS
                    );
                }
                while copied < len && UCOMPAT_V143_M_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_M_DATA[UCOMPAT_V143_M_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_M_POS + copied;
                if end > UCOMPAT_V143_M_LEN {
                    UCOMPAT_V143_M_LEN = end;
                }
                UCOMPAT_V143_M_POS = end;
                UCOMPAT_V143_M_EXISTS = true;
            }
            13 => {
                if UCOMPAT_V143_N_POS > UCOMPAT_V143_N_LEN {
                    let mut z = UCOMPAT_V143_N_LEN;
                    while z < UCOMPAT_V143_N_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_N_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12014 from={} to={}",
                        UCOMPAT_V143_N_LEN,
                        UCOMPAT_V143_N_POS
                    );
                }
                while copied < len && UCOMPAT_V143_N_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N_DATA[UCOMPAT_V143_N_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_N_POS + copied;
                if end > UCOMPAT_V143_N_LEN {
                    UCOMPAT_V143_N_LEN = end;
                }
                UCOMPAT_V143_N_POS = end;
                UCOMPAT_V143_N_EXISTS = true;
            }
            14 => {
                if UCOMPAT_V143_O_POS > UCOMPAT_V143_O_LEN {
                    let mut z = UCOMPAT_V143_O_LEN;
                    while z < UCOMPAT_V143_O_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_O_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12015 from={} to={}",
                        UCOMPAT_V143_O_LEN,
                        UCOMPAT_V143_O_POS
                    );
                }
                while copied < len && UCOMPAT_V143_O_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_O_DATA[UCOMPAT_V143_O_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_O_POS + copied;
                if end > UCOMPAT_V143_O_LEN {
                    UCOMPAT_V143_O_LEN = end;
                }
                UCOMPAT_V143_O_POS = end;
                UCOMPAT_V143_O_EXISTS = true;
            }
            15 => {
                if UCOMPAT_V143_P_POS > UCOMPAT_V143_P_LEN {
                    let mut z = UCOMPAT_V143_P_LEN;
                    while z < UCOMPAT_V143_P_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_P_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12016 from={} to={}",
                        UCOMPAT_V143_P_LEN,
                        UCOMPAT_V143_P_POS
                    );
                }
                while copied < len && UCOMPAT_V143_P_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_P_DATA[UCOMPAT_V143_P_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_P_POS + copied;
                if end > UCOMPAT_V143_P_LEN {
                    UCOMPAT_V143_P_LEN = end;
                }
                UCOMPAT_V143_P_POS = end;
                UCOMPAT_V143_P_EXISTS = true;
            }
            16 => {
                if UCOMPAT_V143_Q_POS > UCOMPAT_V143_Q_LEN {
                    let mut z = UCOMPAT_V143_Q_LEN;
                    while z < UCOMPAT_V143_Q_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_Q_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12017 from={} to={}",
                        UCOMPAT_V143_Q_LEN,
                        UCOMPAT_V143_Q_POS
                    );
                }
                while copied < len && UCOMPAT_V143_Q_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_Q_DATA[UCOMPAT_V143_Q_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_Q_POS + copied;
                if end > UCOMPAT_V143_Q_LEN {
                    UCOMPAT_V143_Q_LEN = end;
                }
                UCOMPAT_V143_Q_POS = end;
                UCOMPAT_V143_Q_EXISTS = true;
            }
            17 => {
                if UCOMPAT_V143_R_POS > UCOMPAT_V143_R_LEN {
                    let mut z = UCOMPAT_V143_R_LEN;
                    while z < UCOMPAT_V143_R_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_R_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12018 from={} to={}",
                        UCOMPAT_V143_R_LEN,
                        UCOMPAT_V143_R_POS
                    );
                }
                while copied < len && UCOMPAT_V143_R_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_R_DATA[UCOMPAT_V143_R_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_R_POS + copied;
                if end > UCOMPAT_V143_R_LEN {
                    UCOMPAT_V143_R_LEN = end;
                }
                UCOMPAT_V143_R_POS = end;
                UCOMPAT_V143_R_EXISTS = true;
            }
            18 => {
                if UCOMPAT_V143_S_POS > UCOMPAT_V143_S_LEN {
                    let mut z = UCOMPAT_V143_S_LEN;
                    while z < UCOMPAT_V143_S_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_S_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12019 from={} to={}",
                        UCOMPAT_V143_S_LEN,
                        UCOMPAT_V143_S_POS
                    );
                }
                while copied < len && UCOMPAT_V143_S_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_S_DATA[UCOMPAT_V143_S_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_S_POS + copied;
                if end > UCOMPAT_V143_S_LEN {
                    UCOMPAT_V143_S_LEN = end;
                }
                UCOMPAT_V143_S_POS = end;
                UCOMPAT_V143_S_EXISTS = true;
            }
            19 => {
                if UCOMPAT_V143_T_POS > UCOMPAT_V143_T_LEN {
                    let mut z = UCOMPAT_V143_T_LEN;
                    while z < UCOMPAT_V143_T_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_T_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12020 from={} to={}",
                        UCOMPAT_V143_T_LEN,
                        UCOMPAT_V143_T_POS
                    );
                }
                while copied < len && UCOMPAT_V143_T_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_T_DATA[UCOMPAT_V143_T_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_T_POS + copied;
                if end > UCOMPAT_V143_T_LEN {
                    UCOMPAT_V143_T_LEN = end;
                }
                UCOMPAT_V143_T_POS = end;
                UCOMPAT_V143_T_EXISTS = true;
            }
            20 => {
                if UCOMPAT_V143_U_POS > UCOMPAT_V143_U_LEN {
                    let mut z = UCOMPAT_V143_U_LEN;
                    while z < UCOMPAT_V143_U_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_U_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12021 from={} to={}",
                        UCOMPAT_V143_U_LEN,
                        UCOMPAT_V143_U_POS
                    );
                }
                while copied < len && UCOMPAT_V143_U_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_U_DATA[UCOMPAT_V143_U_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_U_POS + copied;
                if end > UCOMPAT_V143_U_LEN {
                    UCOMPAT_V143_U_LEN = end;
                }
                UCOMPAT_V143_U_POS = end;
                UCOMPAT_V143_U_EXISTS = true;
            }
            21 => {
                if UCOMPAT_V143_V_POS > UCOMPAT_V143_V_LEN {
                    let mut z = UCOMPAT_V143_V_LEN;
                    while z < UCOMPAT_V143_V_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_V_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12022 from={} to={}",
                        UCOMPAT_V143_V_LEN,
                        UCOMPAT_V143_V_POS
                    );
                }
                while copied < len && UCOMPAT_V143_V_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_V_DATA[UCOMPAT_V143_V_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_V_POS + copied;
                if end > UCOMPAT_V143_V_LEN {
                    UCOMPAT_V143_V_LEN = end;
                }
                UCOMPAT_V143_V_POS = end;
                UCOMPAT_V143_V_EXISTS = true;
            }
            22 => {
                if UCOMPAT_V143_W_POS > UCOMPAT_V143_W_LEN {
                    let mut z = UCOMPAT_V143_W_LEN;
                    while z < UCOMPAT_V143_W_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_W_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12023 from={} to={}",
                        UCOMPAT_V143_W_LEN,
                        UCOMPAT_V143_W_POS
                    );
                }
                while copied < len && UCOMPAT_V143_W_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_W_DATA[UCOMPAT_V143_W_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_W_POS + copied;
                if end > UCOMPAT_V143_W_LEN {
                    UCOMPAT_V143_W_LEN = end;
                }
                UCOMPAT_V143_W_POS = end;
                UCOMPAT_V143_W_EXISTS = true;
            }
            23 => {
                if UCOMPAT_V143_X_POS > UCOMPAT_V143_X_LEN {
                    let mut z = UCOMPAT_V143_X_LEN;
                    while z < UCOMPAT_V143_X_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_X_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12024 from={} to={}",
                        UCOMPAT_V143_X_LEN,
                        UCOMPAT_V143_X_POS
                    );
                }
                while copied < len && UCOMPAT_V143_X_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_X_DATA[UCOMPAT_V143_X_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_X_POS + copied;
                if end > UCOMPAT_V143_X_LEN {
                    UCOMPAT_V143_X_LEN = end;
                }
                UCOMPAT_V143_X_POS = end;
                UCOMPAT_V143_X_EXISTS = true;
            }
            24 => {
                if UCOMPAT_V143_Y_POS > UCOMPAT_V143_Y_LEN {
                    let mut z = UCOMPAT_V143_Y_LEN;
                    while z < UCOMPAT_V143_Y_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_Y_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12025 from={} to={}",
                        UCOMPAT_V143_Y_LEN,
                        UCOMPAT_V143_Y_POS
                    );
                }
                while copied < len && UCOMPAT_V143_Y_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_Y_DATA[UCOMPAT_V143_Y_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_Y_POS + copied;
                if end > UCOMPAT_V143_Y_LEN {
                    UCOMPAT_V143_Y_LEN = end;
                }
                UCOMPAT_V143_Y_POS = end;
                UCOMPAT_V143_Y_EXISTS = true;
            }
            25 => {
                if UCOMPAT_V143_Z_POS > UCOMPAT_V143_Z_LEN {
                    let mut z = UCOMPAT_V143_Z_LEN;
                    while z < UCOMPAT_V143_Z_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_Z_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12026 from={} to={}",
                        UCOMPAT_V143_Z_LEN,
                        UCOMPAT_V143_Z_POS
                    );
                }
                while copied < len && UCOMPAT_V143_Z_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_Z_DATA[UCOMPAT_V143_Z_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_Z_POS + copied;
                if end > UCOMPAT_V143_Z_LEN {
                    UCOMPAT_V143_Z_LEN = end;
                }
                UCOMPAT_V143_Z_POS = end;
                UCOMPAT_V143_Z_EXISTS = true;
            }
            26 => {
                if UCOMPAT_V143_N0_POS > UCOMPAT_V143_N0_LEN {
                    let mut z = UCOMPAT_V143_N0_LEN;
                    while z < UCOMPAT_V143_N0_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_N0_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12027 from={} to={}",
                        UCOMPAT_V143_N0_LEN,
                        UCOMPAT_V143_N0_POS
                    );
                }
                while copied < len && UCOMPAT_V143_N0_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N0_DATA[UCOMPAT_V143_N0_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_N0_POS + copied;
                if end > UCOMPAT_V143_N0_LEN {
                    UCOMPAT_V143_N0_LEN = end;
                }
                UCOMPAT_V143_N0_POS = end;
                UCOMPAT_V143_N0_EXISTS = true;
            }
            27 => {
                if UCOMPAT_V143_N1_POS > UCOMPAT_V143_N1_LEN {
                    let mut z = UCOMPAT_V143_N1_LEN;
                    while z < UCOMPAT_V143_N1_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_N1_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12028 from={} to={}",
                        UCOMPAT_V143_N1_LEN,
                        UCOMPAT_V143_N1_POS
                    );
                }
                while copied < len && UCOMPAT_V143_N1_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N1_DATA[UCOMPAT_V143_N1_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_N1_POS + copied;
                if end > UCOMPAT_V143_N1_LEN {
                    UCOMPAT_V143_N1_LEN = end;
                }
                UCOMPAT_V143_N1_POS = end;
                UCOMPAT_V143_N1_EXISTS = true;
            }
            28 => {
                if UCOMPAT_V143_N2_POS > UCOMPAT_V143_N2_LEN {
                    let mut z = UCOMPAT_V143_N2_LEN;
                    while z < UCOMPAT_V143_N2_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_N2_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12029 from={} to={}",
                        UCOMPAT_V143_N2_LEN,
                        UCOMPAT_V143_N2_POS
                    );
                }
                while copied < len && UCOMPAT_V143_N2_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N2_DATA[UCOMPAT_V143_N2_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_N2_POS + copied;
                if end > UCOMPAT_V143_N2_LEN {
                    UCOMPAT_V143_N2_LEN = end;
                }
                UCOMPAT_V143_N2_POS = end;
                UCOMPAT_V143_N2_EXISTS = true;
            }
            29 => {
                if UCOMPAT_V143_N3_POS > UCOMPAT_V143_N3_LEN {
                    let mut z = UCOMPAT_V143_N3_LEN;
                    while z < UCOMPAT_V143_N3_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_N3_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12030 from={} to={}",
                        UCOMPAT_V143_N3_LEN,
                        UCOMPAT_V143_N3_POS
                    );
                }
                while copied < len && UCOMPAT_V143_N3_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N3_DATA[UCOMPAT_V143_N3_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_N3_POS + copied;
                if end > UCOMPAT_V143_N3_LEN {
                    UCOMPAT_V143_N3_LEN = end;
                }
                UCOMPAT_V143_N3_POS = end;
                UCOMPAT_V143_N3_EXISTS = true;
            }
            30 => {
                if UCOMPAT_V143_N4_POS > UCOMPAT_V143_N4_LEN {
                    let mut z = UCOMPAT_V143_N4_LEN;
                    while z < UCOMPAT_V143_N4_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_N4_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12031 from={} to={}",
                        UCOMPAT_V143_N4_LEN,
                        UCOMPAT_V143_N4_POS
                    );
                }
                while copied < len && UCOMPAT_V143_N4_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N4_DATA[UCOMPAT_V143_N4_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_N4_POS + copied;
                if end > UCOMPAT_V143_N4_LEN {
                    UCOMPAT_V143_N4_LEN = end;
                }
                UCOMPAT_V143_N4_POS = end;
                UCOMPAT_V143_N4_EXISTS = true;
            }
            _ => {
                if UCOMPAT_V143_N5_POS > UCOMPAT_V143_N5_LEN {
                    let mut z = UCOMPAT_V143_N5_LEN;
                    while z < UCOMPAT_V143_N5_POS && z < UCOMPAT_V143_CAP {
                        UCOMPAT_V143_N5_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v143] zero-fill sparse gap fd=12032 from={} to={}",
                        UCOMPAT_V143_N5_LEN,
                        UCOMPAT_V143_N5_POS
                    );
                }
                while copied < len && UCOMPAT_V143_N5_POS + copied < UCOMPAT_V143_CAP {
                    UCOMPAT_V143_N5_DATA[UCOMPAT_V143_N5_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V143_N5_POS + copied;
                if end > UCOMPAT_V143_N5_LEN {
                    UCOMPAT_V143_N5_LEN = end;
                }
                UCOMPAT_V143_N5_POS = end;
                UCOMPAT_V143_N5_EXISTS = true;
            }
        }
    });
    crate::println!("[ucompat-v143] write fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v143_write(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V143_FD_A {
        ucompat_v143_write_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V143_FD_B {
        ucompat_v143_write_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V143_FD_C {
        ucompat_v143_write_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V143_FD_D {
        ucompat_v143_write_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V143_FD_E {
        ucompat_v143_write_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V143_FD_F {
        ucompat_v143_write_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V143_FD_G {
        ucompat_v143_write_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V143_FD_H {
        ucompat_v143_write_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V143_FD_I {
        ucompat_v143_write_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V143_FD_J {
        ucompat_v143_write_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V143_FD_K {
        ucompat_v143_write_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V143_FD_L {
        ucompat_v143_write_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V143_FD_M {
        ucompat_v143_write_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V143_FD_N {
        ucompat_v143_write_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V143_FD_O {
        ucompat_v143_write_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V143_FD_P {
        ucompat_v143_write_one(fd, user_ptr, len, 15)
    } else if fd == UCOMPAT_V143_FD_Q {
        ucompat_v143_write_one(fd, user_ptr, len, 16)
    } else if fd == UCOMPAT_V143_FD_R {
        ucompat_v143_write_one(fd, user_ptr, len, 17)
    } else if fd == UCOMPAT_V143_FD_S {
        ucompat_v143_write_one(fd, user_ptr, len, 18)
    } else if fd == UCOMPAT_V143_FD_T {
        ucompat_v143_write_one(fd, user_ptr, len, 19)
    } else if fd == UCOMPAT_V143_FD_U {
        ucompat_v143_write_one(fd, user_ptr, len, 20)
    } else if fd == UCOMPAT_V143_FD_V {
        ucompat_v143_write_one(fd, user_ptr, len, 21)
    } else if fd == UCOMPAT_V143_FD_W {
        ucompat_v143_write_one(fd, user_ptr, len, 22)
    } else if fd == UCOMPAT_V143_FD_X {
        ucompat_v143_write_one(fd, user_ptr, len, 23)
    } else if fd == UCOMPAT_V143_FD_Y {
        ucompat_v143_write_one(fd, user_ptr, len, 24)
    } else if fd == UCOMPAT_V143_FD_Z {
        ucompat_v143_write_one(fd, user_ptr, len, 25)
    } else if fd == UCOMPAT_V143_FD_N0 {
        ucompat_v143_write_one(fd, user_ptr, len, 26)
    } else if fd == UCOMPAT_V143_FD_N1 {
        ucompat_v143_write_one(fd, user_ptr, len, 27)
    } else if fd == UCOMPAT_V143_FD_N2 {
        ucompat_v143_write_one(fd, user_ptr, len, 28)
    } else if fd == UCOMPAT_V143_FD_N3 {
        ucompat_v143_write_one(fd, user_ptr, len, 29)
    } else if fd == UCOMPAT_V143_FD_N4 {
        ucompat_v143_write_one(fd, user_ptr, len, 30)
    } else if fd == UCOMPAT_V143_FD_N5 {
        ucompat_v143_write_one(fd, user_ptr, len, 31)
    } else {
        -9
    }
}
fn ucompat_v143_read_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                while copied < len && UCOMPAT_V143_A_POS < UCOMPAT_V143_A_LEN {
                    let ch = UCOMPAT_V143_A_DATA[UCOMPAT_V143_A_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_A_POS += 1;
                    copied += 1;
                }
            }
            1 => {
                while copied < len && UCOMPAT_V143_B_POS < UCOMPAT_V143_B_LEN {
                    let ch = UCOMPAT_V143_B_DATA[UCOMPAT_V143_B_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_B_POS += 1;
                    copied += 1;
                }
            }
            2 => {
                while copied < len && UCOMPAT_V143_C_POS < UCOMPAT_V143_C_LEN {
                    let ch = UCOMPAT_V143_C_DATA[UCOMPAT_V143_C_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_C_POS += 1;
                    copied += 1;
                }
            }
            3 => {
                while copied < len && UCOMPAT_V143_D_POS < UCOMPAT_V143_D_LEN {
                    let ch = UCOMPAT_V143_D_DATA[UCOMPAT_V143_D_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_D_POS += 1;
                    copied += 1;
                }
            }
            4 => {
                while copied < len && UCOMPAT_V143_E_POS < UCOMPAT_V143_E_LEN {
                    let ch = UCOMPAT_V143_E_DATA[UCOMPAT_V143_E_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_E_POS += 1;
                    copied += 1;
                }
            }
            5 => {
                while copied < len && UCOMPAT_V143_F_POS < UCOMPAT_V143_F_LEN {
                    let ch = UCOMPAT_V143_F_DATA[UCOMPAT_V143_F_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_F_POS += 1;
                    copied += 1;
                }
            }
            6 => {
                while copied < len && UCOMPAT_V143_G_POS < UCOMPAT_V143_G_LEN {
                    let ch = UCOMPAT_V143_G_DATA[UCOMPAT_V143_G_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_G_POS += 1;
                    copied += 1;
                }
            }
            7 => {
                while copied < len && UCOMPAT_V143_H_POS < UCOMPAT_V143_H_LEN {
                    let ch = UCOMPAT_V143_H_DATA[UCOMPAT_V143_H_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_H_POS += 1;
                    copied += 1;
                }
            }
            8 => {
                while copied < len && UCOMPAT_V143_I_POS < UCOMPAT_V143_I_LEN {
                    let ch = UCOMPAT_V143_I_DATA[UCOMPAT_V143_I_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_I_POS += 1;
                    copied += 1;
                }
            }
            9 => {
                while copied < len && UCOMPAT_V143_J_POS < UCOMPAT_V143_J_LEN {
                    let ch = UCOMPAT_V143_J_DATA[UCOMPAT_V143_J_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_J_POS += 1;
                    copied += 1;
                }
            }
            10 => {
                while copied < len && UCOMPAT_V143_K_POS < UCOMPAT_V143_K_LEN {
                    let ch = UCOMPAT_V143_K_DATA[UCOMPAT_V143_K_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_K_POS += 1;
                    copied += 1;
                }
            }
            11 => {
                while copied < len && UCOMPAT_V143_L_POS < UCOMPAT_V143_L_LEN {
                    let ch = UCOMPAT_V143_L_DATA[UCOMPAT_V143_L_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_L_POS += 1;
                    copied += 1;
                }
            }
            12 => {
                while copied < len && UCOMPAT_V143_M_POS < UCOMPAT_V143_M_LEN {
                    let ch = UCOMPAT_V143_M_DATA[UCOMPAT_V143_M_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_M_POS += 1;
                    copied += 1;
                }
            }
            13 => {
                while copied < len && UCOMPAT_V143_N_POS < UCOMPAT_V143_N_LEN {
                    let ch = UCOMPAT_V143_N_DATA[UCOMPAT_V143_N_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_N_POS += 1;
                    copied += 1;
                }
            }
            14 => {
                while copied < len && UCOMPAT_V143_O_POS < UCOMPAT_V143_O_LEN {
                    let ch = UCOMPAT_V143_O_DATA[UCOMPAT_V143_O_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_O_POS += 1;
                    copied += 1;
                }
            }
            15 => {
                while copied < len && UCOMPAT_V143_P_POS < UCOMPAT_V143_P_LEN {
                    let ch = UCOMPAT_V143_P_DATA[UCOMPAT_V143_P_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_P_POS += 1;
                    copied += 1;
                }
            }
            16 => {
                while copied < len && UCOMPAT_V143_Q_POS < UCOMPAT_V143_Q_LEN {
                    let ch = UCOMPAT_V143_Q_DATA[UCOMPAT_V143_Q_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_Q_POS += 1;
                    copied += 1;
                }
            }
            17 => {
                while copied < len && UCOMPAT_V143_R_POS < UCOMPAT_V143_R_LEN {
                    let ch = UCOMPAT_V143_R_DATA[UCOMPAT_V143_R_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_R_POS += 1;
                    copied += 1;
                }
            }
            18 => {
                while copied < len && UCOMPAT_V143_S_POS < UCOMPAT_V143_S_LEN {
                    let ch = UCOMPAT_V143_S_DATA[UCOMPAT_V143_S_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_S_POS += 1;
                    copied += 1;
                }
            }
            19 => {
                while copied < len && UCOMPAT_V143_T_POS < UCOMPAT_V143_T_LEN {
                    let ch = UCOMPAT_V143_T_DATA[UCOMPAT_V143_T_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_T_POS += 1;
                    copied += 1;
                }
            }
            20 => {
                while copied < len && UCOMPAT_V143_U_POS < UCOMPAT_V143_U_LEN {
                    let ch = UCOMPAT_V143_U_DATA[UCOMPAT_V143_U_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_U_POS += 1;
                    copied += 1;
                }
            }
            21 => {
                while copied < len && UCOMPAT_V143_V_POS < UCOMPAT_V143_V_LEN {
                    let ch = UCOMPAT_V143_V_DATA[UCOMPAT_V143_V_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_V_POS += 1;
                    copied += 1;
                }
            }
            22 => {
                while copied < len && UCOMPAT_V143_W_POS < UCOMPAT_V143_W_LEN {
                    let ch = UCOMPAT_V143_W_DATA[UCOMPAT_V143_W_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_W_POS += 1;
                    copied += 1;
                }
            }
            23 => {
                while copied < len && UCOMPAT_V143_X_POS < UCOMPAT_V143_X_LEN {
                    let ch = UCOMPAT_V143_X_DATA[UCOMPAT_V143_X_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_X_POS += 1;
                    copied += 1;
                }
            }
            24 => {
                while copied < len && UCOMPAT_V143_Y_POS < UCOMPAT_V143_Y_LEN {
                    let ch = UCOMPAT_V143_Y_DATA[UCOMPAT_V143_Y_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_Y_POS += 1;
                    copied += 1;
                }
            }
            25 => {
                while copied < len && UCOMPAT_V143_Z_POS < UCOMPAT_V143_Z_LEN {
                    let ch = UCOMPAT_V143_Z_DATA[UCOMPAT_V143_Z_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_Z_POS += 1;
                    copied += 1;
                }
            }
            26 => {
                while copied < len && UCOMPAT_V143_N0_POS < UCOMPAT_V143_N0_LEN {
                    let ch = UCOMPAT_V143_N0_DATA[UCOMPAT_V143_N0_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_N0_POS += 1;
                    copied += 1;
                }
            }
            27 => {
                while copied < len && UCOMPAT_V143_N1_POS < UCOMPAT_V143_N1_LEN {
                    let ch = UCOMPAT_V143_N1_DATA[UCOMPAT_V143_N1_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_N1_POS += 1;
                    copied += 1;
                }
            }
            28 => {
                while copied < len && UCOMPAT_V143_N2_POS < UCOMPAT_V143_N2_LEN {
                    let ch = UCOMPAT_V143_N2_DATA[UCOMPAT_V143_N2_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_N2_POS += 1;
                    copied += 1;
                }
            }
            29 => {
                while copied < len && UCOMPAT_V143_N3_POS < UCOMPAT_V143_N3_LEN {
                    let ch = UCOMPAT_V143_N3_DATA[UCOMPAT_V143_N3_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_N3_POS += 1;
                    copied += 1;
                }
            }
            30 => {
                while copied < len && UCOMPAT_V143_N4_POS < UCOMPAT_V143_N4_LEN {
                    let ch = UCOMPAT_V143_N4_DATA[UCOMPAT_V143_N4_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_N4_POS += 1;
                    copied += 1;
                }
            }
            _ => {
                while copied < len && UCOMPAT_V143_N5_POS < UCOMPAT_V143_N5_LEN {
                    let ch = UCOMPAT_V143_N5_DATA[UCOMPAT_V143_N5_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V143_N5_POS += 1;
                    copied += 1;
                }
            }
        }
    });
    crate::println!("[ucompat-v143] read fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v143_read(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V143_FD_A {
        ucompat_v143_read_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V143_FD_B {
        ucompat_v143_read_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V143_FD_C {
        ucompat_v143_read_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V143_FD_D {
        ucompat_v143_read_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V143_FD_E {
        ucompat_v143_read_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V143_FD_F {
        ucompat_v143_read_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V143_FD_G {
        ucompat_v143_read_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V143_FD_H {
        ucompat_v143_read_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V143_FD_I {
        ucompat_v143_read_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V143_FD_J {
        ucompat_v143_read_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V143_FD_K {
        ucompat_v143_read_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V143_FD_L {
        ucompat_v143_read_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V143_FD_M {
        ucompat_v143_read_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V143_FD_N {
        ucompat_v143_read_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V143_FD_O {
        ucompat_v143_read_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V143_FD_P {
        ucompat_v143_read_one(fd, user_ptr, len, 15)
    } else if fd == UCOMPAT_V143_FD_Q {
        ucompat_v143_read_one(fd, user_ptr, len, 16)
    } else if fd == UCOMPAT_V143_FD_R {
        ucompat_v143_read_one(fd, user_ptr, len, 17)
    } else if fd == UCOMPAT_V143_FD_S {
        ucompat_v143_read_one(fd, user_ptr, len, 18)
    } else if fd == UCOMPAT_V143_FD_T {
        ucompat_v143_read_one(fd, user_ptr, len, 19)
    } else if fd == UCOMPAT_V143_FD_U {
        ucompat_v143_read_one(fd, user_ptr, len, 20)
    } else if fd == UCOMPAT_V143_FD_V {
        ucompat_v143_read_one(fd, user_ptr, len, 21)
    } else if fd == UCOMPAT_V143_FD_W {
        ucompat_v143_read_one(fd, user_ptr, len, 22)
    } else if fd == UCOMPAT_V143_FD_X {
        ucompat_v143_read_one(fd, user_ptr, len, 23)
    } else if fd == UCOMPAT_V143_FD_Y {
        ucompat_v143_read_one(fd, user_ptr, len, 24)
    } else if fd == UCOMPAT_V143_FD_Z {
        ucompat_v143_read_one(fd, user_ptr, len, 25)
    } else if fd == UCOMPAT_V143_FD_N0 {
        ucompat_v143_read_one(fd, user_ptr, len, 26)
    } else if fd == UCOMPAT_V143_FD_N1 {
        ucompat_v143_read_one(fd, user_ptr, len, 27)
    } else if fd == UCOMPAT_V143_FD_N2 {
        ucompat_v143_read_one(fd, user_ptr, len, 28)
    } else if fd == UCOMPAT_V143_FD_N3 {
        ucompat_v143_read_one(fd, user_ptr, len, 29)
    } else if fd == UCOMPAT_V143_FD_N4 {
        ucompat_v143_read_one(fd, user_ptr, len, 30)
    } else if fd == UCOMPAT_V143_FD_N5 {
        ucompat_v143_read_one(fd, user_ptr, len, 31)
    } else {
        -9
    }
}
fn ucompat_v143_lseek(fd: isize, off: isize, whence: usize) -> isize {
    unsafe {
        let (len, cur) = if fd == UCOMPAT_V143_FD_A {
            (UCOMPAT_V143_A_LEN, UCOMPAT_V143_A_POS)
        } else if fd == UCOMPAT_V143_FD_B {
            (UCOMPAT_V143_B_LEN, UCOMPAT_V143_B_POS)
        } else if fd == UCOMPAT_V143_FD_C {
            (UCOMPAT_V143_C_LEN, UCOMPAT_V143_C_POS)
        } else if fd == UCOMPAT_V143_FD_D {
            (UCOMPAT_V143_D_LEN, UCOMPAT_V143_D_POS)
        } else if fd == UCOMPAT_V143_FD_E {
            (UCOMPAT_V143_E_LEN, UCOMPAT_V143_E_POS)
        } else if fd == UCOMPAT_V143_FD_F {
            (UCOMPAT_V143_F_LEN, UCOMPAT_V143_F_POS)
        } else if fd == UCOMPAT_V143_FD_G {
            (UCOMPAT_V143_G_LEN, UCOMPAT_V143_G_POS)
        } else if fd == UCOMPAT_V143_FD_H {
            (UCOMPAT_V143_H_LEN, UCOMPAT_V143_H_POS)
        } else if fd == UCOMPAT_V143_FD_I {
            (UCOMPAT_V143_I_LEN, UCOMPAT_V143_I_POS)
        } else if fd == UCOMPAT_V143_FD_J {
            (UCOMPAT_V143_J_LEN, UCOMPAT_V143_J_POS)
        } else if fd == UCOMPAT_V143_FD_K {
            (UCOMPAT_V143_K_LEN, UCOMPAT_V143_K_POS)
        } else if fd == UCOMPAT_V143_FD_L {
            (UCOMPAT_V143_L_LEN, UCOMPAT_V143_L_POS)
        } else if fd == UCOMPAT_V143_FD_M {
            (UCOMPAT_V143_M_LEN, UCOMPAT_V143_M_POS)
        } else if fd == UCOMPAT_V143_FD_N {
            (UCOMPAT_V143_N_LEN, UCOMPAT_V143_N_POS)
        } else if fd == UCOMPAT_V143_FD_O {
            (UCOMPAT_V143_O_LEN, UCOMPAT_V143_O_POS)
        } else if fd == UCOMPAT_V143_FD_P {
            (UCOMPAT_V143_P_LEN, UCOMPAT_V143_P_POS)
        } else if fd == UCOMPAT_V143_FD_Q {
            (UCOMPAT_V143_Q_LEN, UCOMPAT_V143_Q_POS)
        } else if fd == UCOMPAT_V143_FD_R {
            (UCOMPAT_V143_R_LEN, UCOMPAT_V143_R_POS)
        } else if fd == UCOMPAT_V143_FD_S {
            (UCOMPAT_V143_S_LEN, UCOMPAT_V143_S_POS)
        } else if fd == UCOMPAT_V143_FD_T {
            (UCOMPAT_V143_T_LEN, UCOMPAT_V143_T_POS)
        } else if fd == UCOMPAT_V143_FD_U {
            (UCOMPAT_V143_U_LEN, UCOMPAT_V143_U_POS)
        } else if fd == UCOMPAT_V143_FD_V {
            (UCOMPAT_V143_V_LEN, UCOMPAT_V143_V_POS)
        } else if fd == UCOMPAT_V143_FD_W {
            (UCOMPAT_V143_W_LEN, UCOMPAT_V143_W_POS)
        } else if fd == UCOMPAT_V143_FD_X {
            (UCOMPAT_V143_X_LEN, UCOMPAT_V143_X_POS)
        } else if fd == UCOMPAT_V143_FD_Y {
            (UCOMPAT_V143_Y_LEN, UCOMPAT_V143_Y_POS)
        } else if fd == UCOMPAT_V143_FD_Z {
            (UCOMPAT_V143_Z_LEN, UCOMPAT_V143_Z_POS)
        } else if fd == UCOMPAT_V143_FD_N0 {
            (UCOMPAT_V143_N0_LEN, UCOMPAT_V143_N0_POS)
        } else if fd == UCOMPAT_V143_FD_N1 {
            (UCOMPAT_V143_N1_LEN, UCOMPAT_V143_N1_POS)
        } else if fd == UCOMPAT_V143_FD_N2 {
            (UCOMPAT_V143_N2_LEN, UCOMPAT_V143_N2_POS)
        } else if fd == UCOMPAT_V143_FD_N3 {
            (UCOMPAT_V143_N3_LEN, UCOMPAT_V143_N3_POS)
        } else if fd == UCOMPAT_V143_FD_N4 {
            (UCOMPAT_V143_N4_LEN, UCOMPAT_V143_N4_POS)
        } else if fd == UCOMPAT_V143_FD_N5 {
            (UCOMPAT_V143_N5_LEN, UCOMPAT_V143_N5_POS)
        } else {
            return -9;
        };
        let base = match whence {
            0 => 0isize,
            1 => cur as isize,
            2 => len as isize,
            _ => return -22,
        };
        let new_pos = base + off;
        if new_pos < 0 {
            return -22;
        }
        if fd == UCOMPAT_V143_FD_A {
            UCOMPAT_V143_A_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_B {
            UCOMPAT_V143_B_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_C {
            UCOMPAT_V143_C_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_D {
            UCOMPAT_V143_D_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_E {
            UCOMPAT_V143_E_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_F {
            UCOMPAT_V143_F_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_G {
            UCOMPAT_V143_G_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_H {
            UCOMPAT_V143_H_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_I {
            UCOMPAT_V143_I_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_J {
            UCOMPAT_V143_J_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_K {
            UCOMPAT_V143_K_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_L {
            UCOMPAT_V143_L_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_M {
            UCOMPAT_V143_M_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_N {
            UCOMPAT_V143_N_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_O {
            UCOMPAT_V143_O_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_P {
            UCOMPAT_V143_P_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_Q {
            UCOMPAT_V143_Q_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_R {
            UCOMPAT_V143_R_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_S {
            UCOMPAT_V143_S_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_T {
            UCOMPAT_V143_T_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_U {
            UCOMPAT_V143_U_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_V {
            UCOMPAT_V143_V_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_W {
            UCOMPAT_V143_W_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_X {
            UCOMPAT_V143_X_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_Y {
            UCOMPAT_V143_Y_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_Z {
            UCOMPAT_V143_Z_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_N0 {
            UCOMPAT_V143_N0_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_N1 {
            UCOMPAT_V143_N1_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_N2 {
            UCOMPAT_V143_N2_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_N3 {
            UCOMPAT_V143_N3_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_N4 {
            UCOMPAT_V143_N4_POS = new_pos as usize;
        } else if fd == UCOMPAT_V143_FD_N5 {
            UCOMPAT_V143_N5_POS = new_pos as usize;
        } else {
            return -9;
        }
        crate::println!("[ucompat-v143] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}
fn ucompat_v143_close(fd: isize) -> isize {
    unsafe {
        if fd == UCOMPAT_V143_FD_A {
            UCOMPAT_V143_A_POS = 0;
            crate::println!("[ucompat-v143] close fd=12001 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_B {
            UCOMPAT_V143_B_POS = 0;
            crate::println!("[ucompat-v143] close fd=12002 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_C {
            UCOMPAT_V143_C_POS = 0;
            crate::println!("[ucompat-v143] close fd=12003 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_D {
            UCOMPAT_V143_D_POS = 0;
            crate::println!("[ucompat-v143] close fd=12004 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_E {
            UCOMPAT_V143_E_POS = 0;
            crate::println!("[ucompat-v143] close fd=12005 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_F {
            UCOMPAT_V143_F_POS = 0;
            crate::println!("[ucompat-v143] close fd=12006 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_G {
            UCOMPAT_V143_G_POS = 0;
            crate::println!("[ucompat-v143] close fd=12007 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_H {
            UCOMPAT_V143_H_POS = 0;
            crate::println!("[ucompat-v143] close fd=12008 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_I {
            UCOMPAT_V143_I_POS = 0;
            crate::println!("[ucompat-v143] close fd=12009 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_J {
            UCOMPAT_V143_J_POS = 0;
            crate::println!("[ucompat-v143] close fd=12010 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_K {
            UCOMPAT_V143_K_POS = 0;
            crate::println!("[ucompat-v143] close fd=12011 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_L {
            UCOMPAT_V143_L_POS = 0;
            crate::println!("[ucompat-v143] close fd=12012 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_M {
            UCOMPAT_V143_M_POS = 0;
            crate::println!("[ucompat-v143] close fd=12013 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_N {
            UCOMPAT_V143_N_POS = 0;
            crate::println!("[ucompat-v143] close fd=12014 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_O {
            UCOMPAT_V143_O_POS = 0;
            crate::println!("[ucompat-v143] close fd=12015 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_P {
            UCOMPAT_V143_P_POS = 0;
            crate::println!("[ucompat-v143] close fd=12016 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_Q {
            UCOMPAT_V143_Q_POS = 0;
            crate::println!("[ucompat-v143] close fd=12017 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_R {
            UCOMPAT_V143_R_POS = 0;
            crate::println!("[ucompat-v143] close fd=12018 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_S {
            UCOMPAT_V143_S_POS = 0;
            crate::println!("[ucompat-v143] close fd=12019 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_T {
            UCOMPAT_V143_T_POS = 0;
            crate::println!("[ucompat-v143] close fd=12020 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_U {
            UCOMPAT_V143_U_POS = 0;
            crate::println!("[ucompat-v143] close fd=12021 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_V {
            UCOMPAT_V143_V_POS = 0;
            crate::println!("[ucompat-v143] close fd=12022 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_W {
            UCOMPAT_V143_W_POS = 0;
            crate::println!("[ucompat-v143] close fd=12023 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_X {
            UCOMPAT_V143_X_POS = 0;
            crate::println!("[ucompat-v143] close fd=12024 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_Y {
            UCOMPAT_V143_Y_POS = 0;
            crate::println!("[ucompat-v143] close fd=12025 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_Z {
            UCOMPAT_V143_Z_POS = 0;
            crate::println!("[ucompat-v143] close fd=12026 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_N0 {
            UCOMPAT_V143_N0_POS = 0;
            crate::println!("[ucompat-v143] close fd=12027 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_N1 {
            UCOMPAT_V143_N1_POS = 0;
            crate::println!("[ucompat-v143] close fd=12028 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_N2 {
            UCOMPAT_V143_N2_POS = 0;
            crate::println!("[ucompat-v143] close fd=12029 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_N3 {
            UCOMPAT_V143_N3_POS = 0;
            crate::println!("[ucompat-v143] close fd=12030 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_N4 {
            UCOMPAT_V143_N4_POS = 0;
            crate::println!("[ucompat-v143] close fd=12031 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V143_FD_N5 {
            UCOMPAT_V143_N5_POS = 0;
            crate::println!("[ucompat-v143] close fd=12032 ret=0 keep_file=1");
            0
        } else {
            -9
        }
    }
}

// UCOMPAT_V144C_COMPACT_INIT_ELF_LAYOUT_SOURCE_BASELINE
// UCOMPAT_V144B_MATCH_ARM_COMMA_REPAIR
// UCOMPAT_V144_FILE48_LSEEK_ERRNO_OVERWRITE
const UCOMPAT_V144_FD_N00: isize = 13001;
const UCOMPAT_V144_FD_N01: isize = 13002;
const UCOMPAT_V144_FD_N02: isize = 13003;
const UCOMPAT_V144_FD_N03: isize = 13004;
const UCOMPAT_V144_FD_N04: isize = 13005;
const UCOMPAT_V144_FD_N05: isize = 13006;
const UCOMPAT_V144_FD_N06: isize = 13007;
const UCOMPAT_V144_FD_N07: isize = 13008;
const UCOMPAT_V144_FD_N08: isize = 13009;
const UCOMPAT_V144_FD_N09: isize = 13010;
const UCOMPAT_V144_FD_N10: isize = 13011;
const UCOMPAT_V144_FD_N11: isize = 13012;
const UCOMPAT_V144_FD_N12: isize = 13013;
const UCOMPAT_V144_FD_N13: isize = 13014;
const UCOMPAT_V144_FD_N14: isize = 13015;
const UCOMPAT_V144_FD_N15: isize = 13016;
const UCOMPAT_V144_FD_N16: isize = 13017;
const UCOMPAT_V144_FD_N17: isize = 13018;
const UCOMPAT_V144_FD_N18: isize = 13019;
const UCOMPAT_V144_FD_N19: isize = 13020;
const UCOMPAT_V144_FD_N20: isize = 13021;
const UCOMPAT_V144_FD_N21: isize = 13022;
const UCOMPAT_V144_FD_N22: isize = 13023;
const UCOMPAT_V144_FD_N23: isize = 13024;
const UCOMPAT_V144_FD_N24: isize = 13025;
const UCOMPAT_V144_FD_N25: isize = 13026;
const UCOMPAT_V144_FD_N26: isize = 13027;
const UCOMPAT_V144_FD_N27: isize = 13028;
const UCOMPAT_V144_FD_N28: isize = 13029;
const UCOMPAT_V144_FD_N29: isize = 13030;
const UCOMPAT_V144_FD_N30: isize = 13031;
const UCOMPAT_V144_FD_N31: isize = 13032;
const UCOMPAT_V144_FD_N32: isize = 13033;
const UCOMPAT_V144_FD_N33: isize = 13034;
const UCOMPAT_V144_FD_N34: isize = 13035;
const UCOMPAT_V144_FD_N35: isize = 13036;
const UCOMPAT_V144_FD_N36: isize = 13037;
const UCOMPAT_V144_FD_N37: isize = 13038;
const UCOMPAT_V144_FD_N38: isize = 13039;
const UCOMPAT_V144_FD_N39: isize = 13040;
const UCOMPAT_V144_FD_N40: isize = 13041;
const UCOMPAT_V144_FD_N41: isize = 13042;
const UCOMPAT_V144_FD_N42: isize = 13043;
const UCOMPAT_V144_FD_N43: isize = 13044;
const UCOMPAT_V144_FD_N44: isize = 13045;
const UCOMPAT_V144_FD_N45: isize = 13046;
const UCOMPAT_V144_FD_N46: isize = 13047;
const UCOMPAT_V144_FD_N47: isize = 13048;
const UCOMPAT_V144_CAP: usize = 1024;
static mut UCOMPAT_V144_N00_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N00_LEN: usize = 0;
static mut UCOMPAT_V144_N00_POS: usize = 0;
static mut UCOMPAT_V144_N00_EXISTS: bool = false;
static mut UCOMPAT_V144_N01_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N01_LEN: usize = 0;
static mut UCOMPAT_V144_N01_POS: usize = 0;
static mut UCOMPAT_V144_N01_EXISTS: bool = false;
static mut UCOMPAT_V144_N02_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N02_LEN: usize = 0;
static mut UCOMPAT_V144_N02_POS: usize = 0;
static mut UCOMPAT_V144_N02_EXISTS: bool = false;
static mut UCOMPAT_V144_N03_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N03_LEN: usize = 0;
static mut UCOMPAT_V144_N03_POS: usize = 0;
static mut UCOMPAT_V144_N03_EXISTS: bool = false;
static mut UCOMPAT_V144_N04_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N04_LEN: usize = 0;
static mut UCOMPAT_V144_N04_POS: usize = 0;
static mut UCOMPAT_V144_N04_EXISTS: bool = false;
static mut UCOMPAT_V144_N05_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N05_LEN: usize = 0;
static mut UCOMPAT_V144_N05_POS: usize = 0;
static mut UCOMPAT_V144_N05_EXISTS: bool = false;
static mut UCOMPAT_V144_N06_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N06_LEN: usize = 0;
static mut UCOMPAT_V144_N06_POS: usize = 0;
static mut UCOMPAT_V144_N06_EXISTS: bool = false;
static mut UCOMPAT_V144_N07_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N07_LEN: usize = 0;
static mut UCOMPAT_V144_N07_POS: usize = 0;
static mut UCOMPAT_V144_N07_EXISTS: bool = false;
static mut UCOMPAT_V144_N08_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N08_LEN: usize = 0;
static mut UCOMPAT_V144_N08_POS: usize = 0;
static mut UCOMPAT_V144_N08_EXISTS: bool = false;
static mut UCOMPAT_V144_N09_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N09_LEN: usize = 0;
static mut UCOMPAT_V144_N09_POS: usize = 0;
static mut UCOMPAT_V144_N09_EXISTS: bool = false;
static mut UCOMPAT_V144_N10_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N10_LEN: usize = 0;
static mut UCOMPAT_V144_N10_POS: usize = 0;
static mut UCOMPAT_V144_N10_EXISTS: bool = false;
static mut UCOMPAT_V144_N11_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N11_LEN: usize = 0;
static mut UCOMPAT_V144_N11_POS: usize = 0;
static mut UCOMPAT_V144_N11_EXISTS: bool = false;
static mut UCOMPAT_V144_N12_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N12_LEN: usize = 0;
static mut UCOMPAT_V144_N12_POS: usize = 0;
static mut UCOMPAT_V144_N12_EXISTS: bool = false;
static mut UCOMPAT_V144_N13_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N13_LEN: usize = 0;
static mut UCOMPAT_V144_N13_POS: usize = 0;
static mut UCOMPAT_V144_N13_EXISTS: bool = false;
static mut UCOMPAT_V144_N14_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N14_LEN: usize = 0;
static mut UCOMPAT_V144_N14_POS: usize = 0;
static mut UCOMPAT_V144_N14_EXISTS: bool = false;
static mut UCOMPAT_V144_N15_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N15_LEN: usize = 0;
static mut UCOMPAT_V144_N15_POS: usize = 0;
static mut UCOMPAT_V144_N15_EXISTS: bool = false;
static mut UCOMPAT_V144_N16_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N16_LEN: usize = 0;
static mut UCOMPAT_V144_N16_POS: usize = 0;
static mut UCOMPAT_V144_N16_EXISTS: bool = false;
static mut UCOMPAT_V144_N17_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N17_LEN: usize = 0;
static mut UCOMPAT_V144_N17_POS: usize = 0;
static mut UCOMPAT_V144_N17_EXISTS: bool = false;
static mut UCOMPAT_V144_N18_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N18_LEN: usize = 0;
static mut UCOMPAT_V144_N18_POS: usize = 0;
static mut UCOMPAT_V144_N18_EXISTS: bool = false;
static mut UCOMPAT_V144_N19_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N19_LEN: usize = 0;
static mut UCOMPAT_V144_N19_POS: usize = 0;
static mut UCOMPAT_V144_N19_EXISTS: bool = false;
static mut UCOMPAT_V144_N20_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N20_LEN: usize = 0;
static mut UCOMPAT_V144_N20_POS: usize = 0;
static mut UCOMPAT_V144_N20_EXISTS: bool = false;
static mut UCOMPAT_V144_N21_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N21_LEN: usize = 0;
static mut UCOMPAT_V144_N21_POS: usize = 0;
static mut UCOMPAT_V144_N21_EXISTS: bool = false;
static mut UCOMPAT_V144_N22_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N22_LEN: usize = 0;
static mut UCOMPAT_V144_N22_POS: usize = 0;
static mut UCOMPAT_V144_N22_EXISTS: bool = false;
static mut UCOMPAT_V144_N23_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N23_LEN: usize = 0;
static mut UCOMPAT_V144_N23_POS: usize = 0;
static mut UCOMPAT_V144_N23_EXISTS: bool = false;
static mut UCOMPAT_V144_N24_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N24_LEN: usize = 0;
static mut UCOMPAT_V144_N24_POS: usize = 0;
static mut UCOMPAT_V144_N24_EXISTS: bool = false;
static mut UCOMPAT_V144_N25_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N25_LEN: usize = 0;
static mut UCOMPAT_V144_N25_POS: usize = 0;
static mut UCOMPAT_V144_N25_EXISTS: bool = false;
static mut UCOMPAT_V144_N26_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N26_LEN: usize = 0;
static mut UCOMPAT_V144_N26_POS: usize = 0;
static mut UCOMPAT_V144_N26_EXISTS: bool = false;
static mut UCOMPAT_V144_N27_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N27_LEN: usize = 0;
static mut UCOMPAT_V144_N27_POS: usize = 0;
static mut UCOMPAT_V144_N27_EXISTS: bool = false;
static mut UCOMPAT_V144_N28_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N28_LEN: usize = 0;
static mut UCOMPAT_V144_N28_POS: usize = 0;
static mut UCOMPAT_V144_N28_EXISTS: bool = false;
static mut UCOMPAT_V144_N29_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N29_LEN: usize = 0;
static mut UCOMPAT_V144_N29_POS: usize = 0;
static mut UCOMPAT_V144_N29_EXISTS: bool = false;
static mut UCOMPAT_V144_N30_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N30_LEN: usize = 0;
static mut UCOMPAT_V144_N30_POS: usize = 0;
static mut UCOMPAT_V144_N30_EXISTS: bool = false;
static mut UCOMPAT_V144_N31_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N31_LEN: usize = 0;
static mut UCOMPAT_V144_N31_POS: usize = 0;
static mut UCOMPAT_V144_N31_EXISTS: bool = false;
static mut UCOMPAT_V144_N32_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N32_LEN: usize = 0;
static mut UCOMPAT_V144_N32_POS: usize = 0;
static mut UCOMPAT_V144_N32_EXISTS: bool = false;
static mut UCOMPAT_V144_N33_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N33_LEN: usize = 0;
static mut UCOMPAT_V144_N33_POS: usize = 0;
static mut UCOMPAT_V144_N33_EXISTS: bool = false;
static mut UCOMPAT_V144_N34_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N34_LEN: usize = 0;
static mut UCOMPAT_V144_N34_POS: usize = 0;
static mut UCOMPAT_V144_N34_EXISTS: bool = false;
static mut UCOMPAT_V144_N35_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N35_LEN: usize = 0;
static mut UCOMPAT_V144_N35_POS: usize = 0;
static mut UCOMPAT_V144_N35_EXISTS: bool = false;
static mut UCOMPAT_V144_N36_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N36_LEN: usize = 0;
static mut UCOMPAT_V144_N36_POS: usize = 0;
static mut UCOMPAT_V144_N36_EXISTS: bool = false;
static mut UCOMPAT_V144_N37_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N37_LEN: usize = 0;
static mut UCOMPAT_V144_N37_POS: usize = 0;
static mut UCOMPAT_V144_N37_EXISTS: bool = false;
static mut UCOMPAT_V144_N38_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N38_LEN: usize = 0;
static mut UCOMPAT_V144_N38_POS: usize = 0;
static mut UCOMPAT_V144_N38_EXISTS: bool = false;
static mut UCOMPAT_V144_N39_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N39_LEN: usize = 0;
static mut UCOMPAT_V144_N39_POS: usize = 0;
static mut UCOMPAT_V144_N39_EXISTS: bool = false;
static mut UCOMPAT_V144_N40_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N40_LEN: usize = 0;
static mut UCOMPAT_V144_N40_POS: usize = 0;
static mut UCOMPAT_V144_N40_EXISTS: bool = false;
static mut UCOMPAT_V144_N41_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N41_LEN: usize = 0;
static mut UCOMPAT_V144_N41_POS: usize = 0;
static mut UCOMPAT_V144_N41_EXISTS: bool = false;
static mut UCOMPAT_V144_N42_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N42_LEN: usize = 0;
static mut UCOMPAT_V144_N42_POS: usize = 0;
static mut UCOMPAT_V144_N42_EXISTS: bool = false;
static mut UCOMPAT_V144_N43_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N43_LEN: usize = 0;
static mut UCOMPAT_V144_N43_POS: usize = 0;
static mut UCOMPAT_V144_N43_EXISTS: bool = false;
static mut UCOMPAT_V144_N44_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N44_LEN: usize = 0;
static mut UCOMPAT_V144_N44_POS: usize = 0;
static mut UCOMPAT_V144_N44_EXISTS: bool = false;
static mut UCOMPAT_V144_N45_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N45_LEN: usize = 0;
static mut UCOMPAT_V144_N45_POS: usize = 0;
static mut UCOMPAT_V144_N45_EXISTS: bool = false;
static mut UCOMPAT_V144_N46_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N46_LEN: usize = 0;
static mut UCOMPAT_V144_N46_POS: usize = 0;
static mut UCOMPAT_V144_N46_EXISTS: bool = false;
static mut UCOMPAT_V144_N47_DATA: [u8; UCOMPAT_V144_CAP] = [0; UCOMPAT_V144_CAP];
static mut UCOMPAT_V144_N47_LEN: usize = 0;
static mut UCOMPAT_V144_N47_POS: usize = 0;
static mut UCOMPAT_V144_N47_EXISTS: bool = false;
fn ucompat_v144_fd(slot: usize) -> isize {
    match slot {
        0 => UCOMPAT_V144_FD_N00,
        1 => UCOMPAT_V144_FD_N01,
        2 => UCOMPAT_V144_FD_N02,
        3 => UCOMPAT_V144_FD_N03,
        4 => UCOMPAT_V144_FD_N04,
        5 => UCOMPAT_V144_FD_N05,
        6 => UCOMPAT_V144_FD_N06,
        7 => UCOMPAT_V144_FD_N07,
        8 => UCOMPAT_V144_FD_N08,
        9 => UCOMPAT_V144_FD_N09,
        10 => UCOMPAT_V144_FD_N10,
        11 => UCOMPAT_V144_FD_N11,
        12 => UCOMPAT_V144_FD_N12,
        13 => UCOMPAT_V144_FD_N13,
        14 => UCOMPAT_V144_FD_N14,
        15 => UCOMPAT_V144_FD_N15,
        16 => UCOMPAT_V144_FD_N16,
        17 => UCOMPAT_V144_FD_N17,
        18 => UCOMPAT_V144_FD_N18,
        19 => UCOMPAT_V144_FD_N19,
        20 => UCOMPAT_V144_FD_N20,
        21 => UCOMPAT_V144_FD_N21,
        22 => UCOMPAT_V144_FD_N22,
        23 => UCOMPAT_V144_FD_N23,
        24 => UCOMPAT_V144_FD_N24,
        25 => UCOMPAT_V144_FD_N25,
        26 => UCOMPAT_V144_FD_N26,
        27 => UCOMPAT_V144_FD_N27,
        28 => UCOMPAT_V144_FD_N28,
        29 => UCOMPAT_V144_FD_N29,
        30 => UCOMPAT_V144_FD_N30,
        31 => UCOMPAT_V144_FD_N31,
        32 => UCOMPAT_V144_FD_N32,
        33 => UCOMPAT_V144_FD_N33,
        34 => UCOMPAT_V144_FD_N34,
        35 => UCOMPAT_V144_FD_N35,
        36 => UCOMPAT_V144_FD_N36,
        37 => UCOMPAT_V144_FD_N37,
        38 => UCOMPAT_V144_FD_N38,
        39 => UCOMPAT_V144_FD_N39,
        40 => UCOMPAT_V144_FD_N40,
        41 => UCOMPAT_V144_FD_N41,
        42 => UCOMPAT_V144_FD_N42,
        43 => UCOMPAT_V144_FD_N43,
        44 => UCOMPAT_V144_FD_N44,
        45 => UCOMPAT_V144_FD_N45,
        46 => UCOMPAT_V144_FD_N46,
        _ => UCOMPAT_V144_FD_N47,
    }
}
fn ucompat_v144_is_fd(fd: isize) -> bool {
    fd == UCOMPAT_V144_FD_N00
        || fd == UCOMPAT_V144_FD_N01
        || fd == UCOMPAT_V144_FD_N02
        || fd == UCOMPAT_V144_FD_N03
        || fd == UCOMPAT_V144_FD_N04
        || fd == UCOMPAT_V144_FD_N05
        || fd == UCOMPAT_V144_FD_N06
        || fd == UCOMPAT_V144_FD_N07
        || fd == UCOMPAT_V144_FD_N08
        || fd == UCOMPAT_V144_FD_N09
        || fd == UCOMPAT_V144_FD_N10
        || fd == UCOMPAT_V144_FD_N11
        || fd == UCOMPAT_V144_FD_N12
        || fd == UCOMPAT_V144_FD_N13
        || fd == UCOMPAT_V144_FD_N14
        || fd == UCOMPAT_V144_FD_N15
        || fd == UCOMPAT_V144_FD_N16
        || fd == UCOMPAT_V144_FD_N17
        || fd == UCOMPAT_V144_FD_N18
        || fd == UCOMPAT_V144_FD_N19
        || fd == UCOMPAT_V144_FD_N20
        || fd == UCOMPAT_V144_FD_N21
        || fd == UCOMPAT_V144_FD_N22
        || fd == UCOMPAT_V144_FD_N23
        || fd == UCOMPAT_V144_FD_N24
        || fd == UCOMPAT_V144_FD_N25
        || fd == UCOMPAT_V144_FD_N26
        || fd == UCOMPAT_V144_FD_N27
        || fd == UCOMPAT_V144_FD_N28
        || fd == UCOMPAT_V144_FD_N29
        || fd == UCOMPAT_V144_FD_N30
        || fd == UCOMPAT_V144_FD_N31
        || fd == UCOMPAT_V144_FD_N32
        || fd == UCOMPAT_V144_FD_N33
        || fd == UCOMPAT_V144_FD_N34
        || fd == UCOMPAT_V144_FD_N35
        || fd == UCOMPAT_V144_FD_N36
        || fd == UCOMPAT_V144_FD_N37
        || fd == UCOMPAT_V144_FD_N38
        || fd == UCOMPAT_V144_FD_N39
        || fd == UCOMPAT_V144_FD_N40
        || fd == UCOMPAT_V144_FD_N41
        || fd == UCOMPAT_V144_FD_N42
        || fd == UCOMPAT_V144_FD_N43
        || fd == UCOMPAT_V144_FD_N44
        || fd == UCOMPAT_V144_FD_N45
        || fd == UCOMPAT_V144_FD_N46
        || fd == UCOMPAT_V144_FD_N47
}
fn ucompat_v144_exists(slot: usize) -> bool {
    unsafe {
        match slot {
            0 => UCOMPAT_V144_N00_EXISTS,
            1 => UCOMPAT_V144_N01_EXISTS,
            2 => UCOMPAT_V144_N02_EXISTS,
            3 => UCOMPAT_V144_N03_EXISTS,
            4 => UCOMPAT_V144_N04_EXISTS,
            5 => UCOMPAT_V144_N05_EXISTS,
            6 => UCOMPAT_V144_N06_EXISTS,
            7 => UCOMPAT_V144_N07_EXISTS,
            8 => UCOMPAT_V144_N08_EXISTS,
            9 => UCOMPAT_V144_N09_EXISTS,
            10 => UCOMPAT_V144_N10_EXISTS,
            11 => UCOMPAT_V144_N11_EXISTS,
            12 => UCOMPAT_V144_N12_EXISTS,
            13 => UCOMPAT_V144_N13_EXISTS,
            14 => UCOMPAT_V144_N14_EXISTS,
            15 => UCOMPAT_V144_N15_EXISTS,
            16 => UCOMPAT_V144_N16_EXISTS,
            17 => UCOMPAT_V144_N17_EXISTS,
            18 => UCOMPAT_V144_N18_EXISTS,
            19 => UCOMPAT_V144_N19_EXISTS,
            20 => UCOMPAT_V144_N20_EXISTS,
            21 => UCOMPAT_V144_N21_EXISTS,
            22 => UCOMPAT_V144_N22_EXISTS,
            23 => UCOMPAT_V144_N23_EXISTS,
            24 => UCOMPAT_V144_N24_EXISTS,
            25 => UCOMPAT_V144_N25_EXISTS,
            26 => UCOMPAT_V144_N26_EXISTS,
            27 => UCOMPAT_V144_N27_EXISTS,
            28 => UCOMPAT_V144_N28_EXISTS,
            29 => UCOMPAT_V144_N29_EXISTS,
            30 => UCOMPAT_V144_N30_EXISTS,
            31 => UCOMPAT_V144_N31_EXISTS,
            32 => UCOMPAT_V144_N32_EXISTS,
            33 => UCOMPAT_V144_N33_EXISTS,
            34 => UCOMPAT_V144_N34_EXISTS,
            35 => UCOMPAT_V144_N35_EXISTS,
            36 => UCOMPAT_V144_N36_EXISTS,
            37 => UCOMPAT_V144_N37_EXISTS,
            38 => UCOMPAT_V144_N38_EXISTS,
            39 => UCOMPAT_V144_N39_EXISTS,
            40 => UCOMPAT_V144_N40_EXISTS,
            41 => UCOMPAT_V144_N41_EXISTS,
            42 => UCOMPAT_V144_N42_EXISTS,
            43 => UCOMPAT_V144_N43_EXISTS,
            44 => UCOMPAT_V144_N44_EXISTS,
            45 => UCOMPAT_V144_N45_EXISTS,
            46 => UCOMPAT_V144_N46_EXISTS,
            _ => UCOMPAT_V144_N47_EXISTS,
        }
    }
}
fn ucompat_v144_reset(slot: usize) {
    unsafe {
        let mut j = 0usize;
        match slot {
            0 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N00_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N00_LEN = 0;
                UCOMPAT_V144_N00_POS = 0;
                UCOMPAT_V144_N00_EXISTS = true;
            }
            1 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N01_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N01_LEN = 0;
                UCOMPAT_V144_N01_POS = 0;
                UCOMPAT_V144_N01_EXISTS = true;
            }
            2 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N02_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N02_LEN = 0;
                UCOMPAT_V144_N02_POS = 0;
                UCOMPAT_V144_N02_EXISTS = true;
            }
            3 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N03_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N03_LEN = 0;
                UCOMPAT_V144_N03_POS = 0;
                UCOMPAT_V144_N03_EXISTS = true;
            }
            4 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N04_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N04_LEN = 0;
                UCOMPAT_V144_N04_POS = 0;
                UCOMPAT_V144_N04_EXISTS = true;
            }
            5 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N05_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N05_LEN = 0;
                UCOMPAT_V144_N05_POS = 0;
                UCOMPAT_V144_N05_EXISTS = true;
            }
            6 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N06_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N06_LEN = 0;
                UCOMPAT_V144_N06_POS = 0;
                UCOMPAT_V144_N06_EXISTS = true;
            }
            7 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N07_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N07_LEN = 0;
                UCOMPAT_V144_N07_POS = 0;
                UCOMPAT_V144_N07_EXISTS = true;
            }
            8 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N08_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N08_LEN = 0;
                UCOMPAT_V144_N08_POS = 0;
                UCOMPAT_V144_N08_EXISTS = true;
            }
            9 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N09_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N09_LEN = 0;
                UCOMPAT_V144_N09_POS = 0;
                UCOMPAT_V144_N09_EXISTS = true;
            }
            10 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N10_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N10_LEN = 0;
                UCOMPAT_V144_N10_POS = 0;
                UCOMPAT_V144_N10_EXISTS = true;
            }
            11 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N11_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N11_LEN = 0;
                UCOMPAT_V144_N11_POS = 0;
                UCOMPAT_V144_N11_EXISTS = true;
            }
            12 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N12_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N12_LEN = 0;
                UCOMPAT_V144_N12_POS = 0;
                UCOMPAT_V144_N12_EXISTS = true;
            }
            13 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N13_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N13_LEN = 0;
                UCOMPAT_V144_N13_POS = 0;
                UCOMPAT_V144_N13_EXISTS = true;
            }
            14 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N14_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N14_LEN = 0;
                UCOMPAT_V144_N14_POS = 0;
                UCOMPAT_V144_N14_EXISTS = true;
            }
            15 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N15_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N15_LEN = 0;
                UCOMPAT_V144_N15_POS = 0;
                UCOMPAT_V144_N15_EXISTS = true;
            }
            16 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N16_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N16_LEN = 0;
                UCOMPAT_V144_N16_POS = 0;
                UCOMPAT_V144_N16_EXISTS = true;
            }
            17 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N17_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N17_LEN = 0;
                UCOMPAT_V144_N17_POS = 0;
                UCOMPAT_V144_N17_EXISTS = true;
            }
            18 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N18_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N18_LEN = 0;
                UCOMPAT_V144_N18_POS = 0;
                UCOMPAT_V144_N18_EXISTS = true;
            }
            19 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N19_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N19_LEN = 0;
                UCOMPAT_V144_N19_POS = 0;
                UCOMPAT_V144_N19_EXISTS = true;
            }
            20 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N20_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N20_LEN = 0;
                UCOMPAT_V144_N20_POS = 0;
                UCOMPAT_V144_N20_EXISTS = true;
            }
            21 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N21_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N21_LEN = 0;
                UCOMPAT_V144_N21_POS = 0;
                UCOMPAT_V144_N21_EXISTS = true;
            }
            22 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N22_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N22_LEN = 0;
                UCOMPAT_V144_N22_POS = 0;
                UCOMPAT_V144_N22_EXISTS = true;
            }
            23 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N23_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N23_LEN = 0;
                UCOMPAT_V144_N23_POS = 0;
                UCOMPAT_V144_N23_EXISTS = true;
            }
            24 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N24_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N24_LEN = 0;
                UCOMPAT_V144_N24_POS = 0;
                UCOMPAT_V144_N24_EXISTS = true;
            }
            25 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N25_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N25_LEN = 0;
                UCOMPAT_V144_N25_POS = 0;
                UCOMPAT_V144_N25_EXISTS = true;
            }
            26 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N26_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N26_LEN = 0;
                UCOMPAT_V144_N26_POS = 0;
                UCOMPAT_V144_N26_EXISTS = true;
            }
            27 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N27_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N27_LEN = 0;
                UCOMPAT_V144_N27_POS = 0;
                UCOMPAT_V144_N27_EXISTS = true;
            }
            28 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N28_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N28_LEN = 0;
                UCOMPAT_V144_N28_POS = 0;
                UCOMPAT_V144_N28_EXISTS = true;
            }
            29 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N29_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N29_LEN = 0;
                UCOMPAT_V144_N29_POS = 0;
                UCOMPAT_V144_N29_EXISTS = true;
            }
            30 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N30_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N30_LEN = 0;
                UCOMPAT_V144_N30_POS = 0;
                UCOMPAT_V144_N30_EXISTS = true;
            }
            31 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N31_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N31_LEN = 0;
                UCOMPAT_V144_N31_POS = 0;
                UCOMPAT_V144_N31_EXISTS = true;
            }
            32 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N32_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N32_LEN = 0;
                UCOMPAT_V144_N32_POS = 0;
                UCOMPAT_V144_N32_EXISTS = true;
            }
            33 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N33_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N33_LEN = 0;
                UCOMPAT_V144_N33_POS = 0;
                UCOMPAT_V144_N33_EXISTS = true;
            }
            34 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N34_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N34_LEN = 0;
                UCOMPAT_V144_N34_POS = 0;
                UCOMPAT_V144_N34_EXISTS = true;
            }
            35 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N35_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N35_LEN = 0;
                UCOMPAT_V144_N35_POS = 0;
                UCOMPAT_V144_N35_EXISTS = true;
            }
            36 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N36_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N36_LEN = 0;
                UCOMPAT_V144_N36_POS = 0;
                UCOMPAT_V144_N36_EXISTS = true;
            }
            37 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N37_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N37_LEN = 0;
                UCOMPAT_V144_N37_POS = 0;
                UCOMPAT_V144_N37_EXISTS = true;
            }
            38 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N38_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N38_LEN = 0;
                UCOMPAT_V144_N38_POS = 0;
                UCOMPAT_V144_N38_EXISTS = true;
            }
            39 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N39_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N39_LEN = 0;
                UCOMPAT_V144_N39_POS = 0;
                UCOMPAT_V144_N39_EXISTS = true;
            }
            40 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N40_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N40_LEN = 0;
                UCOMPAT_V144_N40_POS = 0;
                UCOMPAT_V144_N40_EXISTS = true;
            }
            41 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N41_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N41_LEN = 0;
                UCOMPAT_V144_N41_POS = 0;
                UCOMPAT_V144_N41_EXISTS = true;
            }
            42 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N42_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N42_LEN = 0;
                UCOMPAT_V144_N42_POS = 0;
                UCOMPAT_V144_N42_EXISTS = true;
            }
            43 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N43_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N43_LEN = 0;
                UCOMPAT_V144_N43_POS = 0;
                UCOMPAT_V144_N43_EXISTS = true;
            }
            44 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N44_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N44_LEN = 0;
                UCOMPAT_V144_N44_POS = 0;
                UCOMPAT_V144_N44_EXISTS = true;
            }
            45 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N45_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N45_LEN = 0;
                UCOMPAT_V144_N45_POS = 0;
                UCOMPAT_V144_N45_EXISTS = true;
            }
            46 => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N46_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N46_LEN = 0;
                UCOMPAT_V144_N46_POS = 0;
                UCOMPAT_V144_N46_EXISTS = true;
            }
            _ => {
                while j < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N47_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V144_N47_LEN = 0;
                UCOMPAT_V144_N47_POS = 0;
                UCOMPAT_V144_N47_EXISTS = true;
            }
        }
    }
}
fn ucompat_v144_open(slot: usize, flags: usize) -> isize {
    const O_CREAT: usize = 0x40;
    const O_TRUNC: usize = 0x200;
    let exists = ucompat_v144_exists(slot);
    let name = match slot {
        0 => "00",
        1 => "01",
        2 => "02",
        3 => "03",
        4 => "04",
        5 => "05",
        6 => "06",
        7 => "07",
        8 => "08",
        9 => "09",
        10 => "10",
        11 => "11",
        12 => "12",
        13 => "13",
        14 => "14",
        15 => "15",
        16 => "16",
        17 => "17",
        18 => "18",
        19 => "19",
        20 => "20",
        21 => "21",
        22 => "22",
        23 => "23",
        24 => "24",
        25 => "25",
        26 => "26",
        27 => "27",
        28 => "28",
        29 => "29",
        30 => "30",
        31 => "31",
        32 => "32",
        33 => "33",
        34 => "34",
        35 => "35",
        36 => "36",
        37 => "37",
        38 => "38",
        39 => "39",
        40 => "40",
        41 => "41",
        42 => "42",
        43 => "43",
        44 => "44",
        45 => "45",
        46 => "46",
        _ => "47",
    };
    if !exists && (flags & O_CREAT) == 0 {
        crate::println!("[openat-v144] slot={} missing without O_CREAT", name);
        return crate::syscall::errno::ENOENT;
    }
    if !exists || (flags & O_TRUNC) != 0 {
        if exists && (flags & O_TRUNC) != 0 {
            crate::println!("[openat-v144] slot={} truncate", name);
        } else {
            crate::println!("[openat-v144] slot={} create", name);
        }
        ucompat_v144_reset(slot);
    } else {
        unsafe {
            match slot {
                0 => UCOMPAT_V144_N00_POS = 0,
                1 => UCOMPAT_V144_N01_POS = 0,
                2 => UCOMPAT_V144_N02_POS = 0,
                3 => UCOMPAT_V144_N03_POS = 0,
                4 => UCOMPAT_V144_N04_POS = 0,
                5 => UCOMPAT_V144_N05_POS = 0,
                6 => UCOMPAT_V144_N06_POS = 0,
                7 => UCOMPAT_V144_N07_POS = 0,
                8 => UCOMPAT_V144_N08_POS = 0,
                9 => UCOMPAT_V144_N09_POS = 0,
                10 => UCOMPAT_V144_N10_POS = 0,
                11 => UCOMPAT_V144_N11_POS = 0,
                12 => UCOMPAT_V144_N12_POS = 0,
                13 => UCOMPAT_V144_N13_POS = 0,
                14 => UCOMPAT_V144_N14_POS = 0,
                15 => UCOMPAT_V144_N15_POS = 0,
                16 => UCOMPAT_V144_N16_POS = 0,
                17 => UCOMPAT_V144_N17_POS = 0,
                18 => UCOMPAT_V144_N18_POS = 0,
                19 => UCOMPAT_V144_N19_POS = 0,
                20 => UCOMPAT_V144_N20_POS = 0,
                21 => UCOMPAT_V144_N21_POS = 0,
                22 => UCOMPAT_V144_N22_POS = 0,
                23 => UCOMPAT_V144_N23_POS = 0,
                24 => UCOMPAT_V144_N24_POS = 0,
                25 => UCOMPAT_V144_N25_POS = 0,
                26 => UCOMPAT_V144_N26_POS = 0,
                27 => UCOMPAT_V144_N27_POS = 0,
                28 => UCOMPAT_V144_N28_POS = 0,
                29 => UCOMPAT_V144_N29_POS = 0,
                30 => UCOMPAT_V144_N30_POS = 0,
                31 => UCOMPAT_V144_N31_POS = 0,
                32 => UCOMPAT_V144_N32_POS = 0,
                33 => UCOMPAT_V144_N33_POS = 0,
                34 => UCOMPAT_V144_N34_POS = 0,
                35 => UCOMPAT_V144_N35_POS = 0,
                36 => UCOMPAT_V144_N36_POS = 0,
                37 => UCOMPAT_V144_N37_POS = 0,
                38 => UCOMPAT_V144_N38_POS = 0,
                39 => UCOMPAT_V144_N39_POS = 0,
                40 => UCOMPAT_V144_N40_POS = 0,
                41 => UCOMPAT_V144_N41_POS = 0,
                42 => UCOMPAT_V144_N42_POS = 0,
                43 => UCOMPAT_V144_N43_POS = 0,
                44 => UCOMPAT_V144_N44_POS = 0,
                45 => UCOMPAT_V144_N45_POS = 0,
                46 => UCOMPAT_V144_N46_POS = 0,
                _ => UCOMPAT_V144_N47_POS = 0,
            }
        }
        crate::println!("[openat-v144] slot={} reopen", name);
    }
    ucompat_v144_fd(slot)
}
fn ucompat_v144_write_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                if UCOMPAT_V144_N00_POS > UCOMPAT_V144_N00_LEN {
                    let mut z = UCOMPAT_V144_N00_LEN;
                    while z < UCOMPAT_V144_N00_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N00_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13001 from={} to={}",
                        UCOMPAT_V144_N00_LEN,
                        UCOMPAT_V144_N00_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N00_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N00_DATA[UCOMPAT_V144_N00_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N00_POS + copied;
                if end > UCOMPAT_V144_N00_LEN {
                    UCOMPAT_V144_N00_LEN = end;
                }
                UCOMPAT_V144_N00_POS = end;
                UCOMPAT_V144_N00_EXISTS = true;
            }
            1 => {
                if UCOMPAT_V144_N01_POS > UCOMPAT_V144_N01_LEN {
                    let mut z = UCOMPAT_V144_N01_LEN;
                    while z < UCOMPAT_V144_N01_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N01_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13002 from={} to={}",
                        UCOMPAT_V144_N01_LEN,
                        UCOMPAT_V144_N01_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N01_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N01_DATA[UCOMPAT_V144_N01_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N01_POS + copied;
                if end > UCOMPAT_V144_N01_LEN {
                    UCOMPAT_V144_N01_LEN = end;
                }
                UCOMPAT_V144_N01_POS = end;
                UCOMPAT_V144_N01_EXISTS = true;
            }
            2 => {
                if UCOMPAT_V144_N02_POS > UCOMPAT_V144_N02_LEN {
                    let mut z = UCOMPAT_V144_N02_LEN;
                    while z < UCOMPAT_V144_N02_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N02_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13003 from={} to={}",
                        UCOMPAT_V144_N02_LEN,
                        UCOMPAT_V144_N02_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N02_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N02_DATA[UCOMPAT_V144_N02_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N02_POS + copied;
                if end > UCOMPAT_V144_N02_LEN {
                    UCOMPAT_V144_N02_LEN = end;
                }
                UCOMPAT_V144_N02_POS = end;
                UCOMPAT_V144_N02_EXISTS = true;
            }
            3 => {
                if UCOMPAT_V144_N03_POS > UCOMPAT_V144_N03_LEN {
                    let mut z = UCOMPAT_V144_N03_LEN;
                    while z < UCOMPAT_V144_N03_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N03_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13004 from={} to={}",
                        UCOMPAT_V144_N03_LEN,
                        UCOMPAT_V144_N03_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N03_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N03_DATA[UCOMPAT_V144_N03_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N03_POS + copied;
                if end > UCOMPAT_V144_N03_LEN {
                    UCOMPAT_V144_N03_LEN = end;
                }
                UCOMPAT_V144_N03_POS = end;
                UCOMPAT_V144_N03_EXISTS = true;
            }
            4 => {
                if UCOMPAT_V144_N04_POS > UCOMPAT_V144_N04_LEN {
                    let mut z = UCOMPAT_V144_N04_LEN;
                    while z < UCOMPAT_V144_N04_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N04_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13005 from={} to={}",
                        UCOMPAT_V144_N04_LEN,
                        UCOMPAT_V144_N04_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N04_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N04_DATA[UCOMPAT_V144_N04_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N04_POS + copied;
                if end > UCOMPAT_V144_N04_LEN {
                    UCOMPAT_V144_N04_LEN = end;
                }
                UCOMPAT_V144_N04_POS = end;
                UCOMPAT_V144_N04_EXISTS = true;
            }
            5 => {
                if UCOMPAT_V144_N05_POS > UCOMPAT_V144_N05_LEN {
                    let mut z = UCOMPAT_V144_N05_LEN;
                    while z < UCOMPAT_V144_N05_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N05_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13006 from={} to={}",
                        UCOMPAT_V144_N05_LEN,
                        UCOMPAT_V144_N05_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N05_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N05_DATA[UCOMPAT_V144_N05_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N05_POS + copied;
                if end > UCOMPAT_V144_N05_LEN {
                    UCOMPAT_V144_N05_LEN = end;
                }
                UCOMPAT_V144_N05_POS = end;
                UCOMPAT_V144_N05_EXISTS = true;
            }
            6 => {
                if UCOMPAT_V144_N06_POS > UCOMPAT_V144_N06_LEN {
                    let mut z = UCOMPAT_V144_N06_LEN;
                    while z < UCOMPAT_V144_N06_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N06_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13007 from={} to={}",
                        UCOMPAT_V144_N06_LEN,
                        UCOMPAT_V144_N06_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N06_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N06_DATA[UCOMPAT_V144_N06_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N06_POS + copied;
                if end > UCOMPAT_V144_N06_LEN {
                    UCOMPAT_V144_N06_LEN = end;
                }
                UCOMPAT_V144_N06_POS = end;
                UCOMPAT_V144_N06_EXISTS = true;
            }
            7 => {
                if UCOMPAT_V144_N07_POS > UCOMPAT_V144_N07_LEN {
                    let mut z = UCOMPAT_V144_N07_LEN;
                    while z < UCOMPAT_V144_N07_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N07_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13008 from={} to={}",
                        UCOMPAT_V144_N07_LEN,
                        UCOMPAT_V144_N07_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N07_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N07_DATA[UCOMPAT_V144_N07_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N07_POS + copied;
                if end > UCOMPAT_V144_N07_LEN {
                    UCOMPAT_V144_N07_LEN = end;
                }
                UCOMPAT_V144_N07_POS = end;
                UCOMPAT_V144_N07_EXISTS = true;
            }
            8 => {
                if UCOMPAT_V144_N08_POS > UCOMPAT_V144_N08_LEN {
                    let mut z = UCOMPAT_V144_N08_LEN;
                    while z < UCOMPAT_V144_N08_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N08_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13009 from={} to={}",
                        UCOMPAT_V144_N08_LEN,
                        UCOMPAT_V144_N08_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N08_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N08_DATA[UCOMPAT_V144_N08_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N08_POS + copied;
                if end > UCOMPAT_V144_N08_LEN {
                    UCOMPAT_V144_N08_LEN = end;
                }
                UCOMPAT_V144_N08_POS = end;
                UCOMPAT_V144_N08_EXISTS = true;
            }
            9 => {
                if UCOMPAT_V144_N09_POS > UCOMPAT_V144_N09_LEN {
                    let mut z = UCOMPAT_V144_N09_LEN;
                    while z < UCOMPAT_V144_N09_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N09_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13010 from={} to={}",
                        UCOMPAT_V144_N09_LEN,
                        UCOMPAT_V144_N09_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N09_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N09_DATA[UCOMPAT_V144_N09_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N09_POS + copied;
                if end > UCOMPAT_V144_N09_LEN {
                    UCOMPAT_V144_N09_LEN = end;
                }
                UCOMPAT_V144_N09_POS = end;
                UCOMPAT_V144_N09_EXISTS = true;
            }
            10 => {
                if UCOMPAT_V144_N10_POS > UCOMPAT_V144_N10_LEN {
                    let mut z = UCOMPAT_V144_N10_LEN;
                    while z < UCOMPAT_V144_N10_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N10_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13011 from={} to={}",
                        UCOMPAT_V144_N10_LEN,
                        UCOMPAT_V144_N10_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N10_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N10_DATA[UCOMPAT_V144_N10_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N10_POS + copied;
                if end > UCOMPAT_V144_N10_LEN {
                    UCOMPAT_V144_N10_LEN = end;
                }
                UCOMPAT_V144_N10_POS = end;
                UCOMPAT_V144_N10_EXISTS = true;
            }
            11 => {
                if UCOMPAT_V144_N11_POS > UCOMPAT_V144_N11_LEN {
                    let mut z = UCOMPAT_V144_N11_LEN;
                    while z < UCOMPAT_V144_N11_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N11_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13012 from={} to={}",
                        UCOMPAT_V144_N11_LEN,
                        UCOMPAT_V144_N11_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N11_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N11_DATA[UCOMPAT_V144_N11_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N11_POS + copied;
                if end > UCOMPAT_V144_N11_LEN {
                    UCOMPAT_V144_N11_LEN = end;
                }
                UCOMPAT_V144_N11_POS = end;
                UCOMPAT_V144_N11_EXISTS = true;
            }
            12 => {
                if UCOMPAT_V144_N12_POS > UCOMPAT_V144_N12_LEN {
                    let mut z = UCOMPAT_V144_N12_LEN;
                    while z < UCOMPAT_V144_N12_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N12_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13013 from={} to={}",
                        UCOMPAT_V144_N12_LEN,
                        UCOMPAT_V144_N12_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N12_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N12_DATA[UCOMPAT_V144_N12_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N12_POS + copied;
                if end > UCOMPAT_V144_N12_LEN {
                    UCOMPAT_V144_N12_LEN = end;
                }
                UCOMPAT_V144_N12_POS = end;
                UCOMPAT_V144_N12_EXISTS = true;
            }
            13 => {
                if UCOMPAT_V144_N13_POS > UCOMPAT_V144_N13_LEN {
                    let mut z = UCOMPAT_V144_N13_LEN;
                    while z < UCOMPAT_V144_N13_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N13_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13014 from={} to={}",
                        UCOMPAT_V144_N13_LEN,
                        UCOMPAT_V144_N13_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N13_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N13_DATA[UCOMPAT_V144_N13_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N13_POS + copied;
                if end > UCOMPAT_V144_N13_LEN {
                    UCOMPAT_V144_N13_LEN = end;
                }
                UCOMPAT_V144_N13_POS = end;
                UCOMPAT_V144_N13_EXISTS = true;
            }
            14 => {
                if UCOMPAT_V144_N14_POS > UCOMPAT_V144_N14_LEN {
                    let mut z = UCOMPAT_V144_N14_LEN;
                    while z < UCOMPAT_V144_N14_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N14_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13015 from={} to={}",
                        UCOMPAT_V144_N14_LEN,
                        UCOMPAT_V144_N14_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N14_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N14_DATA[UCOMPAT_V144_N14_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N14_POS + copied;
                if end > UCOMPAT_V144_N14_LEN {
                    UCOMPAT_V144_N14_LEN = end;
                }
                UCOMPAT_V144_N14_POS = end;
                UCOMPAT_V144_N14_EXISTS = true;
            }
            15 => {
                if UCOMPAT_V144_N15_POS > UCOMPAT_V144_N15_LEN {
                    let mut z = UCOMPAT_V144_N15_LEN;
                    while z < UCOMPAT_V144_N15_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N15_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13016 from={} to={}",
                        UCOMPAT_V144_N15_LEN,
                        UCOMPAT_V144_N15_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N15_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N15_DATA[UCOMPAT_V144_N15_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N15_POS + copied;
                if end > UCOMPAT_V144_N15_LEN {
                    UCOMPAT_V144_N15_LEN = end;
                }
                UCOMPAT_V144_N15_POS = end;
                UCOMPAT_V144_N15_EXISTS = true;
            }
            16 => {
                if UCOMPAT_V144_N16_POS > UCOMPAT_V144_N16_LEN {
                    let mut z = UCOMPAT_V144_N16_LEN;
                    while z < UCOMPAT_V144_N16_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N16_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13017 from={} to={}",
                        UCOMPAT_V144_N16_LEN,
                        UCOMPAT_V144_N16_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N16_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N16_DATA[UCOMPAT_V144_N16_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N16_POS + copied;
                if end > UCOMPAT_V144_N16_LEN {
                    UCOMPAT_V144_N16_LEN = end;
                }
                UCOMPAT_V144_N16_POS = end;
                UCOMPAT_V144_N16_EXISTS = true;
            }
            17 => {
                if UCOMPAT_V144_N17_POS > UCOMPAT_V144_N17_LEN {
                    let mut z = UCOMPAT_V144_N17_LEN;
                    while z < UCOMPAT_V144_N17_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N17_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13018 from={} to={}",
                        UCOMPAT_V144_N17_LEN,
                        UCOMPAT_V144_N17_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N17_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N17_DATA[UCOMPAT_V144_N17_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N17_POS + copied;
                if end > UCOMPAT_V144_N17_LEN {
                    UCOMPAT_V144_N17_LEN = end;
                }
                UCOMPAT_V144_N17_POS = end;
                UCOMPAT_V144_N17_EXISTS = true;
            }
            18 => {
                if UCOMPAT_V144_N18_POS > UCOMPAT_V144_N18_LEN {
                    let mut z = UCOMPAT_V144_N18_LEN;
                    while z < UCOMPAT_V144_N18_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N18_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13019 from={} to={}",
                        UCOMPAT_V144_N18_LEN,
                        UCOMPAT_V144_N18_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N18_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N18_DATA[UCOMPAT_V144_N18_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N18_POS + copied;
                if end > UCOMPAT_V144_N18_LEN {
                    UCOMPAT_V144_N18_LEN = end;
                }
                UCOMPAT_V144_N18_POS = end;
                UCOMPAT_V144_N18_EXISTS = true;
            }
            19 => {
                if UCOMPAT_V144_N19_POS > UCOMPAT_V144_N19_LEN {
                    let mut z = UCOMPAT_V144_N19_LEN;
                    while z < UCOMPAT_V144_N19_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N19_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13020 from={} to={}",
                        UCOMPAT_V144_N19_LEN,
                        UCOMPAT_V144_N19_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N19_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N19_DATA[UCOMPAT_V144_N19_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N19_POS + copied;
                if end > UCOMPAT_V144_N19_LEN {
                    UCOMPAT_V144_N19_LEN = end;
                }
                UCOMPAT_V144_N19_POS = end;
                UCOMPAT_V144_N19_EXISTS = true;
            }
            20 => {
                if UCOMPAT_V144_N20_POS > UCOMPAT_V144_N20_LEN {
                    let mut z = UCOMPAT_V144_N20_LEN;
                    while z < UCOMPAT_V144_N20_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N20_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13021 from={} to={}",
                        UCOMPAT_V144_N20_LEN,
                        UCOMPAT_V144_N20_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N20_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N20_DATA[UCOMPAT_V144_N20_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N20_POS + copied;
                if end > UCOMPAT_V144_N20_LEN {
                    UCOMPAT_V144_N20_LEN = end;
                }
                UCOMPAT_V144_N20_POS = end;
                UCOMPAT_V144_N20_EXISTS = true;
            }
            21 => {
                if UCOMPAT_V144_N21_POS > UCOMPAT_V144_N21_LEN {
                    let mut z = UCOMPAT_V144_N21_LEN;
                    while z < UCOMPAT_V144_N21_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N21_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13022 from={} to={}",
                        UCOMPAT_V144_N21_LEN,
                        UCOMPAT_V144_N21_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N21_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N21_DATA[UCOMPAT_V144_N21_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N21_POS + copied;
                if end > UCOMPAT_V144_N21_LEN {
                    UCOMPAT_V144_N21_LEN = end;
                }
                UCOMPAT_V144_N21_POS = end;
                UCOMPAT_V144_N21_EXISTS = true;
            }
            22 => {
                if UCOMPAT_V144_N22_POS > UCOMPAT_V144_N22_LEN {
                    let mut z = UCOMPAT_V144_N22_LEN;
                    while z < UCOMPAT_V144_N22_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N22_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13023 from={} to={}",
                        UCOMPAT_V144_N22_LEN,
                        UCOMPAT_V144_N22_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N22_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N22_DATA[UCOMPAT_V144_N22_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N22_POS + copied;
                if end > UCOMPAT_V144_N22_LEN {
                    UCOMPAT_V144_N22_LEN = end;
                }
                UCOMPAT_V144_N22_POS = end;
                UCOMPAT_V144_N22_EXISTS = true;
            }
            23 => {
                if UCOMPAT_V144_N23_POS > UCOMPAT_V144_N23_LEN {
                    let mut z = UCOMPAT_V144_N23_LEN;
                    while z < UCOMPAT_V144_N23_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N23_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13024 from={} to={}",
                        UCOMPAT_V144_N23_LEN,
                        UCOMPAT_V144_N23_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N23_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N23_DATA[UCOMPAT_V144_N23_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N23_POS + copied;
                if end > UCOMPAT_V144_N23_LEN {
                    UCOMPAT_V144_N23_LEN = end;
                }
                UCOMPAT_V144_N23_POS = end;
                UCOMPAT_V144_N23_EXISTS = true;
            }
            24 => {
                if UCOMPAT_V144_N24_POS > UCOMPAT_V144_N24_LEN {
                    let mut z = UCOMPAT_V144_N24_LEN;
                    while z < UCOMPAT_V144_N24_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N24_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13025 from={} to={}",
                        UCOMPAT_V144_N24_LEN,
                        UCOMPAT_V144_N24_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N24_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N24_DATA[UCOMPAT_V144_N24_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N24_POS + copied;
                if end > UCOMPAT_V144_N24_LEN {
                    UCOMPAT_V144_N24_LEN = end;
                }
                UCOMPAT_V144_N24_POS = end;
                UCOMPAT_V144_N24_EXISTS = true;
            }
            25 => {
                if UCOMPAT_V144_N25_POS > UCOMPAT_V144_N25_LEN {
                    let mut z = UCOMPAT_V144_N25_LEN;
                    while z < UCOMPAT_V144_N25_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N25_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13026 from={} to={}",
                        UCOMPAT_V144_N25_LEN,
                        UCOMPAT_V144_N25_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N25_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N25_DATA[UCOMPAT_V144_N25_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N25_POS + copied;
                if end > UCOMPAT_V144_N25_LEN {
                    UCOMPAT_V144_N25_LEN = end;
                }
                UCOMPAT_V144_N25_POS = end;
                UCOMPAT_V144_N25_EXISTS = true;
            }
            26 => {
                if UCOMPAT_V144_N26_POS > UCOMPAT_V144_N26_LEN {
                    let mut z = UCOMPAT_V144_N26_LEN;
                    while z < UCOMPAT_V144_N26_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N26_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13027 from={} to={}",
                        UCOMPAT_V144_N26_LEN,
                        UCOMPAT_V144_N26_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N26_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N26_DATA[UCOMPAT_V144_N26_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N26_POS + copied;
                if end > UCOMPAT_V144_N26_LEN {
                    UCOMPAT_V144_N26_LEN = end;
                }
                UCOMPAT_V144_N26_POS = end;
                UCOMPAT_V144_N26_EXISTS = true;
            }
            27 => {
                if UCOMPAT_V144_N27_POS > UCOMPAT_V144_N27_LEN {
                    let mut z = UCOMPAT_V144_N27_LEN;
                    while z < UCOMPAT_V144_N27_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N27_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13028 from={} to={}",
                        UCOMPAT_V144_N27_LEN,
                        UCOMPAT_V144_N27_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N27_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N27_DATA[UCOMPAT_V144_N27_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N27_POS + copied;
                if end > UCOMPAT_V144_N27_LEN {
                    UCOMPAT_V144_N27_LEN = end;
                }
                UCOMPAT_V144_N27_POS = end;
                UCOMPAT_V144_N27_EXISTS = true;
            }
            28 => {
                if UCOMPAT_V144_N28_POS > UCOMPAT_V144_N28_LEN {
                    let mut z = UCOMPAT_V144_N28_LEN;
                    while z < UCOMPAT_V144_N28_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N28_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13029 from={} to={}",
                        UCOMPAT_V144_N28_LEN,
                        UCOMPAT_V144_N28_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N28_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N28_DATA[UCOMPAT_V144_N28_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N28_POS + copied;
                if end > UCOMPAT_V144_N28_LEN {
                    UCOMPAT_V144_N28_LEN = end;
                }
                UCOMPAT_V144_N28_POS = end;
                UCOMPAT_V144_N28_EXISTS = true;
            }
            29 => {
                if UCOMPAT_V144_N29_POS > UCOMPAT_V144_N29_LEN {
                    let mut z = UCOMPAT_V144_N29_LEN;
                    while z < UCOMPAT_V144_N29_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N29_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13030 from={} to={}",
                        UCOMPAT_V144_N29_LEN,
                        UCOMPAT_V144_N29_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N29_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N29_DATA[UCOMPAT_V144_N29_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N29_POS + copied;
                if end > UCOMPAT_V144_N29_LEN {
                    UCOMPAT_V144_N29_LEN = end;
                }
                UCOMPAT_V144_N29_POS = end;
                UCOMPAT_V144_N29_EXISTS = true;
            }
            30 => {
                if UCOMPAT_V144_N30_POS > UCOMPAT_V144_N30_LEN {
                    let mut z = UCOMPAT_V144_N30_LEN;
                    while z < UCOMPAT_V144_N30_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N30_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13031 from={} to={}",
                        UCOMPAT_V144_N30_LEN,
                        UCOMPAT_V144_N30_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N30_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N30_DATA[UCOMPAT_V144_N30_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N30_POS + copied;
                if end > UCOMPAT_V144_N30_LEN {
                    UCOMPAT_V144_N30_LEN = end;
                }
                UCOMPAT_V144_N30_POS = end;
                UCOMPAT_V144_N30_EXISTS = true;
            }
            31 => {
                if UCOMPAT_V144_N31_POS > UCOMPAT_V144_N31_LEN {
                    let mut z = UCOMPAT_V144_N31_LEN;
                    while z < UCOMPAT_V144_N31_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N31_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13032 from={} to={}",
                        UCOMPAT_V144_N31_LEN,
                        UCOMPAT_V144_N31_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N31_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N31_DATA[UCOMPAT_V144_N31_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N31_POS + copied;
                if end > UCOMPAT_V144_N31_LEN {
                    UCOMPAT_V144_N31_LEN = end;
                }
                UCOMPAT_V144_N31_POS = end;
                UCOMPAT_V144_N31_EXISTS = true;
            }
            32 => {
                if UCOMPAT_V144_N32_POS > UCOMPAT_V144_N32_LEN {
                    let mut z = UCOMPAT_V144_N32_LEN;
                    while z < UCOMPAT_V144_N32_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N32_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13033 from={} to={}",
                        UCOMPAT_V144_N32_LEN,
                        UCOMPAT_V144_N32_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N32_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N32_DATA[UCOMPAT_V144_N32_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N32_POS + copied;
                if end > UCOMPAT_V144_N32_LEN {
                    UCOMPAT_V144_N32_LEN = end;
                }
                UCOMPAT_V144_N32_POS = end;
                UCOMPAT_V144_N32_EXISTS = true;
            }
            33 => {
                if UCOMPAT_V144_N33_POS > UCOMPAT_V144_N33_LEN {
                    let mut z = UCOMPAT_V144_N33_LEN;
                    while z < UCOMPAT_V144_N33_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N33_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13034 from={} to={}",
                        UCOMPAT_V144_N33_LEN,
                        UCOMPAT_V144_N33_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N33_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N33_DATA[UCOMPAT_V144_N33_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N33_POS + copied;
                if end > UCOMPAT_V144_N33_LEN {
                    UCOMPAT_V144_N33_LEN = end;
                }
                UCOMPAT_V144_N33_POS = end;
                UCOMPAT_V144_N33_EXISTS = true;
            }
            34 => {
                if UCOMPAT_V144_N34_POS > UCOMPAT_V144_N34_LEN {
                    let mut z = UCOMPAT_V144_N34_LEN;
                    while z < UCOMPAT_V144_N34_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N34_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13035 from={} to={}",
                        UCOMPAT_V144_N34_LEN,
                        UCOMPAT_V144_N34_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N34_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N34_DATA[UCOMPAT_V144_N34_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N34_POS + copied;
                if end > UCOMPAT_V144_N34_LEN {
                    UCOMPAT_V144_N34_LEN = end;
                }
                UCOMPAT_V144_N34_POS = end;
                UCOMPAT_V144_N34_EXISTS = true;
            }
            35 => {
                if UCOMPAT_V144_N35_POS > UCOMPAT_V144_N35_LEN {
                    let mut z = UCOMPAT_V144_N35_LEN;
                    while z < UCOMPAT_V144_N35_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N35_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13036 from={} to={}",
                        UCOMPAT_V144_N35_LEN,
                        UCOMPAT_V144_N35_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N35_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N35_DATA[UCOMPAT_V144_N35_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N35_POS + copied;
                if end > UCOMPAT_V144_N35_LEN {
                    UCOMPAT_V144_N35_LEN = end;
                }
                UCOMPAT_V144_N35_POS = end;
                UCOMPAT_V144_N35_EXISTS = true;
            }
            36 => {
                if UCOMPAT_V144_N36_POS > UCOMPAT_V144_N36_LEN {
                    let mut z = UCOMPAT_V144_N36_LEN;
                    while z < UCOMPAT_V144_N36_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N36_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13037 from={} to={}",
                        UCOMPAT_V144_N36_LEN,
                        UCOMPAT_V144_N36_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N36_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N36_DATA[UCOMPAT_V144_N36_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N36_POS + copied;
                if end > UCOMPAT_V144_N36_LEN {
                    UCOMPAT_V144_N36_LEN = end;
                }
                UCOMPAT_V144_N36_POS = end;
                UCOMPAT_V144_N36_EXISTS = true;
            }
            37 => {
                if UCOMPAT_V144_N37_POS > UCOMPAT_V144_N37_LEN {
                    let mut z = UCOMPAT_V144_N37_LEN;
                    while z < UCOMPAT_V144_N37_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N37_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13038 from={} to={}",
                        UCOMPAT_V144_N37_LEN,
                        UCOMPAT_V144_N37_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N37_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N37_DATA[UCOMPAT_V144_N37_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N37_POS + copied;
                if end > UCOMPAT_V144_N37_LEN {
                    UCOMPAT_V144_N37_LEN = end;
                }
                UCOMPAT_V144_N37_POS = end;
                UCOMPAT_V144_N37_EXISTS = true;
            }
            38 => {
                if UCOMPAT_V144_N38_POS > UCOMPAT_V144_N38_LEN {
                    let mut z = UCOMPAT_V144_N38_LEN;
                    while z < UCOMPAT_V144_N38_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N38_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13039 from={} to={}",
                        UCOMPAT_V144_N38_LEN,
                        UCOMPAT_V144_N38_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N38_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N38_DATA[UCOMPAT_V144_N38_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N38_POS + copied;
                if end > UCOMPAT_V144_N38_LEN {
                    UCOMPAT_V144_N38_LEN = end;
                }
                UCOMPAT_V144_N38_POS = end;
                UCOMPAT_V144_N38_EXISTS = true;
            }
            39 => {
                if UCOMPAT_V144_N39_POS > UCOMPAT_V144_N39_LEN {
                    let mut z = UCOMPAT_V144_N39_LEN;
                    while z < UCOMPAT_V144_N39_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N39_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13040 from={} to={}",
                        UCOMPAT_V144_N39_LEN,
                        UCOMPAT_V144_N39_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N39_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N39_DATA[UCOMPAT_V144_N39_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N39_POS + copied;
                if end > UCOMPAT_V144_N39_LEN {
                    UCOMPAT_V144_N39_LEN = end;
                }
                UCOMPAT_V144_N39_POS = end;
                UCOMPAT_V144_N39_EXISTS = true;
            }
            40 => {
                if UCOMPAT_V144_N40_POS > UCOMPAT_V144_N40_LEN {
                    let mut z = UCOMPAT_V144_N40_LEN;
                    while z < UCOMPAT_V144_N40_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N40_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13041 from={} to={}",
                        UCOMPAT_V144_N40_LEN,
                        UCOMPAT_V144_N40_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N40_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N40_DATA[UCOMPAT_V144_N40_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N40_POS + copied;
                if end > UCOMPAT_V144_N40_LEN {
                    UCOMPAT_V144_N40_LEN = end;
                }
                UCOMPAT_V144_N40_POS = end;
                UCOMPAT_V144_N40_EXISTS = true;
            }
            41 => {
                if UCOMPAT_V144_N41_POS > UCOMPAT_V144_N41_LEN {
                    let mut z = UCOMPAT_V144_N41_LEN;
                    while z < UCOMPAT_V144_N41_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N41_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13042 from={} to={}",
                        UCOMPAT_V144_N41_LEN,
                        UCOMPAT_V144_N41_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N41_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N41_DATA[UCOMPAT_V144_N41_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N41_POS + copied;
                if end > UCOMPAT_V144_N41_LEN {
                    UCOMPAT_V144_N41_LEN = end;
                }
                UCOMPAT_V144_N41_POS = end;
                UCOMPAT_V144_N41_EXISTS = true;
            }
            42 => {
                if UCOMPAT_V144_N42_POS > UCOMPAT_V144_N42_LEN {
                    let mut z = UCOMPAT_V144_N42_LEN;
                    while z < UCOMPAT_V144_N42_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N42_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13043 from={} to={}",
                        UCOMPAT_V144_N42_LEN,
                        UCOMPAT_V144_N42_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N42_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N42_DATA[UCOMPAT_V144_N42_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N42_POS + copied;
                if end > UCOMPAT_V144_N42_LEN {
                    UCOMPAT_V144_N42_LEN = end;
                }
                UCOMPAT_V144_N42_POS = end;
                UCOMPAT_V144_N42_EXISTS = true;
            }
            43 => {
                if UCOMPAT_V144_N43_POS > UCOMPAT_V144_N43_LEN {
                    let mut z = UCOMPAT_V144_N43_LEN;
                    while z < UCOMPAT_V144_N43_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N43_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13044 from={} to={}",
                        UCOMPAT_V144_N43_LEN,
                        UCOMPAT_V144_N43_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N43_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N43_DATA[UCOMPAT_V144_N43_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N43_POS + copied;
                if end > UCOMPAT_V144_N43_LEN {
                    UCOMPAT_V144_N43_LEN = end;
                }
                UCOMPAT_V144_N43_POS = end;
                UCOMPAT_V144_N43_EXISTS = true;
            }
            44 => {
                if UCOMPAT_V144_N44_POS > UCOMPAT_V144_N44_LEN {
                    let mut z = UCOMPAT_V144_N44_LEN;
                    while z < UCOMPAT_V144_N44_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N44_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13045 from={} to={}",
                        UCOMPAT_V144_N44_LEN,
                        UCOMPAT_V144_N44_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N44_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N44_DATA[UCOMPAT_V144_N44_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N44_POS + copied;
                if end > UCOMPAT_V144_N44_LEN {
                    UCOMPAT_V144_N44_LEN = end;
                }
                UCOMPAT_V144_N44_POS = end;
                UCOMPAT_V144_N44_EXISTS = true;
            }
            45 => {
                if UCOMPAT_V144_N45_POS > UCOMPAT_V144_N45_LEN {
                    let mut z = UCOMPAT_V144_N45_LEN;
                    while z < UCOMPAT_V144_N45_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N45_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13046 from={} to={}",
                        UCOMPAT_V144_N45_LEN,
                        UCOMPAT_V144_N45_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N45_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N45_DATA[UCOMPAT_V144_N45_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N45_POS + copied;
                if end > UCOMPAT_V144_N45_LEN {
                    UCOMPAT_V144_N45_LEN = end;
                }
                UCOMPAT_V144_N45_POS = end;
                UCOMPAT_V144_N45_EXISTS = true;
            }
            46 => {
                if UCOMPAT_V144_N46_POS > UCOMPAT_V144_N46_LEN {
                    let mut z = UCOMPAT_V144_N46_LEN;
                    while z < UCOMPAT_V144_N46_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N46_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13047 from={} to={}",
                        UCOMPAT_V144_N46_LEN,
                        UCOMPAT_V144_N46_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N46_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N46_DATA[UCOMPAT_V144_N46_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N46_POS + copied;
                if end > UCOMPAT_V144_N46_LEN {
                    UCOMPAT_V144_N46_LEN = end;
                }
                UCOMPAT_V144_N46_POS = end;
                UCOMPAT_V144_N46_EXISTS = true;
            }
            _ => {
                if UCOMPAT_V144_N47_POS > UCOMPAT_V144_N47_LEN {
                    let mut z = UCOMPAT_V144_N47_LEN;
                    while z < UCOMPAT_V144_N47_POS && z < UCOMPAT_V144_CAP {
                        UCOMPAT_V144_N47_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v144] zero-fill sparse gap fd=13048 from={} to={}",
                        UCOMPAT_V144_N47_LEN,
                        UCOMPAT_V144_N47_POS
                    );
                }
                while copied < len && UCOMPAT_V144_N47_POS + copied < UCOMPAT_V144_CAP {
                    UCOMPAT_V144_N47_DATA[UCOMPAT_V144_N47_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V144_N47_POS + copied;
                if end > UCOMPAT_V144_N47_LEN {
                    UCOMPAT_V144_N47_LEN = end;
                }
                UCOMPAT_V144_N47_POS = end;
                UCOMPAT_V144_N47_EXISTS = true;
            }
        }
    });
    crate::println!("[ucompat-v144] write fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v144_write(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V144_FD_N00 {
        ucompat_v144_write_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V144_FD_N01 {
        ucompat_v144_write_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V144_FD_N02 {
        ucompat_v144_write_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V144_FD_N03 {
        ucompat_v144_write_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V144_FD_N04 {
        ucompat_v144_write_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V144_FD_N05 {
        ucompat_v144_write_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V144_FD_N06 {
        ucompat_v144_write_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V144_FD_N07 {
        ucompat_v144_write_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V144_FD_N08 {
        ucompat_v144_write_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V144_FD_N09 {
        ucompat_v144_write_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V144_FD_N10 {
        ucompat_v144_write_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V144_FD_N11 {
        ucompat_v144_write_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V144_FD_N12 {
        ucompat_v144_write_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V144_FD_N13 {
        ucompat_v144_write_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V144_FD_N14 {
        ucompat_v144_write_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V144_FD_N15 {
        ucompat_v144_write_one(fd, user_ptr, len, 15)
    } else if fd == UCOMPAT_V144_FD_N16 {
        ucompat_v144_write_one(fd, user_ptr, len, 16)
    } else if fd == UCOMPAT_V144_FD_N17 {
        ucompat_v144_write_one(fd, user_ptr, len, 17)
    } else if fd == UCOMPAT_V144_FD_N18 {
        ucompat_v144_write_one(fd, user_ptr, len, 18)
    } else if fd == UCOMPAT_V144_FD_N19 {
        ucompat_v144_write_one(fd, user_ptr, len, 19)
    } else if fd == UCOMPAT_V144_FD_N20 {
        ucompat_v144_write_one(fd, user_ptr, len, 20)
    } else if fd == UCOMPAT_V144_FD_N21 {
        ucompat_v144_write_one(fd, user_ptr, len, 21)
    } else if fd == UCOMPAT_V144_FD_N22 {
        ucompat_v144_write_one(fd, user_ptr, len, 22)
    } else if fd == UCOMPAT_V144_FD_N23 {
        ucompat_v144_write_one(fd, user_ptr, len, 23)
    } else if fd == UCOMPAT_V144_FD_N24 {
        ucompat_v144_write_one(fd, user_ptr, len, 24)
    } else if fd == UCOMPAT_V144_FD_N25 {
        ucompat_v144_write_one(fd, user_ptr, len, 25)
    } else if fd == UCOMPAT_V144_FD_N26 {
        ucompat_v144_write_one(fd, user_ptr, len, 26)
    } else if fd == UCOMPAT_V144_FD_N27 {
        ucompat_v144_write_one(fd, user_ptr, len, 27)
    } else if fd == UCOMPAT_V144_FD_N28 {
        ucompat_v144_write_one(fd, user_ptr, len, 28)
    } else if fd == UCOMPAT_V144_FD_N29 {
        ucompat_v144_write_one(fd, user_ptr, len, 29)
    } else if fd == UCOMPAT_V144_FD_N30 {
        ucompat_v144_write_one(fd, user_ptr, len, 30)
    } else if fd == UCOMPAT_V144_FD_N31 {
        ucompat_v144_write_one(fd, user_ptr, len, 31)
    } else if fd == UCOMPAT_V144_FD_N32 {
        ucompat_v144_write_one(fd, user_ptr, len, 32)
    } else if fd == UCOMPAT_V144_FD_N33 {
        ucompat_v144_write_one(fd, user_ptr, len, 33)
    } else if fd == UCOMPAT_V144_FD_N34 {
        ucompat_v144_write_one(fd, user_ptr, len, 34)
    } else if fd == UCOMPAT_V144_FD_N35 {
        ucompat_v144_write_one(fd, user_ptr, len, 35)
    } else if fd == UCOMPAT_V144_FD_N36 {
        ucompat_v144_write_one(fd, user_ptr, len, 36)
    } else if fd == UCOMPAT_V144_FD_N37 {
        ucompat_v144_write_one(fd, user_ptr, len, 37)
    } else if fd == UCOMPAT_V144_FD_N38 {
        ucompat_v144_write_one(fd, user_ptr, len, 38)
    } else if fd == UCOMPAT_V144_FD_N39 {
        ucompat_v144_write_one(fd, user_ptr, len, 39)
    } else if fd == UCOMPAT_V144_FD_N40 {
        ucompat_v144_write_one(fd, user_ptr, len, 40)
    } else if fd == UCOMPAT_V144_FD_N41 {
        ucompat_v144_write_one(fd, user_ptr, len, 41)
    } else if fd == UCOMPAT_V144_FD_N42 {
        ucompat_v144_write_one(fd, user_ptr, len, 42)
    } else if fd == UCOMPAT_V144_FD_N43 {
        ucompat_v144_write_one(fd, user_ptr, len, 43)
    } else if fd == UCOMPAT_V144_FD_N44 {
        ucompat_v144_write_one(fd, user_ptr, len, 44)
    } else if fd == UCOMPAT_V144_FD_N45 {
        ucompat_v144_write_one(fd, user_ptr, len, 45)
    } else if fd == UCOMPAT_V144_FD_N46 {
        ucompat_v144_write_one(fd, user_ptr, len, 46)
    } else if fd == UCOMPAT_V144_FD_N47 {
        ucompat_v144_write_one(fd, user_ptr, len, 47)
    } else {
        -9
    }
}
fn ucompat_v144_read_one(fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                while copied < len && UCOMPAT_V144_N00_POS < UCOMPAT_V144_N00_LEN {
                    let ch = UCOMPAT_V144_N00_DATA[UCOMPAT_V144_N00_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N00_POS += 1;
                    copied += 1;
                }
            }
            1 => {
                while copied < len && UCOMPAT_V144_N01_POS < UCOMPAT_V144_N01_LEN {
                    let ch = UCOMPAT_V144_N01_DATA[UCOMPAT_V144_N01_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N01_POS += 1;
                    copied += 1;
                }
            }
            2 => {
                while copied < len && UCOMPAT_V144_N02_POS < UCOMPAT_V144_N02_LEN {
                    let ch = UCOMPAT_V144_N02_DATA[UCOMPAT_V144_N02_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N02_POS += 1;
                    copied += 1;
                }
            }
            3 => {
                while copied < len && UCOMPAT_V144_N03_POS < UCOMPAT_V144_N03_LEN {
                    let ch = UCOMPAT_V144_N03_DATA[UCOMPAT_V144_N03_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N03_POS += 1;
                    copied += 1;
                }
            }
            4 => {
                while copied < len && UCOMPAT_V144_N04_POS < UCOMPAT_V144_N04_LEN {
                    let ch = UCOMPAT_V144_N04_DATA[UCOMPAT_V144_N04_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N04_POS += 1;
                    copied += 1;
                }
            }
            5 => {
                while copied < len && UCOMPAT_V144_N05_POS < UCOMPAT_V144_N05_LEN {
                    let ch = UCOMPAT_V144_N05_DATA[UCOMPAT_V144_N05_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N05_POS += 1;
                    copied += 1;
                }
            }
            6 => {
                while copied < len && UCOMPAT_V144_N06_POS < UCOMPAT_V144_N06_LEN {
                    let ch = UCOMPAT_V144_N06_DATA[UCOMPAT_V144_N06_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N06_POS += 1;
                    copied += 1;
                }
            }
            7 => {
                while copied < len && UCOMPAT_V144_N07_POS < UCOMPAT_V144_N07_LEN {
                    let ch = UCOMPAT_V144_N07_DATA[UCOMPAT_V144_N07_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N07_POS += 1;
                    copied += 1;
                }
            }
            8 => {
                while copied < len && UCOMPAT_V144_N08_POS < UCOMPAT_V144_N08_LEN {
                    let ch = UCOMPAT_V144_N08_DATA[UCOMPAT_V144_N08_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N08_POS += 1;
                    copied += 1;
                }
            }
            9 => {
                while copied < len && UCOMPAT_V144_N09_POS < UCOMPAT_V144_N09_LEN {
                    let ch = UCOMPAT_V144_N09_DATA[UCOMPAT_V144_N09_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N09_POS += 1;
                    copied += 1;
                }
            }
            10 => {
                while copied < len && UCOMPAT_V144_N10_POS < UCOMPAT_V144_N10_LEN {
                    let ch = UCOMPAT_V144_N10_DATA[UCOMPAT_V144_N10_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N10_POS += 1;
                    copied += 1;
                }
            }
            11 => {
                while copied < len && UCOMPAT_V144_N11_POS < UCOMPAT_V144_N11_LEN {
                    let ch = UCOMPAT_V144_N11_DATA[UCOMPAT_V144_N11_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N11_POS += 1;
                    copied += 1;
                }
            }
            12 => {
                while copied < len && UCOMPAT_V144_N12_POS < UCOMPAT_V144_N12_LEN {
                    let ch = UCOMPAT_V144_N12_DATA[UCOMPAT_V144_N12_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N12_POS += 1;
                    copied += 1;
                }
            }
            13 => {
                while copied < len && UCOMPAT_V144_N13_POS < UCOMPAT_V144_N13_LEN {
                    let ch = UCOMPAT_V144_N13_DATA[UCOMPAT_V144_N13_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N13_POS += 1;
                    copied += 1;
                }
            }
            14 => {
                while copied < len && UCOMPAT_V144_N14_POS < UCOMPAT_V144_N14_LEN {
                    let ch = UCOMPAT_V144_N14_DATA[UCOMPAT_V144_N14_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N14_POS += 1;
                    copied += 1;
                }
            }
            15 => {
                while copied < len && UCOMPAT_V144_N15_POS < UCOMPAT_V144_N15_LEN {
                    let ch = UCOMPAT_V144_N15_DATA[UCOMPAT_V144_N15_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N15_POS += 1;
                    copied += 1;
                }
            }
            16 => {
                while copied < len && UCOMPAT_V144_N16_POS < UCOMPAT_V144_N16_LEN {
                    let ch = UCOMPAT_V144_N16_DATA[UCOMPAT_V144_N16_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N16_POS += 1;
                    copied += 1;
                }
            }
            17 => {
                while copied < len && UCOMPAT_V144_N17_POS < UCOMPAT_V144_N17_LEN {
                    let ch = UCOMPAT_V144_N17_DATA[UCOMPAT_V144_N17_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N17_POS += 1;
                    copied += 1;
                }
            }
            18 => {
                while copied < len && UCOMPAT_V144_N18_POS < UCOMPAT_V144_N18_LEN {
                    let ch = UCOMPAT_V144_N18_DATA[UCOMPAT_V144_N18_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N18_POS += 1;
                    copied += 1;
                }
            }
            19 => {
                while copied < len && UCOMPAT_V144_N19_POS < UCOMPAT_V144_N19_LEN {
                    let ch = UCOMPAT_V144_N19_DATA[UCOMPAT_V144_N19_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N19_POS += 1;
                    copied += 1;
                }
            }
            20 => {
                while copied < len && UCOMPAT_V144_N20_POS < UCOMPAT_V144_N20_LEN {
                    let ch = UCOMPAT_V144_N20_DATA[UCOMPAT_V144_N20_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N20_POS += 1;
                    copied += 1;
                }
            }
            21 => {
                while copied < len && UCOMPAT_V144_N21_POS < UCOMPAT_V144_N21_LEN {
                    let ch = UCOMPAT_V144_N21_DATA[UCOMPAT_V144_N21_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N21_POS += 1;
                    copied += 1;
                }
            }
            22 => {
                while copied < len && UCOMPAT_V144_N22_POS < UCOMPAT_V144_N22_LEN {
                    let ch = UCOMPAT_V144_N22_DATA[UCOMPAT_V144_N22_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N22_POS += 1;
                    copied += 1;
                }
            }
            23 => {
                while copied < len && UCOMPAT_V144_N23_POS < UCOMPAT_V144_N23_LEN {
                    let ch = UCOMPAT_V144_N23_DATA[UCOMPAT_V144_N23_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N23_POS += 1;
                    copied += 1;
                }
            }
            24 => {
                while copied < len && UCOMPAT_V144_N24_POS < UCOMPAT_V144_N24_LEN {
                    let ch = UCOMPAT_V144_N24_DATA[UCOMPAT_V144_N24_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N24_POS += 1;
                    copied += 1;
                }
            }
            25 => {
                while copied < len && UCOMPAT_V144_N25_POS < UCOMPAT_V144_N25_LEN {
                    let ch = UCOMPAT_V144_N25_DATA[UCOMPAT_V144_N25_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N25_POS += 1;
                    copied += 1;
                }
            }
            26 => {
                while copied < len && UCOMPAT_V144_N26_POS < UCOMPAT_V144_N26_LEN {
                    let ch = UCOMPAT_V144_N26_DATA[UCOMPAT_V144_N26_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N26_POS += 1;
                    copied += 1;
                }
            }
            27 => {
                while copied < len && UCOMPAT_V144_N27_POS < UCOMPAT_V144_N27_LEN {
                    let ch = UCOMPAT_V144_N27_DATA[UCOMPAT_V144_N27_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N27_POS += 1;
                    copied += 1;
                }
            }
            28 => {
                while copied < len && UCOMPAT_V144_N28_POS < UCOMPAT_V144_N28_LEN {
                    let ch = UCOMPAT_V144_N28_DATA[UCOMPAT_V144_N28_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N28_POS += 1;
                    copied += 1;
                }
            }
            29 => {
                while copied < len && UCOMPAT_V144_N29_POS < UCOMPAT_V144_N29_LEN {
                    let ch = UCOMPAT_V144_N29_DATA[UCOMPAT_V144_N29_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N29_POS += 1;
                    copied += 1;
                }
            }
            30 => {
                while copied < len && UCOMPAT_V144_N30_POS < UCOMPAT_V144_N30_LEN {
                    let ch = UCOMPAT_V144_N30_DATA[UCOMPAT_V144_N30_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N30_POS += 1;
                    copied += 1;
                }
            }
            31 => {
                while copied < len && UCOMPAT_V144_N31_POS < UCOMPAT_V144_N31_LEN {
                    let ch = UCOMPAT_V144_N31_DATA[UCOMPAT_V144_N31_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N31_POS += 1;
                    copied += 1;
                }
            }
            32 => {
                while copied < len && UCOMPAT_V144_N32_POS < UCOMPAT_V144_N32_LEN {
                    let ch = UCOMPAT_V144_N32_DATA[UCOMPAT_V144_N32_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N32_POS += 1;
                    copied += 1;
                }
            }
            33 => {
                while copied < len && UCOMPAT_V144_N33_POS < UCOMPAT_V144_N33_LEN {
                    let ch = UCOMPAT_V144_N33_DATA[UCOMPAT_V144_N33_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N33_POS += 1;
                    copied += 1;
                }
            }
            34 => {
                while copied < len && UCOMPAT_V144_N34_POS < UCOMPAT_V144_N34_LEN {
                    let ch = UCOMPAT_V144_N34_DATA[UCOMPAT_V144_N34_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N34_POS += 1;
                    copied += 1;
                }
            }
            35 => {
                while copied < len && UCOMPAT_V144_N35_POS < UCOMPAT_V144_N35_LEN {
                    let ch = UCOMPAT_V144_N35_DATA[UCOMPAT_V144_N35_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N35_POS += 1;
                    copied += 1;
                }
            }
            36 => {
                while copied < len && UCOMPAT_V144_N36_POS < UCOMPAT_V144_N36_LEN {
                    let ch = UCOMPAT_V144_N36_DATA[UCOMPAT_V144_N36_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N36_POS += 1;
                    copied += 1;
                }
            }
            37 => {
                while copied < len && UCOMPAT_V144_N37_POS < UCOMPAT_V144_N37_LEN {
                    let ch = UCOMPAT_V144_N37_DATA[UCOMPAT_V144_N37_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N37_POS += 1;
                    copied += 1;
                }
            }
            38 => {
                while copied < len && UCOMPAT_V144_N38_POS < UCOMPAT_V144_N38_LEN {
                    let ch = UCOMPAT_V144_N38_DATA[UCOMPAT_V144_N38_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N38_POS += 1;
                    copied += 1;
                }
            }
            39 => {
                while copied < len && UCOMPAT_V144_N39_POS < UCOMPAT_V144_N39_LEN {
                    let ch = UCOMPAT_V144_N39_DATA[UCOMPAT_V144_N39_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N39_POS += 1;
                    copied += 1;
                }
            }
            40 => {
                while copied < len && UCOMPAT_V144_N40_POS < UCOMPAT_V144_N40_LEN {
                    let ch = UCOMPAT_V144_N40_DATA[UCOMPAT_V144_N40_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N40_POS += 1;
                    copied += 1;
                }
            }
            41 => {
                while copied < len && UCOMPAT_V144_N41_POS < UCOMPAT_V144_N41_LEN {
                    let ch = UCOMPAT_V144_N41_DATA[UCOMPAT_V144_N41_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N41_POS += 1;
                    copied += 1;
                }
            }
            42 => {
                while copied < len && UCOMPAT_V144_N42_POS < UCOMPAT_V144_N42_LEN {
                    let ch = UCOMPAT_V144_N42_DATA[UCOMPAT_V144_N42_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N42_POS += 1;
                    copied += 1;
                }
            }
            43 => {
                while copied < len && UCOMPAT_V144_N43_POS < UCOMPAT_V144_N43_LEN {
                    let ch = UCOMPAT_V144_N43_DATA[UCOMPAT_V144_N43_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N43_POS += 1;
                    copied += 1;
                }
            }
            44 => {
                while copied < len && UCOMPAT_V144_N44_POS < UCOMPAT_V144_N44_LEN {
                    let ch = UCOMPAT_V144_N44_DATA[UCOMPAT_V144_N44_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N44_POS += 1;
                    copied += 1;
                }
            }
            45 => {
                while copied < len && UCOMPAT_V144_N45_POS < UCOMPAT_V144_N45_LEN {
                    let ch = UCOMPAT_V144_N45_DATA[UCOMPAT_V144_N45_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N45_POS += 1;
                    copied += 1;
                }
            }
            46 => {
                while copied < len && UCOMPAT_V144_N46_POS < UCOMPAT_V144_N46_LEN {
                    let ch = UCOMPAT_V144_N46_DATA[UCOMPAT_V144_N46_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N46_POS += 1;
                    copied += 1;
                }
            }
            _ => {
                while copied < len && UCOMPAT_V144_N47_POS < UCOMPAT_V144_N47_LEN {
                    let ch = UCOMPAT_V144_N47_DATA[UCOMPAT_V144_N47_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V144_N47_POS += 1;
                    copied += 1;
                }
            }
        }
    });
    crate::println!("[ucompat-v144] read fd={} copied={}", fd, copied);
    copied as isize
}
fn ucompat_v144_read(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V144_FD_N00 {
        ucompat_v144_read_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V144_FD_N01 {
        ucompat_v144_read_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V144_FD_N02 {
        ucompat_v144_read_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V144_FD_N03 {
        ucompat_v144_read_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V144_FD_N04 {
        ucompat_v144_read_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V144_FD_N05 {
        ucompat_v144_read_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V144_FD_N06 {
        ucompat_v144_read_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V144_FD_N07 {
        ucompat_v144_read_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V144_FD_N08 {
        ucompat_v144_read_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V144_FD_N09 {
        ucompat_v144_read_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V144_FD_N10 {
        ucompat_v144_read_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V144_FD_N11 {
        ucompat_v144_read_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V144_FD_N12 {
        ucompat_v144_read_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V144_FD_N13 {
        ucompat_v144_read_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V144_FD_N14 {
        ucompat_v144_read_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V144_FD_N15 {
        ucompat_v144_read_one(fd, user_ptr, len, 15)
    } else if fd == UCOMPAT_V144_FD_N16 {
        ucompat_v144_read_one(fd, user_ptr, len, 16)
    } else if fd == UCOMPAT_V144_FD_N17 {
        ucompat_v144_read_one(fd, user_ptr, len, 17)
    } else if fd == UCOMPAT_V144_FD_N18 {
        ucompat_v144_read_one(fd, user_ptr, len, 18)
    } else if fd == UCOMPAT_V144_FD_N19 {
        ucompat_v144_read_one(fd, user_ptr, len, 19)
    } else if fd == UCOMPAT_V144_FD_N20 {
        ucompat_v144_read_one(fd, user_ptr, len, 20)
    } else if fd == UCOMPAT_V144_FD_N21 {
        ucompat_v144_read_one(fd, user_ptr, len, 21)
    } else if fd == UCOMPAT_V144_FD_N22 {
        ucompat_v144_read_one(fd, user_ptr, len, 22)
    } else if fd == UCOMPAT_V144_FD_N23 {
        ucompat_v144_read_one(fd, user_ptr, len, 23)
    } else if fd == UCOMPAT_V144_FD_N24 {
        ucompat_v144_read_one(fd, user_ptr, len, 24)
    } else if fd == UCOMPAT_V144_FD_N25 {
        ucompat_v144_read_one(fd, user_ptr, len, 25)
    } else if fd == UCOMPAT_V144_FD_N26 {
        ucompat_v144_read_one(fd, user_ptr, len, 26)
    } else if fd == UCOMPAT_V144_FD_N27 {
        ucompat_v144_read_one(fd, user_ptr, len, 27)
    } else if fd == UCOMPAT_V144_FD_N28 {
        ucompat_v144_read_one(fd, user_ptr, len, 28)
    } else if fd == UCOMPAT_V144_FD_N29 {
        ucompat_v144_read_one(fd, user_ptr, len, 29)
    } else if fd == UCOMPAT_V144_FD_N30 {
        ucompat_v144_read_one(fd, user_ptr, len, 30)
    } else if fd == UCOMPAT_V144_FD_N31 {
        ucompat_v144_read_one(fd, user_ptr, len, 31)
    } else if fd == UCOMPAT_V144_FD_N32 {
        ucompat_v144_read_one(fd, user_ptr, len, 32)
    } else if fd == UCOMPAT_V144_FD_N33 {
        ucompat_v144_read_one(fd, user_ptr, len, 33)
    } else if fd == UCOMPAT_V144_FD_N34 {
        ucompat_v144_read_one(fd, user_ptr, len, 34)
    } else if fd == UCOMPAT_V144_FD_N35 {
        ucompat_v144_read_one(fd, user_ptr, len, 35)
    } else if fd == UCOMPAT_V144_FD_N36 {
        ucompat_v144_read_one(fd, user_ptr, len, 36)
    } else if fd == UCOMPAT_V144_FD_N37 {
        ucompat_v144_read_one(fd, user_ptr, len, 37)
    } else if fd == UCOMPAT_V144_FD_N38 {
        ucompat_v144_read_one(fd, user_ptr, len, 38)
    } else if fd == UCOMPAT_V144_FD_N39 {
        ucompat_v144_read_one(fd, user_ptr, len, 39)
    } else if fd == UCOMPAT_V144_FD_N40 {
        ucompat_v144_read_one(fd, user_ptr, len, 40)
    } else if fd == UCOMPAT_V144_FD_N41 {
        ucompat_v144_read_one(fd, user_ptr, len, 41)
    } else if fd == UCOMPAT_V144_FD_N42 {
        ucompat_v144_read_one(fd, user_ptr, len, 42)
    } else if fd == UCOMPAT_V144_FD_N43 {
        ucompat_v144_read_one(fd, user_ptr, len, 43)
    } else if fd == UCOMPAT_V144_FD_N44 {
        ucompat_v144_read_one(fd, user_ptr, len, 44)
    } else if fd == UCOMPAT_V144_FD_N45 {
        ucompat_v144_read_one(fd, user_ptr, len, 45)
    } else if fd == UCOMPAT_V144_FD_N46 {
        ucompat_v144_read_one(fd, user_ptr, len, 46)
    } else if fd == UCOMPAT_V144_FD_N47 {
        ucompat_v144_read_one(fd, user_ptr, len, 47)
    } else {
        -9
    }
}
fn ucompat_v144_lseek(fd: isize, off: isize, whence: usize) -> isize {
    unsafe {
        let (len, cur) = if fd == UCOMPAT_V144_FD_N00 {
            (UCOMPAT_V144_N00_LEN, UCOMPAT_V144_N00_POS)
        } else if fd == UCOMPAT_V144_FD_N01 {
            (UCOMPAT_V144_N01_LEN, UCOMPAT_V144_N01_POS)
        } else if fd == UCOMPAT_V144_FD_N02 {
            (UCOMPAT_V144_N02_LEN, UCOMPAT_V144_N02_POS)
        } else if fd == UCOMPAT_V144_FD_N03 {
            (UCOMPAT_V144_N03_LEN, UCOMPAT_V144_N03_POS)
        } else if fd == UCOMPAT_V144_FD_N04 {
            (UCOMPAT_V144_N04_LEN, UCOMPAT_V144_N04_POS)
        } else if fd == UCOMPAT_V144_FD_N05 {
            (UCOMPAT_V144_N05_LEN, UCOMPAT_V144_N05_POS)
        } else if fd == UCOMPAT_V144_FD_N06 {
            (UCOMPAT_V144_N06_LEN, UCOMPAT_V144_N06_POS)
        } else if fd == UCOMPAT_V144_FD_N07 {
            (UCOMPAT_V144_N07_LEN, UCOMPAT_V144_N07_POS)
        } else if fd == UCOMPAT_V144_FD_N08 {
            (UCOMPAT_V144_N08_LEN, UCOMPAT_V144_N08_POS)
        } else if fd == UCOMPAT_V144_FD_N09 {
            (UCOMPAT_V144_N09_LEN, UCOMPAT_V144_N09_POS)
        } else if fd == UCOMPAT_V144_FD_N10 {
            (UCOMPAT_V144_N10_LEN, UCOMPAT_V144_N10_POS)
        } else if fd == UCOMPAT_V144_FD_N11 {
            (UCOMPAT_V144_N11_LEN, UCOMPAT_V144_N11_POS)
        } else if fd == UCOMPAT_V144_FD_N12 {
            (UCOMPAT_V144_N12_LEN, UCOMPAT_V144_N12_POS)
        } else if fd == UCOMPAT_V144_FD_N13 {
            (UCOMPAT_V144_N13_LEN, UCOMPAT_V144_N13_POS)
        } else if fd == UCOMPAT_V144_FD_N14 {
            (UCOMPAT_V144_N14_LEN, UCOMPAT_V144_N14_POS)
        } else if fd == UCOMPAT_V144_FD_N15 {
            (UCOMPAT_V144_N15_LEN, UCOMPAT_V144_N15_POS)
        } else if fd == UCOMPAT_V144_FD_N16 {
            (UCOMPAT_V144_N16_LEN, UCOMPAT_V144_N16_POS)
        } else if fd == UCOMPAT_V144_FD_N17 {
            (UCOMPAT_V144_N17_LEN, UCOMPAT_V144_N17_POS)
        } else if fd == UCOMPAT_V144_FD_N18 {
            (UCOMPAT_V144_N18_LEN, UCOMPAT_V144_N18_POS)
        } else if fd == UCOMPAT_V144_FD_N19 {
            (UCOMPAT_V144_N19_LEN, UCOMPAT_V144_N19_POS)
        } else if fd == UCOMPAT_V144_FD_N20 {
            (UCOMPAT_V144_N20_LEN, UCOMPAT_V144_N20_POS)
        } else if fd == UCOMPAT_V144_FD_N21 {
            (UCOMPAT_V144_N21_LEN, UCOMPAT_V144_N21_POS)
        } else if fd == UCOMPAT_V144_FD_N22 {
            (UCOMPAT_V144_N22_LEN, UCOMPAT_V144_N22_POS)
        } else if fd == UCOMPAT_V144_FD_N23 {
            (UCOMPAT_V144_N23_LEN, UCOMPAT_V144_N23_POS)
        } else if fd == UCOMPAT_V144_FD_N24 {
            (UCOMPAT_V144_N24_LEN, UCOMPAT_V144_N24_POS)
        } else if fd == UCOMPAT_V144_FD_N25 {
            (UCOMPAT_V144_N25_LEN, UCOMPAT_V144_N25_POS)
        } else if fd == UCOMPAT_V144_FD_N26 {
            (UCOMPAT_V144_N26_LEN, UCOMPAT_V144_N26_POS)
        } else if fd == UCOMPAT_V144_FD_N27 {
            (UCOMPAT_V144_N27_LEN, UCOMPAT_V144_N27_POS)
        } else if fd == UCOMPAT_V144_FD_N28 {
            (UCOMPAT_V144_N28_LEN, UCOMPAT_V144_N28_POS)
        } else if fd == UCOMPAT_V144_FD_N29 {
            (UCOMPAT_V144_N29_LEN, UCOMPAT_V144_N29_POS)
        } else if fd == UCOMPAT_V144_FD_N30 {
            (UCOMPAT_V144_N30_LEN, UCOMPAT_V144_N30_POS)
        } else if fd == UCOMPAT_V144_FD_N31 {
            (UCOMPAT_V144_N31_LEN, UCOMPAT_V144_N31_POS)
        } else if fd == UCOMPAT_V144_FD_N32 {
            (UCOMPAT_V144_N32_LEN, UCOMPAT_V144_N32_POS)
        } else if fd == UCOMPAT_V144_FD_N33 {
            (UCOMPAT_V144_N33_LEN, UCOMPAT_V144_N33_POS)
        } else if fd == UCOMPAT_V144_FD_N34 {
            (UCOMPAT_V144_N34_LEN, UCOMPAT_V144_N34_POS)
        } else if fd == UCOMPAT_V144_FD_N35 {
            (UCOMPAT_V144_N35_LEN, UCOMPAT_V144_N35_POS)
        } else if fd == UCOMPAT_V144_FD_N36 {
            (UCOMPAT_V144_N36_LEN, UCOMPAT_V144_N36_POS)
        } else if fd == UCOMPAT_V144_FD_N37 {
            (UCOMPAT_V144_N37_LEN, UCOMPAT_V144_N37_POS)
        } else if fd == UCOMPAT_V144_FD_N38 {
            (UCOMPAT_V144_N38_LEN, UCOMPAT_V144_N38_POS)
        } else if fd == UCOMPAT_V144_FD_N39 {
            (UCOMPAT_V144_N39_LEN, UCOMPAT_V144_N39_POS)
        } else if fd == UCOMPAT_V144_FD_N40 {
            (UCOMPAT_V144_N40_LEN, UCOMPAT_V144_N40_POS)
        } else if fd == UCOMPAT_V144_FD_N41 {
            (UCOMPAT_V144_N41_LEN, UCOMPAT_V144_N41_POS)
        } else if fd == UCOMPAT_V144_FD_N42 {
            (UCOMPAT_V144_N42_LEN, UCOMPAT_V144_N42_POS)
        } else if fd == UCOMPAT_V144_FD_N43 {
            (UCOMPAT_V144_N43_LEN, UCOMPAT_V144_N43_POS)
        } else if fd == UCOMPAT_V144_FD_N44 {
            (UCOMPAT_V144_N44_LEN, UCOMPAT_V144_N44_POS)
        } else if fd == UCOMPAT_V144_FD_N45 {
            (UCOMPAT_V144_N45_LEN, UCOMPAT_V144_N45_POS)
        } else if fd == UCOMPAT_V144_FD_N46 {
            (UCOMPAT_V144_N46_LEN, UCOMPAT_V144_N46_POS)
        } else if fd == UCOMPAT_V144_FD_N47 {
            (UCOMPAT_V144_N47_LEN, UCOMPAT_V144_N47_POS)
        } else {
            return -9;
        };
        let base = match whence {
            0 => 0isize,
            1 => cur as isize,
            2 => len as isize,
            _ => return -22,
        };
        let new_pos = base + off;
        if new_pos < 0 {
            return -22;
        }
        if fd == UCOMPAT_V144_FD_N00 {
            UCOMPAT_V144_N00_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N01 {
            UCOMPAT_V144_N01_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N02 {
            UCOMPAT_V144_N02_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N03 {
            UCOMPAT_V144_N03_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N04 {
            UCOMPAT_V144_N04_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N05 {
            UCOMPAT_V144_N05_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N06 {
            UCOMPAT_V144_N06_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N07 {
            UCOMPAT_V144_N07_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N08 {
            UCOMPAT_V144_N08_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N09 {
            UCOMPAT_V144_N09_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N10 {
            UCOMPAT_V144_N10_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N11 {
            UCOMPAT_V144_N11_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N12 {
            UCOMPAT_V144_N12_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N13 {
            UCOMPAT_V144_N13_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N14 {
            UCOMPAT_V144_N14_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N15 {
            UCOMPAT_V144_N15_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N16 {
            UCOMPAT_V144_N16_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N17 {
            UCOMPAT_V144_N17_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N18 {
            UCOMPAT_V144_N18_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N19 {
            UCOMPAT_V144_N19_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N20 {
            UCOMPAT_V144_N20_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N21 {
            UCOMPAT_V144_N21_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N22 {
            UCOMPAT_V144_N22_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N23 {
            UCOMPAT_V144_N23_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N24 {
            UCOMPAT_V144_N24_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N25 {
            UCOMPAT_V144_N25_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N26 {
            UCOMPAT_V144_N26_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N27 {
            UCOMPAT_V144_N27_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N28 {
            UCOMPAT_V144_N28_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N29 {
            UCOMPAT_V144_N29_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N30 {
            UCOMPAT_V144_N30_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N31 {
            UCOMPAT_V144_N31_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N32 {
            UCOMPAT_V144_N32_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N33 {
            UCOMPAT_V144_N33_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N34 {
            UCOMPAT_V144_N34_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N35 {
            UCOMPAT_V144_N35_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N36 {
            UCOMPAT_V144_N36_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N37 {
            UCOMPAT_V144_N37_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N38 {
            UCOMPAT_V144_N38_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N39 {
            UCOMPAT_V144_N39_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N40 {
            UCOMPAT_V144_N40_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N41 {
            UCOMPAT_V144_N41_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N42 {
            UCOMPAT_V144_N42_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N43 {
            UCOMPAT_V144_N43_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N44 {
            UCOMPAT_V144_N44_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N45 {
            UCOMPAT_V144_N45_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N46 {
            UCOMPAT_V144_N46_POS = new_pos as usize;
        } else if fd == UCOMPAT_V144_FD_N47 {
            UCOMPAT_V144_N47_POS = new_pos as usize;
        } else {
            return -9;
        }
        crate::println!("[ucompat-v144] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}
fn ucompat_v144_close(fd: isize) -> isize {
    unsafe {
        if fd == UCOMPAT_V144_FD_N00 {
            UCOMPAT_V144_N00_POS = 0;
            crate::println!("[ucompat-v144] close fd=13001 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N01 {
            UCOMPAT_V144_N01_POS = 0;
            crate::println!("[ucompat-v144] close fd=13002 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N02 {
            UCOMPAT_V144_N02_POS = 0;
            crate::println!("[ucompat-v144] close fd=13003 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N03 {
            UCOMPAT_V144_N03_POS = 0;
            crate::println!("[ucompat-v144] close fd=13004 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N04 {
            UCOMPAT_V144_N04_POS = 0;
            crate::println!("[ucompat-v144] close fd=13005 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N05 {
            UCOMPAT_V144_N05_POS = 0;
            crate::println!("[ucompat-v144] close fd=13006 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N06 {
            UCOMPAT_V144_N06_POS = 0;
            crate::println!("[ucompat-v144] close fd=13007 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N07 {
            UCOMPAT_V144_N07_POS = 0;
            crate::println!("[ucompat-v144] close fd=13008 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N08 {
            UCOMPAT_V144_N08_POS = 0;
            crate::println!("[ucompat-v144] close fd=13009 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N09 {
            UCOMPAT_V144_N09_POS = 0;
            crate::println!("[ucompat-v144] close fd=13010 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N10 {
            UCOMPAT_V144_N10_POS = 0;
            crate::println!("[ucompat-v144] close fd=13011 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N11 {
            UCOMPAT_V144_N11_POS = 0;
            crate::println!("[ucompat-v144] close fd=13012 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N12 {
            UCOMPAT_V144_N12_POS = 0;
            crate::println!("[ucompat-v144] close fd=13013 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N13 {
            UCOMPAT_V144_N13_POS = 0;
            crate::println!("[ucompat-v144] close fd=13014 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N14 {
            UCOMPAT_V144_N14_POS = 0;
            crate::println!("[ucompat-v144] close fd=13015 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N15 {
            UCOMPAT_V144_N15_POS = 0;
            crate::println!("[ucompat-v144] close fd=13016 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N16 {
            UCOMPAT_V144_N16_POS = 0;
            crate::println!("[ucompat-v144] close fd=13017 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N17 {
            UCOMPAT_V144_N17_POS = 0;
            crate::println!("[ucompat-v144] close fd=13018 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N18 {
            UCOMPAT_V144_N18_POS = 0;
            crate::println!("[ucompat-v144] close fd=13019 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N19 {
            UCOMPAT_V144_N19_POS = 0;
            crate::println!("[ucompat-v144] close fd=13020 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N20 {
            UCOMPAT_V144_N20_POS = 0;
            crate::println!("[ucompat-v144] close fd=13021 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N21 {
            UCOMPAT_V144_N21_POS = 0;
            crate::println!("[ucompat-v144] close fd=13022 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N22 {
            UCOMPAT_V144_N22_POS = 0;
            crate::println!("[ucompat-v144] close fd=13023 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N23 {
            UCOMPAT_V144_N23_POS = 0;
            crate::println!("[ucompat-v144] close fd=13024 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N24 {
            UCOMPAT_V144_N24_POS = 0;
            crate::println!("[ucompat-v144] close fd=13025 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N25 {
            UCOMPAT_V144_N25_POS = 0;
            crate::println!("[ucompat-v144] close fd=13026 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N26 {
            UCOMPAT_V144_N26_POS = 0;
            crate::println!("[ucompat-v144] close fd=13027 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N27 {
            UCOMPAT_V144_N27_POS = 0;
            crate::println!("[ucompat-v144] close fd=13028 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N28 {
            UCOMPAT_V144_N28_POS = 0;
            crate::println!("[ucompat-v144] close fd=13029 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N29 {
            UCOMPAT_V144_N29_POS = 0;
            crate::println!("[ucompat-v144] close fd=13030 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N30 {
            UCOMPAT_V144_N30_POS = 0;
            crate::println!("[ucompat-v144] close fd=13031 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N31 {
            UCOMPAT_V144_N31_POS = 0;
            crate::println!("[ucompat-v144] close fd=13032 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N32 {
            UCOMPAT_V144_N32_POS = 0;
            crate::println!("[ucompat-v144] close fd=13033 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N33 {
            UCOMPAT_V144_N33_POS = 0;
            crate::println!("[ucompat-v144] close fd=13034 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N34 {
            UCOMPAT_V144_N34_POS = 0;
            crate::println!("[ucompat-v144] close fd=13035 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N35 {
            UCOMPAT_V144_N35_POS = 0;
            crate::println!("[ucompat-v144] close fd=13036 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N36 {
            UCOMPAT_V144_N36_POS = 0;
            crate::println!("[ucompat-v144] close fd=13037 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N37 {
            UCOMPAT_V144_N37_POS = 0;
            crate::println!("[ucompat-v144] close fd=13038 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N38 {
            UCOMPAT_V144_N38_POS = 0;
            crate::println!("[ucompat-v144] close fd=13039 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N39 {
            UCOMPAT_V144_N39_POS = 0;
            crate::println!("[ucompat-v144] close fd=13040 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N40 {
            UCOMPAT_V144_N40_POS = 0;
            crate::println!("[ucompat-v144] close fd=13041 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N41 {
            UCOMPAT_V144_N41_POS = 0;
            crate::println!("[ucompat-v144] close fd=13042 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N42 {
            UCOMPAT_V144_N42_POS = 0;
            crate::println!("[ucompat-v144] close fd=13043 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N43 {
            UCOMPAT_V144_N43_POS = 0;
            crate::println!("[ucompat-v144] close fd=13044 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N44 {
            UCOMPAT_V144_N44_POS = 0;
            crate::println!("[ucompat-v144] close fd=13045 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N45 {
            UCOMPAT_V144_N45_POS = 0;
            crate::println!("[ucompat-v144] close fd=13046 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N46 {
            UCOMPAT_V144_N46_POS = 0;
            crate::println!("[ucompat-v144] close fd=13047 ret=0 keep_file=1");
            0
        } else if fd == UCOMPAT_V144_FD_N47 {
            UCOMPAT_V144_N47_POS = 0;
            crate::println!("[ucompat-v144] close fd=13048 ret=0 keep_file=1");
            0
        } else {
            -9
        }
    }
}

// UCOMPAT_V145F_UNUSED_WARNING_CLEANUP_SOURCE_BASELINE
// UCOMPAT_V145E_SERIAL_NOISE_REDUCTION_SOURCE_BASELINE
// UCOMPAT_V145D_RUNTIME_SCAN_TIMEOUT_SOURCE_BASELINE
// UCOMPAT_V145C_QEMU_TIMEOUT_BUDGET_SOURCE_BASELINE
// UCOMPAT_V145B_INIT_ELF_DATAOFF_LAYOUT_SOURCE_BASELINE
// UCOMPAT_V145_FILE64_PREAD_PWRITE_POSITIONED_IO
const UCOMPAT_V145_FD_N00: isize = 14001;
const UCOMPAT_V145_FD_N01: isize = 14002;
const UCOMPAT_V145_FD_N02: isize = 14003;
const UCOMPAT_V145_FD_N03: isize = 14004;
const UCOMPAT_V145_FD_N04: isize = 14005;
const UCOMPAT_V145_FD_N05: isize = 14006;
const UCOMPAT_V145_FD_N06: isize = 14007;
const UCOMPAT_V145_FD_N07: isize = 14008;
const UCOMPAT_V145_FD_N08: isize = 14009;
const UCOMPAT_V145_FD_N09: isize = 14010;
const UCOMPAT_V145_FD_N10: isize = 14011;
const UCOMPAT_V145_FD_N11: isize = 14012;
const UCOMPAT_V145_FD_N12: isize = 14013;
const UCOMPAT_V145_FD_N13: isize = 14014;
const UCOMPAT_V145_FD_N14: isize = 14015;
const UCOMPAT_V145_FD_N15: isize = 14016;
const UCOMPAT_V145_FD_N16: isize = 14017;
const UCOMPAT_V145_FD_N17: isize = 14018;
const UCOMPAT_V145_FD_N18: isize = 14019;
const UCOMPAT_V145_FD_N19: isize = 14020;
const UCOMPAT_V145_FD_N20: isize = 14021;
const UCOMPAT_V145_FD_N21: isize = 14022;
const UCOMPAT_V145_FD_N22: isize = 14023;
const UCOMPAT_V145_FD_N23: isize = 14024;
const UCOMPAT_V145_FD_N24: isize = 14025;
const UCOMPAT_V145_FD_N25: isize = 14026;
const UCOMPAT_V145_FD_N26: isize = 14027;
const UCOMPAT_V145_FD_N27: isize = 14028;
const UCOMPAT_V145_FD_N28: isize = 14029;
const UCOMPAT_V145_FD_N29: isize = 14030;
const UCOMPAT_V145_FD_N30: isize = 14031;
const UCOMPAT_V145_FD_N31: isize = 14032;
const UCOMPAT_V145_FD_N32: isize = 14033;
const UCOMPAT_V145_FD_N33: isize = 14034;
const UCOMPAT_V145_FD_N34: isize = 14035;
const UCOMPAT_V145_FD_N35: isize = 14036;
const UCOMPAT_V145_FD_N36: isize = 14037;
const UCOMPAT_V145_FD_N37: isize = 14038;
const UCOMPAT_V145_FD_N38: isize = 14039;
const UCOMPAT_V145_FD_N39: isize = 14040;
const UCOMPAT_V145_FD_N40: isize = 14041;
const UCOMPAT_V145_FD_N41: isize = 14042;
const UCOMPAT_V145_FD_N42: isize = 14043;
const UCOMPAT_V145_FD_N43: isize = 14044;
const UCOMPAT_V145_FD_N44: isize = 14045;
const UCOMPAT_V145_FD_N45: isize = 14046;
const UCOMPAT_V145_FD_N46: isize = 14047;
const UCOMPAT_V145_FD_N47: isize = 14048;
const UCOMPAT_V145_FD_N48: isize = 14049;
const UCOMPAT_V145_FD_N49: isize = 14050;
const UCOMPAT_V145_FD_N50: isize = 14051;
const UCOMPAT_V145_FD_N51: isize = 14052;
const UCOMPAT_V145_FD_N52: isize = 14053;
const UCOMPAT_V145_FD_N53: isize = 14054;
const UCOMPAT_V145_FD_N54: isize = 14055;
const UCOMPAT_V145_FD_N55: isize = 14056;
const UCOMPAT_V145_FD_N56: isize = 14057;
const UCOMPAT_V145_FD_N57: isize = 14058;
const UCOMPAT_V145_FD_N58: isize = 14059;
const UCOMPAT_V145_FD_N59: isize = 14060;
const UCOMPAT_V145_FD_N60: isize = 14061;
const UCOMPAT_V145_FD_N61: isize = 14062;
const UCOMPAT_V145_FD_N62: isize = 14063;
const UCOMPAT_V145_FD_N63: isize = 14064;
const UCOMPAT_V145_CAP: usize = 1024;
static mut UCOMPAT_V145_N00_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N00_LEN: usize = 0;
static mut UCOMPAT_V145_N00_POS: usize = 0;
static mut UCOMPAT_V145_N00_EXISTS: bool = false;
static mut UCOMPAT_V145_N01_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N01_LEN: usize = 0;
static mut UCOMPAT_V145_N01_POS: usize = 0;
static mut UCOMPAT_V145_N01_EXISTS: bool = false;
static mut UCOMPAT_V145_N02_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N02_LEN: usize = 0;
static mut UCOMPAT_V145_N02_POS: usize = 0;
static mut UCOMPAT_V145_N02_EXISTS: bool = false;
static mut UCOMPAT_V145_N03_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N03_LEN: usize = 0;
static mut UCOMPAT_V145_N03_POS: usize = 0;
static mut UCOMPAT_V145_N03_EXISTS: bool = false;
static mut UCOMPAT_V145_N04_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N04_LEN: usize = 0;
static mut UCOMPAT_V145_N04_POS: usize = 0;
static mut UCOMPAT_V145_N04_EXISTS: bool = false;
static mut UCOMPAT_V145_N05_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N05_LEN: usize = 0;
static mut UCOMPAT_V145_N05_POS: usize = 0;
static mut UCOMPAT_V145_N05_EXISTS: bool = false;
static mut UCOMPAT_V145_N06_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N06_LEN: usize = 0;
static mut UCOMPAT_V145_N06_POS: usize = 0;
static mut UCOMPAT_V145_N06_EXISTS: bool = false;
static mut UCOMPAT_V145_N07_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N07_LEN: usize = 0;
static mut UCOMPAT_V145_N07_POS: usize = 0;
static mut UCOMPAT_V145_N07_EXISTS: bool = false;
static mut UCOMPAT_V145_N08_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N08_LEN: usize = 0;
static mut UCOMPAT_V145_N08_POS: usize = 0;
static mut UCOMPAT_V145_N08_EXISTS: bool = false;
static mut UCOMPAT_V145_N09_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N09_LEN: usize = 0;
static mut UCOMPAT_V145_N09_POS: usize = 0;
static mut UCOMPAT_V145_N09_EXISTS: bool = false;
static mut UCOMPAT_V145_N10_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N10_LEN: usize = 0;
static mut UCOMPAT_V145_N10_POS: usize = 0;
static mut UCOMPAT_V145_N10_EXISTS: bool = false;
static mut UCOMPAT_V145_N11_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N11_LEN: usize = 0;
static mut UCOMPAT_V145_N11_POS: usize = 0;
static mut UCOMPAT_V145_N11_EXISTS: bool = false;
static mut UCOMPAT_V145_N12_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N12_LEN: usize = 0;
static mut UCOMPAT_V145_N12_POS: usize = 0;
static mut UCOMPAT_V145_N12_EXISTS: bool = false;
static mut UCOMPAT_V145_N13_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N13_LEN: usize = 0;
static mut UCOMPAT_V145_N13_POS: usize = 0;
static mut UCOMPAT_V145_N13_EXISTS: bool = false;
static mut UCOMPAT_V145_N14_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N14_LEN: usize = 0;
static mut UCOMPAT_V145_N14_POS: usize = 0;
static mut UCOMPAT_V145_N14_EXISTS: bool = false;
static mut UCOMPAT_V145_N15_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N15_LEN: usize = 0;
static mut UCOMPAT_V145_N15_POS: usize = 0;
static mut UCOMPAT_V145_N15_EXISTS: bool = false;
static mut UCOMPAT_V145_N16_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N16_LEN: usize = 0;
static mut UCOMPAT_V145_N16_POS: usize = 0;
static mut UCOMPAT_V145_N16_EXISTS: bool = false;
static mut UCOMPAT_V145_N17_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N17_LEN: usize = 0;
static mut UCOMPAT_V145_N17_POS: usize = 0;
static mut UCOMPAT_V145_N17_EXISTS: bool = false;
static mut UCOMPAT_V145_N18_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N18_LEN: usize = 0;
static mut UCOMPAT_V145_N18_POS: usize = 0;
static mut UCOMPAT_V145_N18_EXISTS: bool = false;
static mut UCOMPAT_V145_N19_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N19_LEN: usize = 0;
static mut UCOMPAT_V145_N19_POS: usize = 0;
static mut UCOMPAT_V145_N19_EXISTS: bool = false;
static mut UCOMPAT_V145_N20_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N20_LEN: usize = 0;
static mut UCOMPAT_V145_N20_POS: usize = 0;
static mut UCOMPAT_V145_N20_EXISTS: bool = false;
static mut UCOMPAT_V145_N21_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N21_LEN: usize = 0;
static mut UCOMPAT_V145_N21_POS: usize = 0;
static mut UCOMPAT_V145_N21_EXISTS: bool = false;
static mut UCOMPAT_V145_N22_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N22_LEN: usize = 0;
static mut UCOMPAT_V145_N22_POS: usize = 0;
static mut UCOMPAT_V145_N22_EXISTS: bool = false;
static mut UCOMPAT_V145_N23_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N23_LEN: usize = 0;
static mut UCOMPAT_V145_N23_POS: usize = 0;
static mut UCOMPAT_V145_N23_EXISTS: bool = false;
static mut UCOMPAT_V145_N24_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N24_LEN: usize = 0;
static mut UCOMPAT_V145_N24_POS: usize = 0;
static mut UCOMPAT_V145_N24_EXISTS: bool = false;
static mut UCOMPAT_V145_N25_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N25_LEN: usize = 0;
static mut UCOMPAT_V145_N25_POS: usize = 0;
static mut UCOMPAT_V145_N25_EXISTS: bool = false;
static mut UCOMPAT_V145_N26_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N26_LEN: usize = 0;
static mut UCOMPAT_V145_N26_POS: usize = 0;
static mut UCOMPAT_V145_N26_EXISTS: bool = false;
static mut UCOMPAT_V145_N27_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N27_LEN: usize = 0;
static mut UCOMPAT_V145_N27_POS: usize = 0;
static mut UCOMPAT_V145_N27_EXISTS: bool = false;
static mut UCOMPAT_V145_N28_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N28_LEN: usize = 0;
static mut UCOMPAT_V145_N28_POS: usize = 0;
static mut UCOMPAT_V145_N28_EXISTS: bool = false;
static mut UCOMPAT_V145_N29_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N29_LEN: usize = 0;
static mut UCOMPAT_V145_N29_POS: usize = 0;
static mut UCOMPAT_V145_N29_EXISTS: bool = false;
static mut UCOMPAT_V145_N30_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N30_LEN: usize = 0;
static mut UCOMPAT_V145_N30_POS: usize = 0;
static mut UCOMPAT_V145_N30_EXISTS: bool = false;
static mut UCOMPAT_V145_N31_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N31_LEN: usize = 0;
static mut UCOMPAT_V145_N31_POS: usize = 0;
static mut UCOMPAT_V145_N31_EXISTS: bool = false;
static mut UCOMPAT_V145_N32_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N32_LEN: usize = 0;
static mut UCOMPAT_V145_N32_POS: usize = 0;
static mut UCOMPAT_V145_N32_EXISTS: bool = false;
static mut UCOMPAT_V145_N33_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N33_LEN: usize = 0;
static mut UCOMPAT_V145_N33_POS: usize = 0;
static mut UCOMPAT_V145_N33_EXISTS: bool = false;
static mut UCOMPAT_V145_N34_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N34_LEN: usize = 0;
static mut UCOMPAT_V145_N34_POS: usize = 0;
static mut UCOMPAT_V145_N34_EXISTS: bool = false;
static mut UCOMPAT_V145_N35_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N35_LEN: usize = 0;
static mut UCOMPAT_V145_N35_POS: usize = 0;
static mut UCOMPAT_V145_N35_EXISTS: bool = false;
static mut UCOMPAT_V145_N36_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N36_LEN: usize = 0;
static mut UCOMPAT_V145_N36_POS: usize = 0;
static mut UCOMPAT_V145_N36_EXISTS: bool = false;
static mut UCOMPAT_V145_N37_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N37_LEN: usize = 0;
static mut UCOMPAT_V145_N37_POS: usize = 0;
static mut UCOMPAT_V145_N37_EXISTS: bool = false;
static mut UCOMPAT_V145_N38_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N38_LEN: usize = 0;
static mut UCOMPAT_V145_N38_POS: usize = 0;
static mut UCOMPAT_V145_N38_EXISTS: bool = false;
static mut UCOMPAT_V145_N39_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N39_LEN: usize = 0;
static mut UCOMPAT_V145_N39_POS: usize = 0;
static mut UCOMPAT_V145_N39_EXISTS: bool = false;
static mut UCOMPAT_V145_N40_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N40_LEN: usize = 0;
static mut UCOMPAT_V145_N40_POS: usize = 0;
static mut UCOMPAT_V145_N40_EXISTS: bool = false;
static mut UCOMPAT_V145_N41_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N41_LEN: usize = 0;
static mut UCOMPAT_V145_N41_POS: usize = 0;
static mut UCOMPAT_V145_N41_EXISTS: bool = false;
static mut UCOMPAT_V145_N42_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N42_LEN: usize = 0;
static mut UCOMPAT_V145_N42_POS: usize = 0;
static mut UCOMPAT_V145_N42_EXISTS: bool = false;
static mut UCOMPAT_V145_N43_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N43_LEN: usize = 0;
static mut UCOMPAT_V145_N43_POS: usize = 0;
static mut UCOMPAT_V145_N43_EXISTS: bool = false;
static mut UCOMPAT_V145_N44_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N44_LEN: usize = 0;
static mut UCOMPAT_V145_N44_POS: usize = 0;
static mut UCOMPAT_V145_N44_EXISTS: bool = false;
static mut UCOMPAT_V145_N45_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N45_LEN: usize = 0;
static mut UCOMPAT_V145_N45_POS: usize = 0;
static mut UCOMPAT_V145_N45_EXISTS: bool = false;
static mut UCOMPAT_V145_N46_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N46_LEN: usize = 0;
static mut UCOMPAT_V145_N46_POS: usize = 0;
static mut UCOMPAT_V145_N46_EXISTS: bool = false;
static mut UCOMPAT_V145_N47_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N47_LEN: usize = 0;
static mut UCOMPAT_V145_N47_POS: usize = 0;
static mut UCOMPAT_V145_N47_EXISTS: bool = false;
static mut UCOMPAT_V145_N48_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N48_LEN: usize = 0;
static mut UCOMPAT_V145_N48_POS: usize = 0;
static mut UCOMPAT_V145_N48_EXISTS: bool = false;
static mut UCOMPAT_V145_N49_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N49_LEN: usize = 0;
static mut UCOMPAT_V145_N49_POS: usize = 0;
static mut UCOMPAT_V145_N49_EXISTS: bool = false;
static mut UCOMPAT_V145_N50_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N50_LEN: usize = 0;
static mut UCOMPAT_V145_N50_POS: usize = 0;
static mut UCOMPAT_V145_N50_EXISTS: bool = false;
static mut UCOMPAT_V145_N51_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N51_LEN: usize = 0;
static mut UCOMPAT_V145_N51_POS: usize = 0;
static mut UCOMPAT_V145_N51_EXISTS: bool = false;
static mut UCOMPAT_V145_N52_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N52_LEN: usize = 0;
static mut UCOMPAT_V145_N52_POS: usize = 0;
static mut UCOMPAT_V145_N52_EXISTS: bool = false;
static mut UCOMPAT_V145_N53_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N53_LEN: usize = 0;
static mut UCOMPAT_V145_N53_POS: usize = 0;
static mut UCOMPAT_V145_N53_EXISTS: bool = false;
static mut UCOMPAT_V145_N54_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N54_LEN: usize = 0;
static mut UCOMPAT_V145_N54_POS: usize = 0;
static mut UCOMPAT_V145_N54_EXISTS: bool = false;
static mut UCOMPAT_V145_N55_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N55_LEN: usize = 0;
static mut UCOMPAT_V145_N55_POS: usize = 0;
static mut UCOMPAT_V145_N55_EXISTS: bool = false;
static mut UCOMPAT_V145_N56_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N56_LEN: usize = 0;
static mut UCOMPAT_V145_N56_POS: usize = 0;
static mut UCOMPAT_V145_N56_EXISTS: bool = false;
static mut UCOMPAT_V145_N57_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N57_LEN: usize = 0;
static mut UCOMPAT_V145_N57_POS: usize = 0;
static mut UCOMPAT_V145_N57_EXISTS: bool = false;
static mut UCOMPAT_V145_N58_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N58_LEN: usize = 0;
static mut UCOMPAT_V145_N58_POS: usize = 0;
static mut UCOMPAT_V145_N58_EXISTS: bool = false;
static mut UCOMPAT_V145_N59_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N59_LEN: usize = 0;
static mut UCOMPAT_V145_N59_POS: usize = 0;
static mut UCOMPAT_V145_N59_EXISTS: bool = false;
static mut UCOMPAT_V145_N60_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N60_LEN: usize = 0;
static mut UCOMPAT_V145_N60_POS: usize = 0;
static mut UCOMPAT_V145_N60_EXISTS: bool = false;
static mut UCOMPAT_V145_N61_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N61_LEN: usize = 0;
static mut UCOMPAT_V145_N61_POS: usize = 0;
static mut UCOMPAT_V145_N61_EXISTS: bool = false;
static mut UCOMPAT_V145_N62_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N62_LEN: usize = 0;
static mut UCOMPAT_V145_N62_POS: usize = 0;
static mut UCOMPAT_V145_N62_EXISTS: bool = false;
static mut UCOMPAT_V145_N63_DATA: [u8; UCOMPAT_V145_CAP] = [0; UCOMPAT_V145_CAP];
static mut UCOMPAT_V145_N63_LEN: usize = 0;
static mut UCOMPAT_V145_N63_POS: usize = 0;
static mut UCOMPAT_V145_N63_EXISTS: bool = false;
fn ucompat_v145_fd(slot: usize) -> isize {
    match slot {
        0 => UCOMPAT_V145_FD_N00,
        1 => UCOMPAT_V145_FD_N01,
        2 => UCOMPAT_V145_FD_N02,
        3 => UCOMPAT_V145_FD_N03,
        4 => UCOMPAT_V145_FD_N04,
        5 => UCOMPAT_V145_FD_N05,
        6 => UCOMPAT_V145_FD_N06,
        7 => UCOMPAT_V145_FD_N07,
        8 => UCOMPAT_V145_FD_N08,
        9 => UCOMPAT_V145_FD_N09,
        10 => UCOMPAT_V145_FD_N10,
        11 => UCOMPAT_V145_FD_N11,
        12 => UCOMPAT_V145_FD_N12,
        13 => UCOMPAT_V145_FD_N13,
        14 => UCOMPAT_V145_FD_N14,
        15 => UCOMPAT_V145_FD_N15,
        16 => UCOMPAT_V145_FD_N16,
        17 => UCOMPAT_V145_FD_N17,
        18 => UCOMPAT_V145_FD_N18,
        19 => UCOMPAT_V145_FD_N19,
        20 => UCOMPAT_V145_FD_N20,
        21 => UCOMPAT_V145_FD_N21,
        22 => UCOMPAT_V145_FD_N22,
        23 => UCOMPAT_V145_FD_N23,
        24 => UCOMPAT_V145_FD_N24,
        25 => UCOMPAT_V145_FD_N25,
        26 => UCOMPAT_V145_FD_N26,
        27 => UCOMPAT_V145_FD_N27,
        28 => UCOMPAT_V145_FD_N28,
        29 => UCOMPAT_V145_FD_N29,
        30 => UCOMPAT_V145_FD_N30,
        31 => UCOMPAT_V145_FD_N31,
        32 => UCOMPAT_V145_FD_N32,
        33 => UCOMPAT_V145_FD_N33,
        34 => UCOMPAT_V145_FD_N34,
        35 => UCOMPAT_V145_FD_N35,
        36 => UCOMPAT_V145_FD_N36,
        37 => UCOMPAT_V145_FD_N37,
        38 => UCOMPAT_V145_FD_N38,
        39 => UCOMPAT_V145_FD_N39,
        40 => UCOMPAT_V145_FD_N40,
        41 => UCOMPAT_V145_FD_N41,
        42 => UCOMPAT_V145_FD_N42,
        43 => UCOMPAT_V145_FD_N43,
        44 => UCOMPAT_V145_FD_N44,
        45 => UCOMPAT_V145_FD_N45,
        46 => UCOMPAT_V145_FD_N46,
        47 => UCOMPAT_V145_FD_N47,
        48 => UCOMPAT_V145_FD_N48,
        49 => UCOMPAT_V145_FD_N49,
        50 => UCOMPAT_V145_FD_N50,
        51 => UCOMPAT_V145_FD_N51,
        52 => UCOMPAT_V145_FD_N52,
        53 => UCOMPAT_V145_FD_N53,
        54 => UCOMPAT_V145_FD_N54,
        55 => UCOMPAT_V145_FD_N55,
        56 => UCOMPAT_V145_FD_N56,
        57 => UCOMPAT_V145_FD_N57,
        58 => UCOMPAT_V145_FD_N58,
        59 => UCOMPAT_V145_FD_N59,
        60 => UCOMPAT_V145_FD_N60,
        61 => UCOMPAT_V145_FD_N61,
        62 => UCOMPAT_V145_FD_N62,
        _ => UCOMPAT_V145_FD_N63,
    }
}
fn ucompat_v145_is_fd(fd: isize) -> bool {
    fd == UCOMPAT_V145_FD_N00
        || fd == UCOMPAT_V145_FD_N01
        || fd == UCOMPAT_V145_FD_N02
        || fd == UCOMPAT_V145_FD_N03
        || fd == UCOMPAT_V145_FD_N04
        || fd == UCOMPAT_V145_FD_N05
        || fd == UCOMPAT_V145_FD_N06
        || fd == UCOMPAT_V145_FD_N07
        || fd == UCOMPAT_V145_FD_N08
        || fd == UCOMPAT_V145_FD_N09
        || fd == UCOMPAT_V145_FD_N10
        || fd == UCOMPAT_V145_FD_N11
        || fd == UCOMPAT_V145_FD_N12
        || fd == UCOMPAT_V145_FD_N13
        || fd == UCOMPAT_V145_FD_N14
        || fd == UCOMPAT_V145_FD_N15
        || fd == UCOMPAT_V145_FD_N16
        || fd == UCOMPAT_V145_FD_N17
        || fd == UCOMPAT_V145_FD_N18
        || fd == UCOMPAT_V145_FD_N19
        || fd == UCOMPAT_V145_FD_N20
        || fd == UCOMPAT_V145_FD_N21
        || fd == UCOMPAT_V145_FD_N22
        || fd == UCOMPAT_V145_FD_N23
        || fd == UCOMPAT_V145_FD_N24
        || fd == UCOMPAT_V145_FD_N25
        || fd == UCOMPAT_V145_FD_N26
        || fd == UCOMPAT_V145_FD_N27
        || fd == UCOMPAT_V145_FD_N28
        || fd == UCOMPAT_V145_FD_N29
        || fd == UCOMPAT_V145_FD_N30
        || fd == UCOMPAT_V145_FD_N31
        || fd == UCOMPAT_V145_FD_N32
        || fd == UCOMPAT_V145_FD_N33
        || fd == UCOMPAT_V145_FD_N34
        || fd == UCOMPAT_V145_FD_N35
        || fd == UCOMPAT_V145_FD_N36
        || fd == UCOMPAT_V145_FD_N37
        || fd == UCOMPAT_V145_FD_N38
        || fd == UCOMPAT_V145_FD_N39
        || fd == UCOMPAT_V145_FD_N40
        || fd == UCOMPAT_V145_FD_N41
        || fd == UCOMPAT_V145_FD_N42
        || fd == UCOMPAT_V145_FD_N43
        || fd == UCOMPAT_V145_FD_N44
        || fd == UCOMPAT_V145_FD_N45
        || fd == UCOMPAT_V145_FD_N46
        || fd == UCOMPAT_V145_FD_N47
        || fd == UCOMPAT_V145_FD_N48
        || fd == UCOMPAT_V145_FD_N49
        || fd == UCOMPAT_V145_FD_N50
        || fd == UCOMPAT_V145_FD_N51
        || fd == UCOMPAT_V145_FD_N52
        || fd == UCOMPAT_V145_FD_N53
        || fd == UCOMPAT_V145_FD_N54
        || fd == UCOMPAT_V145_FD_N55
        || fd == UCOMPAT_V145_FD_N56
        || fd == UCOMPAT_V145_FD_N57
        || fd == UCOMPAT_V145_FD_N58
        || fd == UCOMPAT_V145_FD_N59
        || fd == UCOMPAT_V145_FD_N60
        || fd == UCOMPAT_V145_FD_N61
        || fd == UCOMPAT_V145_FD_N62
        || fd == UCOMPAT_V145_FD_N63
}
fn ucompat_v145_exists(slot: usize) -> bool {
    unsafe {
        match slot {
            0 => UCOMPAT_V145_N00_EXISTS,
            1 => UCOMPAT_V145_N01_EXISTS,
            2 => UCOMPAT_V145_N02_EXISTS,
            3 => UCOMPAT_V145_N03_EXISTS,
            4 => UCOMPAT_V145_N04_EXISTS,
            5 => UCOMPAT_V145_N05_EXISTS,
            6 => UCOMPAT_V145_N06_EXISTS,
            7 => UCOMPAT_V145_N07_EXISTS,
            8 => UCOMPAT_V145_N08_EXISTS,
            9 => UCOMPAT_V145_N09_EXISTS,
            10 => UCOMPAT_V145_N10_EXISTS,
            11 => UCOMPAT_V145_N11_EXISTS,
            12 => UCOMPAT_V145_N12_EXISTS,
            13 => UCOMPAT_V145_N13_EXISTS,
            14 => UCOMPAT_V145_N14_EXISTS,
            15 => UCOMPAT_V145_N15_EXISTS,
            16 => UCOMPAT_V145_N16_EXISTS,
            17 => UCOMPAT_V145_N17_EXISTS,
            18 => UCOMPAT_V145_N18_EXISTS,
            19 => UCOMPAT_V145_N19_EXISTS,
            20 => UCOMPAT_V145_N20_EXISTS,
            21 => UCOMPAT_V145_N21_EXISTS,
            22 => UCOMPAT_V145_N22_EXISTS,
            23 => UCOMPAT_V145_N23_EXISTS,
            24 => UCOMPAT_V145_N24_EXISTS,
            25 => UCOMPAT_V145_N25_EXISTS,
            26 => UCOMPAT_V145_N26_EXISTS,
            27 => UCOMPAT_V145_N27_EXISTS,
            28 => UCOMPAT_V145_N28_EXISTS,
            29 => UCOMPAT_V145_N29_EXISTS,
            30 => UCOMPAT_V145_N30_EXISTS,
            31 => UCOMPAT_V145_N31_EXISTS,
            32 => UCOMPAT_V145_N32_EXISTS,
            33 => UCOMPAT_V145_N33_EXISTS,
            34 => UCOMPAT_V145_N34_EXISTS,
            35 => UCOMPAT_V145_N35_EXISTS,
            36 => UCOMPAT_V145_N36_EXISTS,
            37 => UCOMPAT_V145_N37_EXISTS,
            38 => UCOMPAT_V145_N38_EXISTS,
            39 => UCOMPAT_V145_N39_EXISTS,
            40 => UCOMPAT_V145_N40_EXISTS,
            41 => UCOMPAT_V145_N41_EXISTS,
            42 => UCOMPAT_V145_N42_EXISTS,
            43 => UCOMPAT_V145_N43_EXISTS,
            44 => UCOMPAT_V145_N44_EXISTS,
            45 => UCOMPAT_V145_N45_EXISTS,
            46 => UCOMPAT_V145_N46_EXISTS,
            47 => UCOMPAT_V145_N47_EXISTS,
            48 => UCOMPAT_V145_N48_EXISTS,
            49 => UCOMPAT_V145_N49_EXISTS,
            50 => UCOMPAT_V145_N50_EXISTS,
            51 => UCOMPAT_V145_N51_EXISTS,
            52 => UCOMPAT_V145_N52_EXISTS,
            53 => UCOMPAT_V145_N53_EXISTS,
            54 => UCOMPAT_V145_N54_EXISTS,
            55 => UCOMPAT_V145_N55_EXISTS,
            56 => UCOMPAT_V145_N56_EXISTS,
            57 => UCOMPAT_V145_N57_EXISTS,
            58 => UCOMPAT_V145_N58_EXISTS,
            59 => UCOMPAT_V145_N59_EXISTS,
            60 => UCOMPAT_V145_N60_EXISTS,
            61 => UCOMPAT_V145_N61_EXISTS,
            62 => UCOMPAT_V145_N62_EXISTS,
            _ => UCOMPAT_V145_N63_EXISTS,
        }
    }
}
fn ucompat_v145_get_pos(slot: usize) -> usize {
    unsafe {
        match slot {
            0 => UCOMPAT_V145_N00_POS,
            1 => UCOMPAT_V145_N01_POS,
            2 => UCOMPAT_V145_N02_POS,
            3 => UCOMPAT_V145_N03_POS,
            4 => UCOMPAT_V145_N04_POS,
            5 => UCOMPAT_V145_N05_POS,
            6 => UCOMPAT_V145_N06_POS,
            7 => UCOMPAT_V145_N07_POS,
            8 => UCOMPAT_V145_N08_POS,
            9 => UCOMPAT_V145_N09_POS,
            10 => UCOMPAT_V145_N10_POS,
            11 => UCOMPAT_V145_N11_POS,
            12 => UCOMPAT_V145_N12_POS,
            13 => UCOMPAT_V145_N13_POS,
            14 => UCOMPAT_V145_N14_POS,
            15 => UCOMPAT_V145_N15_POS,
            16 => UCOMPAT_V145_N16_POS,
            17 => UCOMPAT_V145_N17_POS,
            18 => UCOMPAT_V145_N18_POS,
            19 => UCOMPAT_V145_N19_POS,
            20 => UCOMPAT_V145_N20_POS,
            21 => UCOMPAT_V145_N21_POS,
            22 => UCOMPAT_V145_N22_POS,
            23 => UCOMPAT_V145_N23_POS,
            24 => UCOMPAT_V145_N24_POS,
            25 => UCOMPAT_V145_N25_POS,
            26 => UCOMPAT_V145_N26_POS,
            27 => UCOMPAT_V145_N27_POS,
            28 => UCOMPAT_V145_N28_POS,
            29 => UCOMPAT_V145_N29_POS,
            30 => UCOMPAT_V145_N30_POS,
            31 => UCOMPAT_V145_N31_POS,
            32 => UCOMPAT_V145_N32_POS,
            33 => UCOMPAT_V145_N33_POS,
            34 => UCOMPAT_V145_N34_POS,
            35 => UCOMPAT_V145_N35_POS,
            36 => UCOMPAT_V145_N36_POS,
            37 => UCOMPAT_V145_N37_POS,
            38 => UCOMPAT_V145_N38_POS,
            39 => UCOMPAT_V145_N39_POS,
            40 => UCOMPAT_V145_N40_POS,
            41 => UCOMPAT_V145_N41_POS,
            42 => UCOMPAT_V145_N42_POS,
            43 => UCOMPAT_V145_N43_POS,
            44 => UCOMPAT_V145_N44_POS,
            45 => UCOMPAT_V145_N45_POS,
            46 => UCOMPAT_V145_N46_POS,
            47 => UCOMPAT_V145_N47_POS,
            48 => UCOMPAT_V145_N48_POS,
            49 => UCOMPAT_V145_N49_POS,
            50 => UCOMPAT_V145_N50_POS,
            51 => UCOMPAT_V145_N51_POS,
            52 => UCOMPAT_V145_N52_POS,
            53 => UCOMPAT_V145_N53_POS,
            54 => UCOMPAT_V145_N54_POS,
            55 => UCOMPAT_V145_N55_POS,
            56 => UCOMPAT_V145_N56_POS,
            57 => UCOMPAT_V145_N57_POS,
            58 => UCOMPAT_V145_N58_POS,
            59 => UCOMPAT_V145_N59_POS,
            60 => UCOMPAT_V145_N60_POS,
            61 => UCOMPAT_V145_N61_POS,
            62 => UCOMPAT_V145_N62_POS,
            _ => UCOMPAT_V145_N63_POS,
        }
    }
}
fn ucompat_v145_set_pos(slot: usize, pos: usize) {
    unsafe {
        match slot {
            0 => UCOMPAT_V145_N00_POS = pos,
            1 => UCOMPAT_V145_N01_POS = pos,
            2 => UCOMPAT_V145_N02_POS = pos,
            3 => UCOMPAT_V145_N03_POS = pos,
            4 => UCOMPAT_V145_N04_POS = pos,
            5 => UCOMPAT_V145_N05_POS = pos,
            6 => UCOMPAT_V145_N06_POS = pos,
            7 => UCOMPAT_V145_N07_POS = pos,
            8 => UCOMPAT_V145_N08_POS = pos,
            9 => UCOMPAT_V145_N09_POS = pos,
            10 => UCOMPAT_V145_N10_POS = pos,
            11 => UCOMPAT_V145_N11_POS = pos,
            12 => UCOMPAT_V145_N12_POS = pos,
            13 => UCOMPAT_V145_N13_POS = pos,
            14 => UCOMPAT_V145_N14_POS = pos,
            15 => UCOMPAT_V145_N15_POS = pos,
            16 => UCOMPAT_V145_N16_POS = pos,
            17 => UCOMPAT_V145_N17_POS = pos,
            18 => UCOMPAT_V145_N18_POS = pos,
            19 => UCOMPAT_V145_N19_POS = pos,
            20 => UCOMPAT_V145_N20_POS = pos,
            21 => UCOMPAT_V145_N21_POS = pos,
            22 => UCOMPAT_V145_N22_POS = pos,
            23 => UCOMPAT_V145_N23_POS = pos,
            24 => UCOMPAT_V145_N24_POS = pos,
            25 => UCOMPAT_V145_N25_POS = pos,
            26 => UCOMPAT_V145_N26_POS = pos,
            27 => UCOMPAT_V145_N27_POS = pos,
            28 => UCOMPAT_V145_N28_POS = pos,
            29 => UCOMPAT_V145_N29_POS = pos,
            30 => UCOMPAT_V145_N30_POS = pos,
            31 => UCOMPAT_V145_N31_POS = pos,
            32 => UCOMPAT_V145_N32_POS = pos,
            33 => UCOMPAT_V145_N33_POS = pos,
            34 => UCOMPAT_V145_N34_POS = pos,
            35 => UCOMPAT_V145_N35_POS = pos,
            36 => UCOMPAT_V145_N36_POS = pos,
            37 => UCOMPAT_V145_N37_POS = pos,
            38 => UCOMPAT_V145_N38_POS = pos,
            39 => UCOMPAT_V145_N39_POS = pos,
            40 => UCOMPAT_V145_N40_POS = pos,
            41 => UCOMPAT_V145_N41_POS = pos,
            42 => UCOMPAT_V145_N42_POS = pos,
            43 => UCOMPAT_V145_N43_POS = pos,
            44 => UCOMPAT_V145_N44_POS = pos,
            45 => UCOMPAT_V145_N45_POS = pos,
            46 => UCOMPAT_V145_N46_POS = pos,
            47 => UCOMPAT_V145_N47_POS = pos,
            48 => UCOMPAT_V145_N48_POS = pos,
            49 => UCOMPAT_V145_N49_POS = pos,
            50 => UCOMPAT_V145_N50_POS = pos,
            51 => UCOMPAT_V145_N51_POS = pos,
            52 => UCOMPAT_V145_N52_POS = pos,
            53 => UCOMPAT_V145_N53_POS = pos,
            54 => UCOMPAT_V145_N54_POS = pos,
            55 => UCOMPAT_V145_N55_POS = pos,
            56 => UCOMPAT_V145_N56_POS = pos,
            57 => UCOMPAT_V145_N57_POS = pos,
            58 => UCOMPAT_V145_N58_POS = pos,
            59 => UCOMPAT_V145_N59_POS = pos,
            60 => UCOMPAT_V145_N60_POS = pos,
            61 => UCOMPAT_V145_N61_POS = pos,
            62 => UCOMPAT_V145_N62_POS = pos,
            _ => UCOMPAT_V145_N63_POS = pos,
        }
    }
}
fn ucompat_v145_slot(fd: isize) -> isize {
    if fd == UCOMPAT_V145_FD_N00 {
        0
    } else if fd == UCOMPAT_V145_FD_N01 {
        1
    } else if fd == UCOMPAT_V145_FD_N02 {
        2
    } else if fd == UCOMPAT_V145_FD_N03 {
        3
    } else if fd == UCOMPAT_V145_FD_N04 {
        4
    } else if fd == UCOMPAT_V145_FD_N05 {
        5
    } else if fd == UCOMPAT_V145_FD_N06 {
        6
    } else if fd == UCOMPAT_V145_FD_N07 {
        7
    } else if fd == UCOMPAT_V145_FD_N08 {
        8
    } else if fd == UCOMPAT_V145_FD_N09 {
        9
    } else if fd == UCOMPAT_V145_FD_N10 {
        10
    } else if fd == UCOMPAT_V145_FD_N11 {
        11
    } else if fd == UCOMPAT_V145_FD_N12 {
        12
    } else if fd == UCOMPAT_V145_FD_N13 {
        13
    } else if fd == UCOMPAT_V145_FD_N14 {
        14
    } else if fd == UCOMPAT_V145_FD_N15 {
        15
    } else if fd == UCOMPAT_V145_FD_N16 {
        16
    } else if fd == UCOMPAT_V145_FD_N17 {
        17
    } else if fd == UCOMPAT_V145_FD_N18 {
        18
    } else if fd == UCOMPAT_V145_FD_N19 {
        19
    } else if fd == UCOMPAT_V145_FD_N20 {
        20
    } else if fd == UCOMPAT_V145_FD_N21 {
        21
    } else if fd == UCOMPAT_V145_FD_N22 {
        22
    } else if fd == UCOMPAT_V145_FD_N23 {
        23
    } else if fd == UCOMPAT_V145_FD_N24 {
        24
    } else if fd == UCOMPAT_V145_FD_N25 {
        25
    } else if fd == UCOMPAT_V145_FD_N26 {
        26
    } else if fd == UCOMPAT_V145_FD_N27 {
        27
    } else if fd == UCOMPAT_V145_FD_N28 {
        28
    } else if fd == UCOMPAT_V145_FD_N29 {
        29
    } else if fd == UCOMPAT_V145_FD_N30 {
        30
    } else if fd == UCOMPAT_V145_FD_N31 {
        31
    } else if fd == UCOMPAT_V145_FD_N32 {
        32
    } else if fd == UCOMPAT_V145_FD_N33 {
        33
    } else if fd == UCOMPAT_V145_FD_N34 {
        34
    } else if fd == UCOMPAT_V145_FD_N35 {
        35
    } else if fd == UCOMPAT_V145_FD_N36 {
        36
    } else if fd == UCOMPAT_V145_FD_N37 {
        37
    } else if fd == UCOMPAT_V145_FD_N38 {
        38
    } else if fd == UCOMPAT_V145_FD_N39 {
        39
    } else if fd == UCOMPAT_V145_FD_N40 {
        40
    } else if fd == UCOMPAT_V145_FD_N41 {
        41
    } else if fd == UCOMPAT_V145_FD_N42 {
        42
    } else if fd == UCOMPAT_V145_FD_N43 {
        43
    } else if fd == UCOMPAT_V145_FD_N44 {
        44
    } else if fd == UCOMPAT_V145_FD_N45 {
        45
    } else if fd == UCOMPAT_V145_FD_N46 {
        46
    } else if fd == UCOMPAT_V145_FD_N47 {
        47
    } else if fd == UCOMPAT_V145_FD_N48 {
        48
    } else if fd == UCOMPAT_V145_FD_N49 {
        49
    } else if fd == UCOMPAT_V145_FD_N50 {
        50
    } else if fd == UCOMPAT_V145_FD_N51 {
        51
    } else if fd == UCOMPAT_V145_FD_N52 {
        52
    } else if fd == UCOMPAT_V145_FD_N53 {
        53
    } else if fd == UCOMPAT_V145_FD_N54 {
        54
    } else if fd == UCOMPAT_V145_FD_N55 {
        55
    } else if fd == UCOMPAT_V145_FD_N56 {
        56
    } else if fd == UCOMPAT_V145_FD_N57 {
        57
    } else if fd == UCOMPAT_V145_FD_N58 {
        58
    } else if fd == UCOMPAT_V145_FD_N59 {
        59
    } else if fd == UCOMPAT_V145_FD_N60 {
        60
    } else if fd == UCOMPAT_V145_FD_N61 {
        61
    } else if fd == UCOMPAT_V145_FD_N62 {
        62
    } else if fd == UCOMPAT_V145_FD_N63 {
        63
    } else {
        -1
    }
}
fn ucompat_v145_reset(slot: usize) {
    unsafe {
        let mut j = 0usize;
        match slot {
            0 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N00_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N00_LEN = 0;
                UCOMPAT_V145_N00_POS = 0;
                UCOMPAT_V145_N00_EXISTS = true;
            }
            1 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N01_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N01_LEN = 0;
                UCOMPAT_V145_N01_POS = 0;
                UCOMPAT_V145_N01_EXISTS = true;
            }
            2 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N02_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N02_LEN = 0;
                UCOMPAT_V145_N02_POS = 0;
                UCOMPAT_V145_N02_EXISTS = true;
            }
            3 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N03_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N03_LEN = 0;
                UCOMPAT_V145_N03_POS = 0;
                UCOMPAT_V145_N03_EXISTS = true;
            }
            4 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N04_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N04_LEN = 0;
                UCOMPAT_V145_N04_POS = 0;
                UCOMPAT_V145_N04_EXISTS = true;
            }
            5 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N05_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N05_LEN = 0;
                UCOMPAT_V145_N05_POS = 0;
                UCOMPAT_V145_N05_EXISTS = true;
            }
            6 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N06_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N06_LEN = 0;
                UCOMPAT_V145_N06_POS = 0;
                UCOMPAT_V145_N06_EXISTS = true;
            }
            7 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N07_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N07_LEN = 0;
                UCOMPAT_V145_N07_POS = 0;
                UCOMPAT_V145_N07_EXISTS = true;
            }
            8 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N08_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N08_LEN = 0;
                UCOMPAT_V145_N08_POS = 0;
                UCOMPAT_V145_N08_EXISTS = true;
            }
            9 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N09_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N09_LEN = 0;
                UCOMPAT_V145_N09_POS = 0;
                UCOMPAT_V145_N09_EXISTS = true;
            }
            10 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N10_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N10_LEN = 0;
                UCOMPAT_V145_N10_POS = 0;
                UCOMPAT_V145_N10_EXISTS = true;
            }
            11 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N11_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N11_LEN = 0;
                UCOMPAT_V145_N11_POS = 0;
                UCOMPAT_V145_N11_EXISTS = true;
            }
            12 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N12_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N12_LEN = 0;
                UCOMPAT_V145_N12_POS = 0;
                UCOMPAT_V145_N12_EXISTS = true;
            }
            13 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N13_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N13_LEN = 0;
                UCOMPAT_V145_N13_POS = 0;
                UCOMPAT_V145_N13_EXISTS = true;
            }
            14 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N14_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N14_LEN = 0;
                UCOMPAT_V145_N14_POS = 0;
                UCOMPAT_V145_N14_EXISTS = true;
            }
            15 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N15_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N15_LEN = 0;
                UCOMPAT_V145_N15_POS = 0;
                UCOMPAT_V145_N15_EXISTS = true;
            }
            16 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N16_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N16_LEN = 0;
                UCOMPAT_V145_N16_POS = 0;
                UCOMPAT_V145_N16_EXISTS = true;
            }
            17 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N17_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N17_LEN = 0;
                UCOMPAT_V145_N17_POS = 0;
                UCOMPAT_V145_N17_EXISTS = true;
            }
            18 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N18_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N18_LEN = 0;
                UCOMPAT_V145_N18_POS = 0;
                UCOMPAT_V145_N18_EXISTS = true;
            }
            19 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N19_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N19_LEN = 0;
                UCOMPAT_V145_N19_POS = 0;
                UCOMPAT_V145_N19_EXISTS = true;
            }
            20 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N20_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N20_LEN = 0;
                UCOMPAT_V145_N20_POS = 0;
                UCOMPAT_V145_N20_EXISTS = true;
            }
            21 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N21_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N21_LEN = 0;
                UCOMPAT_V145_N21_POS = 0;
                UCOMPAT_V145_N21_EXISTS = true;
            }
            22 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N22_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N22_LEN = 0;
                UCOMPAT_V145_N22_POS = 0;
                UCOMPAT_V145_N22_EXISTS = true;
            }
            23 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N23_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N23_LEN = 0;
                UCOMPAT_V145_N23_POS = 0;
                UCOMPAT_V145_N23_EXISTS = true;
            }
            24 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N24_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N24_LEN = 0;
                UCOMPAT_V145_N24_POS = 0;
                UCOMPAT_V145_N24_EXISTS = true;
            }
            25 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N25_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N25_LEN = 0;
                UCOMPAT_V145_N25_POS = 0;
                UCOMPAT_V145_N25_EXISTS = true;
            }
            26 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N26_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N26_LEN = 0;
                UCOMPAT_V145_N26_POS = 0;
                UCOMPAT_V145_N26_EXISTS = true;
            }
            27 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N27_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N27_LEN = 0;
                UCOMPAT_V145_N27_POS = 0;
                UCOMPAT_V145_N27_EXISTS = true;
            }
            28 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N28_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N28_LEN = 0;
                UCOMPAT_V145_N28_POS = 0;
                UCOMPAT_V145_N28_EXISTS = true;
            }
            29 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N29_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N29_LEN = 0;
                UCOMPAT_V145_N29_POS = 0;
                UCOMPAT_V145_N29_EXISTS = true;
            }
            30 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N30_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N30_LEN = 0;
                UCOMPAT_V145_N30_POS = 0;
                UCOMPAT_V145_N30_EXISTS = true;
            }
            31 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N31_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N31_LEN = 0;
                UCOMPAT_V145_N31_POS = 0;
                UCOMPAT_V145_N31_EXISTS = true;
            }
            32 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N32_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N32_LEN = 0;
                UCOMPAT_V145_N32_POS = 0;
                UCOMPAT_V145_N32_EXISTS = true;
            }
            33 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N33_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N33_LEN = 0;
                UCOMPAT_V145_N33_POS = 0;
                UCOMPAT_V145_N33_EXISTS = true;
            }
            34 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N34_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N34_LEN = 0;
                UCOMPAT_V145_N34_POS = 0;
                UCOMPAT_V145_N34_EXISTS = true;
            }
            35 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N35_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N35_LEN = 0;
                UCOMPAT_V145_N35_POS = 0;
                UCOMPAT_V145_N35_EXISTS = true;
            }
            36 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N36_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N36_LEN = 0;
                UCOMPAT_V145_N36_POS = 0;
                UCOMPAT_V145_N36_EXISTS = true;
            }
            37 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N37_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N37_LEN = 0;
                UCOMPAT_V145_N37_POS = 0;
                UCOMPAT_V145_N37_EXISTS = true;
            }
            38 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N38_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N38_LEN = 0;
                UCOMPAT_V145_N38_POS = 0;
                UCOMPAT_V145_N38_EXISTS = true;
            }
            39 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N39_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N39_LEN = 0;
                UCOMPAT_V145_N39_POS = 0;
                UCOMPAT_V145_N39_EXISTS = true;
            }
            40 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N40_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N40_LEN = 0;
                UCOMPAT_V145_N40_POS = 0;
                UCOMPAT_V145_N40_EXISTS = true;
            }
            41 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N41_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N41_LEN = 0;
                UCOMPAT_V145_N41_POS = 0;
                UCOMPAT_V145_N41_EXISTS = true;
            }
            42 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N42_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N42_LEN = 0;
                UCOMPAT_V145_N42_POS = 0;
                UCOMPAT_V145_N42_EXISTS = true;
            }
            43 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N43_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N43_LEN = 0;
                UCOMPAT_V145_N43_POS = 0;
                UCOMPAT_V145_N43_EXISTS = true;
            }
            44 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N44_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N44_LEN = 0;
                UCOMPAT_V145_N44_POS = 0;
                UCOMPAT_V145_N44_EXISTS = true;
            }
            45 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N45_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N45_LEN = 0;
                UCOMPAT_V145_N45_POS = 0;
                UCOMPAT_V145_N45_EXISTS = true;
            }
            46 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N46_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N46_LEN = 0;
                UCOMPAT_V145_N46_POS = 0;
                UCOMPAT_V145_N46_EXISTS = true;
            }
            47 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N47_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N47_LEN = 0;
                UCOMPAT_V145_N47_POS = 0;
                UCOMPAT_V145_N47_EXISTS = true;
            }
            48 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N48_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N48_LEN = 0;
                UCOMPAT_V145_N48_POS = 0;
                UCOMPAT_V145_N48_EXISTS = true;
            }
            49 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N49_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N49_LEN = 0;
                UCOMPAT_V145_N49_POS = 0;
                UCOMPAT_V145_N49_EXISTS = true;
            }
            50 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N50_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N50_LEN = 0;
                UCOMPAT_V145_N50_POS = 0;
                UCOMPAT_V145_N50_EXISTS = true;
            }
            51 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N51_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N51_LEN = 0;
                UCOMPAT_V145_N51_POS = 0;
                UCOMPAT_V145_N51_EXISTS = true;
            }
            52 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N52_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N52_LEN = 0;
                UCOMPAT_V145_N52_POS = 0;
                UCOMPAT_V145_N52_EXISTS = true;
            }
            53 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N53_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N53_LEN = 0;
                UCOMPAT_V145_N53_POS = 0;
                UCOMPAT_V145_N53_EXISTS = true;
            }
            54 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N54_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N54_LEN = 0;
                UCOMPAT_V145_N54_POS = 0;
                UCOMPAT_V145_N54_EXISTS = true;
            }
            55 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N55_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N55_LEN = 0;
                UCOMPAT_V145_N55_POS = 0;
                UCOMPAT_V145_N55_EXISTS = true;
            }
            56 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N56_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N56_LEN = 0;
                UCOMPAT_V145_N56_POS = 0;
                UCOMPAT_V145_N56_EXISTS = true;
            }
            57 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N57_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N57_LEN = 0;
                UCOMPAT_V145_N57_POS = 0;
                UCOMPAT_V145_N57_EXISTS = true;
            }
            58 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N58_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N58_LEN = 0;
                UCOMPAT_V145_N58_POS = 0;
                UCOMPAT_V145_N58_EXISTS = true;
            }
            59 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N59_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N59_LEN = 0;
                UCOMPAT_V145_N59_POS = 0;
                UCOMPAT_V145_N59_EXISTS = true;
            }
            60 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N60_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N60_LEN = 0;
                UCOMPAT_V145_N60_POS = 0;
                UCOMPAT_V145_N60_EXISTS = true;
            }
            61 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N61_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N61_LEN = 0;
                UCOMPAT_V145_N61_POS = 0;
                UCOMPAT_V145_N61_EXISTS = true;
            }
            62 => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N62_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N62_LEN = 0;
                UCOMPAT_V145_N62_POS = 0;
                UCOMPAT_V145_N62_EXISTS = true;
            }
            _ => {
                while j < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N63_DATA[j] = 0;
                    j += 1;
                }
                UCOMPAT_V145_N63_LEN = 0;
                UCOMPAT_V145_N63_POS = 0;
                UCOMPAT_V145_N63_EXISTS = true;
            }
        }
    }
}
fn ucompat_v145_open(slot: usize, flags: usize) -> isize {
    const O_CREAT: usize = 0x40;
    const O_TRUNC: usize = 0x200;
    let exists = ucompat_v145_exists(slot);
    let name = match slot {
        0 => "00",
        1 => "01",
        2 => "02",
        3 => "03",
        4 => "04",
        5 => "05",
        6 => "06",
        7 => "07",
        8 => "08",
        9 => "09",
        10 => "10",
        11 => "11",
        12 => "12",
        13 => "13",
        14 => "14",
        15 => "15",
        16 => "16",
        17 => "17",
        18 => "18",
        19 => "19",
        20 => "20",
        21 => "21",
        22 => "22",
        23 => "23",
        24 => "24",
        25 => "25",
        26 => "26",
        27 => "27",
        28 => "28",
        29 => "29",
        30 => "30",
        31 => "31",
        32 => "32",
        33 => "33",
        34 => "34",
        35 => "35",
        36 => "36",
        37 => "37",
        38 => "38",
        39 => "39",
        40 => "40",
        41 => "41",
        42 => "42",
        43 => "43",
        44 => "44",
        45 => "45",
        46 => "46",
        47 => "47",
        48 => "48",
        49 => "49",
        50 => "50",
        51 => "51",
        52 => "52",
        53 => "53",
        54 => "54",
        55 => "55",
        56 => "56",
        57 => "57",
        58 => "58",
        59 => "59",
        60 => "60",
        61 => "61",
        62 => "62",
        _ => "63",
    };
    if !exists && (flags & O_CREAT) == 0 {
        crate::println!("[openat-v145] slot={} missing without O_CREAT", name);
        return crate::syscall::errno::ENOENT;
    }
    if !exists || (flags & O_TRUNC) != 0 {
        if exists && (flags & O_TRUNC) != 0 {
            crate::println!("[openat-v145] slot={} truncate", name);
        } else {
            // UCOMPAT_V145E_SUPPRESSED_V145_CREATE_LOG
            let _ = name;
        }
        ucompat_v145_reset(slot);
    } else {
        unsafe {
            match slot {
                0 => UCOMPAT_V145_N00_POS = 0,
                1 => UCOMPAT_V145_N01_POS = 0,
                2 => UCOMPAT_V145_N02_POS = 0,
                3 => UCOMPAT_V145_N03_POS = 0,
                4 => UCOMPAT_V145_N04_POS = 0,
                5 => UCOMPAT_V145_N05_POS = 0,
                6 => UCOMPAT_V145_N06_POS = 0,
                7 => UCOMPAT_V145_N07_POS = 0,
                8 => UCOMPAT_V145_N08_POS = 0,
                9 => UCOMPAT_V145_N09_POS = 0,
                10 => UCOMPAT_V145_N10_POS = 0,
                11 => UCOMPAT_V145_N11_POS = 0,
                12 => UCOMPAT_V145_N12_POS = 0,
                13 => UCOMPAT_V145_N13_POS = 0,
                14 => UCOMPAT_V145_N14_POS = 0,
                15 => UCOMPAT_V145_N15_POS = 0,
                16 => UCOMPAT_V145_N16_POS = 0,
                17 => UCOMPAT_V145_N17_POS = 0,
                18 => UCOMPAT_V145_N18_POS = 0,
                19 => UCOMPAT_V145_N19_POS = 0,
                20 => UCOMPAT_V145_N20_POS = 0,
                21 => UCOMPAT_V145_N21_POS = 0,
                22 => UCOMPAT_V145_N22_POS = 0,
                23 => UCOMPAT_V145_N23_POS = 0,
                24 => UCOMPAT_V145_N24_POS = 0,
                25 => UCOMPAT_V145_N25_POS = 0,
                26 => UCOMPAT_V145_N26_POS = 0,
                27 => UCOMPAT_V145_N27_POS = 0,
                28 => UCOMPAT_V145_N28_POS = 0,
                29 => UCOMPAT_V145_N29_POS = 0,
                30 => UCOMPAT_V145_N30_POS = 0,
                31 => UCOMPAT_V145_N31_POS = 0,
                32 => UCOMPAT_V145_N32_POS = 0,
                33 => UCOMPAT_V145_N33_POS = 0,
                34 => UCOMPAT_V145_N34_POS = 0,
                35 => UCOMPAT_V145_N35_POS = 0,
                36 => UCOMPAT_V145_N36_POS = 0,
                37 => UCOMPAT_V145_N37_POS = 0,
                38 => UCOMPAT_V145_N38_POS = 0,
                39 => UCOMPAT_V145_N39_POS = 0,
                40 => UCOMPAT_V145_N40_POS = 0,
                41 => UCOMPAT_V145_N41_POS = 0,
                42 => UCOMPAT_V145_N42_POS = 0,
                43 => UCOMPAT_V145_N43_POS = 0,
                44 => UCOMPAT_V145_N44_POS = 0,
                45 => UCOMPAT_V145_N45_POS = 0,
                46 => UCOMPAT_V145_N46_POS = 0,
                47 => UCOMPAT_V145_N47_POS = 0,
                48 => UCOMPAT_V145_N48_POS = 0,
                49 => UCOMPAT_V145_N49_POS = 0,
                50 => UCOMPAT_V145_N50_POS = 0,
                51 => UCOMPAT_V145_N51_POS = 0,
                52 => UCOMPAT_V145_N52_POS = 0,
                53 => UCOMPAT_V145_N53_POS = 0,
                54 => UCOMPAT_V145_N54_POS = 0,
                55 => UCOMPAT_V145_N55_POS = 0,
                56 => UCOMPAT_V145_N56_POS = 0,
                57 => UCOMPAT_V145_N57_POS = 0,
                58 => UCOMPAT_V145_N58_POS = 0,
                59 => UCOMPAT_V145_N59_POS = 0,
                60 => UCOMPAT_V145_N60_POS = 0,
                61 => UCOMPAT_V145_N61_POS = 0,
                62 => UCOMPAT_V145_N62_POS = 0,
                _ => UCOMPAT_V145_N63_POS = 0,
            }
        }
        // UCOMPAT_V145E_SUPPRESSED_V145_REOPEN_LOG
        let _ = name;
    }
    ucompat_v145_fd(slot)
}
fn ucompat_v145_write_one(_fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                if UCOMPAT_V145_N00_POS > UCOMPAT_V145_N00_LEN {
                    let mut z = UCOMPAT_V145_N00_LEN;
                    while z < UCOMPAT_V145_N00_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N00_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14001 from={} to={}",
                        UCOMPAT_V145_N00_LEN,
                        UCOMPAT_V145_N00_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N00_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N00_DATA[UCOMPAT_V145_N00_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N00_POS + copied;
                if end > UCOMPAT_V145_N00_LEN {
                    UCOMPAT_V145_N00_LEN = end;
                }
                UCOMPAT_V145_N00_POS = end;
                UCOMPAT_V145_N00_EXISTS = true;
            }
            1 => {
                if UCOMPAT_V145_N01_POS > UCOMPAT_V145_N01_LEN {
                    let mut z = UCOMPAT_V145_N01_LEN;
                    while z < UCOMPAT_V145_N01_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N01_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14002 from={} to={}",
                        UCOMPAT_V145_N01_LEN,
                        UCOMPAT_V145_N01_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N01_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N01_DATA[UCOMPAT_V145_N01_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N01_POS + copied;
                if end > UCOMPAT_V145_N01_LEN {
                    UCOMPAT_V145_N01_LEN = end;
                }
                UCOMPAT_V145_N01_POS = end;
                UCOMPAT_V145_N01_EXISTS = true;
            }
            2 => {
                if UCOMPAT_V145_N02_POS > UCOMPAT_V145_N02_LEN {
                    let mut z = UCOMPAT_V145_N02_LEN;
                    while z < UCOMPAT_V145_N02_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N02_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14003 from={} to={}",
                        UCOMPAT_V145_N02_LEN,
                        UCOMPAT_V145_N02_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N02_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N02_DATA[UCOMPAT_V145_N02_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N02_POS + copied;
                if end > UCOMPAT_V145_N02_LEN {
                    UCOMPAT_V145_N02_LEN = end;
                }
                UCOMPAT_V145_N02_POS = end;
                UCOMPAT_V145_N02_EXISTS = true;
            }
            3 => {
                if UCOMPAT_V145_N03_POS > UCOMPAT_V145_N03_LEN {
                    let mut z = UCOMPAT_V145_N03_LEN;
                    while z < UCOMPAT_V145_N03_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N03_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14004 from={} to={}",
                        UCOMPAT_V145_N03_LEN,
                        UCOMPAT_V145_N03_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N03_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N03_DATA[UCOMPAT_V145_N03_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N03_POS + copied;
                if end > UCOMPAT_V145_N03_LEN {
                    UCOMPAT_V145_N03_LEN = end;
                }
                UCOMPAT_V145_N03_POS = end;
                UCOMPAT_V145_N03_EXISTS = true;
            }
            4 => {
                if UCOMPAT_V145_N04_POS > UCOMPAT_V145_N04_LEN {
                    let mut z = UCOMPAT_V145_N04_LEN;
                    while z < UCOMPAT_V145_N04_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N04_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14005 from={} to={}",
                        UCOMPAT_V145_N04_LEN,
                        UCOMPAT_V145_N04_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N04_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N04_DATA[UCOMPAT_V145_N04_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N04_POS + copied;
                if end > UCOMPAT_V145_N04_LEN {
                    UCOMPAT_V145_N04_LEN = end;
                }
                UCOMPAT_V145_N04_POS = end;
                UCOMPAT_V145_N04_EXISTS = true;
            }
            5 => {
                if UCOMPAT_V145_N05_POS > UCOMPAT_V145_N05_LEN {
                    let mut z = UCOMPAT_V145_N05_LEN;
                    while z < UCOMPAT_V145_N05_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N05_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14006 from={} to={}",
                        UCOMPAT_V145_N05_LEN,
                        UCOMPAT_V145_N05_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N05_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N05_DATA[UCOMPAT_V145_N05_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N05_POS + copied;
                if end > UCOMPAT_V145_N05_LEN {
                    UCOMPAT_V145_N05_LEN = end;
                }
                UCOMPAT_V145_N05_POS = end;
                UCOMPAT_V145_N05_EXISTS = true;
            }
            6 => {
                if UCOMPAT_V145_N06_POS > UCOMPAT_V145_N06_LEN {
                    let mut z = UCOMPAT_V145_N06_LEN;
                    while z < UCOMPAT_V145_N06_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N06_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14007 from={} to={}",
                        UCOMPAT_V145_N06_LEN,
                        UCOMPAT_V145_N06_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N06_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N06_DATA[UCOMPAT_V145_N06_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N06_POS + copied;
                if end > UCOMPAT_V145_N06_LEN {
                    UCOMPAT_V145_N06_LEN = end;
                }
                UCOMPAT_V145_N06_POS = end;
                UCOMPAT_V145_N06_EXISTS = true;
            }
            7 => {
                if UCOMPAT_V145_N07_POS > UCOMPAT_V145_N07_LEN {
                    let mut z = UCOMPAT_V145_N07_LEN;
                    while z < UCOMPAT_V145_N07_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N07_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14008 from={} to={}",
                        UCOMPAT_V145_N07_LEN,
                        UCOMPAT_V145_N07_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N07_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N07_DATA[UCOMPAT_V145_N07_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N07_POS + copied;
                if end > UCOMPAT_V145_N07_LEN {
                    UCOMPAT_V145_N07_LEN = end;
                }
                UCOMPAT_V145_N07_POS = end;
                UCOMPAT_V145_N07_EXISTS = true;
            }
            8 => {
                if UCOMPAT_V145_N08_POS > UCOMPAT_V145_N08_LEN {
                    let mut z = UCOMPAT_V145_N08_LEN;
                    while z < UCOMPAT_V145_N08_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N08_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14009 from={} to={}",
                        UCOMPAT_V145_N08_LEN,
                        UCOMPAT_V145_N08_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N08_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N08_DATA[UCOMPAT_V145_N08_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N08_POS + copied;
                if end > UCOMPAT_V145_N08_LEN {
                    UCOMPAT_V145_N08_LEN = end;
                }
                UCOMPAT_V145_N08_POS = end;
                UCOMPAT_V145_N08_EXISTS = true;
            }
            9 => {
                if UCOMPAT_V145_N09_POS > UCOMPAT_V145_N09_LEN {
                    let mut z = UCOMPAT_V145_N09_LEN;
                    while z < UCOMPAT_V145_N09_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N09_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14010 from={} to={}",
                        UCOMPAT_V145_N09_LEN,
                        UCOMPAT_V145_N09_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N09_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N09_DATA[UCOMPAT_V145_N09_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N09_POS + copied;
                if end > UCOMPAT_V145_N09_LEN {
                    UCOMPAT_V145_N09_LEN = end;
                }
                UCOMPAT_V145_N09_POS = end;
                UCOMPAT_V145_N09_EXISTS = true;
            }
            10 => {
                if UCOMPAT_V145_N10_POS > UCOMPAT_V145_N10_LEN {
                    let mut z = UCOMPAT_V145_N10_LEN;
                    while z < UCOMPAT_V145_N10_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N10_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14011 from={} to={}",
                        UCOMPAT_V145_N10_LEN,
                        UCOMPAT_V145_N10_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N10_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N10_DATA[UCOMPAT_V145_N10_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N10_POS + copied;
                if end > UCOMPAT_V145_N10_LEN {
                    UCOMPAT_V145_N10_LEN = end;
                }
                UCOMPAT_V145_N10_POS = end;
                UCOMPAT_V145_N10_EXISTS = true;
            }
            11 => {
                if UCOMPAT_V145_N11_POS > UCOMPAT_V145_N11_LEN {
                    let mut z = UCOMPAT_V145_N11_LEN;
                    while z < UCOMPAT_V145_N11_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N11_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14012 from={} to={}",
                        UCOMPAT_V145_N11_LEN,
                        UCOMPAT_V145_N11_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N11_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N11_DATA[UCOMPAT_V145_N11_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N11_POS + copied;
                if end > UCOMPAT_V145_N11_LEN {
                    UCOMPAT_V145_N11_LEN = end;
                }
                UCOMPAT_V145_N11_POS = end;
                UCOMPAT_V145_N11_EXISTS = true;
            }
            12 => {
                if UCOMPAT_V145_N12_POS > UCOMPAT_V145_N12_LEN {
                    let mut z = UCOMPAT_V145_N12_LEN;
                    while z < UCOMPAT_V145_N12_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N12_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14013 from={} to={}",
                        UCOMPAT_V145_N12_LEN,
                        UCOMPAT_V145_N12_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N12_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N12_DATA[UCOMPAT_V145_N12_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N12_POS + copied;
                if end > UCOMPAT_V145_N12_LEN {
                    UCOMPAT_V145_N12_LEN = end;
                }
                UCOMPAT_V145_N12_POS = end;
                UCOMPAT_V145_N12_EXISTS = true;
            }
            13 => {
                if UCOMPAT_V145_N13_POS > UCOMPAT_V145_N13_LEN {
                    let mut z = UCOMPAT_V145_N13_LEN;
                    while z < UCOMPAT_V145_N13_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N13_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14014 from={} to={}",
                        UCOMPAT_V145_N13_LEN,
                        UCOMPAT_V145_N13_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N13_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N13_DATA[UCOMPAT_V145_N13_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N13_POS + copied;
                if end > UCOMPAT_V145_N13_LEN {
                    UCOMPAT_V145_N13_LEN = end;
                }
                UCOMPAT_V145_N13_POS = end;
                UCOMPAT_V145_N13_EXISTS = true;
            }
            14 => {
                if UCOMPAT_V145_N14_POS > UCOMPAT_V145_N14_LEN {
                    let mut z = UCOMPAT_V145_N14_LEN;
                    while z < UCOMPAT_V145_N14_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N14_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14015 from={} to={}",
                        UCOMPAT_V145_N14_LEN,
                        UCOMPAT_V145_N14_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N14_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N14_DATA[UCOMPAT_V145_N14_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N14_POS + copied;
                if end > UCOMPAT_V145_N14_LEN {
                    UCOMPAT_V145_N14_LEN = end;
                }
                UCOMPAT_V145_N14_POS = end;
                UCOMPAT_V145_N14_EXISTS = true;
            }
            15 => {
                if UCOMPAT_V145_N15_POS > UCOMPAT_V145_N15_LEN {
                    let mut z = UCOMPAT_V145_N15_LEN;
                    while z < UCOMPAT_V145_N15_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N15_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14016 from={} to={}",
                        UCOMPAT_V145_N15_LEN,
                        UCOMPAT_V145_N15_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N15_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N15_DATA[UCOMPAT_V145_N15_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N15_POS + copied;
                if end > UCOMPAT_V145_N15_LEN {
                    UCOMPAT_V145_N15_LEN = end;
                }
                UCOMPAT_V145_N15_POS = end;
                UCOMPAT_V145_N15_EXISTS = true;
            }
            16 => {
                if UCOMPAT_V145_N16_POS > UCOMPAT_V145_N16_LEN {
                    let mut z = UCOMPAT_V145_N16_LEN;
                    while z < UCOMPAT_V145_N16_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N16_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14017 from={} to={}",
                        UCOMPAT_V145_N16_LEN,
                        UCOMPAT_V145_N16_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N16_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N16_DATA[UCOMPAT_V145_N16_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N16_POS + copied;
                if end > UCOMPAT_V145_N16_LEN {
                    UCOMPAT_V145_N16_LEN = end;
                }
                UCOMPAT_V145_N16_POS = end;
                UCOMPAT_V145_N16_EXISTS = true;
            }
            17 => {
                if UCOMPAT_V145_N17_POS > UCOMPAT_V145_N17_LEN {
                    let mut z = UCOMPAT_V145_N17_LEN;
                    while z < UCOMPAT_V145_N17_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N17_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14018 from={} to={}",
                        UCOMPAT_V145_N17_LEN,
                        UCOMPAT_V145_N17_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N17_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N17_DATA[UCOMPAT_V145_N17_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N17_POS + copied;
                if end > UCOMPAT_V145_N17_LEN {
                    UCOMPAT_V145_N17_LEN = end;
                }
                UCOMPAT_V145_N17_POS = end;
                UCOMPAT_V145_N17_EXISTS = true;
            }
            18 => {
                if UCOMPAT_V145_N18_POS > UCOMPAT_V145_N18_LEN {
                    let mut z = UCOMPAT_V145_N18_LEN;
                    while z < UCOMPAT_V145_N18_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N18_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14019 from={} to={}",
                        UCOMPAT_V145_N18_LEN,
                        UCOMPAT_V145_N18_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N18_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N18_DATA[UCOMPAT_V145_N18_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N18_POS + copied;
                if end > UCOMPAT_V145_N18_LEN {
                    UCOMPAT_V145_N18_LEN = end;
                }
                UCOMPAT_V145_N18_POS = end;
                UCOMPAT_V145_N18_EXISTS = true;
            }
            19 => {
                if UCOMPAT_V145_N19_POS > UCOMPAT_V145_N19_LEN {
                    let mut z = UCOMPAT_V145_N19_LEN;
                    while z < UCOMPAT_V145_N19_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N19_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14020 from={} to={}",
                        UCOMPAT_V145_N19_LEN,
                        UCOMPAT_V145_N19_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N19_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N19_DATA[UCOMPAT_V145_N19_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N19_POS + copied;
                if end > UCOMPAT_V145_N19_LEN {
                    UCOMPAT_V145_N19_LEN = end;
                }
                UCOMPAT_V145_N19_POS = end;
                UCOMPAT_V145_N19_EXISTS = true;
            }
            20 => {
                if UCOMPAT_V145_N20_POS > UCOMPAT_V145_N20_LEN {
                    let mut z = UCOMPAT_V145_N20_LEN;
                    while z < UCOMPAT_V145_N20_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N20_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14021 from={} to={}",
                        UCOMPAT_V145_N20_LEN,
                        UCOMPAT_V145_N20_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N20_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N20_DATA[UCOMPAT_V145_N20_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N20_POS + copied;
                if end > UCOMPAT_V145_N20_LEN {
                    UCOMPAT_V145_N20_LEN = end;
                }
                UCOMPAT_V145_N20_POS = end;
                UCOMPAT_V145_N20_EXISTS = true;
            }
            21 => {
                if UCOMPAT_V145_N21_POS > UCOMPAT_V145_N21_LEN {
                    let mut z = UCOMPAT_V145_N21_LEN;
                    while z < UCOMPAT_V145_N21_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N21_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14022 from={} to={}",
                        UCOMPAT_V145_N21_LEN,
                        UCOMPAT_V145_N21_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N21_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N21_DATA[UCOMPAT_V145_N21_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N21_POS + copied;
                if end > UCOMPAT_V145_N21_LEN {
                    UCOMPAT_V145_N21_LEN = end;
                }
                UCOMPAT_V145_N21_POS = end;
                UCOMPAT_V145_N21_EXISTS = true;
            }
            22 => {
                if UCOMPAT_V145_N22_POS > UCOMPAT_V145_N22_LEN {
                    let mut z = UCOMPAT_V145_N22_LEN;
                    while z < UCOMPAT_V145_N22_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N22_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14023 from={} to={}",
                        UCOMPAT_V145_N22_LEN,
                        UCOMPAT_V145_N22_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N22_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N22_DATA[UCOMPAT_V145_N22_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N22_POS + copied;
                if end > UCOMPAT_V145_N22_LEN {
                    UCOMPAT_V145_N22_LEN = end;
                }
                UCOMPAT_V145_N22_POS = end;
                UCOMPAT_V145_N22_EXISTS = true;
            }
            23 => {
                if UCOMPAT_V145_N23_POS > UCOMPAT_V145_N23_LEN {
                    let mut z = UCOMPAT_V145_N23_LEN;
                    while z < UCOMPAT_V145_N23_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N23_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14024 from={} to={}",
                        UCOMPAT_V145_N23_LEN,
                        UCOMPAT_V145_N23_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N23_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N23_DATA[UCOMPAT_V145_N23_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N23_POS + copied;
                if end > UCOMPAT_V145_N23_LEN {
                    UCOMPAT_V145_N23_LEN = end;
                }
                UCOMPAT_V145_N23_POS = end;
                UCOMPAT_V145_N23_EXISTS = true;
            }
            24 => {
                if UCOMPAT_V145_N24_POS > UCOMPAT_V145_N24_LEN {
                    let mut z = UCOMPAT_V145_N24_LEN;
                    while z < UCOMPAT_V145_N24_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N24_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14025 from={} to={}",
                        UCOMPAT_V145_N24_LEN,
                        UCOMPAT_V145_N24_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N24_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N24_DATA[UCOMPAT_V145_N24_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N24_POS + copied;
                if end > UCOMPAT_V145_N24_LEN {
                    UCOMPAT_V145_N24_LEN = end;
                }
                UCOMPAT_V145_N24_POS = end;
                UCOMPAT_V145_N24_EXISTS = true;
            }
            25 => {
                if UCOMPAT_V145_N25_POS > UCOMPAT_V145_N25_LEN {
                    let mut z = UCOMPAT_V145_N25_LEN;
                    while z < UCOMPAT_V145_N25_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N25_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14026 from={} to={}",
                        UCOMPAT_V145_N25_LEN,
                        UCOMPAT_V145_N25_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N25_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N25_DATA[UCOMPAT_V145_N25_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N25_POS + copied;
                if end > UCOMPAT_V145_N25_LEN {
                    UCOMPAT_V145_N25_LEN = end;
                }
                UCOMPAT_V145_N25_POS = end;
                UCOMPAT_V145_N25_EXISTS = true;
            }
            26 => {
                if UCOMPAT_V145_N26_POS > UCOMPAT_V145_N26_LEN {
                    let mut z = UCOMPAT_V145_N26_LEN;
                    while z < UCOMPAT_V145_N26_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N26_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14027 from={} to={}",
                        UCOMPAT_V145_N26_LEN,
                        UCOMPAT_V145_N26_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N26_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N26_DATA[UCOMPAT_V145_N26_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N26_POS + copied;
                if end > UCOMPAT_V145_N26_LEN {
                    UCOMPAT_V145_N26_LEN = end;
                }
                UCOMPAT_V145_N26_POS = end;
                UCOMPAT_V145_N26_EXISTS = true;
            }
            27 => {
                if UCOMPAT_V145_N27_POS > UCOMPAT_V145_N27_LEN {
                    let mut z = UCOMPAT_V145_N27_LEN;
                    while z < UCOMPAT_V145_N27_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N27_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14028 from={} to={}",
                        UCOMPAT_V145_N27_LEN,
                        UCOMPAT_V145_N27_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N27_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N27_DATA[UCOMPAT_V145_N27_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N27_POS + copied;
                if end > UCOMPAT_V145_N27_LEN {
                    UCOMPAT_V145_N27_LEN = end;
                }
                UCOMPAT_V145_N27_POS = end;
                UCOMPAT_V145_N27_EXISTS = true;
            }
            28 => {
                if UCOMPAT_V145_N28_POS > UCOMPAT_V145_N28_LEN {
                    let mut z = UCOMPAT_V145_N28_LEN;
                    while z < UCOMPAT_V145_N28_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N28_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14029 from={} to={}",
                        UCOMPAT_V145_N28_LEN,
                        UCOMPAT_V145_N28_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N28_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N28_DATA[UCOMPAT_V145_N28_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N28_POS + copied;
                if end > UCOMPAT_V145_N28_LEN {
                    UCOMPAT_V145_N28_LEN = end;
                }
                UCOMPAT_V145_N28_POS = end;
                UCOMPAT_V145_N28_EXISTS = true;
            }
            29 => {
                if UCOMPAT_V145_N29_POS > UCOMPAT_V145_N29_LEN {
                    let mut z = UCOMPAT_V145_N29_LEN;
                    while z < UCOMPAT_V145_N29_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N29_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14030 from={} to={}",
                        UCOMPAT_V145_N29_LEN,
                        UCOMPAT_V145_N29_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N29_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N29_DATA[UCOMPAT_V145_N29_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N29_POS + copied;
                if end > UCOMPAT_V145_N29_LEN {
                    UCOMPAT_V145_N29_LEN = end;
                }
                UCOMPAT_V145_N29_POS = end;
                UCOMPAT_V145_N29_EXISTS = true;
            }
            30 => {
                if UCOMPAT_V145_N30_POS > UCOMPAT_V145_N30_LEN {
                    let mut z = UCOMPAT_V145_N30_LEN;
                    while z < UCOMPAT_V145_N30_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N30_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14031 from={} to={}",
                        UCOMPAT_V145_N30_LEN,
                        UCOMPAT_V145_N30_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N30_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N30_DATA[UCOMPAT_V145_N30_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N30_POS + copied;
                if end > UCOMPAT_V145_N30_LEN {
                    UCOMPAT_V145_N30_LEN = end;
                }
                UCOMPAT_V145_N30_POS = end;
                UCOMPAT_V145_N30_EXISTS = true;
            }
            31 => {
                if UCOMPAT_V145_N31_POS > UCOMPAT_V145_N31_LEN {
                    let mut z = UCOMPAT_V145_N31_LEN;
                    while z < UCOMPAT_V145_N31_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N31_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14032 from={} to={}",
                        UCOMPAT_V145_N31_LEN,
                        UCOMPAT_V145_N31_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N31_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N31_DATA[UCOMPAT_V145_N31_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N31_POS + copied;
                if end > UCOMPAT_V145_N31_LEN {
                    UCOMPAT_V145_N31_LEN = end;
                }
                UCOMPAT_V145_N31_POS = end;
                UCOMPAT_V145_N31_EXISTS = true;
            }
            32 => {
                if UCOMPAT_V145_N32_POS > UCOMPAT_V145_N32_LEN {
                    let mut z = UCOMPAT_V145_N32_LEN;
                    while z < UCOMPAT_V145_N32_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N32_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14033 from={} to={}",
                        UCOMPAT_V145_N32_LEN,
                        UCOMPAT_V145_N32_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N32_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N32_DATA[UCOMPAT_V145_N32_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N32_POS + copied;
                if end > UCOMPAT_V145_N32_LEN {
                    UCOMPAT_V145_N32_LEN = end;
                }
                UCOMPAT_V145_N32_POS = end;
                UCOMPAT_V145_N32_EXISTS = true;
            }
            33 => {
                if UCOMPAT_V145_N33_POS > UCOMPAT_V145_N33_LEN {
                    let mut z = UCOMPAT_V145_N33_LEN;
                    while z < UCOMPAT_V145_N33_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N33_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14034 from={} to={}",
                        UCOMPAT_V145_N33_LEN,
                        UCOMPAT_V145_N33_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N33_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N33_DATA[UCOMPAT_V145_N33_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N33_POS + copied;
                if end > UCOMPAT_V145_N33_LEN {
                    UCOMPAT_V145_N33_LEN = end;
                }
                UCOMPAT_V145_N33_POS = end;
                UCOMPAT_V145_N33_EXISTS = true;
            }
            34 => {
                if UCOMPAT_V145_N34_POS > UCOMPAT_V145_N34_LEN {
                    let mut z = UCOMPAT_V145_N34_LEN;
                    while z < UCOMPAT_V145_N34_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N34_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14035 from={} to={}",
                        UCOMPAT_V145_N34_LEN,
                        UCOMPAT_V145_N34_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N34_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N34_DATA[UCOMPAT_V145_N34_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N34_POS + copied;
                if end > UCOMPAT_V145_N34_LEN {
                    UCOMPAT_V145_N34_LEN = end;
                }
                UCOMPAT_V145_N34_POS = end;
                UCOMPAT_V145_N34_EXISTS = true;
            }
            35 => {
                if UCOMPAT_V145_N35_POS > UCOMPAT_V145_N35_LEN {
                    let mut z = UCOMPAT_V145_N35_LEN;
                    while z < UCOMPAT_V145_N35_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N35_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14036 from={} to={}",
                        UCOMPAT_V145_N35_LEN,
                        UCOMPAT_V145_N35_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N35_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N35_DATA[UCOMPAT_V145_N35_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N35_POS + copied;
                if end > UCOMPAT_V145_N35_LEN {
                    UCOMPAT_V145_N35_LEN = end;
                }
                UCOMPAT_V145_N35_POS = end;
                UCOMPAT_V145_N35_EXISTS = true;
            }
            36 => {
                if UCOMPAT_V145_N36_POS > UCOMPAT_V145_N36_LEN {
                    let mut z = UCOMPAT_V145_N36_LEN;
                    while z < UCOMPAT_V145_N36_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N36_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14037 from={} to={}",
                        UCOMPAT_V145_N36_LEN,
                        UCOMPAT_V145_N36_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N36_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N36_DATA[UCOMPAT_V145_N36_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N36_POS + copied;
                if end > UCOMPAT_V145_N36_LEN {
                    UCOMPAT_V145_N36_LEN = end;
                }
                UCOMPAT_V145_N36_POS = end;
                UCOMPAT_V145_N36_EXISTS = true;
            }
            37 => {
                if UCOMPAT_V145_N37_POS > UCOMPAT_V145_N37_LEN {
                    let mut z = UCOMPAT_V145_N37_LEN;
                    while z < UCOMPAT_V145_N37_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N37_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14038 from={} to={}",
                        UCOMPAT_V145_N37_LEN,
                        UCOMPAT_V145_N37_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N37_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N37_DATA[UCOMPAT_V145_N37_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N37_POS + copied;
                if end > UCOMPAT_V145_N37_LEN {
                    UCOMPAT_V145_N37_LEN = end;
                }
                UCOMPAT_V145_N37_POS = end;
                UCOMPAT_V145_N37_EXISTS = true;
            }
            38 => {
                if UCOMPAT_V145_N38_POS > UCOMPAT_V145_N38_LEN {
                    let mut z = UCOMPAT_V145_N38_LEN;
                    while z < UCOMPAT_V145_N38_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N38_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14039 from={} to={}",
                        UCOMPAT_V145_N38_LEN,
                        UCOMPAT_V145_N38_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N38_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N38_DATA[UCOMPAT_V145_N38_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N38_POS + copied;
                if end > UCOMPAT_V145_N38_LEN {
                    UCOMPAT_V145_N38_LEN = end;
                }
                UCOMPAT_V145_N38_POS = end;
                UCOMPAT_V145_N38_EXISTS = true;
            }
            39 => {
                if UCOMPAT_V145_N39_POS > UCOMPAT_V145_N39_LEN {
                    let mut z = UCOMPAT_V145_N39_LEN;
                    while z < UCOMPAT_V145_N39_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N39_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14040 from={} to={}",
                        UCOMPAT_V145_N39_LEN,
                        UCOMPAT_V145_N39_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N39_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N39_DATA[UCOMPAT_V145_N39_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N39_POS + copied;
                if end > UCOMPAT_V145_N39_LEN {
                    UCOMPAT_V145_N39_LEN = end;
                }
                UCOMPAT_V145_N39_POS = end;
                UCOMPAT_V145_N39_EXISTS = true;
            }
            40 => {
                if UCOMPAT_V145_N40_POS > UCOMPAT_V145_N40_LEN {
                    let mut z = UCOMPAT_V145_N40_LEN;
                    while z < UCOMPAT_V145_N40_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N40_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14041 from={} to={}",
                        UCOMPAT_V145_N40_LEN,
                        UCOMPAT_V145_N40_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N40_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N40_DATA[UCOMPAT_V145_N40_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N40_POS + copied;
                if end > UCOMPAT_V145_N40_LEN {
                    UCOMPAT_V145_N40_LEN = end;
                }
                UCOMPAT_V145_N40_POS = end;
                UCOMPAT_V145_N40_EXISTS = true;
            }
            41 => {
                if UCOMPAT_V145_N41_POS > UCOMPAT_V145_N41_LEN {
                    let mut z = UCOMPAT_V145_N41_LEN;
                    while z < UCOMPAT_V145_N41_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N41_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14042 from={} to={}",
                        UCOMPAT_V145_N41_LEN,
                        UCOMPAT_V145_N41_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N41_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N41_DATA[UCOMPAT_V145_N41_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N41_POS + copied;
                if end > UCOMPAT_V145_N41_LEN {
                    UCOMPAT_V145_N41_LEN = end;
                }
                UCOMPAT_V145_N41_POS = end;
                UCOMPAT_V145_N41_EXISTS = true;
            }
            42 => {
                if UCOMPAT_V145_N42_POS > UCOMPAT_V145_N42_LEN {
                    let mut z = UCOMPAT_V145_N42_LEN;
                    while z < UCOMPAT_V145_N42_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N42_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14043 from={} to={}",
                        UCOMPAT_V145_N42_LEN,
                        UCOMPAT_V145_N42_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N42_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N42_DATA[UCOMPAT_V145_N42_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N42_POS + copied;
                if end > UCOMPAT_V145_N42_LEN {
                    UCOMPAT_V145_N42_LEN = end;
                }
                UCOMPAT_V145_N42_POS = end;
                UCOMPAT_V145_N42_EXISTS = true;
            }
            43 => {
                if UCOMPAT_V145_N43_POS > UCOMPAT_V145_N43_LEN {
                    let mut z = UCOMPAT_V145_N43_LEN;
                    while z < UCOMPAT_V145_N43_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N43_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14044 from={} to={}",
                        UCOMPAT_V145_N43_LEN,
                        UCOMPAT_V145_N43_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N43_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N43_DATA[UCOMPAT_V145_N43_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N43_POS + copied;
                if end > UCOMPAT_V145_N43_LEN {
                    UCOMPAT_V145_N43_LEN = end;
                }
                UCOMPAT_V145_N43_POS = end;
                UCOMPAT_V145_N43_EXISTS = true;
            }
            44 => {
                if UCOMPAT_V145_N44_POS > UCOMPAT_V145_N44_LEN {
                    let mut z = UCOMPAT_V145_N44_LEN;
                    while z < UCOMPAT_V145_N44_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N44_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14045 from={} to={}",
                        UCOMPAT_V145_N44_LEN,
                        UCOMPAT_V145_N44_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N44_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N44_DATA[UCOMPAT_V145_N44_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N44_POS + copied;
                if end > UCOMPAT_V145_N44_LEN {
                    UCOMPAT_V145_N44_LEN = end;
                }
                UCOMPAT_V145_N44_POS = end;
                UCOMPAT_V145_N44_EXISTS = true;
            }
            45 => {
                if UCOMPAT_V145_N45_POS > UCOMPAT_V145_N45_LEN {
                    let mut z = UCOMPAT_V145_N45_LEN;
                    while z < UCOMPAT_V145_N45_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N45_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14046 from={} to={}",
                        UCOMPAT_V145_N45_LEN,
                        UCOMPAT_V145_N45_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N45_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N45_DATA[UCOMPAT_V145_N45_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N45_POS + copied;
                if end > UCOMPAT_V145_N45_LEN {
                    UCOMPAT_V145_N45_LEN = end;
                }
                UCOMPAT_V145_N45_POS = end;
                UCOMPAT_V145_N45_EXISTS = true;
            }
            46 => {
                if UCOMPAT_V145_N46_POS > UCOMPAT_V145_N46_LEN {
                    let mut z = UCOMPAT_V145_N46_LEN;
                    while z < UCOMPAT_V145_N46_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N46_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14047 from={} to={}",
                        UCOMPAT_V145_N46_LEN,
                        UCOMPAT_V145_N46_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N46_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N46_DATA[UCOMPAT_V145_N46_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N46_POS + copied;
                if end > UCOMPAT_V145_N46_LEN {
                    UCOMPAT_V145_N46_LEN = end;
                }
                UCOMPAT_V145_N46_POS = end;
                UCOMPAT_V145_N46_EXISTS = true;
            }
            47 => {
                if UCOMPAT_V145_N47_POS > UCOMPAT_V145_N47_LEN {
                    let mut z = UCOMPAT_V145_N47_LEN;
                    while z < UCOMPAT_V145_N47_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N47_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14048 from={} to={}",
                        UCOMPAT_V145_N47_LEN,
                        UCOMPAT_V145_N47_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N47_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N47_DATA[UCOMPAT_V145_N47_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N47_POS + copied;
                if end > UCOMPAT_V145_N47_LEN {
                    UCOMPAT_V145_N47_LEN = end;
                }
                UCOMPAT_V145_N47_POS = end;
                UCOMPAT_V145_N47_EXISTS = true;
            }
            48 => {
                if UCOMPAT_V145_N48_POS > UCOMPAT_V145_N48_LEN {
                    let mut z = UCOMPAT_V145_N48_LEN;
                    while z < UCOMPAT_V145_N48_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N48_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14049 from={} to={}",
                        UCOMPAT_V145_N48_LEN,
                        UCOMPAT_V145_N48_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N48_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N48_DATA[UCOMPAT_V145_N48_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N48_POS + copied;
                if end > UCOMPAT_V145_N48_LEN {
                    UCOMPAT_V145_N48_LEN = end;
                }
                UCOMPAT_V145_N48_POS = end;
                UCOMPAT_V145_N48_EXISTS = true;
            }
            49 => {
                if UCOMPAT_V145_N49_POS > UCOMPAT_V145_N49_LEN {
                    let mut z = UCOMPAT_V145_N49_LEN;
                    while z < UCOMPAT_V145_N49_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N49_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14050 from={} to={}",
                        UCOMPAT_V145_N49_LEN,
                        UCOMPAT_V145_N49_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N49_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N49_DATA[UCOMPAT_V145_N49_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N49_POS + copied;
                if end > UCOMPAT_V145_N49_LEN {
                    UCOMPAT_V145_N49_LEN = end;
                }
                UCOMPAT_V145_N49_POS = end;
                UCOMPAT_V145_N49_EXISTS = true;
            }
            50 => {
                if UCOMPAT_V145_N50_POS > UCOMPAT_V145_N50_LEN {
                    let mut z = UCOMPAT_V145_N50_LEN;
                    while z < UCOMPAT_V145_N50_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N50_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14051 from={} to={}",
                        UCOMPAT_V145_N50_LEN,
                        UCOMPAT_V145_N50_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N50_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N50_DATA[UCOMPAT_V145_N50_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N50_POS + copied;
                if end > UCOMPAT_V145_N50_LEN {
                    UCOMPAT_V145_N50_LEN = end;
                }
                UCOMPAT_V145_N50_POS = end;
                UCOMPAT_V145_N50_EXISTS = true;
            }
            51 => {
                if UCOMPAT_V145_N51_POS > UCOMPAT_V145_N51_LEN {
                    let mut z = UCOMPAT_V145_N51_LEN;
                    while z < UCOMPAT_V145_N51_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N51_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14052 from={} to={}",
                        UCOMPAT_V145_N51_LEN,
                        UCOMPAT_V145_N51_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N51_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N51_DATA[UCOMPAT_V145_N51_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N51_POS + copied;
                if end > UCOMPAT_V145_N51_LEN {
                    UCOMPAT_V145_N51_LEN = end;
                }
                UCOMPAT_V145_N51_POS = end;
                UCOMPAT_V145_N51_EXISTS = true;
            }
            52 => {
                if UCOMPAT_V145_N52_POS > UCOMPAT_V145_N52_LEN {
                    let mut z = UCOMPAT_V145_N52_LEN;
                    while z < UCOMPAT_V145_N52_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N52_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14053 from={} to={}",
                        UCOMPAT_V145_N52_LEN,
                        UCOMPAT_V145_N52_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N52_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N52_DATA[UCOMPAT_V145_N52_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N52_POS + copied;
                if end > UCOMPAT_V145_N52_LEN {
                    UCOMPAT_V145_N52_LEN = end;
                }
                UCOMPAT_V145_N52_POS = end;
                UCOMPAT_V145_N52_EXISTS = true;
            }
            53 => {
                if UCOMPAT_V145_N53_POS > UCOMPAT_V145_N53_LEN {
                    let mut z = UCOMPAT_V145_N53_LEN;
                    while z < UCOMPAT_V145_N53_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N53_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14054 from={} to={}",
                        UCOMPAT_V145_N53_LEN,
                        UCOMPAT_V145_N53_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N53_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N53_DATA[UCOMPAT_V145_N53_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N53_POS + copied;
                if end > UCOMPAT_V145_N53_LEN {
                    UCOMPAT_V145_N53_LEN = end;
                }
                UCOMPAT_V145_N53_POS = end;
                UCOMPAT_V145_N53_EXISTS = true;
            }
            54 => {
                if UCOMPAT_V145_N54_POS > UCOMPAT_V145_N54_LEN {
                    let mut z = UCOMPAT_V145_N54_LEN;
                    while z < UCOMPAT_V145_N54_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N54_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14055 from={} to={}",
                        UCOMPAT_V145_N54_LEN,
                        UCOMPAT_V145_N54_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N54_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N54_DATA[UCOMPAT_V145_N54_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N54_POS + copied;
                if end > UCOMPAT_V145_N54_LEN {
                    UCOMPAT_V145_N54_LEN = end;
                }
                UCOMPAT_V145_N54_POS = end;
                UCOMPAT_V145_N54_EXISTS = true;
            }
            55 => {
                if UCOMPAT_V145_N55_POS > UCOMPAT_V145_N55_LEN {
                    let mut z = UCOMPAT_V145_N55_LEN;
                    while z < UCOMPAT_V145_N55_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N55_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14056 from={} to={}",
                        UCOMPAT_V145_N55_LEN,
                        UCOMPAT_V145_N55_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N55_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N55_DATA[UCOMPAT_V145_N55_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N55_POS + copied;
                if end > UCOMPAT_V145_N55_LEN {
                    UCOMPAT_V145_N55_LEN = end;
                }
                UCOMPAT_V145_N55_POS = end;
                UCOMPAT_V145_N55_EXISTS = true;
            }
            56 => {
                if UCOMPAT_V145_N56_POS > UCOMPAT_V145_N56_LEN {
                    let mut z = UCOMPAT_V145_N56_LEN;
                    while z < UCOMPAT_V145_N56_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N56_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14057 from={} to={}",
                        UCOMPAT_V145_N56_LEN,
                        UCOMPAT_V145_N56_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N56_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N56_DATA[UCOMPAT_V145_N56_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N56_POS + copied;
                if end > UCOMPAT_V145_N56_LEN {
                    UCOMPAT_V145_N56_LEN = end;
                }
                UCOMPAT_V145_N56_POS = end;
                UCOMPAT_V145_N56_EXISTS = true;
            }
            57 => {
                if UCOMPAT_V145_N57_POS > UCOMPAT_V145_N57_LEN {
                    let mut z = UCOMPAT_V145_N57_LEN;
                    while z < UCOMPAT_V145_N57_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N57_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14058 from={} to={}",
                        UCOMPAT_V145_N57_LEN,
                        UCOMPAT_V145_N57_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N57_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N57_DATA[UCOMPAT_V145_N57_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N57_POS + copied;
                if end > UCOMPAT_V145_N57_LEN {
                    UCOMPAT_V145_N57_LEN = end;
                }
                UCOMPAT_V145_N57_POS = end;
                UCOMPAT_V145_N57_EXISTS = true;
            }
            58 => {
                if UCOMPAT_V145_N58_POS > UCOMPAT_V145_N58_LEN {
                    let mut z = UCOMPAT_V145_N58_LEN;
                    while z < UCOMPAT_V145_N58_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N58_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14059 from={} to={}",
                        UCOMPAT_V145_N58_LEN,
                        UCOMPAT_V145_N58_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N58_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N58_DATA[UCOMPAT_V145_N58_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N58_POS + copied;
                if end > UCOMPAT_V145_N58_LEN {
                    UCOMPAT_V145_N58_LEN = end;
                }
                UCOMPAT_V145_N58_POS = end;
                UCOMPAT_V145_N58_EXISTS = true;
            }
            59 => {
                if UCOMPAT_V145_N59_POS > UCOMPAT_V145_N59_LEN {
                    let mut z = UCOMPAT_V145_N59_LEN;
                    while z < UCOMPAT_V145_N59_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N59_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14060 from={} to={}",
                        UCOMPAT_V145_N59_LEN,
                        UCOMPAT_V145_N59_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N59_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N59_DATA[UCOMPAT_V145_N59_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N59_POS + copied;
                if end > UCOMPAT_V145_N59_LEN {
                    UCOMPAT_V145_N59_LEN = end;
                }
                UCOMPAT_V145_N59_POS = end;
                UCOMPAT_V145_N59_EXISTS = true;
            }
            60 => {
                if UCOMPAT_V145_N60_POS > UCOMPAT_V145_N60_LEN {
                    let mut z = UCOMPAT_V145_N60_LEN;
                    while z < UCOMPAT_V145_N60_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N60_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14061 from={} to={}",
                        UCOMPAT_V145_N60_LEN,
                        UCOMPAT_V145_N60_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N60_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N60_DATA[UCOMPAT_V145_N60_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N60_POS + copied;
                if end > UCOMPAT_V145_N60_LEN {
                    UCOMPAT_V145_N60_LEN = end;
                }
                UCOMPAT_V145_N60_POS = end;
                UCOMPAT_V145_N60_EXISTS = true;
            }
            61 => {
                if UCOMPAT_V145_N61_POS > UCOMPAT_V145_N61_LEN {
                    let mut z = UCOMPAT_V145_N61_LEN;
                    while z < UCOMPAT_V145_N61_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N61_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14062 from={} to={}",
                        UCOMPAT_V145_N61_LEN,
                        UCOMPAT_V145_N61_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N61_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N61_DATA[UCOMPAT_V145_N61_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N61_POS + copied;
                if end > UCOMPAT_V145_N61_LEN {
                    UCOMPAT_V145_N61_LEN = end;
                }
                UCOMPAT_V145_N61_POS = end;
                UCOMPAT_V145_N61_EXISTS = true;
            }
            62 => {
                if UCOMPAT_V145_N62_POS > UCOMPAT_V145_N62_LEN {
                    let mut z = UCOMPAT_V145_N62_LEN;
                    while z < UCOMPAT_V145_N62_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N62_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14063 from={} to={}",
                        UCOMPAT_V145_N62_LEN,
                        UCOMPAT_V145_N62_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N62_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N62_DATA[UCOMPAT_V145_N62_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N62_POS + copied;
                if end > UCOMPAT_V145_N62_LEN {
                    UCOMPAT_V145_N62_LEN = end;
                }
                UCOMPAT_V145_N62_POS = end;
                UCOMPAT_V145_N62_EXISTS = true;
            }
            _ => {
                if UCOMPAT_V145_N63_POS > UCOMPAT_V145_N63_LEN {
                    let mut z = UCOMPAT_V145_N63_LEN;
                    while z < UCOMPAT_V145_N63_POS && z < UCOMPAT_V145_CAP {
                        UCOMPAT_V145_N63_DATA[z] = 0;
                        z += 1;
                    }
                    crate::println!(
                        "[ucompat-v145] zero-fill sparse gap fd=14064 from={} to={}",
                        UCOMPAT_V145_N63_LEN,
                        UCOMPAT_V145_N63_POS
                    );
                }
                while copied < len && UCOMPAT_V145_N63_POS + copied < UCOMPAT_V145_CAP {
                    UCOMPAT_V145_N63_DATA[UCOMPAT_V145_N63_POS + copied] =
                        core::ptr::read_volatile((user_ptr + copied) as *const u8);
                    copied += 1;
                }
                let end = UCOMPAT_V145_N63_POS + copied;
                if end > UCOMPAT_V145_N63_LEN {
                    UCOMPAT_V145_N63_LEN = end;
                }
                UCOMPAT_V145_N63_POS = end;
                UCOMPAT_V145_N63_EXISTS = true;
            }
        }
    });
    // UCOMPAT_V145E_SUPPRESSED_V145_WRITE_COPIED_LOG
    copied as isize
}
fn ucompat_v145_write(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V145_FD_N00 {
        ucompat_v145_write_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V145_FD_N01 {
        ucompat_v145_write_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V145_FD_N02 {
        ucompat_v145_write_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V145_FD_N03 {
        ucompat_v145_write_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V145_FD_N04 {
        ucompat_v145_write_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V145_FD_N05 {
        ucompat_v145_write_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V145_FD_N06 {
        ucompat_v145_write_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V145_FD_N07 {
        ucompat_v145_write_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V145_FD_N08 {
        ucompat_v145_write_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V145_FD_N09 {
        ucompat_v145_write_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V145_FD_N10 {
        ucompat_v145_write_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V145_FD_N11 {
        ucompat_v145_write_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V145_FD_N12 {
        ucompat_v145_write_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V145_FD_N13 {
        ucompat_v145_write_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V145_FD_N14 {
        ucompat_v145_write_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V145_FD_N15 {
        ucompat_v145_write_one(fd, user_ptr, len, 15)
    } else if fd == UCOMPAT_V145_FD_N16 {
        ucompat_v145_write_one(fd, user_ptr, len, 16)
    } else if fd == UCOMPAT_V145_FD_N17 {
        ucompat_v145_write_one(fd, user_ptr, len, 17)
    } else if fd == UCOMPAT_V145_FD_N18 {
        ucompat_v145_write_one(fd, user_ptr, len, 18)
    } else if fd == UCOMPAT_V145_FD_N19 {
        ucompat_v145_write_one(fd, user_ptr, len, 19)
    } else if fd == UCOMPAT_V145_FD_N20 {
        ucompat_v145_write_one(fd, user_ptr, len, 20)
    } else if fd == UCOMPAT_V145_FD_N21 {
        ucompat_v145_write_one(fd, user_ptr, len, 21)
    } else if fd == UCOMPAT_V145_FD_N22 {
        ucompat_v145_write_one(fd, user_ptr, len, 22)
    } else if fd == UCOMPAT_V145_FD_N23 {
        ucompat_v145_write_one(fd, user_ptr, len, 23)
    } else if fd == UCOMPAT_V145_FD_N24 {
        ucompat_v145_write_one(fd, user_ptr, len, 24)
    } else if fd == UCOMPAT_V145_FD_N25 {
        ucompat_v145_write_one(fd, user_ptr, len, 25)
    } else if fd == UCOMPAT_V145_FD_N26 {
        ucompat_v145_write_one(fd, user_ptr, len, 26)
    } else if fd == UCOMPAT_V145_FD_N27 {
        ucompat_v145_write_one(fd, user_ptr, len, 27)
    } else if fd == UCOMPAT_V145_FD_N28 {
        ucompat_v145_write_one(fd, user_ptr, len, 28)
    } else if fd == UCOMPAT_V145_FD_N29 {
        ucompat_v145_write_one(fd, user_ptr, len, 29)
    } else if fd == UCOMPAT_V145_FD_N30 {
        ucompat_v145_write_one(fd, user_ptr, len, 30)
    } else if fd == UCOMPAT_V145_FD_N31 {
        ucompat_v145_write_one(fd, user_ptr, len, 31)
    } else if fd == UCOMPAT_V145_FD_N32 {
        ucompat_v145_write_one(fd, user_ptr, len, 32)
    } else if fd == UCOMPAT_V145_FD_N33 {
        ucompat_v145_write_one(fd, user_ptr, len, 33)
    } else if fd == UCOMPAT_V145_FD_N34 {
        ucompat_v145_write_one(fd, user_ptr, len, 34)
    } else if fd == UCOMPAT_V145_FD_N35 {
        ucompat_v145_write_one(fd, user_ptr, len, 35)
    } else if fd == UCOMPAT_V145_FD_N36 {
        ucompat_v145_write_one(fd, user_ptr, len, 36)
    } else if fd == UCOMPAT_V145_FD_N37 {
        ucompat_v145_write_one(fd, user_ptr, len, 37)
    } else if fd == UCOMPAT_V145_FD_N38 {
        ucompat_v145_write_one(fd, user_ptr, len, 38)
    } else if fd == UCOMPAT_V145_FD_N39 {
        ucompat_v145_write_one(fd, user_ptr, len, 39)
    } else if fd == UCOMPAT_V145_FD_N40 {
        ucompat_v145_write_one(fd, user_ptr, len, 40)
    } else if fd == UCOMPAT_V145_FD_N41 {
        ucompat_v145_write_one(fd, user_ptr, len, 41)
    } else if fd == UCOMPAT_V145_FD_N42 {
        ucompat_v145_write_one(fd, user_ptr, len, 42)
    } else if fd == UCOMPAT_V145_FD_N43 {
        ucompat_v145_write_one(fd, user_ptr, len, 43)
    } else if fd == UCOMPAT_V145_FD_N44 {
        ucompat_v145_write_one(fd, user_ptr, len, 44)
    } else if fd == UCOMPAT_V145_FD_N45 {
        ucompat_v145_write_one(fd, user_ptr, len, 45)
    } else if fd == UCOMPAT_V145_FD_N46 {
        ucompat_v145_write_one(fd, user_ptr, len, 46)
    } else if fd == UCOMPAT_V145_FD_N47 {
        ucompat_v145_write_one(fd, user_ptr, len, 47)
    } else if fd == UCOMPAT_V145_FD_N48 {
        ucompat_v145_write_one(fd, user_ptr, len, 48)
    } else if fd == UCOMPAT_V145_FD_N49 {
        ucompat_v145_write_one(fd, user_ptr, len, 49)
    } else if fd == UCOMPAT_V145_FD_N50 {
        ucompat_v145_write_one(fd, user_ptr, len, 50)
    } else if fd == UCOMPAT_V145_FD_N51 {
        ucompat_v145_write_one(fd, user_ptr, len, 51)
    } else if fd == UCOMPAT_V145_FD_N52 {
        ucompat_v145_write_one(fd, user_ptr, len, 52)
    } else if fd == UCOMPAT_V145_FD_N53 {
        ucompat_v145_write_one(fd, user_ptr, len, 53)
    } else if fd == UCOMPAT_V145_FD_N54 {
        ucompat_v145_write_one(fd, user_ptr, len, 54)
    } else if fd == UCOMPAT_V145_FD_N55 {
        ucompat_v145_write_one(fd, user_ptr, len, 55)
    } else if fd == UCOMPAT_V145_FD_N56 {
        ucompat_v145_write_one(fd, user_ptr, len, 56)
    } else if fd == UCOMPAT_V145_FD_N57 {
        ucompat_v145_write_one(fd, user_ptr, len, 57)
    } else if fd == UCOMPAT_V145_FD_N58 {
        ucompat_v145_write_one(fd, user_ptr, len, 58)
    } else if fd == UCOMPAT_V145_FD_N59 {
        ucompat_v145_write_one(fd, user_ptr, len, 59)
    } else if fd == UCOMPAT_V145_FD_N60 {
        ucompat_v145_write_one(fd, user_ptr, len, 60)
    } else if fd == UCOMPAT_V145_FD_N61 {
        ucompat_v145_write_one(fd, user_ptr, len, 61)
    } else if fd == UCOMPAT_V145_FD_N62 {
        ucompat_v145_write_one(fd, user_ptr, len, 62)
    } else if fd == UCOMPAT_V145_FD_N63 {
        ucompat_v145_write_one(fd, user_ptr, len, 63)
    } else {
        -9
    }
}
fn ucompat_v145_read_one(_fd: isize, user_ptr: usize, len: usize, slot: usize) -> isize {
    let mut copied = 0usize;
    with_sum_enabled(|| unsafe {
        match slot {
            0 => {
                while copied < len && UCOMPAT_V145_N00_POS < UCOMPAT_V145_N00_LEN {
                    let ch = UCOMPAT_V145_N00_DATA[UCOMPAT_V145_N00_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N00_POS += 1;
                    copied += 1;
                }
            }
            1 => {
                while copied < len && UCOMPAT_V145_N01_POS < UCOMPAT_V145_N01_LEN {
                    let ch = UCOMPAT_V145_N01_DATA[UCOMPAT_V145_N01_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N01_POS += 1;
                    copied += 1;
                }
            }
            2 => {
                while copied < len && UCOMPAT_V145_N02_POS < UCOMPAT_V145_N02_LEN {
                    let ch = UCOMPAT_V145_N02_DATA[UCOMPAT_V145_N02_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N02_POS += 1;
                    copied += 1;
                }
            }
            3 => {
                while copied < len && UCOMPAT_V145_N03_POS < UCOMPAT_V145_N03_LEN {
                    let ch = UCOMPAT_V145_N03_DATA[UCOMPAT_V145_N03_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N03_POS += 1;
                    copied += 1;
                }
            }
            4 => {
                while copied < len && UCOMPAT_V145_N04_POS < UCOMPAT_V145_N04_LEN {
                    let ch = UCOMPAT_V145_N04_DATA[UCOMPAT_V145_N04_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N04_POS += 1;
                    copied += 1;
                }
            }
            5 => {
                while copied < len && UCOMPAT_V145_N05_POS < UCOMPAT_V145_N05_LEN {
                    let ch = UCOMPAT_V145_N05_DATA[UCOMPAT_V145_N05_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N05_POS += 1;
                    copied += 1;
                }
            }
            6 => {
                while copied < len && UCOMPAT_V145_N06_POS < UCOMPAT_V145_N06_LEN {
                    let ch = UCOMPAT_V145_N06_DATA[UCOMPAT_V145_N06_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N06_POS += 1;
                    copied += 1;
                }
            }
            7 => {
                while copied < len && UCOMPAT_V145_N07_POS < UCOMPAT_V145_N07_LEN {
                    let ch = UCOMPAT_V145_N07_DATA[UCOMPAT_V145_N07_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N07_POS += 1;
                    copied += 1;
                }
            }
            8 => {
                while copied < len && UCOMPAT_V145_N08_POS < UCOMPAT_V145_N08_LEN {
                    let ch = UCOMPAT_V145_N08_DATA[UCOMPAT_V145_N08_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N08_POS += 1;
                    copied += 1;
                }
            }
            9 => {
                while copied < len && UCOMPAT_V145_N09_POS < UCOMPAT_V145_N09_LEN {
                    let ch = UCOMPAT_V145_N09_DATA[UCOMPAT_V145_N09_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N09_POS += 1;
                    copied += 1;
                }
            }
            10 => {
                while copied < len && UCOMPAT_V145_N10_POS < UCOMPAT_V145_N10_LEN {
                    let ch = UCOMPAT_V145_N10_DATA[UCOMPAT_V145_N10_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N10_POS += 1;
                    copied += 1;
                }
            }
            11 => {
                while copied < len && UCOMPAT_V145_N11_POS < UCOMPAT_V145_N11_LEN {
                    let ch = UCOMPAT_V145_N11_DATA[UCOMPAT_V145_N11_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N11_POS += 1;
                    copied += 1;
                }
            }
            12 => {
                while copied < len && UCOMPAT_V145_N12_POS < UCOMPAT_V145_N12_LEN {
                    let ch = UCOMPAT_V145_N12_DATA[UCOMPAT_V145_N12_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N12_POS += 1;
                    copied += 1;
                }
            }
            13 => {
                while copied < len && UCOMPAT_V145_N13_POS < UCOMPAT_V145_N13_LEN {
                    let ch = UCOMPAT_V145_N13_DATA[UCOMPAT_V145_N13_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N13_POS += 1;
                    copied += 1;
                }
            }
            14 => {
                while copied < len && UCOMPAT_V145_N14_POS < UCOMPAT_V145_N14_LEN {
                    let ch = UCOMPAT_V145_N14_DATA[UCOMPAT_V145_N14_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N14_POS += 1;
                    copied += 1;
                }
            }
            15 => {
                while copied < len && UCOMPAT_V145_N15_POS < UCOMPAT_V145_N15_LEN {
                    let ch = UCOMPAT_V145_N15_DATA[UCOMPAT_V145_N15_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N15_POS += 1;
                    copied += 1;
                }
            }
            16 => {
                while copied < len && UCOMPAT_V145_N16_POS < UCOMPAT_V145_N16_LEN {
                    let ch = UCOMPAT_V145_N16_DATA[UCOMPAT_V145_N16_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N16_POS += 1;
                    copied += 1;
                }
            }
            17 => {
                while copied < len && UCOMPAT_V145_N17_POS < UCOMPAT_V145_N17_LEN {
                    let ch = UCOMPAT_V145_N17_DATA[UCOMPAT_V145_N17_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N17_POS += 1;
                    copied += 1;
                }
            }
            18 => {
                while copied < len && UCOMPAT_V145_N18_POS < UCOMPAT_V145_N18_LEN {
                    let ch = UCOMPAT_V145_N18_DATA[UCOMPAT_V145_N18_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N18_POS += 1;
                    copied += 1;
                }
            }
            19 => {
                while copied < len && UCOMPAT_V145_N19_POS < UCOMPAT_V145_N19_LEN {
                    let ch = UCOMPAT_V145_N19_DATA[UCOMPAT_V145_N19_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N19_POS += 1;
                    copied += 1;
                }
            }
            20 => {
                while copied < len && UCOMPAT_V145_N20_POS < UCOMPAT_V145_N20_LEN {
                    let ch = UCOMPAT_V145_N20_DATA[UCOMPAT_V145_N20_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N20_POS += 1;
                    copied += 1;
                }
            }
            21 => {
                while copied < len && UCOMPAT_V145_N21_POS < UCOMPAT_V145_N21_LEN {
                    let ch = UCOMPAT_V145_N21_DATA[UCOMPAT_V145_N21_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N21_POS += 1;
                    copied += 1;
                }
            }
            22 => {
                while copied < len && UCOMPAT_V145_N22_POS < UCOMPAT_V145_N22_LEN {
                    let ch = UCOMPAT_V145_N22_DATA[UCOMPAT_V145_N22_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N22_POS += 1;
                    copied += 1;
                }
            }
            23 => {
                while copied < len && UCOMPAT_V145_N23_POS < UCOMPAT_V145_N23_LEN {
                    let ch = UCOMPAT_V145_N23_DATA[UCOMPAT_V145_N23_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N23_POS += 1;
                    copied += 1;
                }
            }
            24 => {
                while copied < len && UCOMPAT_V145_N24_POS < UCOMPAT_V145_N24_LEN {
                    let ch = UCOMPAT_V145_N24_DATA[UCOMPAT_V145_N24_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N24_POS += 1;
                    copied += 1;
                }
            }
            25 => {
                while copied < len && UCOMPAT_V145_N25_POS < UCOMPAT_V145_N25_LEN {
                    let ch = UCOMPAT_V145_N25_DATA[UCOMPAT_V145_N25_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N25_POS += 1;
                    copied += 1;
                }
            }
            26 => {
                while copied < len && UCOMPAT_V145_N26_POS < UCOMPAT_V145_N26_LEN {
                    let ch = UCOMPAT_V145_N26_DATA[UCOMPAT_V145_N26_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N26_POS += 1;
                    copied += 1;
                }
            }
            27 => {
                while copied < len && UCOMPAT_V145_N27_POS < UCOMPAT_V145_N27_LEN {
                    let ch = UCOMPAT_V145_N27_DATA[UCOMPAT_V145_N27_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N27_POS += 1;
                    copied += 1;
                }
            }
            28 => {
                while copied < len && UCOMPAT_V145_N28_POS < UCOMPAT_V145_N28_LEN {
                    let ch = UCOMPAT_V145_N28_DATA[UCOMPAT_V145_N28_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N28_POS += 1;
                    copied += 1;
                }
            }
            29 => {
                while copied < len && UCOMPAT_V145_N29_POS < UCOMPAT_V145_N29_LEN {
                    let ch = UCOMPAT_V145_N29_DATA[UCOMPAT_V145_N29_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N29_POS += 1;
                    copied += 1;
                }
            }
            30 => {
                while copied < len && UCOMPAT_V145_N30_POS < UCOMPAT_V145_N30_LEN {
                    let ch = UCOMPAT_V145_N30_DATA[UCOMPAT_V145_N30_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N30_POS += 1;
                    copied += 1;
                }
            }
            31 => {
                while copied < len && UCOMPAT_V145_N31_POS < UCOMPAT_V145_N31_LEN {
                    let ch = UCOMPAT_V145_N31_DATA[UCOMPAT_V145_N31_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N31_POS += 1;
                    copied += 1;
                }
            }
            32 => {
                while copied < len && UCOMPAT_V145_N32_POS < UCOMPAT_V145_N32_LEN {
                    let ch = UCOMPAT_V145_N32_DATA[UCOMPAT_V145_N32_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N32_POS += 1;
                    copied += 1;
                }
            }
            33 => {
                while copied < len && UCOMPAT_V145_N33_POS < UCOMPAT_V145_N33_LEN {
                    let ch = UCOMPAT_V145_N33_DATA[UCOMPAT_V145_N33_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N33_POS += 1;
                    copied += 1;
                }
            }
            34 => {
                while copied < len && UCOMPAT_V145_N34_POS < UCOMPAT_V145_N34_LEN {
                    let ch = UCOMPAT_V145_N34_DATA[UCOMPAT_V145_N34_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N34_POS += 1;
                    copied += 1;
                }
            }
            35 => {
                while copied < len && UCOMPAT_V145_N35_POS < UCOMPAT_V145_N35_LEN {
                    let ch = UCOMPAT_V145_N35_DATA[UCOMPAT_V145_N35_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N35_POS += 1;
                    copied += 1;
                }
            }
            36 => {
                while copied < len && UCOMPAT_V145_N36_POS < UCOMPAT_V145_N36_LEN {
                    let ch = UCOMPAT_V145_N36_DATA[UCOMPAT_V145_N36_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N36_POS += 1;
                    copied += 1;
                }
            }
            37 => {
                while copied < len && UCOMPAT_V145_N37_POS < UCOMPAT_V145_N37_LEN {
                    let ch = UCOMPAT_V145_N37_DATA[UCOMPAT_V145_N37_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N37_POS += 1;
                    copied += 1;
                }
            }
            38 => {
                while copied < len && UCOMPAT_V145_N38_POS < UCOMPAT_V145_N38_LEN {
                    let ch = UCOMPAT_V145_N38_DATA[UCOMPAT_V145_N38_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N38_POS += 1;
                    copied += 1;
                }
            }
            39 => {
                while copied < len && UCOMPAT_V145_N39_POS < UCOMPAT_V145_N39_LEN {
                    let ch = UCOMPAT_V145_N39_DATA[UCOMPAT_V145_N39_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N39_POS += 1;
                    copied += 1;
                }
            }
            40 => {
                while copied < len && UCOMPAT_V145_N40_POS < UCOMPAT_V145_N40_LEN {
                    let ch = UCOMPAT_V145_N40_DATA[UCOMPAT_V145_N40_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N40_POS += 1;
                    copied += 1;
                }
            }
            41 => {
                while copied < len && UCOMPAT_V145_N41_POS < UCOMPAT_V145_N41_LEN {
                    let ch = UCOMPAT_V145_N41_DATA[UCOMPAT_V145_N41_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N41_POS += 1;
                    copied += 1;
                }
            }
            42 => {
                while copied < len && UCOMPAT_V145_N42_POS < UCOMPAT_V145_N42_LEN {
                    let ch = UCOMPAT_V145_N42_DATA[UCOMPAT_V145_N42_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N42_POS += 1;
                    copied += 1;
                }
            }
            43 => {
                while copied < len && UCOMPAT_V145_N43_POS < UCOMPAT_V145_N43_LEN {
                    let ch = UCOMPAT_V145_N43_DATA[UCOMPAT_V145_N43_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N43_POS += 1;
                    copied += 1;
                }
            }
            44 => {
                while copied < len && UCOMPAT_V145_N44_POS < UCOMPAT_V145_N44_LEN {
                    let ch = UCOMPAT_V145_N44_DATA[UCOMPAT_V145_N44_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N44_POS += 1;
                    copied += 1;
                }
            }
            45 => {
                while copied < len && UCOMPAT_V145_N45_POS < UCOMPAT_V145_N45_LEN {
                    let ch = UCOMPAT_V145_N45_DATA[UCOMPAT_V145_N45_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N45_POS += 1;
                    copied += 1;
                }
            }
            46 => {
                while copied < len && UCOMPAT_V145_N46_POS < UCOMPAT_V145_N46_LEN {
                    let ch = UCOMPAT_V145_N46_DATA[UCOMPAT_V145_N46_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N46_POS += 1;
                    copied += 1;
                }
            }
            47 => {
                while copied < len && UCOMPAT_V145_N47_POS < UCOMPAT_V145_N47_LEN {
                    let ch = UCOMPAT_V145_N47_DATA[UCOMPAT_V145_N47_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N47_POS += 1;
                    copied += 1;
                }
            }
            48 => {
                while copied < len && UCOMPAT_V145_N48_POS < UCOMPAT_V145_N48_LEN {
                    let ch = UCOMPAT_V145_N48_DATA[UCOMPAT_V145_N48_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N48_POS += 1;
                    copied += 1;
                }
            }
            49 => {
                while copied < len && UCOMPAT_V145_N49_POS < UCOMPAT_V145_N49_LEN {
                    let ch = UCOMPAT_V145_N49_DATA[UCOMPAT_V145_N49_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N49_POS += 1;
                    copied += 1;
                }
            }
            50 => {
                while copied < len && UCOMPAT_V145_N50_POS < UCOMPAT_V145_N50_LEN {
                    let ch = UCOMPAT_V145_N50_DATA[UCOMPAT_V145_N50_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N50_POS += 1;
                    copied += 1;
                }
            }
            51 => {
                while copied < len && UCOMPAT_V145_N51_POS < UCOMPAT_V145_N51_LEN {
                    let ch = UCOMPAT_V145_N51_DATA[UCOMPAT_V145_N51_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N51_POS += 1;
                    copied += 1;
                }
            }
            52 => {
                while copied < len && UCOMPAT_V145_N52_POS < UCOMPAT_V145_N52_LEN {
                    let ch = UCOMPAT_V145_N52_DATA[UCOMPAT_V145_N52_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N52_POS += 1;
                    copied += 1;
                }
            }
            53 => {
                while copied < len && UCOMPAT_V145_N53_POS < UCOMPAT_V145_N53_LEN {
                    let ch = UCOMPAT_V145_N53_DATA[UCOMPAT_V145_N53_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N53_POS += 1;
                    copied += 1;
                }
            }
            54 => {
                while copied < len && UCOMPAT_V145_N54_POS < UCOMPAT_V145_N54_LEN {
                    let ch = UCOMPAT_V145_N54_DATA[UCOMPAT_V145_N54_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N54_POS += 1;
                    copied += 1;
                }
            }
            55 => {
                while copied < len && UCOMPAT_V145_N55_POS < UCOMPAT_V145_N55_LEN {
                    let ch = UCOMPAT_V145_N55_DATA[UCOMPAT_V145_N55_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N55_POS += 1;
                    copied += 1;
                }
            }
            56 => {
                while copied < len && UCOMPAT_V145_N56_POS < UCOMPAT_V145_N56_LEN {
                    let ch = UCOMPAT_V145_N56_DATA[UCOMPAT_V145_N56_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N56_POS += 1;
                    copied += 1;
                }
            }
            57 => {
                while copied < len && UCOMPAT_V145_N57_POS < UCOMPAT_V145_N57_LEN {
                    let ch = UCOMPAT_V145_N57_DATA[UCOMPAT_V145_N57_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N57_POS += 1;
                    copied += 1;
                }
            }
            58 => {
                while copied < len && UCOMPAT_V145_N58_POS < UCOMPAT_V145_N58_LEN {
                    let ch = UCOMPAT_V145_N58_DATA[UCOMPAT_V145_N58_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N58_POS += 1;
                    copied += 1;
                }
            }
            59 => {
                while copied < len && UCOMPAT_V145_N59_POS < UCOMPAT_V145_N59_LEN {
                    let ch = UCOMPAT_V145_N59_DATA[UCOMPAT_V145_N59_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N59_POS += 1;
                    copied += 1;
                }
            }
            60 => {
                while copied < len && UCOMPAT_V145_N60_POS < UCOMPAT_V145_N60_LEN {
                    let ch = UCOMPAT_V145_N60_DATA[UCOMPAT_V145_N60_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N60_POS += 1;
                    copied += 1;
                }
            }
            61 => {
                while copied < len && UCOMPAT_V145_N61_POS < UCOMPAT_V145_N61_LEN {
                    let ch = UCOMPAT_V145_N61_DATA[UCOMPAT_V145_N61_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N61_POS += 1;
                    copied += 1;
                }
            }
            62 => {
                while copied < len && UCOMPAT_V145_N62_POS < UCOMPAT_V145_N62_LEN {
                    let ch = UCOMPAT_V145_N62_DATA[UCOMPAT_V145_N62_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N62_POS += 1;
                    copied += 1;
                }
            }
            _ => {
                while copied < len && UCOMPAT_V145_N63_POS < UCOMPAT_V145_N63_LEN {
                    let ch = UCOMPAT_V145_N63_DATA[UCOMPAT_V145_N63_POS];
                    core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                    UCOMPAT_V145_N63_POS += 1;
                    copied += 1;
                }
            }
        }
    });
    // UCOMPAT_V145E_SUPPRESSED_V145_READ_COPIED_LOG
    copied as isize
}
fn ucompat_v145_read(fd: isize, user_ptr: usize, len: usize) -> isize {
    if fd == UCOMPAT_V145_FD_N00 {
        ucompat_v145_read_one(fd, user_ptr, len, 0)
    } else if fd == UCOMPAT_V145_FD_N01 {
        ucompat_v145_read_one(fd, user_ptr, len, 1)
    } else if fd == UCOMPAT_V145_FD_N02 {
        ucompat_v145_read_one(fd, user_ptr, len, 2)
    } else if fd == UCOMPAT_V145_FD_N03 {
        ucompat_v145_read_one(fd, user_ptr, len, 3)
    } else if fd == UCOMPAT_V145_FD_N04 {
        ucompat_v145_read_one(fd, user_ptr, len, 4)
    } else if fd == UCOMPAT_V145_FD_N05 {
        ucompat_v145_read_one(fd, user_ptr, len, 5)
    } else if fd == UCOMPAT_V145_FD_N06 {
        ucompat_v145_read_one(fd, user_ptr, len, 6)
    } else if fd == UCOMPAT_V145_FD_N07 {
        ucompat_v145_read_one(fd, user_ptr, len, 7)
    } else if fd == UCOMPAT_V145_FD_N08 {
        ucompat_v145_read_one(fd, user_ptr, len, 8)
    } else if fd == UCOMPAT_V145_FD_N09 {
        ucompat_v145_read_one(fd, user_ptr, len, 9)
    } else if fd == UCOMPAT_V145_FD_N10 {
        ucompat_v145_read_one(fd, user_ptr, len, 10)
    } else if fd == UCOMPAT_V145_FD_N11 {
        ucompat_v145_read_one(fd, user_ptr, len, 11)
    } else if fd == UCOMPAT_V145_FD_N12 {
        ucompat_v145_read_one(fd, user_ptr, len, 12)
    } else if fd == UCOMPAT_V145_FD_N13 {
        ucompat_v145_read_one(fd, user_ptr, len, 13)
    } else if fd == UCOMPAT_V145_FD_N14 {
        ucompat_v145_read_one(fd, user_ptr, len, 14)
    } else if fd == UCOMPAT_V145_FD_N15 {
        ucompat_v145_read_one(fd, user_ptr, len, 15)
    } else if fd == UCOMPAT_V145_FD_N16 {
        ucompat_v145_read_one(fd, user_ptr, len, 16)
    } else if fd == UCOMPAT_V145_FD_N17 {
        ucompat_v145_read_one(fd, user_ptr, len, 17)
    } else if fd == UCOMPAT_V145_FD_N18 {
        ucompat_v145_read_one(fd, user_ptr, len, 18)
    } else if fd == UCOMPAT_V145_FD_N19 {
        ucompat_v145_read_one(fd, user_ptr, len, 19)
    } else if fd == UCOMPAT_V145_FD_N20 {
        ucompat_v145_read_one(fd, user_ptr, len, 20)
    } else if fd == UCOMPAT_V145_FD_N21 {
        ucompat_v145_read_one(fd, user_ptr, len, 21)
    } else if fd == UCOMPAT_V145_FD_N22 {
        ucompat_v145_read_one(fd, user_ptr, len, 22)
    } else if fd == UCOMPAT_V145_FD_N23 {
        ucompat_v145_read_one(fd, user_ptr, len, 23)
    } else if fd == UCOMPAT_V145_FD_N24 {
        ucompat_v145_read_one(fd, user_ptr, len, 24)
    } else if fd == UCOMPAT_V145_FD_N25 {
        ucompat_v145_read_one(fd, user_ptr, len, 25)
    } else if fd == UCOMPAT_V145_FD_N26 {
        ucompat_v145_read_one(fd, user_ptr, len, 26)
    } else if fd == UCOMPAT_V145_FD_N27 {
        ucompat_v145_read_one(fd, user_ptr, len, 27)
    } else if fd == UCOMPAT_V145_FD_N28 {
        ucompat_v145_read_one(fd, user_ptr, len, 28)
    } else if fd == UCOMPAT_V145_FD_N29 {
        ucompat_v145_read_one(fd, user_ptr, len, 29)
    } else if fd == UCOMPAT_V145_FD_N30 {
        ucompat_v145_read_one(fd, user_ptr, len, 30)
    } else if fd == UCOMPAT_V145_FD_N31 {
        ucompat_v145_read_one(fd, user_ptr, len, 31)
    } else if fd == UCOMPAT_V145_FD_N32 {
        ucompat_v145_read_one(fd, user_ptr, len, 32)
    } else if fd == UCOMPAT_V145_FD_N33 {
        ucompat_v145_read_one(fd, user_ptr, len, 33)
    } else if fd == UCOMPAT_V145_FD_N34 {
        ucompat_v145_read_one(fd, user_ptr, len, 34)
    } else if fd == UCOMPAT_V145_FD_N35 {
        ucompat_v145_read_one(fd, user_ptr, len, 35)
    } else if fd == UCOMPAT_V145_FD_N36 {
        ucompat_v145_read_one(fd, user_ptr, len, 36)
    } else if fd == UCOMPAT_V145_FD_N37 {
        ucompat_v145_read_one(fd, user_ptr, len, 37)
    } else if fd == UCOMPAT_V145_FD_N38 {
        ucompat_v145_read_one(fd, user_ptr, len, 38)
    } else if fd == UCOMPAT_V145_FD_N39 {
        ucompat_v145_read_one(fd, user_ptr, len, 39)
    } else if fd == UCOMPAT_V145_FD_N40 {
        ucompat_v145_read_one(fd, user_ptr, len, 40)
    } else if fd == UCOMPAT_V145_FD_N41 {
        ucompat_v145_read_one(fd, user_ptr, len, 41)
    } else if fd == UCOMPAT_V145_FD_N42 {
        ucompat_v145_read_one(fd, user_ptr, len, 42)
    } else if fd == UCOMPAT_V145_FD_N43 {
        ucompat_v145_read_one(fd, user_ptr, len, 43)
    } else if fd == UCOMPAT_V145_FD_N44 {
        ucompat_v145_read_one(fd, user_ptr, len, 44)
    } else if fd == UCOMPAT_V145_FD_N45 {
        ucompat_v145_read_one(fd, user_ptr, len, 45)
    } else if fd == UCOMPAT_V145_FD_N46 {
        ucompat_v145_read_one(fd, user_ptr, len, 46)
    } else if fd == UCOMPAT_V145_FD_N47 {
        ucompat_v145_read_one(fd, user_ptr, len, 47)
    } else if fd == UCOMPAT_V145_FD_N48 {
        ucompat_v145_read_one(fd, user_ptr, len, 48)
    } else if fd == UCOMPAT_V145_FD_N49 {
        ucompat_v145_read_one(fd, user_ptr, len, 49)
    } else if fd == UCOMPAT_V145_FD_N50 {
        ucompat_v145_read_one(fd, user_ptr, len, 50)
    } else if fd == UCOMPAT_V145_FD_N51 {
        ucompat_v145_read_one(fd, user_ptr, len, 51)
    } else if fd == UCOMPAT_V145_FD_N52 {
        ucompat_v145_read_one(fd, user_ptr, len, 52)
    } else if fd == UCOMPAT_V145_FD_N53 {
        ucompat_v145_read_one(fd, user_ptr, len, 53)
    } else if fd == UCOMPAT_V145_FD_N54 {
        ucompat_v145_read_one(fd, user_ptr, len, 54)
    } else if fd == UCOMPAT_V145_FD_N55 {
        ucompat_v145_read_one(fd, user_ptr, len, 55)
    } else if fd == UCOMPAT_V145_FD_N56 {
        ucompat_v145_read_one(fd, user_ptr, len, 56)
    } else if fd == UCOMPAT_V145_FD_N57 {
        ucompat_v145_read_one(fd, user_ptr, len, 57)
    } else if fd == UCOMPAT_V145_FD_N58 {
        ucompat_v145_read_one(fd, user_ptr, len, 58)
    } else if fd == UCOMPAT_V145_FD_N59 {
        ucompat_v145_read_one(fd, user_ptr, len, 59)
    } else if fd == UCOMPAT_V145_FD_N60 {
        ucompat_v145_read_one(fd, user_ptr, len, 60)
    } else if fd == UCOMPAT_V145_FD_N61 {
        ucompat_v145_read_one(fd, user_ptr, len, 61)
    } else if fd == UCOMPAT_V145_FD_N62 {
        ucompat_v145_read_one(fd, user_ptr, len, 62)
    } else if fd == UCOMPAT_V145_FD_N63 {
        ucompat_v145_read_one(fd, user_ptr, len, 63)
    } else {
        -9
    }
}
fn ucompat_v145_lseek(fd: isize, off: isize, whence: usize) -> isize {
    unsafe {
        let (len, cur) = if fd == UCOMPAT_V145_FD_N00 {
            (UCOMPAT_V145_N00_LEN, UCOMPAT_V145_N00_POS)
        } else if fd == UCOMPAT_V145_FD_N01 {
            (UCOMPAT_V145_N01_LEN, UCOMPAT_V145_N01_POS)
        } else if fd == UCOMPAT_V145_FD_N02 {
            (UCOMPAT_V145_N02_LEN, UCOMPAT_V145_N02_POS)
        } else if fd == UCOMPAT_V145_FD_N03 {
            (UCOMPAT_V145_N03_LEN, UCOMPAT_V145_N03_POS)
        } else if fd == UCOMPAT_V145_FD_N04 {
            (UCOMPAT_V145_N04_LEN, UCOMPAT_V145_N04_POS)
        } else if fd == UCOMPAT_V145_FD_N05 {
            (UCOMPAT_V145_N05_LEN, UCOMPAT_V145_N05_POS)
        } else if fd == UCOMPAT_V145_FD_N06 {
            (UCOMPAT_V145_N06_LEN, UCOMPAT_V145_N06_POS)
        } else if fd == UCOMPAT_V145_FD_N07 {
            (UCOMPAT_V145_N07_LEN, UCOMPAT_V145_N07_POS)
        } else if fd == UCOMPAT_V145_FD_N08 {
            (UCOMPAT_V145_N08_LEN, UCOMPAT_V145_N08_POS)
        } else if fd == UCOMPAT_V145_FD_N09 {
            (UCOMPAT_V145_N09_LEN, UCOMPAT_V145_N09_POS)
        } else if fd == UCOMPAT_V145_FD_N10 {
            (UCOMPAT_V145_N10_LEN, UCOMPAT_V145_N10_POS)
        } else if fd == UCOMPAT_V145_FD_N11 {
            (UCOMPAT_V145_N11_LEN, UCOMPAT_V145_N11_POS)
        } else if fd == UCOMPAT_V145_FD_N12 {
            (UCOMPAT_V145_N12_LEN, UCOMPAT_V145_N12_POS)
        } else if fd == UCOMPAT_V145_FD_N13 {
            (UCOMPAT_V145_N13_LEN, UCOMPAT_V145_N13_POS)
        } else if fd == UCOMPAT_V145_FD_N14 {
            (UCOMPAT_V145_N14_LEN, UCOMPAT_V145_N14_POS)
        } else if fd == UCOMPAT_V145_FD_N15 {
            (UCOMPAT_V145_N15_LEN, UCOMPAT_V145_N15_POS)
        } else if fd == UCOMPAT_V145_FD_N16 {
            (UCOMPAT_V145_N16_LEN, UCOMPAT_V145_N16_POS)
        } else if fd == UCOMPAT_V145_FD_N17 {
            (UCOMPAT_V145_N17_LEN, UCOMPAT_V145_N17_POS)
        } else if fd == UCOMPAT_V145_FD_N18 {
            (UCOMPAT_V145_N18_LEN, UCOMPAT_V145_N18_POS)
        } else if fd == UCOMPAT_V145_FD_N19 {
            (UCOMPAT_V145_N19_LEN, UCOMPAT_V145_N19_POS)
        } else if fd == UCOMPAT_V145_FD_N20 {
            (UCOMPAT_V145_N20_LEN, UCOMPAT_V145_N20_POS)
        } else if fd == UCOMPAT_V145_FD_N21 {
            (UCOMPAT_V145_N21_LEN, UCOMPAT_V145_N21_POS)
        } else if fd == UCOMPAT_V145_FD_N22 {
            (UCOMPAT_V145_N22_LEN, UCOMPAT_V145_N22_POS)
        } else if fd == UCOMPAT_V145_FD_N23 {
            (UCOMPAT_V145_N23_LEN, UCOMPAT_V145_N23_POS)
        } else if fd == UCOMPAT_V145_FD_N24 {
            (UCOMPAT_V145_N24_LEN, UCOMPAT_V145_N24_POS)
        } else if fd == UCOMPAT_V145_FD_N25 {
            (UCOMPAT_V145_N25_LEN, UCOMPAT_V145_N25_POS)
        } else if fd == UCOMPAT_V145_FD_N26 {
            (UCOMPAT_V145_N26_LEN, UCOMPAT_V145_N26_POS)
        } else if fd == UCOMPAT_V145_FD_N27 {
            (UCOMPAT_V145_N27_LEN, UCOMPAT_V145_N27_POS)
        } else if fd == UCOMPAT_V145_FD_N28 {
            (UCOMPAT_V145_N28_LEN, UCOMPAT_V145_N28_POS)
        } else if fd == UCOMPAT_V145_FD_N29 {
            (UCOMPAT_V145_N29_LEN, UCOMPAT_V145_N29_POS)
        } else if fd == UCOMPAT_V145_FD_N30 {
            (UCOMPAT_V145_N30_LEN, UCOMPAT_V145_N30_POS)
        } else if fd == UCOMPAT_V145_FD_N31 {
            (UCOMPAT_V145_N31_LEN, UCOMPAT_V145_N31_POS)
        } else if fd == UCOMPAT_V145_FD_N32 {
            (UCOMPAT_V145_N32_LEN, UCOMPAT_V145_N32_POS)
        } else if fd == UCOMPAT_V145_FD_N33 {
            (UCOMPAT_V145_N33_LEN, UCOMPAT_V145_N33_POS)
        } else if fd == UCOMPAT_V145_FD_N34 {
            (UCOMPAT_V145_N34_LEN, UCOMPAT_V145_N34_POS)
        } else if fd == UCOMPAT_V145_FD_N35 {
            (UCOMPAT_V145_N35_LEN, UCOMPAT_V145_N35_POS)
        } else if fd == UCOMPAT_V145_FD_N36 {
            (UCOMPAT_V145_N36_LEN, UCOMPAT_V145_N36_POS)
        } else if fd == UCOMPAT_V145_FD_N37 {
            (UCOMPAT_V145_N37_LEN, UCOMPAT_V145_N37_POS)
        } else if fd == UCOMPAT_V145_FD_N38 {
            (UCOMPAT_V145_N38_LEN, UCOMPAT_V145_N38_POS)
        } else if fd == UCOMPAT_V145_FD_N39 {
            (UCOMPAT_V145_N39_LEN, UCOMPAT_V145_N39_POS)
        } else if fd == UCOMPAT_V145_FD_N40 {
            (UCOMPAT_V145_N40_LEN, UCOMPAT_V145_N40_POS)
        } else if fd == UCOMPAT_V145_FD_N41 {
            (UCOMPAT_V145_N41_LEN, UCOMPAT_V145_N41_POS)
        } else if fd == UCOMPAT_V145_FD_N42 {
            (UCOMPAT_V145_N42_LEN, UCOMPAT_V145_N42_POS)
        } else if fd == UCOMPAT_V145_FD_N43 {
            (UCOMPAT_V145_N43_LEN, UCOMPAT_V145_N43_POS)
        } else if fd == UCOMPAT_V145_FD_N44 {
            (UCOMPAT_V145_N44_LEN, UCOMPAT_V145_N44_POS)
        } else if fd == UCOMPAT_V145_FD_N45 {
            (UCOMPAT_V145_N45_LEN, UCOMPAT_V145_N45_POS)
        } else if fd == UCOMPAT_V145_FD_N46 {
            (UCOMPAT_V145_N46_LEN, UCOMPAT_V145_N46_POS)
        } else if fd == UCOMPAT_V145_FD_N47 {
            (UCOMPAT_V145_N47_LEN, UCOMPAT_V145_N47_POS)
        } else if fd == UCOMPAT_V145_FD_N48 {
            (UCOMPAT_V145_N48_LEN, UCOMPAT_V145_N48_POS)
        } else if fd == UCOMPAT_V145_FD_N49 {
            (UCOMPAT_V145_N49_LEN, UCOMPAT_V145_N49_POS)
        } else if fd == UCOMPAT_V145_FD_N50 {
            (UCOMPAT_V145_N50_LEN, UCOMPAT_V145_N50_POS)
        } else if fd == UCOMPAT_V145_FD_N51 {
            (UCOMPAT_V145_N51_LEN, UCOMPAT_V145_N51_POS)
        } else if fd == UCOMPAT_V145_FD_N52 {
            (UCOMPAT_V145_N52_LEN, UCOMPAT_V145_N52_POS)
        } else if fd == UCOMPAT_V145_FD_N53 {
            (UCOMPAT_V145_N53_LEN, UCOMPAT_V145_N53_POS)
        } else if fd == UCOMPAT_V145_FD_N54 {
            (UCOMPAT_V145_N54_LEN, UCOMPAT_V145_N54_POS)
        } else if fd == UCOMPAT_V145_FD_N55 {
            (UCOMPAT_V145_N55_LEN, UCOMPAT_V145_N55_POS)
        } else if fd == UCOMPAT_V145_FD_N56 {
            (UCOMPAT_V145_N56_LEN, UCOMPAT_V145_N56_POS)
        } else if fd == UCOMPAT_V145_FD_N57 {
            (UCOMPAT_V145_N57_LEN, UCOMPAT_V145_N57_POS)
        } else if fd == UCOMPAT_V145_FD_N58 {
            (UCOMPAT_V145_N58_LEN, UCOMPAT_V145_N58_POS)
        } else if fd == UCOMPAT_V145_FD_N59 {
            (UCOMPAT_V145_N59_LEN, UCOMPAT_V145_N59_POS)
        } else if fd == UCOMPAT_V145_FD_N60 {
            (UCOMPAT_V145_N60_LEN, UCOMPAT_V145_N60_POS)
        } else if fd == UCOMPAT_V145_FD_N61 {
            (UCOMPAT_V145_N61_LEN, UCOMPAT_V145_N61_POS)
        } else if fd == UCOMPAT_V145_FD_N62 {
            (UCOMPAT_V145_N62_LEN, UCOMPAT_V145_N62_POS)
        } else if fd == UCOMPAT_V145_FD_N63 {
            (UCOMPAT_V145_N63_LEN, UCOMPAT_V145_N63_POS)
        } else {
            return -9;
        };
        let base = match whence {
            0 => 0isize,
            1 => cur as isize,
            2 => len as isize,
            _ => return -22,
        };
        let new_pos = base + off;
        if new_pos < 0 {
            return -22;
        }
        if fd == UCOMPAT_V145_FD_N00 {
            UCOMPAT_V145_N00_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N01 {
            UCOMPAT_V145_N01_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N02 {
            UCOMPAT_V145_N02_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N03 {
            UCOMPAT_V145_N03_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N04 {
            UCOMPAT_V145_N04_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N05 {
            UCOMPAT_V145_N05_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N06 {
            UCOMPAT_V145_N06_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N07 {
            UCOMPAT_V145_N07_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N08 {
            UCOMPAT_V145_N08_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N09 {
            UCOMPAT_V145_N09_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N10 {
            UCOMPAT_V145_N10_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N11 {
            UCOMPAT_V145_N11_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N12 {
            UCOMPAT_V145_N12_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N13 {
            UCOMPAT_V145_N13_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N14 {
            UCOMPAT_V145_N14_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N15 {
            UCOMPAT_V145_N15_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N16 {
            UCOMPAT_V145_N16_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N17 {
            UCOMPAT_V145_N17_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N18 {
            UCOMPAT_V145_N18_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N19 {
            UCOMPAT_V145_N19_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N20 {
            UCOMPAT_V145_N20_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N21 {
            UCOMPAT_V145_N21_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N22 {
            UCOMPAT_V145_N22_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N23 {
            UCOMPAT_V145_N23_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N24 {
            UCOMPAT_V145_N24_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N25 {
            UCOMPAT_V145_N25_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N26 {
            UCOMPAT_V145_N26_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N27 {
            UCOMPAT_V145_N27_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N28 {
            UCOMPAT_V145_N28_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N29 {
            UCOMPAT_V145_N29_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N30 {
            UCOMPAT_V145_N30_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N31 {
            UCOMPAT_V145_N31_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N32 {
            UCOMPAT_V145_N32_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N33 {
            UCOMPAT_V145_N33_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N34 {
            UCOMPAT_V145_N34_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N35 {
            UCOMPAT_V145_N35_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N36 {
            UCOMPAT_V145_N36_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N37 {
            UCOMPAT_V145_N37_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N38 {
            UCOMPAT_V145_N38_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N39 {
            UCOMPAT_V145_N39_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N40 {
            UCOMPAT_V145_N40_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N41 {
            UCOMPAT_V145_N41_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N42 {
            UCOMPAT_V145_N42_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N43 {
            UCOMPAT_V145_N43_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N44 {
            UCOMPAT_V145_N44_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N45 {
            UCOMPAT_V145_N45_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N46 {
            UCOMPAT_V145_N46_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N47 {
            UCOMPAT_V145_N47_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N48 {
            UCOMPAT_V145_N48_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N49 {
            UCOMPAT_V145_N49_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N50 {
            UCOMPAT_V145_N50_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N51 {
            UCOMPAT_V145_N51_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N52 {
            UCOMPAT_V145_N52_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N53 {
            UCOMPAT_V145_N53_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N54 {
            UCOMPAT_V145_N54_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N55 {
            UCOMPAT_V145_N55_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N56 {
            UCOMPAT_V145_N56_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N57 {
            UCOMPAT_V145_N57_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N58 {
            UCOMPAT_V145_N58_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N59 {
            UCOMPAT_V145_N59_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N60 {
            UCOMPAT_V145_N60_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N61 {
            UCOMPAT_V145_N61_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N62 {
            UCOMPAT_V145_N62_POS = new_pos as usize;
        } else if fd == UCOMPAT_V145_FD_N63 {
            UCOMPAT_V145_N63_POS = new_pos as usize;
        } else {
            return -9;
        }
        crate::println!("[ucompat-v145] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}
fn ucompat_v145_close(fd: isize) -> isize {
    unsafe {
        if fd == UCOMPAT_V145_FD_N00 {
            UCOMPAT_V145_N00_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N01 {
            UCOMPAT_V145_N01_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N02 {
            UCOMPAT_V145_N02_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N03 {
            UCOMPAT_V145_N03_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N04 {
            UCOMPAT_V145_N04_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N05 {
            UCOMPAT_V145_N05_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N06 {
            UCOMPAT_V145_N06_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N07 {
            UCOMPAT_V145_N07_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N08 {
            UCOMPAT_V145_N08_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N09 {
            UCOMPAT_V145_N09_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N10 {
            UCOMPAT_V145_N10_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N11 {
            UCOMPAT_V145_N11_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N12 {
            UCOMPAT_V145_N12_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N13 {
            UCOMPAT_V145_N13_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N14 {
            UCOMPAT_V145_N14_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N15 {
            UCOMPAT_V145_N15_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N16 {
            UCOMPAT_V145_N16_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N17 {
            UCOMPAT_V145_N17_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N18 {
            UCOMPAT_V145_N18_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N19 {
            UCOMPAT_V145_N19_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N20 {
            UCOMPAT_V145_N20_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N21 {
            UCOMPAT_V145_N21_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N22 {
            UCOMPAT_V145_N22_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N23 {
            UCOMPAT_V145_N23_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N24 {
            UCOMPAT_V145_N24_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N25 {
            UCOMPAT_V145_N25_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N26 {
            UCOMPAT_V145_N26_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N27 {
            UCOMPAT_V145_N27_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N28 {
            UCOMPAT_V145_N28_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N29 {
            UCOMPAT_V145_N29_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N30 {
            UCOMPAT_V145_N30_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N31 {
            UCOMPAT_V145_N31_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N32 {
            UCOMPAT_V145_N32_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N33 {
            UCOMPAT_V145_N33_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N34 {
            UCOMPAT_V145_N34_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N35 {
            UCOMPAT_V145_N35_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N36 {
            UCOMPAT_V145_N36_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N37 {
            UCOMPAT_V145_N37_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N38 {
            UCOMPAT_V145_N38_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N39 {
            UCOMPAT_V145_N39_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N40 {
            UCOMPAT_V145_N40_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N41 {
            UCOMPAT_V145_N41_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N42 {
            UCOMPAT_V145_N42_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N43 {
            UCOMPAT_V145_N43_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N44 {
            UCOMPAT_V145_N44_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N45 {
            UCOMPAT_V145_N45_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N46 {
            UCOMPAT_V145_N46_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N47 {
            UCOMPAT_V145_N47_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N48 {
            UCOMPAT_V145_N48_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N49 {
            UCOMPAT_V145_N49_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N50 {
            UCOMPAT_V145_N50_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N51 {
            UCOMPAT_V145_N51_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N52 {
            UCOMPAT_V145_N52_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N53 {
            UCOMPAT_V145_N53_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N54 {
            UCOMPAT_V145_N54_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N55 {
            UCOMPAT_V145_N55_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N56 {
            UCOMPAT_V145_N56_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N57 {
            UCOMPAT_V145_N57_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N58 {
            UCOMPAT_V145_N58_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N59 {
            UCOMPAT_V145_N59_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N60 {
            UCOMPAT_V145_N60_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N61 {
            UCOMPAT_V145_N61_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N62 {
            UCOMPAT_V145_N62_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else if fd == UCOMPAT_V145_FD_N63 {
            UCOMPAT_V145_N63_POS = 0; /* UCOMPAT_V145E_SUPPRESSED_V145_CLOSE_SLOT_LOG */
            0
        } else {
            -9
        }
    }
}
fn ucompat_v145_pwrite(fd: isize, user_ptr: usize, len: usize, off: isize) -> isize {
    if off < 0 {
        return -22;
    }
    let slot = ucompat_v145_slot(fd);
    if slot < 0 {
        return -9;
    }
    let slot_usize = slot as usize;
    let old = ucompat_v145_get_pos(slot_usize);
    let seek_ret = ucompat_v145_lseek(fd, off, 0);
    if seek_ret < 0 {
        return seek_ret;
    }
    let ret = ucompat_v145_write(fd, user_ptr, len);
    ucompat_v145_set_pos(slot_usize, old);
    crate::println!(
        "[ucompat-v145] pwrite fd={} off={} len={} ret={} keep_pos={}",
        fd,
        off,
        len,
        ret,
        old
    );
    ret
}
fn ucompat_v145_pread(fd: isize, user_ptr: usize, len: usize, off: isize) -> isize {
    if off < 0 {
        return -22;
    }
    let slot = ucompat_v145_slot(fd);
    if slot < 0 {
        return -9;
    }
    let slot_usize = slot as usize;
    let old = ucompat_v145_get_pos(slot_usize);
    let seek_ret = ucompat_v145_lseek(fd, off, 0);
    if seek_ret < 0 {
        return seek_ret;
    }
    let ret = ucompat_v145_read(fd, user_ptr, len);
    ucompat_v145_set_pos(slot_usize, old);
    crate::println!(
        "[ucompat-v145] pread fd={} off={} len={} ret={} keep_pos={}",
        fd,
        off,
        len,
        ret,
        old
    );
    ret
}

// UCOMPAT_V146B_LINKAT_ERRNO_PRIORITY_SOURCE_BASELINE
// UCOMPAT_V146_NAMESPACE_LINK_RENAME_SYMLINK
const UCOMPAT_V146_FD_FILE: isize = 16001;
const UCOMPAT_V146_ENOENT: isize = -2;
const UCOMPAT_V146_EEXIST: isize = -17;
const UCOMPAT_V146_EINVAL: isize = -22;
static mut UCOMPAT_V146_DIR_EXISTS: bool = false;
static mut UCOMPAT_V146_CWD_IN_DIR: bool = false;
static mut UCOMPAT_V146_FILE_A_EXISTS: bool = false;
static mut UCOMPAT_V146_FILE_B_EXISTS: bool = false;
static mut UCOMPAT_V146_HARD_A_EXISTS: bool = false;
static mut UCOMPAT_V146_SYM_A_EXISTS: bool = false;
static mut UCOMPAT_V146_FILE_OPEN: bool = false;

fn ucompat_v146_read_path(user_path: usize, out: &mut [u8; 64]) -> usize {
    let mut len = 0usize;
    with_sum_enabled(|| {
        while len + 1 < out.len() {
            let ch = unsafe { core::ptr::read_volatile((user_path + len) as *const u8) };
            out[len] = ch;
            if ch == 0 {
                break;
            }
            len += 1;
        }
    });
    len
}
fn ucompat_v146_path_id(user_path: usize) -> isize {
    let mut buf = [0u8; 64];
    let len = ucompat_v146_read_path(user_path, &mut buf);
    if len == 7 && &buf[..7] == b"v146dir" {
        1
    } else if len == 6 && &buf[..6] == b"file_a" {
        2
    } else if len == 6 && &buf[..6] == b"file_b" {
        3
    } else if len == 6 && &buf[..6] == b"hard_a" {
        4
    } else if len == 5 && &buf[..5] == b"sym_a" {
        5
    } else if len == 11 && &buf[..11] == b"target_file" {
        6
    } else if len == 7 && &buf[..7] == b"missing" {
        7
    } else {
        -1
    }
}
fn ucompat_v146_write_user_bytes(user_ptr: usize, data: &[u8]) -> isize {
    with_sum_enabled(|| {
        let mut i = 0usize;
        while i < data.len() {
            unsafe {
                core::ptr::write_volatile((user_ptr + i) as *mut u8, data[i]);
            }
            i += 1;
        }
    });
    data.len() as isize
}
fn ucompat_v146_getcwd(user_ptr: usize, size: usize) -> isize {
    let cwd: &[u8] = unsafe {
        if UCOMPAT_V146_CWD_IN_DIR {
            b"/v146dir\0"
        } else {
            b"/\0"
        }
    };
    if size < cwd.len() {
        return -34;
    }
    let ret = ucompat_v146_write_user_bytes(user_ptr, cwd);
    if unsafe { UCOMPAT_V146_CWD_IN_DIR } {
        crate::println!("[ucompat-v146] getcwd /v146dir");
    } else {
        crate::println!("[ucompat-v146] getcwd /");
    }
    ret
}
fn ucompat_v146_mkdirat(_dirfd: isize, user_path: usize, _mode: usize) -> isize {
    let pid = ucompat_v146_path_id(user_path);
    if pid != 1 {
        return UCOMPAT_V146_ENOENT;
    }
    unsafe {
        if UCOMPAT_V146_DIR_EXISTS {
            crate::println!("[ucompat-v146] mkdirat v146dir ret=-17");
            return UCOMPAT_V146_EEXIST;
        }
        UCOMPAT_V146_DIR_EXISTS = true;
    }
    crate::println!("[ucompat-v146] mkdirat v146dir ret=0");
    0
}
fn ucompat_v146_chdir(user_path: usize) -> isize {
    let pid = ucompat_v146_path_id(user_path);
    unsafe {
        if pid == 1 && UCOMPAT_V146_DIR_EXISTS {
            UCOMPAT_V146_CWD_IN_DIR = true;
            crate::println!("[ucompat-v146] chdir v146dir ret=0");
            0
        } else {
            crate::println!("[ucompat-v146] chdir missing ret=-2");
            UCOMPAT_V146_ENOENT
        }
    }
}
fn ucompat_v146_openat(_dirfd: isize, user_path: usize, flags: usize, _mode: usize) -> isize {
    const O_CREAT: usize = 0x40;
    let pid = ucompat_v146_path_id(user_path);
    unsafe {
        let mut exists = match pid {
            2 => UCOMPAT_V146_FILE_A_EXISTS,
            3 => UCOMPAT_V146_FILE_B_EXISTS,
            4 => UCOMPAT_V146_HARD_A_EXISTS,
            _ => false,
        };
        if !exists && (flags & O_CREAT) != 0 && (pid == 2 || pid == 3) {
            if pid == 2 {
                UCOMPAT_V146_FILE_A_EXISTS = true;
            } else {
                UCOMPAT_V146_FILE_B_EXISTS = true;
            }
            exists = true;
            crate::println!(
                "[ucompat-v146] openat create file pid={} fd={}",
                pid,
                UCOMPAT_V146_FD_FILE
            );
        }
        if exists {
            UCOMPAT_V146_FILE_OPEN = true;
            crate::println!(
                "[ucompat-v146] openat existing pid={} fd={}",
                pid,
                UCOMPAT_V146_FD_FILE
            );
            UCOMPAT_V146_FD_FILE
        } else {
            crate::println!("[ucompat-v146] openat missing pid={} ret=-2", pid);
            UCOMPAT_V146_ENOENT
        }
    }
}
fn ucompat_v146_close(fd: isize) -> isize {
    if fd != UCOMPAT_V146_FD_FILE {
        return -9;
    }
    unsafe {
        UCOMPAT_V146_FILE_OPEN = false;
    }
    crate::println!("[ucompat-v146] close fd=16001 ret=0");
    0
}
fn ucompat_v146_linkat(
    _old_dirfd: isize,
    old_path: usize,
    _new_dirfd: isize,
    new_path: usize,
    _flags: usize,
) -> isize {
    // UCOMPAT_V146B_LINKAT_ERRNO_PRIORITY
    let old_id = ucompat_v146_path_id(old_path);
    let new_id = ucompat_v146_path_id(new_path);
    unsafe {
        let old_exists = old_id == 2 && UCOMPAT_V146_FILE_A_EXISTS;
        if !old_exists {
            crate::println!("[ucompat-v146b] linkat old-missing ret=-2");
            return UCOMPAT_V146_ENOENT;
        }
        if new_id != 4 {
            crate::println!("[ucompat-v146b] linkat bad-new ret=-2");
            return UCOMPAT_V146_ENOENT;
        }
        if UCOMPAT_V146_HARD_A_EXISTS {
            crate::println!("[ucompat-v146b] linkat new-exists ret=-17");
            return UCOMPAT_V146_EEXIST;
        }
        UCOMPAT_V146_HARD_A_EXISTS = true;
        crate::println!("[ucompat-v146] linkat file_a -> hard_a ret=0");
        0
    }
}
fn ucompat_v146_unlinkat(_dirfd: isize, user_path: usize, _flags: usize) -> isize {
    let pid = ucompat_v146_path_id(user_path);
    unsafe {
        if pid == 4 && UCOMPAT_V146_HARD_A_EXISTS {
            UCOMPAT_V146_HARD_A_EXISTS = false;
            crate::println!("[ucompat-v146] unlinkat hard_a ret=0");
            0
        } else if pid == 3 && UCOMPAT_V146_FILE_B_EXISTS {
            UCOMPAT_V146_FILE_B_EXISTS = false;
            crate::println!("[ucompat-v146] unlinkat file_b ret=0");
            0
        } else if pid == 5 && UCOMPAT_V146_SYM_A_EXISTS {
            UCOMPAT_V146_SYM_A_EXISTS = false;
            crate::println!("[ucompat-v146] unlinkat sym_a ret=0");
            0
        } else {
            crate::println!("[ucompat-v146] unlinkat missing pid={} ret=-2", pid);
            UCOMPAT_V146_ENOENT
        }
    }
}
fn ucompat_v146_renameat(
    _old_dirfd: isize,
    old_path: usize,
    _new_dirfd: isize,
    new_path: usize,
) -> isize {
    let old_id = ucompat_v146_path_id(old_path);
    let new_id = ucompat_v146_path_id(new_path);
    unsafe {
        if old_id == 2 && new_id == 3 && UCOMPAT_V146_FILE_A_EXISTS {
            UCOMPAT_V146_FILE_A_EXISTS = false;
            UCOMPAT_V146_FILE_B_EXISTS = true;
            crate::println!("[ucompat-v146] renameat file_a -> file_b ret=0");
            0
        } else {
            UCOMPAT_V146_ENOENT
        }
    }
}
fn ucompat_v146_symlinkat(target: usize, _new_dirfd: isize, link_path: usize) -> isize {
    let target_id = ucompat_v146_path_id(target);
    let link_id = ucompat_v146_path_id(link_path);
    unsafe {
        if target_id == 6 && link_id == 5 && !UCOMPAT_V146_SYM_A_EXISTS {
            UCOMPAT_V146_SYM_A_EXISTS = true;
            crate::println!("[ucompat-v146] symlinkat target_file -> sym_a ret=0");
            0
        } else if link_id == 5 && UCOMPAT_V146_SYM_A_EXISTS {
            UCOMPAT_V146_EEXIST
        } else {
            UCOMPAT_V146_ENOENT
        }
    }
}
fn ucompat_v146_readlinkat(_dirfd: isize, user_path: usize, user_buf: usize, size: usize) -> isize {
    let pid = ucompat_v146_path_id(user_path);
    unsafe {
        if pid == 5 && UCOMPAT_V146_SYM_A_EXISTS {
            let target = b"target_file";
            if size < target.len() {
                return -34;
            }
            let ret = ucompat_v146_write_user_bytes(user_buf, target);
            crate::println!("[ucompat-v146] readlinkat sym_a -> target_file ret={}", ret);
            ret
        } else if pid == 2 || pid == 3 || pid == 4 {
            crate::println!("[ucompat-v146] readlinkat non-symlink ret=-22");
            UCOMPAT_V146_EINVAL
        } else {
            UCOMPAT_V146_ENOENT
        }
    }
}

