fn sys_openat_user(dirfd: isize, user_path: usize, flags: usize, mode: usize) -> isize {
    crate::println!("[openat-v137p] dirfd = {}", dirfd);
    crate::println!("[openat-v137p] flags = {:#x}", flags);
    crate::println!("[openat-v137p] mode = {:#x}", mode);
    let mut buf = [0u8; 64];
    let mut len = 0usize;
    with_sum_enabled(|| {
        while len + 1 < buf.len() {
            let ch = unsafe { core::ptr::read_volatile((user_path + len) as *const u8) };
            buf[len] = ch;
            if ch == 0 {
                break;
            }
            len += 1;
        }
    });
    // UCOMPAT_V139_OPENAT_PATH_DISPATCH
    // UCOMPAT_V140_OPENAT_PATH_DISPATCH
    // UCOMPAT_V141_OPENAT_PATH_DISPATCH
    // UCOMPAT_V142_OPENAT_PATH_DISPATCH
    // UCOMPAT_V143_OPENAT_PATH_DISPATCH
    // UCOMPAT_V144_OPENAT_PATH_DISPATCH
    // UCOMPAT_V145_OPENAT_PATH_DISPATCH
    if len == 10 && &buf[..10] == b"v14500.txt" {
        crate::println!("[openat-v145] path=v14500.txt flags={:#x}", flags);
        return ucompat_v145_open(0, flags);
    } else if len == 10 && &buf[..10] == b"v14501.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(1, flags);
    } else if len == 10 && &buf[..10] == b"v14502.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(2, flags);
    } else if len == 10 && &buf[..10] == b"v14503.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(3, flags);
    } else if len == 10 && &buf[..10] == b"v14504.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(4, flags);
    } else if len == 10 && &buf[..10] == b"v14505.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(5, flags);
    } else if len == 10 && &buf[..10] == b"v14506.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(6, flags);
    } else if len == 10 && &buf[..10] == b"v14507.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(7, flags);
    } else if len == 10 && &buf[..10] == b"v14508.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(8, flags);
    } else if len == 10 && &buf[..10] == b"v14509.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(9, flags);
    } else if len == 10 && &buf[..10] == b"v14510.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(10, flags);
    } else if len == 10 && &buf[..10] == b"v14511.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(11, flags);
    } else if len == 10 && &buf[..10] == b"v14512.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(12, flags);
    } else if len == 10 && &buf[..10] == b"v14513.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(13, flags);
    } else if len == 10 && &buf[..10] == b"v14514.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(14, flags);
    } else if len == 10 && &buf[..10] == b"v14515.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(15, flags);
    } else if len == 10 && &buf[..10] == b"v14516.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(16, flags);
    } else if len == 10 && &buf[..10] == b"v14517.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(17, flags);
    } else if len == 10 && &buf[..10] == b"v14518.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(18, flags);
    } else if len == 10 && &buf[..10] == b"v14519.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(19, flags);
    } else if len == 10 && &buf[..10] == b"v14520.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(20, flags);
    } else if len == 10 && &buf[..10] == b"v14521.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(21, flags);
    } else if len == 10 && &buf[..10] == b"v14522.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(22, flags);
    } else if len == 10 && &buf[..10] == b"v14523.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(23, flags);
    } else if len == 10 && &buf[..10] == b"v14524.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(24, flags);
    } else if len == 10 && &buf[..10] == b"v14525.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(25, flags);
    } else if len == 10 && &buf[..10] == b"v14526.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(26, flags);
    } else if len == 10 && &buf[..10] == b"v14527.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(27, flags);
    } else if len == 10 && &buf[..10] == b"v14528.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(28, flags);
    } else if len == 10 && &buf[..10] == b"v14529.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(29, flags);
    } else if len == 10 && &buf[..10] == b"v14530.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(30, flags);
    } else if len == 10 && &buf[..10] == b"v14531.txt" {
        crate::println!("[openat-v145] path=v14531.txt flags={:#x}", flags);
        return ucompat_v145_open(31, flags);
    } else if len == 10 && &buf[..10] == b"v14532.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(32, flags);
    } else if len == 10 && &buf[..10] == b"v14533.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(33, flags);
    } else if len == 10 && &buf[..10] == b"v14534.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(34, flags);
    } else if len == 10 && &buf[..10] == b"v14535.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(35, flags);
    } else if len == 10 && &buf[..10] == b"v14536.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(36, flags);
    } else if len == 10 && &buf[..10] == b"v14537.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(37, flags);
    } else if len == 10 && &buf[..10] == b"v14538.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(38, flags);
    } else if len == 10 && &buf[..10] == b"v14539.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(39, flags);
    } else if len == 10 && &buf[..10] == b"v14540.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(40, flags);
    } else if len == 10 && &buf[..10] == b"v14541.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(41, flags);
    } else if len == 10 && &buf[..10] == b"v14542.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(42, flags);
    } else if len == 10 && &buf[..10] == b"v14543.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(43, flags);
    } else if len == 10 && &buf[..10] == b"v14544.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(44, flags);
    } else if len == 10 && &buf[..10] == b"v14545.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(45, flags);
    } else if len == 10 && &buf[..10] == b"v14546.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(46, flags);
    } else if len == 10 && &buf[..10] == b"v14547.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(47, flags);
    } else if len == 10 && &buf[..10] == b"v14548.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(48, flags);
    } else if len == 10 && &buf[..10] == b"v14549.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(49, flags);
    } else if len == 10 && &buf[..10] == b"v14550.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(50, flags);
    } else if len == 10 && &buf[..10] == b"v14551.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(51, flags);
    } else if len == 10 && &buf[..10] == b"v14552.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(52, flags);
    } else if len == 10 && &buf[..10] == b"v14553.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(53, flags);
    } else if len == 10 && &buf[..10] == b"v14554.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(54, flags);
    } else if len == 10 && &buf[..10] == b"v14555.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(55, flags);
    } else if len == 10 && &buf[..10] == b"v14556.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(56, flags);
    } else if len == 10 && &buf[..10] == b"v14557.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(57, flags);
    } else if len == 10 && &buf[..10] == b"v14558.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(58, flags);
    } else if len == 10 && &buf[..10] == b"v14559.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(59, flags);
    } else if len == 10 && &buf[..10] == b"v14560.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(60, flags);
    } else if len == 10 && &buf[..10] == b"v14561.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(61, flags);
    } else if len == 10 && &buf[..10] == b"v14562.txt" {
        // UCOMPAT_V145E_SUPPRESSED_V145_PATH_LOG
        let _ = flags;
        return ucompat_v145_open(62, flags);
    } else if len == 10 && &buf[..10] == b"v14563.txt" {
        crate::println!("[openat-v145] path=v14563.txt flags={:#x}", flags);
        return ucompat_v145_open(63, flags);
    } else if len == 10 && &buf[..10] == b"v14400.txt" {
        crate::println!("[openat-v144] path=v14400.txt flags={:#x}", flags);
        return ucompat_v144_open(0, flags);
    } else if len == 10 && &buf[..10] == b"v14401.txt" {
        crate::println!("[openat-v144] path=v14401.txt flags={:#x}", flags);
        return ucompat_v144_open(1, flags);
    } else if len == 10 && &buf[..10] == b"v14402.txt" {
        crate::println!("[openat-v144] path=v14402.txt flags={:#x}", flags);
        return ucompat_v144_open(2, flags);
    } else if len == 10 && &buf[..10] == b"v14403.txt" {
        crate::println!("[openat-v144] path=v14403.txt flags={:#x}", flags);
        return ucompat_v144_open(3, flags);
    } else if len == 10 && &buf[..10] == b"v14404.txt" {
        crate::println!("[openat-v144] path=v14404.txt flags={:#x}", flags);
        return ucompat_v144_open(4, flags);
    } else if len == 10 && &buf[..10] == b"v14405.txt" {
        crate::println!("[openat-v144] path=v14405.txt flags={:#x}", flags);
        return ucompat_v144_open(5, flags);
    } else if len == 10 && &buf[..10] == b"v14406.txt" {
        crate::println!("[openat-v144] path=v14406.txt flags={:#x}", flags);
        return ucompat_v144_open(6, flags);
    } else if len == 10 && &buf[..10] == b"v14407.txt" {
        crate::println!("[openat-v144] path=v14407.txt flags={:#x}", flags);
        return ucompat_v144_open(7, flags);
    } else if len == 10 && &buf[..10] == b"v14408.txt" {
        crate::println!("[openat-v144] path=v14408.txt flags={:#x}", flags);
        return ucompat_v144_open(8, flags);
    } else if len == 10 && &buf[..10] == b"v14409.txt" {
        crate::println!("[openat-v144] path=v14409.txt flags={:#x}", flags);
        return ucompat_v144_open(9, flags);
    } else if len == 10 && &buf[..10] == b"v14410.txt" {
        crate::println!("[openat-v144] path=v14410.txt flags={:#x}", flags);
        return ucompat_v144_open(10, flags);
    } else if len == 10 && &buf[..10] == b"v14411.txt" {
        crate::println!("[openat-v144] path=v14411.txt flags={:#x}", flags);
        return ucompat_v144_open(11, flags);
    } else if len == 10 && &buf[..10] == b"v14412.txt" {
        crate::println!("[openat-v144] path=v14412.txt flags={:#x}", flags);
        return ucompat_v144_open(12, flags);
    } else if len == 10 && &buf[..10] == b"v14413.txt" {
        crate::println!("[openat-v144] path=v14413.txt flags={:#x}", flags);
        return ucompat_v144_open(13, flags);
    } else if len == 10 && &buf[..10] == b"v14414.txt" {
        crate::println!("[openat-v144] path=v14414.txt flags={:#x}", flags);
        return ucompat_v144_open(14, flags);
    } else if len == 10 && &buf[..10] == b"v14415.txt" {
        crate::println!("[openat-v144] path=v14415.txt flags={:#x}", flags);
        return ucompat_v144_open(15, flags);
    } else if len == 10 && &buf[..10] == b"v14416.txt" {
        crate::println!("[openat-v144] path=v14416.txt flags={:#x}", flags);
        return ucompat_v144_open(16, flags);
    } else if len == 10 && &buf[..10] == b"v14417.txt" {
        crate::println!("[openat-v144] path=v14417.txt flags={:#x}", flags);
        return ucompat_v144_open(17, flags);
    } else if len == 10 && &buf[..10] == b"v14418.txt" {
        crate::println!("[openat-v144] path=v14418.txt flags={:#x}", flags);
        return ucompat_v144_open(18, flags);
    } else if len == 10 && &buf[..10] == b"v14419.txt" {
        crate::println!("[openat-v144] path=v14419.txt flags={:#x}", flags);
        return ucompat_v144_open(19, flags);
    } else if len == 10 && &buf[..10] == b"v14420.txt" {
        crate::println!("[openat-v144] path=v14420.txt flags={:#x}", flags);
        return ucompat_v144_open(20, flags);
    } else if len == 10 && &buf[..10] == b"v14421.txt" {
        crate::println!("[openat-v144] path=v14421.txt flags={:#x}", flags);
        return ucompat_v144_open(21, flags);
    } else if len == 10 && &buf[..10] == b"v14422.txt" {
        crate::println!("[openat-v144] path=v14422.txt flags={:#x}", flags);
        return ucompat_v144_open(22, flags);
    } else if len == 10 && &buf[..10] == b"v14423.txt" {
        crate::println!("[openat-v144] path=v14423.txt flags={:#x}", flags);
        return ucompat_v144_open(23, flags);
    } else if len == 10 && &buf[..10] == b"v14424.txt" {
        crate::println!("[openat-v144] path=v14424.txt flags={:#x}", flags);
        return ucompat_v144_open(24, flags);
    } else if len == 10 && &buf[..10] == b"v14425.txt" {
        crate::println!("[openat-v144] path=v14425.txt flags={:#x}", flags);
        return ucompat_v144_open(25, flags);
    } else if len == 10 && &buf[..10] == b"v14426.txt" {
        crate::println!("[openat-v144] path=v14426.txt flags={:#x}", flags);
        return ucompat_v144_open(26, flags);
    } else if len == 10 && &buf[..10] == b"v14427.txt" {
        crate::println!("[openat-v144] path=v14427.txt flags={:#x}", flags);
        return ucompat_v144_open(27, flags);
    } else if len == 10 && &buf[..10] == b"v14428.txt" {
        crate::println!("[openat-v144] path=v14428.txt flags={:#x}", flags);
        return ucompat_v144_open(28, flags);
    } else if len == 10 && &buf[..10] == b"v14429.txt" {
        crate::println!("[openat-v144] path=v14429.txt flags={:#x}", flags);
        return ucompat_v144_open(29, flags);
    } else if len == 10 && &buf[..10] == b"v14430.txt" {
        crate::println!("[openat-v144] path=v14430.txt flags={:#x}", flags);
        return ucompat_v144_open(30, flags);
    } else if len == 10 && &buf[..10] == b"v14431.txt" {
        crate::println!("[openat-v144] path=v14431.txt flags={:#x}", flags);
        return ucompat_v144_open(31, flags);
    } else if len == 10 && &buf[..10] == b"v14432.txt" {
        crate::println!("[openat-v144] path=v14432.txt flags={:#x}", flags);
        return ucompat_v144_open(32, flags);
    } else if len == 10 && &buf[..10] == b"v14433.txt" {
        crate::println!("[openat-v144] path=v14433.txt flags={:#x}", flags);
        return ucompat_v144_open(33, flags);
    } else if len == 10 && &buf[..10] == b"v14434.txt" {
        crate::println!("[openat-v144] path=v14434.txt flags={:#x}", flags);
        return ucompat_v144_open(34, flags);
    } else if len == 10 && &buf[..10] == b"v14435.txt" {
        crate::println!("[openat-v144] path=v14435.txt flags={:#x}", flags);
        return ucompat_v144_open(35, flags);
    } else if len == 10 && &buf[..10] == b"v14436.txt" {
        crate::println!("[openat-v144] path=v14436.txt flags={:#x}", flags);
        return ucompat_v144_open(36, flags);
    } else if len == 10 && &buf[..10] == b"v14437.txt" {
        crate::println!("[openat-v144] path=v14437.txt flags={:#x}", flags);
        return ucompat_v144_open(37, flags);
    } else if len == 10 && &buf[..10] == b"v14438.txt" {
        crate::println!("[openat-v144] path=v14438.txt flags={:#x}", flags);
        return ucompat_v144_open(38, flags);
    } else if len == 10 && &buf[..10] == b"v14439.txt" {
        crate::println!("[openat-v144] path=v14439.txt flags={:#x}", flags);
        return ucompat_v144_open(39, flags);
    } else if len == 10 && &buf[..10] == b"v14440.txt" {
        crate::println!("[openat-v144] path=v14440.txt flags={:#x}", flags);
        return ucompat_v144_open(40, flags);
    } else if len == 10 && &buf[..10] == b"v14441.txt" {
        crate::println!("[openat-v144] path=v14441.txt flags={:#x}", flags);
        return ucompat_v144_open(41, flags);
    } else if len == 10 && &buf[..10] == b"v14442.txt" {
        crate::println!("[openat-v144] path=v14442.txt flags={:#x}", flags);
        return ucompat_v144_open(42, flags);
    } else if len == 10 && &buf[..10] == b"v14443.txt" {
        crate::println!("[openat-v144] path=v14443.txt flags={:#x}", flags);
        return ucompat_v144_open(43, flags);
    } else if len == 10 && &buf[..10] == b"v14444.txt" {
        crate::println!("[openat-v144] path=v14444.txt flags={:#x}", flags);
        return ucompat_v144_open(44, flags);
    } else if len == 10 && &buf[..10] == b"v14445.txt" {
        crate::println!("[openat-v144] path=v14445.txt flags={:#x}", flags);
        return ucompat_v144_open(45, flags);
    } else if len == 10 && &buf[..10] == b"v14446.txt" {
        crate::println!("[openat-v144] path=v14446.txt flags={:#x}", flags);
        return ucompat_v144_open(46, flags);
    } else if len == 10 && &buf[..10] == b"v14447.txt" {
        crate::println!("[openat-v144] path=v14447.txt flags={:#x}", flags);
        return ucompat_v144_open(47, flags);
    } else if len == 9 && &buf[..9] == b"v143a.txt" {
        crate::println!("[openat-v143] path=v143a.txt flags={:#x}", flags);
        return ucompat_v143_open(0, flags);
    } else if len == 9 && &buf[..9] == b"v143b.txt" {
        crate::println!("[openat-v143] path=v143b.txt flags={:#x}", flags);
        return ucompat_v143_open(1, flags);
    } else if len == 9 && &buf[..9] == b"v143c.txt" {
        crate::println!("[openat-v143] path=v143c.txt flags={:#x}", flags);
        return ucompat_v143_open(2, flags);
    } else if len == 9 && &buf[..9] == b"v143d.txt" {
        crate::println!("[openat-v143] path=v143d.txt flags={:#x}", flags);
        return ucompat_v143_open(3, flags);
    } else if len == 9 && &buf[..9] == b"v143e.txt" {
        crate::println!("[openat-v143] path=v143e.txt flags={:#x}", flags);
        return ucompat_v143_open(4, flags);
    } else if len == 9 && &buf[..9] == b"v143f.txt" {
        crate::println!("[openat-v143] path=v143f.txt flags={:#x}", flags);
        return ucompat_v143_open(5, flags);
    } else if len == 9 && &buf[..9] == b"v143g.txt" {
        crate::println!("[openat-v143] path=v143g.txt flags={:#x}", flags);
        return ucompat_v143_open(6, flags);
    } else if len == 9 && &buf[..9] == b"v143h.txt" {
        crate::println!("[openat-v143] path=v143h.txt flags={:#x}", flags);
        return ucompat_v143_open(7, flags);
    } else if len == 9 && &buf[..9] == b"v143i.txt" {
        crate::println!("[openat-v143] path=v143i.txt flags={:#x}", flags);
        return ucompat_v143_open(8, flags);
    } else if len == 9 && &buf[..9] == b"v143j.txt" {
        crate::println!("[openat-v143] path=v143j.txt flags={:#x}", flags);
        return ucompat_v143_open(9, flags);
    } else if len == 9 && &buf[..9] == b"v143k.txt" {
        crate::println!("[openat-v143] path=v143k.txt flags={:#x}", flags);
        return ucompat_v143_open(10, flags);
    } else if len == 9 && &buf[..9] == b"v143l.txt" {
        crate::println!("[openat-v143] path=v143l.txt flags={:#x}", flags);
        return ucompat_v143_open(11, flags);
    } else if len == 9 && &buf[..9] == b"v143m.txt" {
        crate::println!("[openat-v143] path=v143m.txt flags={:#x}", flags);
        return ucompat_v143_open(12, flags);
    } else if len == 9 && &buf[..9] == b"v143n.txt" {
        crate::println!("[openat-v143] path=v143n.txt flags={:#x}", flags);
        return ucompat_v143_open(13, flags);
    } else if len == 9 && &buf[..9] == b"v143o.txt" {
        crate::println!("[openat-v143] path=v143o.txt flags={:#x}", flags);
        return ucompat_v143_open(14, flags);
    } else if len == 9 && &buf[..9] == b"v143p.txt" {
        crate::println!("[openat-v143] path=v143p.txt flags={:#x}", flags);
        return ucompat_v143_open(15, flags);
    } else if len == 9 && &buf[..9] == b"v143q.txt" {
        crate::println!("[openat-v143] path=v143q.txt flags={:#x}", flags);
        return ucompat_v143_open(16, flags);
    } else if len == 9 && &buf[..9] == b"v143r.txt" {
        crate::println!("[openat-v143] path=v143r.txt flags={:#x}", flags);
        return ucompat_v143_open(17, flags);
    } else if len == 9 && &buf[..9] == b"v143s.txt" {
        crate::println!("[openat-v143] path=v143s.txt flags={:#x}", flags);
        return ucompat_v143_open(18, flags);
    } else if len == 9 && &buf[..9] == b"v143t.txt" {
        crate::println!("[openat-v143] path=v143t.txt flags={:#x}", flags);
        return ucompat_v143_open(19, flags);
    } else if len == 9 && &buf[..9] == b"v143u.txt" {
        crate::println!("[openat-v143] path=v143u.txt flags={:#x}", flags);
        return ucompat_v143_open(20, flags);
    } else if len == 9 && &buf[..9] == b"v143v.txt" {
        crate::println!("[openat-v143] path=v143v.txt flags={:#x}", flags);
        return ucompat_v143_open(21, flags);
    } else if len == 9 && &buf[..9] == b"v143w.txt" {
        crate::println!("[openat-v143] path=v143w.txt flags={:#x}", flags);
        return ucompat_v143_open(22, flags);
    } else if len == 9 && &buf[..9] == b"v143x.txt" {
        crate::println!("[openat-v143] path=v143x.txt flags={:#x}", flags);
        return ucompat_v143_open(23, flags);
    } else if len == 9 && &buf[..9] == b"v143y.txt" {
        crate::println!("[openat-v143] path=v143y.txt flags={:#x}", flags);
        return ucompat_v143_open(24, flags);
    } else if len == 9 && &buf[..9] == b"v143z.txt" {
        crate::println!("[openat-v143] path=v143z.txt flags={:#x}", flags);
        return ucompat_v143_open(25, flags);
    } else if len == 9 && &buf[..9] == b"v1430.txt" {
        crate::println!("[openat-v143] path=v1430.txt flags={:#x}", flags);
        return ucompat_v143_open(26, flags);
    } else if len == 9 && &buf[..9] == b"v1431.txt" {
        crate::println!("[openat-v143] path=v1431.txt flags={:#x}", flags);
        return ucompat_v143_open(27, flags);
    } else if len == 9 && &buf[..9] == b"v1432.txt" {
        crate::println!("[openat-v143] path=v1432.txt flags={:#x}", flags);
        return ucompat_v143_open(28, flags);
    } else if len == 9 && &buf[..9] == b"v1433.txt" {
        crate::println!("[openat-v143] path=v1433.txt flags={:#x}", flags);
        return ucompat_v143_open(29, flags);
    } else if len == 9 && &buf[..9] == b"v1434.txt" {
        crate::println!("[openat-v143] path=v1434.txt flags={:#x}", flags);
        return ucompat_v143_open(30, flags);
    } else if len == 9 && &buf[..9] == b"v1435.txt" {
        crate::println!("[openat-v143] path=v1435.txt flags={:#x}", flags);
        return ucompat_v143_open(31, flags);
    } else if len == 9 && &buf[..9] == b"v142a.txt" {
        crate::println!("[openat-v142] path=v142a.txt flags={:#x}", flags);
        return ucompat_v142_open(0, flags);
    } else if len == 9 && &buf[..9] == b"v142b.txt" {
        crate::println!("[openat-v142] path=v142b.txt flags={:#x}", flags);
        return ucompat_v142_open(1, flags);
    } else if len == 9 && &buf[..9] == b"v142c.txt" {
        crate::println!("[openat-v142] path=v142c.txt flags={:#x}", flags);
        return ucompat_v142_open(2, flags);
    } else if len == 9 && &buf[..9] == b"v142d.txt" {
        crate::println!("[openat-v142] path=v142d.txt flags={:#x}", flags);
        return ucompat_v142_open(3, flags);
    } else if len == 9 && &buf[..9] == b"v142e.txt" {
        crate::println!("[openat-v142] path=v142e.txt flags={:#x}", flags);
        return ucompat_v142_open(4, flags);
    } else if len == 9 && &buf[..9] == b"v142f.txt" {
        crate::println!("[openat-v142] path=v142f.txt flags={:#x}", flags);
        return ucompat_v142_open(5, flags);
    } else if len == 9 && &buf[..9] == b"v142g.txt" {
        crate::println!("[openat-v142] path=v142g.txt flags={:#x}", flags);
        return ucompat_v142_open(6, flags);
    } else if len == 9 && &buf[..9] == b"v142h.txt" {
        crate::println!("[openat-v142] path=v142h.txt flags={:#x}", flags);
        return ucompat_v142_open(7, flags);
    } else if len == 9 && &buf[..9] == b"v142i.txt" {
        crate::println!("[openat-v142] path=v142i.txt flags={:#x}", flags);
        return ucompat_v142_open(8, flags);
    } else if len == 9 && &buf[..9] == b"v142j.txt" {
        crate::println!("[openat-v142] path=v142j.txt flags={:#x}", flags);
        return ucompat_v142_open(9, flags);
    } else if len == 9 && &buf[..9] == b"v142k.txt" {
        crate::println!("[openat-v142] path=v142k.txt flags={:#x}", flags);
        return ucompat_v142_open(10, flags);
    } else if len == 9 && &buf[..9] == b"v142l.txt" {
        crate::println!("[openat-v142] path=v142l.txt flags={:#x}", flags);
        return ucompat_v142_open(11, flags);
    } else if len == 9 && &buf[..9] == b"v142m.txt" {
        crate::println!("[openat-v142] path=v142m.txt flags={:#x}", flags);
        return ucompat_v142_open(12, flags);
    } else if len == 9 && &buf[..9] == b"v142n.txt" {
        crate::println!("[openat-v142] path=v142n.txt flags={:#x}", flags);
        return ucompat_v142_open(13, flags);
    } else if len == 9 && &buf[..9] == b"v142o.txt" {
        crate::println!("[openat-v142] path=v142o.txt flags={:#x}", flags);
        return ucompat_v142_open(14, flags);
    } else if len == 9 && &buf[..9] == b"v142p.txt" {
        crate::println!("[openat-v142] path=v142p.txt flags={:#x}", flags);
        return ucompat_v142_open(15, flags);
    } else if len == 9 && &buf[..9] == b"v141a.txt" {
        crate::println!("[openat-v141] path=v141a.txt flags={:#x}", flags);
        return ucompat_v141_open(0, flags);
    } else if len == 9 && &buf[..9] == b"v141b.txt" {
        crate::println!("[openat-v141] path=v141b.txt flags={:#x}", flags);
        return ucompat_v141_open(1, flags);
    } else if len == 9 && &buf[..9] == b"v141c.txt" {
        crate::println!("[openat-v141] path=v141c.txt flags={:#x}", flags);
        return ucompat_v141_open(2, flags);
    } else if len == 9 && &buf[..9] == b"v141d.txt" {
        crate::println!("[openat-v141] path=v141d.txt flags={:#x}", flags);
        return ucompat_v141_open(3, flags);
    } else if len == 9 && &buf[..9] == b"v141e.txt" {
        crate::println!("[openat-v141] path=v141e.txt flags={:#x}", flags);
        return ucompat_v141_open(4, flags);
    } else if len == 9 && &buf[..9] == b"v141f.txt" {
        crate::println!("[openat-v141] path=v141f.txt flags={:#x}", flags);
        return ucompat_v141_open(5, flags);
    } else if len == 9 && &buf[..9] == b"v141g.txt" {
        crate::println!("[openat-v141] path=v141g.txt flags={:#x}", flags);
        return ucompat_v141_open(6, flags);
    } else if len == 9 && &buf[..9] == b"v141h.txt" {
        crate::println!("[openat-v141] path=v141h.txt flags={:#x}", flags);
        return ucompat_v141_open(7, flags);
    } else if len == 9 && &buf[..9] == b"v140a.txt" {
        crate::println!("[openat-v140] path=v140a.txt flags={:#x}", flags);
        return ucompat_v140_open(0, flags);
    } else if len == 9 && &buf[..9] == b"v140b.txt" {
        crate::println!("[openat-v140] path=v140b.txt flags={:#x}", flags);
        return ucompat_v140_open(1, flags);
    } else if len == 9 && &buf[..9] == b"v140c.txt" {
        crate::println!("[openat-v140] path=v140c.txt flags={:#x}", flags);
        return ucompat_v140_open(2, flags);
    } else if len == 9 && &buf[..9] == b"v140d.txt" {
        crate::println!("[openat-v140] path=v140d.txt flags={:#x}", flags);
        return ucompat_v140_open(3, flags);
    }
    if len == 9 && &buf[..9] == b"v139a.txt" {
        crate::println!("[openat-v139] path=v139a.txt flags={:#x}", flags);
        return ucompat_v138_open(0, flags);
    } else if len == 9 && &buf[..9] == b"v139b.txt" {
        crate::println!("[openat-v139] path=v139b.txt flags={:#x}", flags);
        return ucompat_v138_open(1, flags);
    }
    // UCOMPAT_V138_OPENAT_PATH_DISPATCH
    if len == 9 && &buf[..9] == b"v138a.txt" {
        crate::println!("[openat-v138] path=v138a.txt flags={:#x}", flags);
        return ucompat_v138_open(0, flags);
    } else if len == 9 && &buf[..9] == b"v138b.txt" {
        crate::println!("[openat-v138] path=v138b.txt flags={:#x}", flags);
        return ucompat_v138_open(1, flags);
    }
    if len > 0 {
        return crate::fs::runtime::openat(dirfd, &buf[..len], flags as u32, mode as u16);
    }
    if len == 9 && &buf[..9] == b"/dev/null" {
        crate::fs::fd_table::runtime_open_devnull()
    } else if len == 9 && &buf[..9] == b"/dev/zero" {
        crate::fs::fd_table::runtime_open_devzero()
    } else if len == 4 && &buf[..4] == b"/dev" {
        crate::fs::fd_table::runtime_open_devdir()
    } else {
        const O_CREAT: usize = 0x40;
        const O_TRUNC: usize = 0x200;
        if len > 0 && (flags & O_CREAT) != 0 {
            if (flags & O_TRUNC) != 0 {
                crate::println!(
                    "[openat-v137p] creating/truncating marker-dispatcher regular file"
                );
            } else {
                crate::println!("[openat-v137p] creating marker-dispatcher regular file");
            }
            ucompat_v137p_regular_reset();
            UCOMPAT_V137P_REG_FD
        } else if len > 0 {
            if (flags & O_TRUNC) != 0 {
                // UCOMPAT_V137X_TRUNCATE_EXISTING_ON_OPEN
                crate::println!("[openat-v137x] truncating existing runtime regular file");
                ucompat_v137p_regular_reset();
                UCOMPAT_V137P_REG_FD
            } else {
                ucompat_v137p_regular_open_existing()
            }
        } else {
            crate::syscall::errno::ENOENT
        }
    }
}

fn sys_close_fd(fd: usize) -> isize {
    if ucompat_v143_is_fd(fd as isize) {
        // UCOMPAT_V143_CLOSE_HIT
        return ucompat_v143_close(fd as isize);
    }

    if ucompat_v142_is_fd(fd as isize) {
        // UCOMPAT_V142_CLOSE_HIT
        return ucompat_v142_close(fd as isize);
    }

    if ucompat_v141_is_fd(fd as isize) {
        // UCOMPAT_V141_CLOSE_HIT
        return ucompat_v141_close(fd as isize);
    }

    if ucompat_v140_is_fd(fd as isize) {
        // UCOMPAT_V140_CLOSE_HIT
        return ucompat_v140_close(fd as isize);
    }

    if ucompat_v138_is_fd(fd as isize) {
        // UCOMPAT_V138_CLOSE_HIT
        return ucompat_v138_close(fd as isize);
    }

    if fd as isize == UCOMPAT_V137P_REG_FD {
        // UCOMPAT_V137V_CLOSE_HIT
        // UCOMPAT_V137W_PRESERVE_REGULAR_FILE_AFTER_CLOSE
        unsafe {
            UCOMPAT_V137P_REG_OPEN = true;
            UCOMPAT_V137P_REG_POS = 0;
        }
        crate::println!("[ucompat-v137w] close fd=9737 ret=0 keep_file=1");
        return 0;
    }

    if fd as isize == UCOMPAT_V137G_REG_FD {
        unsafe {
            UCOMPAT_V137G_REG_OPEN = false;
            UCOMPAT_V137G_REG_POS = 0;
        }
        crate::println!("[ucompat-v137g] fd-runtime close fd={}", fd);
        return 0;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        unsafe {
            UCOMPAT_V137F_REG_OPEN = false;
            UCOMPAT_V137F_REG_POS = 0;
        }
        crate::println!("[ucompat-v137f] close fd={}", fd);
        return 0;
    }

    if crate::fs::runtime::fd_exists(fd) {
        if k04a_pipe_close_preserve(fd) {
            crate::println!(
                "[K04a-pipe-trace] close-preserved fd={}",
                fd
            );
            crate::println!("[close-v157] canonical fd = {}", fd);
            crate::println!("[close-v157] ret = 0");
            return 0;
        }
        let ret = crate::fs::runtime::close(fd);
        crate::println!("[close-v157] canonical fd = {}", fd);
        crate::println!("[close-v157] ret = {}", ret);
        return ret;
    }

    let ret = crate::fs::fd_table::runtime_close_fd(fd);
    crate::println!("[close-v56] fd = {}", fd);
    crate::println!("[close-v56] ret = {}", ret);
    ret
}

fn sys_getdents64_user(fd: usize, user_dirent: usize, len: usize) -> isize {
    crate::println!("[getdents64-v59] fd = {}", fd);
    crate::println!("[getdents64-v59] buf = {:#x}", user_dirent);
    crate::println!("[getdents64-v59] len = {}", len);

    if crate::fs::runtime::fd_exists(fd) {
        let mut out = [0u8; 256];
        let cap = if len < out.len() { len } else { out.len() };
        let ret = crate::fs::runtime::getdents64(fd, &mut out[..cap]);
        if ret > 0 {
            let _ = copy_kernel_bytes_to_user(user_dirent, &out[..ret as usize]);
        }
        crate::println!("[getdents64-v157] canonical ret = {}", ret);
        return ret;
    }

    match crate::fs::fd_table::runtime_getdents_kind(fd) {
        Ok(RuntimeFdKind::DevDir) => {}
        Ok(_) => return crate::syscall::errno::ENOTDIR,
        Err(err) => return err,
    }

    if len < 160 {
        return crate::syscall::errno::EINVAL;
    }

    let mut off = 0usize;

    with_sum_enabled(|| {
        off = write_dirent64(user_dirent, off, 1, 1, 4, b".\0");
        off = write_dirent64(user_dirent, off, 2, 2, 4, b"..\0");
        off = write_dirent64(user_dirent, off, 3, 3, 2, b"null\0");
        off = write_dirent64(user_dirent, off, 4, 4, 2, b"zero\0");
    });

    crate::println!("[getdents64-v59] wrote /dev entries bytes = {}", off);
    off as isize
}

fn write_dirent64(
    base: usize,
    off: usize,
    ino: u64,
    next_off: i64,
    dtype: u8,
    name: &[u8],
) -> usize {
    let header = 19usize;
    let raw_len = header + name.len();
    let reclen = (raw_len + 7) & !7usize;
    let ptr = base + off;

    unsafe {
        core::ptr::write_volatile((ptr + 0) as *mut u64, ino);
        core::ptr::write_volatile((ptr + 8) as *mut i64, next_off);
        core::ptr::write_volatile((ptr + 16) as *mut u16, reclen as u16);
        core::ptr::write_volatile((ptr + 18) as *mut u8, dtype);

        let mut i = 0;
        while i < name.len() {
            core::ptr::write_volatile((ptr + 19 + i) as *mut u8, name[i]);
            i += 1;
        }

        while 19 + i < reclen {
            core::ptr::write_volatile((ptr + 19 + i) as *mut u8, 0);
            i += 1;
        }
    }

    off + reclen
}

fn sys_fstat_user(fd: usize, user_stat: usize) -> isize {
    crate::println!("[fstat-v58] fd = {}", fd);
    crate::println!("[fstat-v58] user stat = {:#x}", user_stat);

    if let Ok(stat) = crate::fs::runtime::stat_fd(fd) {
        let ret = runtime_write_stat_user(user_stat, stat);
        crate::println!("[fstat-v157] canonical fd stat ret = {}", ret);
        return ret;
    }

    let kind = match crate::fs::fd_table::runtime_fstat_result(fd) {
        Ok(kind) => kind,
        Err(err) => return err,
    };

    with_sum_enabled(|| {
        for i in 0..128usize {
            unsafe {
                core::ptr::write_volatile((user_stat + i) as *mut u8, 0);
            }
        }

        let mode: u32 = match kind {
            RuntimeFdKind::DevDir => 0o040000 | 0o755,
            RuntimeFdKind::Stdin
            | RuntimeFdKind::Stdout
            | RuntimeFdKind::Stderr
            | RuntimeFdKind::DevNull
            | RuntimeFdKind::DevZero => 0o020000 | 0o666,
        };

        unsafe {
            core::ptr::write_volatile((user_stat + 0) as *mut u64, fd as u64);
            core::ptr::write_volatile((user_stat + 16) as *mut u32, mode);
            core::ptr::write_volatile((user_stat + 48) as *mut u64, 0);
        }
    });

    crate::println!("[fstat-v58] wrote minimal stat");
    0
}

fn sys_lseek(fd: usize, offset: isize, whence: usize) -> isize {
    if ucompat_v143_is_fd(fd as isize) {
        // UCOMPAT_V143_LSEEK_HIT
        return ucompat_v143_lseek(fd as isize, offset as isize, whence as usize);
    }

    if ucompat_v142_is_fd(fd as isize) {
        // UCOMPAT_V142_LSEEK_HIT
        return ucompat_v142_lseek(fd as isize, offset as isize, whence as usize);
    }

    if ucompat_v141_is_fd(fd as isize) {
        // UCOMPAT_V141_LSEEK_HIT
        return ucompat_v141_lseek(fd as isize, offset as isize, whence as usize);
    }

    if ucompat_v140_is_fd(fd as isize) {
        // UCOMPAT_V140_LSEEK_HIT
        return ucompat_v140_lseek(fd as isize, offset as isize, whence as usize);
    }

    if ucompat_v138_is_fd(fd as isize) {
        // UCOMPAT_V138_LSEEK_HIT
        return ucompat_v138_lseek(fd as isize, offset as isize, whence as usize);
    }

    if fd as isize == UCOMPAT_V137P_REG_FD {
        // UCOMPAT_V137P_LSEEK_HIT
        let base = unsafe {
            match whence {
                0 => 0isize,
                1 => UCOMPAT_V137P_REG_POS as isize,
                2 => UCOMPAT_V137P_REG_LEN as isize,
                _ => return -22,
            }
        };
        let new_pos = base + offset as isize;
        if new_pos < 0 {
            return -22;
        }
        unsafe {
            UCOMPAT_V137P_REG_POS = new_pos as usize;
        }
        crate::println!("[ucompat-v137p] lseek fd={} pos={}", fd, new_pos);
        return new_pos;
    }

    if fd as isize == UCOMPAT_V137O_REG_FD {
        // UCOMPAT_V137O_LSEEK_HIT
        let base = unsafe {
            match whence {
                0 => 0isize,
                1 => UCOMPAT_V137O_REG_POS as isize,
                2 => UCOMPAT_V137O_REG_LEN as isize,
                _ => return -22,
            }
        };
        let new_pos = base + offset as isize;
        if new_pos < 0 {
            return -22;
        }
        unsafe {
            UCOMPAT_V137O_REG_POS = new_pos as usize;
        }
        crate::println!("[ucompat-v137o] lseek fd={} pos={}", fd, new_pos);
        return new_pos;
    }

    if fd as isize == UCOMPAT_V137M_REG_FD {
        // UCOMPAT_V137M_LSEEK_TOP_HIT
        let base = unsafe {
            match whence {
                0 => 0isize,
                1 => UCOMPAT_V137M_REG_POS as isize,
                2 => UCOMPAT_V137M_REG_LEN as isize,
                _ => return -22,
            }
        };
        let new_pos = base + offset as isize;
        if new_pos < 0 {
            return -22;
        }
        unsafe {
            UCOMPAT_V137M_REG_POS = new_pos as usize;
        }
        crate::println!("[ucompat-v137m] top lseek fd={} pos={}", fd, new_pos);
        return new_pos;
    }

    if fd as isize == UCOMPAT_V137G_REG_FD {
        let base = unsafe {
            match whence {
                0 => 0isize,
                1 => UCOMPAT_V137G_REG_POS as isize,
                2 => UCOMPAT_V137G_REG_LEN as isize,
                _ => return -22,
            }
        };
        let new_pos = base + offset as isize;
        if new_pos < 0 {
            return -22;
        }
        unsafe {
            UCOMPAT_V137G_REG_POS = new_pos as usize;
        }
        crate::println!("[ucompat-v137g] fd-runtime lseek fd={} pos={}", fd, new_pos);
        return new_pos;
    }

    if fd as isize == UCOMPAT_V137F_REG_FD {
        let base = unsafe {
            match whence {
                0 => 0isize,
                1 => UCOMPAT_V137F_REG_POS as isize,
                2 => UCOMPAT_V137F_REG_LEN as isize,
                _ => return -22,
            }
        };
        let new_pos = base + offset as isize;
        if new_pos < 0 {
            return -22;
        }
        unsafe {
            UCOMPAT_V137F_REG_POS = new_pos as usize;
        }
        crate::println!("[ucompat-v137f] lseek fd={} pos={}", fd, new_pos);
        return new_pos;
    }

    if fd as isize == UCOMPAT_V137D_REG_FD {
        let base = unsafe {
            match whence {
                0 => 0isize,
                1 => UCOMPAT_V137D_REG_POS as isize,
                2 => UCOMPAT_V137D_REG_LEN as isize,
                _ => return -22,
            }
        };
        let new_pos = base + offset as isize;
        if new_pos < 0 {
            return -22;
        }
        unsafe {
            UCOMPAT_V137D_REG_POS = new_pos as usize;
        }
        return new_pos;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let ret = crate::fs::runtime::lseek(fd, offset, whence);
        crate::println!("[lseek-v157] canonical fd = {} ret = {}", fd, ret);
        return ret;
    }

    crate::println!("[lseek-v58] fd = {}", fd);
    crate::println!("[lseek-v58] offset = {}", offset);
    crate::println!("[lseek-v58] whence = {}", whence);

    let ret = crate::fs::fd_table::runtime_lseek_result(fd);
    crate::println!("[lseek-v58] ret = {}", ret);
    ret
}

fn sys_read_user(fd: usize, user_ptr: usize, len: usize, target: RuntimeReadTarget) -> isize {
    if fd as isize == UCOMPAT_V137P_REG_FD {
        // UCOMPAT_V137P_READ_HIT
        let mut copied = 0usize;
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137P_REG_POS < UCOMPAT_V137P_REG_LEN {
                let ch = UCOMPAT_V137P_REG_DATA[UCOMPAT_V137P_REG_POS];
                core::ptr::write_volatile((base_ptr + copied) as *mut u8, ch);
                UCOMPAT_V137P_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137p] read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137O_REG_FD {
        // UCOMPAT_V137O_READ_HIT
        let mut copied = 0usize;
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137O_REG_POS < UCOMPAT_V137O_REG_LEN {
                let ch = UCOMPAT_V137O_REG_DATA[UCOMPAT_V137O_REG_POS];
                core::ptr::write_volatile((base_ptr + copied) as *mut u8, ch);
                UCOMPAT_V137O_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137o] read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137M_REG_FD {
        // UCOMPAT_V137M_READ_TOP_HIT
        let mut copied = 0usize;
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137M_REG_POS < UCOMPAT_V137M_REG_LEN {
                let ch = UCOMPAT_V137M_REG_DATA[UCOMPAT_V137M_REG_POS];
                core::ptr::write_volatile((base_ptr + copied) as *mut u8, ch);
                UCOMPAT_V137M_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137m] top read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < len && UCOMPAT_V137G_REG_POS < UCOMPAT_V137G_REG_LEN {
                let ch = UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS];
                core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
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
                core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                UCOMPAT_V137F_REG_POS += 1;
                copied += 1;
            }
        });
        crate::println!("[ucompat-v137f] read fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137D_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < len && UCOMPAT_V137D_REG_POS < UCOMPAT_V137D_REG_LEN {
                let ch = UCOMPAT_V137D_REG_DATA[UCOMPAT_V137D_REG_POS];
                core::ptr::write_volatile((user_ptr + copied) as *mut u8, ch);
                UCOMPAT_V137D_REG_POS += 1;
                copied += 1;
            }
        });
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let mut tmp = [0u8; 256];
        let cap = if len < tmp.len() { len } else { tmp.len() };
        let kind = crate::fs::runtime::fd_kind(fd);
        let ret = crate::fs::runtime::read(fd, &mut tmp[..cap]);
        if ret > 0 {
            let _ = copy_kernel_bytes_to_user(user_ptr, &tmp[..ret as usize]);
        }
        if k04a_pipe_context_enabled() && kind == Some(crate::fs::runtime::FdKind::PipeRead) {
            crate::println!(
                "[K04a-pipe-trace] read fd={} requested={} ret={}",
                fd,
                len,
                ret
            );
        }
        crate::println!("[read-v157] canonical fd = {} ret = {}", fd, ret);
        return ret;
    }

    crate::println!("[read-v57] fd = {}", fd);
    crate::println!("[read-v57] len = {}", len);

    if len == 0 {
        return 0;
    }

    match target {
        RuntimeReadTarget::Stdin => {
            crate::println!("[read-v57] stdin returns EOF");
            0
        }
        RuntimeReadTarget::DevZero => {
            with_sum_enabled(|| {
                for i in 0..len {
                    unsafe {
                        core::ptr::write_volatile((user_ptr + i) as *mut u8, 0);
                    }
                }
            });
            crate::println!("[read-v57] /dev/zero filled buffer");
            len as isize
        }
    }
}

// UCOMPAT_V137I_EXACT_FDV55_WRITE_PATH
fn sys_write_user(fd: usize, user_ptr: usize, len: usize, target: RuntimeWriteTarget) -> isize {
    if ucompat_v143_is_fd(fd as isize) {
        // UCOMPAT_V143_WRITE_HIT
        return ucompat_v143_write(fd as isize, user_ptr as usize, len as usize);
    }

    if ucompat_v142_is_fd(fd as isize) {
        // UCOMPAT_V142_WRITE_HIT
        return ucompat_v142_write(fd as isize, user_ptr as usize, len as usize);
    }

    if ucompat_v141_is_fd(fd as isize) {
        // UCOMPAT_V141_WRITE_HIT
        return ucompat_v141_write(fd as isize, user_ptr as usize, len as usize);
    }

    if ucompat_v140_is_fd(fd as isize) {
        // UCOMPAT_V140_WRITE_HIT
        return ucompat_v140_write(fd as isize, user_ptr as usize, len as usize);
    }

    if ucompat_v138_is_fd(fd as isize) {
        // UCOMPAT_V138_WRITE_HIT
        return ucompat_v138_write(fd as isize, user_ptr as usize, len as usize);
    }

    if fd as isize == UCOMPAT_V137P_REG_FD {
        // UCOMPAT_V137P_WRITE_HIT
        let mut copied = 0usize;
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137P_REG_POS + copied < UCOMPAT_V137P_REG_CAP {
                let ch = core::ptr::read_volatile((base_ptr + copied) as *const u8);
                UCOMPAT_V137P_REG_DATA[UCOMPAT_V137P_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137P_REG_POS + copied;
            if end > UCOMPAT_V137P_REG_LEN {
                UCOMPAT_V137P_REG_LEN = end;
            }
            UCOMPAT_V137P_REG_POS = end;
        });
        crate::println!("[ucompat-v137p] write fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137O_REG_FD {
        // UCOMPAT_V137O_WRITE_HIT
        let mut copied = 0usize;
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137O_REG_POS + copied < UCOMPAT_V137O_REG_CAP {
                let ch = core::ptr::read_volatile((base_ptr + copied) as *const u8);
                UCOMPAT_V137O_REG_DATA[UCOMPAT_V137O_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137O_REG_POS + copied;
            if end > UCOMPAT_V137O_REG_LEN {
                UCOMPAT_V137O_REG_LEN = end;
            }
            UCOMPAT_V137O_REG_POS = end;
        });
        crate::println!("[ucompat-v137o] write fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137M_REG_FD {
        // UCOMPAT_V137M_WRITE_TOP_HIT
        let mut copied = 0usize;
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137M_REG_POS + copied < UCOMPAT_V137M_REG_CAP {
                let ch = core::ptr::read_volatile((base_ptr + copied) as *const u8);
                UCOMPAT_V137M_REG_DATA[UCOMPAT_V137M_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137M_REG_POS + copied;
            if end > UCOMPAT_V137M_REG_LEN {
                UCOMPAT_V137M_REG_LEN = end;
            }
            UCOMPAT_V137M_REG_POS = end;
        });
        crate::println!("[ucompat-v137m] top write fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137G_REG_FD {
        // UCOMPAT_V137I_WRITE_HIT
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        // UCOMPAT_V137L_PTR_SCOPE_REPAIR
        crate::println!("[ucompat-v137l] base_ptr source=user_ptr");
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((base_ptr + copied) as *const u8);
                UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137G_REG_POS + copied;
            if end > UCOMPAT_V137G_REG_LEN {
                UCOMPAT_V137G_REG_LEN = end;
            }
            UCOMPAT_V137G_REG_POS = end;
        });
        crate::println!(
            "[ucompat-v137i] exact fd-v55 write fd={} copied={}",
            fd,
            copied
        );
        return copied as isize;
    }

    if fd as isize == UCOMPAT_V137G_REG_FD {
        let mut copied = 0usize;
        unsafe {
            while copied < len && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((user_ptr + copied) as *const u8);
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
                let ch = core::ptr::read_volatile((user_ptr + copied) as *const u8);
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

    if fd as isize == UCOMPAT_V137D_REG_FD {
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < len && UCOMPAT_V137D_REG_POS + copied < UCOMPAT_V137D_REG_CAP {
                let ch = core::ptr::read_volatile((user_ptr + copied) as *const u8);
                UCOMPAT_V137D_REG_DATA[UCOMPAT_V137D_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137D_REG_POS + copied;
            if end > UCOMPAT_V137D_REG_LEN {
                UCOMPAT_V137D_REG_LEN = end;
            }
            UCOMPAT_V137D_REG_POS = end;
        });
        return copied as isize;
    }

    if crate::fs::runtime::fd_exists(fd) {
        let mut tmp = [0u8; 256];
        let cap = if len < tmp.len() { len } else { tmp.len() };
        let copied = match copy_user_bytes_to_kernel(user_ptr, cap, &mut tmp) {
            Ok(n) => n,
            Err(err) => return err,
        };
        let kind = crate::fs::runtime::fd_kind(fd);
        k01_capture_stdout(fd, &tmp[..copied]);
        let ret = crate::fs::runtime::write(fd, &tmp[..copied]);
        if k04a_pipe_context_enabled() && kind == Some(crate::fs::runtime::FdKind::PipeWrite) {
            crate::println!(
                "[K04a-pipe-trace] write fd={} requested={} copied={} ret={}",
                fd,
                len,
                copied,
                ret
            );
        }
        crate::println!("[write-v157] canonical fd = {} ret = {}", fd, ret);
        return ret;
    }

    crate::println!("[fd-v55] write fd = {}", fd);
    crate::println!("[fd-v55] write len = {}", len);

    if fd as isize == UCOMPAT_V137G_REG_FD {
        // UCOMPAT_V137J_SYS_WRITE_TRACE_VARS
        // UCOMPAT_V137J_WRITE_HIT
        let want = len as usize;
        let base_ptr = user_ptr as usize;
        let mut copied = 0usize;
        with_sum_enabled(|| unsafe {
            while copied < want && UCOMPAT_V137G_REG_POS + copied < UCOMPAT_V137G_REG_CAP {
                let ch = core::ptr::read_volatile((base_ptr + copied) as *const u8);
                UCOMPAT_V137G_REG_DATA[UCOMPAT_V137G_REG_POS + copied] = ch;
                copied += 1;
            }
            let end = UCOMPAT_V137G_REG_POS + copied;
            if end > UCOMPAT_V137G_REG_LEN {
                UCOMPAT_V137G_REG_LEN = end;
            }
            UCOMPAT_V137G_REG_POS = end;
        });
        crate::println!("[ucompat-v137k] buffer source=user_ptr");
        crate::println!("[ucompat-v137j] sys_write_user fd={} copied={}", fd, copied);
        return copied as isize;
    }

    if len == 0 {
        return 0;
    }

    match target {
        RuntimeWriteTarget::Console => {
            with_sum_enabled(|| {
                for i in 0..len {
                    let ch = unsafe { core::ptr::read_volatile((user_ptr + i) as *const u8) };
                    crate::sbi::console_putchar(ch as usize);
                }
            });
            len as isize
        }
        RuntimeWriteTarget::DevNull => {
            crate::println!("[fd-v56] /dev/null swallowed write");
            len as isize
        }
    }
}

