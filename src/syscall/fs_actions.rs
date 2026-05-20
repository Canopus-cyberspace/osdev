fn dispatch_fs_action(cx: &mut TrapContext, action: RuntimeSyscallAction) -> RuntimeDispatchOutcome {
    match action {
        RuntimeSyscallAction::Write {
            fd,
            user_ptr,
            len,
            target,
        } => {
            if ucompat_v143_is_fd(fd as isize) {
                // UCOMPAT_V143E_WRITE_ACTION_HIT
                let written = ucompat_v143_write(fd as isize, user_ptr as usize, len as usize);
                crate::println!("[ucompat-v143e] write action fd={} ret={}", fd, written);
                // UCOMPAT_V143F_WRITE_ACTION_RET_REGISTER
                // UCOMPAT_V143G_TRAPCONTEXT_REGS_RETURN
                // UCOMPAT_V143H_MALFORMED_COMMENT_REPAIR
                cx.regs[10] = written as usize;
                return RuntimeDispatchOutcome::ReturnNow;
            }

            let written = sys_write_user(fd, user_ptr, len, target);
            cx.regs[10] = written as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Read {
            fd,
            user_ptr,
            len,
            target,
        } => {
            let read = if ucompat_v143_is_fd(fd as isize) {
                // UCOMPAT_V143_READ_ACTION_HIT
                ucompat_v143_read(fd as isize, user_ptr as usize, len as usize)
            } else if ucompat_v142_is_fd(fd as isize) {
                // UCOMPAT_V142_READ_ACTION_HIT
                ucompat_v142_read(fd as isize, user_ptr as usize, len as usize)
            } else if ucompat_v141_is_fd(fd as isize) {
                // UCOMPAT_V141_READ_ACTION_HIT
                ucompat_v141_read(fd as isize, user_ptr as usize, len as usize)
            } else if ucompat_v140_is_fd(fd as isize) {
                // UCOMPAT_V140_READ_ACTION_HIT
                ucompat_v140_read(fd as isize, user_ptr as usize, len as usize)
            } else if ucompat_v138_is_fd(fd as isize) {
                // UCOMPAT_V138_READ_ACTION_HIT
                ucompat_v138_read(fd as isize, user_ptr as usize, len as usize)
            } else if fd as isize == UCOMPAT_V137P_REG_FD {
                // UCOMPAT_V137S_READ_ACTION_HIT
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
                crate::println!("[ucompat-v137s] read action fd=9737 copied={}", copied);
                copied as isize
            } else {
                sys_read_user(fd, user_ptr, len, target)
            };
            if read == 0
                && k04a_pipe_context_enabled()
                && crate::fs::runtime::fd_kind(fd) == Some(crate::fs::runtime::FdKind::PipeRead)
                && k04a_block_parent_and_run_child(cx, "pipe_read_empty")
            {
                crate::println!(
                    "[K04a-pipe-trace] read-block fd={} switched_to_child=1",
                    fd
                );
                return RuntimeDispatchOutcome::ReturnNow;
            }
            cx.regs[10] = read as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::OpenAt {
            dirfd,
            user_path,
            flags,
            mode,
        } => {
            let fd = sys_openat_user(dirfd, user_path, flags, mode);
            cx.regs[10] = fd as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Close { fd } => {
            let ret = sys_close_fd(fd);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::FStat { fd, user_stat } => {
            let ret = sys_fstat_user(fd, user_stat);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::LSeek { fd, offset, whence } => {
            let ret = sys_lseek(fd, offset, whence);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::GetDents64 {
            fd,
            user_dirent,
            len,
        } => {
            let ret = sys_getdents64_user(fd, user_dirent, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getcwd { user_buf, len } => {
            let ret = sys_getcwd_user(user_buf, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fcntl { fd, cmd, arg } => {
            let ret = sys_fcntl(fd, cmd, arg);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Ioctl { fd, request, argp } => {
            let ret = sys_ioctl_user(fd, request, argp);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Readlinkat {
            dirfd,
            user_path,
            user_buf,
            len,
        } => {
            let ret = sys_readlinkat_user(dirfd, user_path, user_buf, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Umask { mask } => {
            let ret = sys_umask(mask);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Chdir { user_path } => {
            let ret = sys_chdir_user(user_path);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Pipe2 { user_pipefd, flags } => {
            let ret = sys_pipe2_user(user_pipefd, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Dup { oldfd } => {
            let ret = sys_dup(oldfd);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Dup3 {
            oldfd,
            newfd,
            flags,
        } => {
            let ret = sys_dup3(oldfd, newfd, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mkdirat {
            dirfd,
            user_path,
            mode,
        } => {
            let ret = sys_mkdirat_user(dirfd, user_path, mode);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Unlinkat {
            dirfd,
            user_path,
            flags,
        } => {
            let ret = sys_unlinkat_user(dirfd, user_path, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Faccessat {
            dirfd,
            user_path,
            mode,
            flags,
        } => {
            let ret = sys_faccessat_user(dirfd, user_path, mode, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Newfstatat {
            dirfd,
            user_path,
            user_stat,
            flags,
        } => {
            let ret = sys_newfstatat_user(dirfd, user_path, user_stat, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Renameat2 {
            olddirfd,
            oldpath,
            newdirfd,
            newpath,
            flags,
        } => {
            let ret = sys_renameat2_user(olddirfd, oldpath, newdirfd, newpath, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Statx {
            dirfd,
            user_path,
            flags,
            mask,
            user_statx,
        } => {
            let ret = sys_statx_user(dirfd, user_path, flags, mask, user_statx);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mount {
            source,
            target,
            fstype,
            flags,
            data,
        } => {
            let ret = sys_mount_user(source, target, fstype, flags, data);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Umount2 { target, flags } => {
            let ret = sys_umount2_user(target, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Statfs {
            user_path,
            user_buf,
        } => {
            let ret = sys_statfs_user(user_path, user_buf);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fstatfs { fd, user_buf } => {
            let ret = sys_fstatfs_user(fd, user_buf);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Truncate { user_path, length } => {
            let ret = sys_truncate_user(user_path, length);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Ftruncate { fd, length } => {
            let ret = sys_ftruncate(fd, length);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fallocate {
            fd,
            mode,
            offset,
            len,
        } => {
            let ret = sys_fallocate(fd, mode, offset, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Sync => {
            let ret = sys_sync();
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fsync { fd } => {
            let ret = sys_fsync(fd);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fdatasync { fd } => {
            let ret = sys_fdatasync(fd);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Utimensat {
            dirfd,
            user_path,
            user_times,
            flags,
        } => {
            let ret = sys_utimensat_user(dirfd, user_path, user_times, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Readv { fd, iov, iovcnt } => {
            let ret = sys_readv_user(fd, iov, iovcnt);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Writev { fd, iov, iovcnt } => {
            let ret = sys_writev_user(fd, iov, iovcnt);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Pread64 {
            fd,
            buf,
            len,
            offset,
        } => {
            let ret = sys_pread64_user(fd, buf, len, offset);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Pwrite64 {
            fd,
            buf,
            len,
            offset,
        } => {
            let ret = sys_pwrite64_user(fd, buf, len, offset);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Preadv {
            fd,
            iov,
            iovcnt,
            offset,
        } => {
            let ret = sys_preadv_user(fd, iov, iovcnt, offset);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Pwritev {
            fd,
            iov,
            iovcnt,
            offset,
        } => {
            let ret = sys_pwritev_user(fd, iov, iovcnt, offset);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Sendfile {
            out_fd,
            in_fd,
            offset,
            count,
        } => {
            let ret = sys_sendfile_user(out_fd, in_fd, offset, count);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Vmsplice {
            fd,
            iov,
            nr_segs,
            flags,
        } => {
            let ret = sys_vmsplice_user(fd, iov, nr_segs, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Splice {
            fd_in,
            off_in,
            fd_out,
            off_out,
            len,
            flags,
        } => {
            let ret = sys_splice_user(fd_in, off_in, fd_out, off_out, len, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Tee {
            fd_in,
            fd_out,
            len,
            flags,
        } => {
            let ret = sys_tee_user(fd_in, fd_out, len, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::CopyFileRange {
            fd_in,
            off_in,
            fd_out,
            off_out,
            len,
            flags,
        } => {
            let ret = sys_copy_file_range_user(fd_in, off_in, fd_out, off_out, len, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MemfdCreate { name, flags } => {
            let ret = sys_memfd_create_user(name, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::CloseRange { first, last, flags } => {
            let ret = sys_close_range(first, last, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Openat2 {
            dirfd,
            user_path,
            user_how,
            size,
        } => {
            let ret = sys_openat2_user(dirfd, user_path, user_how, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Faccessat2 {
            dirfd,
            user_path,
            mode,
            flags,
        } => {
            let ret = sys_faccessat2_user(dirfd, user_path, mode, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::OpenTree {
            dfd,
            user_path,
            flags,
        } => {
            let ret = sys_open_tree_user(dfd, user_path, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MoveMount {
            from_dfd,
            from_path,
            to_dfd,
            to_path,
            flags,
        } => {
            let ret = sys_move_mount_user(from_dfd, from_path, to_dfd, to_path, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fsopen { user_fsname, flags } => {
            let ret = sys_fsopen_user(user_fsname, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fsconfig {
            fs_fd,
            cmd,
            key,
            value,
            aux,
        } => {
            let ret = sys_fsconfig_user(fs_fd, cmd, key, value, aux);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fsmount {
            fs_fd,
            flags,
            ms_flags,
        } => {
            let ret = sys_fsmount(fs_fd, flags, ms_flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fspick {
            dfd,
            user_path,
            flags,
        } => {
            let ret = sys_fspick_user(dfd, user_path, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MountSetattr {
            dfd,
            user_path,
            flags,
            attr,
            size,
        } => {
            let ret = sys_mount_setattr_user(dfd, user_path, flags, attr, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::QuotactlFd { fd, cmd, id, addr } => {
            let ret = sys_quotactl_fd_user(fd, cmd, id, addr);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Acct { user_path } => {
            let ret = sys_acct_user(user_path);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Swapon { user_path, flags } => {
            let ret = sys_swapon_user(user_path, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Swapoff { user_path } => {
            let ret = sys_swapoff_user(user_path);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::FanotifyInit {
            flags,
            event_f_flags,
        } => {
            let ret = sys_fanotify_init(flags, event_f_flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::FanotifyMark {
            fd,
            flags,
            mask,
            dirfd,
            user_path,
        } => {
            let ret = sys_fanotify_mark_user(fd, flags, mask, dirfd, user_path);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::NameToHandleAt {
            dirfd,
            user_path,
            handle,
            mount_id,
            flags,
        } => {
            let ret = sys_name_to_handle_at_user(dirfd, user_path, handle, mount_id, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::OpenByHandleAt {
            mount_fd,
            handle,
            flags,
        } => {
            let ret = sys_open_by_handle_at_user(mount_fd, handle, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Syncfs { fd } => {
            let ret = sys_syncfs(fd);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setxattr {
            path,
            name,
            value,
            size,
            flags,
        } => {
            let ret = sys_setxattr_user(path, name, value, size, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Lsetxattr {
            path,
            name,
            value,
            size,
            flags,
        } => {
            let ret = sys_lsetxattr_user(path, name, value, size, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fsetxattr {
            fd,
            name,
            value,
            size,
            flags,
        } => {
            let ret = sys_fsetxattr_user(fd, name, value, size, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getxattr {
            path,
            name,
            value,
            size,
        } => {
            let ret = sys_getxattr_user(path, name, value, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Lgetxattr {
            path,
            name,
            value,
            size,
        } => {
            let ret = sys_lgetxattr_user(path, name, value, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fgetxattr {
            fd,
            name,
            value,
            size,
        } => {
            let ret = sys_fgetxattr_user(fd, name, value, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Listxattr { path, list, size } => {
            let ret = sys_listxattr_user(path, list, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Llistxattr { path, list, size } => {
            let ret = sys_llistxattr_user(path, list, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Flistxattr { fd, list, size } => {
            let ret = sys_flistxattr_user(fd, list, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Removexattr { path, name } => {
            let ret = sys_removexattr_user(path, name);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Lremovexattr { path, name } => {
            let ret = sys_lremovexattr_user(path, name);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fremovexattr { fd, name } => {
            let ret = sys_fremovexattr_user(fd, name);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::LookupDcookie { cookie, buf, len } => {
            let ret = sys_lookup_dcookie_user(cookie, buf, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Symlinkat {
            oldname,
            newdirfd,
            newname,
        } => {
            let ret = sys_symlinkat_user(oldname, newdirfd, newname);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Linkat {
            olddirfd,
            oldpath,
            newdirfd,
            newpath,
            flags,
        } => {
            let ret = sys_linkat_user(olddirfd, oldpath, newdirfd, newpath, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PivotRoot { new_root, put_old } => {
            let ret = sys_pivot_root_user(new_root, put_old);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Nfsservctl { cmd, argp, resp } => {
            let ret = sys_nfsservctl_user(cmd, argp, resp);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fchdir { fd } => {
            let ret = sys_fchdir(fd);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Chroot { path } => {
            let ret = sys_chroot_user(path);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fchmod { fd, mode } => {
            let ret = sys_fchmod(fd, mode);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fchmodat { dirfd, path, mode } => {
            let ret = sys_fchmodat_user(dirfd, path, mode);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fchownat {
            dirfd,
            path,
            owner,
            group,
            flags,
        } => {
            let ret = sys_fchownat_user(dirfd, path, owner, group, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fchown { fd, owner, group } => {
            let ret = sys_fchown(fd, owner, group);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Quotactl {
            cmd,
            special,
            id,
            addr,
        } => {
            let ret = sys_quotactl_user(cmd, special, id, addr);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Readahead { fd, offset, count } => {
            crate::println!("[netmsg-v82] readahead fd = {}", fd);
            crate::println!("[netmsg-v82] readahead offset = {}", offset);
            crate::println!("[netmsg-v82] readahead count = {}", count);
            crate::println!("[netmsg-v82] readahead ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Fadvise64 {
            fd,
            offset,
            len,
            advice,
        } => {
            crate::println!("[netmsg-v82] fadvise64 fd = {}", fd);
            crate::println!("[netmsg-v82] fadvise64 offset = {}", offset);
            crate::println!("[netmsg-v82] fadvise64 len = {}", len);
            crate::println!("[netmsg-v82] fadvise64 advice = {}", advice);
            crate::println!("[netmsg-v82] fadvise64 ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        _ => RuntimeDispatchOutcome::Unhandled,
    }
}
