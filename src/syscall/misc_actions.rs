fn dispatch_misc_action(cx: &mut TrapContext, action: RuntimeSyscallAction) -> RuntimeDispatchOutcome {
    match action {
        RuntimeSyscallAction::Uname { user_uts } => {
            let ret = sys_uname_user(user_uts);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Sysinfo { user_info } => {
            let ret = sys_sysinfo_user(user_info);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getrandom {
            user_buf,
            len,
            flags,
        } => {
            let ret = sys_getrandom_user(user_buf, len, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::RiscvFlushIcache { start, end, flags } => {
            let ret = sys_riscv_flush_icache(start, end, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Membarrier { cmd, flags, cpu_id } => {
            let ret = sys_membarrier(cmd, flags, cpu_id);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoUringSetup { entries, params } => {
            let ret = sys_io_uring_setup_user(entries, params);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoUringEnter {
            fd,
            to_submit,
            min_complete,
            flags,
            sig,
            sigsz,
        } => {
            let ret = sys_io_uring_enter(fd, to_submit, min_complete, flags, sig, sigsz);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoUringRegister {
            fd,
            opcode,
            arg,
            nr_args,
        } => {
            let ret = sys_io_uring_register(fd, opcode, arg, nr_args);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Syslog { type_, buf, len } => {
            let ret = sys_syslog_user(type_, buf, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Reboot {
            magic1,
            magic2,
            cmd,
            arg,
        } => {
            let ret = sys_reboot_user(magic1, magic2, cmd, arg);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PerfEventOpen {
            attr,
            pid,
            cpu,
            group_fd,
            flags,
        } => {
            let ret = sys_perf_event_open_user(attr, pid, cpu, group_fd, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::FinitModule { fd, uargs, flags } => {
            let ret = sys_finit_module_user(fd, uargs, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedSetattr { pid, attr, flags } => {
            let ret = sys_sched_setattr_user(pid, attr, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedGetattr {
            pid,
            attr,
            size,
            flags,
        } => {
            let ret = sys_sched_getattr_user(pid, attr, size, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Seccomp { op, flags, args } => {
            let ret = sys_seccomp_user(op, flags, args);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Bpf { cmd, attr, size } => {
            let ret = sys_bpf_user(cmd, attr, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Execveat {
            dirfd,
            user_path,
            argv,
            envp,
            flags,
        } => {
            let ret = sys_execveat_user(dirfd, user_path, argv, envp, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Preadv2 {
            fd,
            iov,
            iovcnt,
            offset,
            flags,
        } => {
            let ret = sys_preadv2_user(fd, iov, iovcnt, offset, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Pwritev2 {
            fd,
            iov,
            iovcnt,
            offset,
            flags,
        } => {
            let ret = sys_pwritev2_user(fd, iov, iovcnt, offset, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Vhangup => {
            let ret = sys_vhangup();
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoSetup { nr_events, ctxp } => {
            crate::println!("[aio-v81] io_setup nr_events = {}", nr_events);
            if ctxp != 0 {
                with_sum_enabled(|| unsafe {
                    core::ptr::write_volatile(ctxp as *mut u64, 1);
                });
            }
            crate::println!("[aio-v81] io_setup ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoDestroy { ctx } => {
            crate::println!("[aio-v81] io_destroy ctx = {}", ctx);
            crate::println!("[aio-v81] io_destroy ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoSubmit { ctx, nr, iocbpp } => {
            crate::println!("[aio-v81] io_submit ctx = {}", ctx);
            crate::println!("[aio-v81] io_submit nr = {}", nr);
            crate::println!("[aio-v81] io_submit iocbpp = {:#x}", iocbpp);
            crate::println!("[aio-v81] io_submit ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoCancel { ctx, iocb, result } => {
            crate::println!("[aio-v81] io_cancel ctx = {}", ctx);
            crate::println!("[aio-v81] io_cancel iocb = {:#x}", iocb);
            zero_user_bytes(result, 32);
            crate::println!("[aio-v81] io_cancel ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoGetevents {
            ctx,
            min_nr,
            nr,
            events,
            timeout,
        } => {
            crate::println!("[aio-v81] io_getevents ctx = {}", ctx);
            crate::println!("[aio-v81] io_getevents min_nr = {}", min_nr);
            crate::println!("[aio-v81] io_getevents nr = {}", nr);
            crate::println!("[aio-v81] io_getevents events = {:#x}", events);
            crate::println!("[aio-v81] io_getevents timeout = {:#x}", timeout);
            crate::println!("[aio-v81] io_getevents ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::AddKey {
            type_,
            description,
            payload,
            plen,
            keyring,
        } => {
            crate::println!("[key-v81] add_key type = {:#x}", type_);
            crate::println!("[key-v81] add_key description = {:#x}", description);
            crate::println!("[key-v81] add_key payload = {:#x}", payload);
            crate::println!("[key-v81] add_key plen = {}", plen);
            crate::println!("[key-v81] add_key keyring = {}", keyring);
            crate::println!("[key-v81] add_key serial = 1");
            cx.regs[10] = 1;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::RequestKey {
            type_,
            description,
            callout_info,
            keyring,
        } => {
            crate::println!("[key-v81] request_key type = {:#x}", type_);
            crate::println!("[key-v81] request_key description = {:#x}", description);
            crate::println!("[key-v81] request_key callout = {:#x}", callout_info);
            crate::println!("[key-v81] request_key keyring = {}", keyring);
            crate::println!("[key-v81] request_key serial = 1");
            cx.regs[10] = 1;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Keyctl {
            option,
            arg2,
            arg3,
            arg4,
            arg5,
        } => {
            crate::println!("[key-v81] keyctl option = {}", option);
            crate::println!("[key-v81] keyctl arg2 = {:#x}", arg2);
            crate::println!("[key-v81] keyctl arg3 = {:#x}", arg3);
            crate::println!("[key-v81] keyctl arg4 = {:#x}", arg4);
            crate::println!("[key-v81] keyctl arg5 = {:#x}", arg5);
            crate::println!("[key-v81] keyctl ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MqOpen {
            name,
            oflag,
            mode,
            attr,
        } => {
            crate::println!("[ipc-v82] mq_open name = {:#x}", name);
            crate::println!("[ipc-v82] mq_open oflag = {:#x}", oflag);
            crate::println!("[ipc-v82] mq_open mode = {:#o}", mode);
            crate::println!("[ipc-v82] mq_open attr = {:#x}", attr);
            let ret = crate::fs::runtime::mq_open(name, oflag as u32);
            crate::println!("[ipc-v157] mq_open canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MqUnlink { name } => {
            crate::println!("[ipc-v82] mq_unlink name = {:#x}", name);
            let ret = crate::fs::runtime::mq_unlink(name);
            crate::println!("[ipc-v157] mq_unlink canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MqTimedsend {
            mqdes,
            msg_ptr,
            msg_len,
            msg_prio,
            abs_timeout,
        } => {
            crate::println!("[ipc-v82] mq_timedsend mqdes = {}", mqdes);
            crate::println!("[ipc-v82] mq_timedsend msg = {:#x}", msg_ptr);
            crate::println!("[ipc-v82] mq_timedsend len = {}", msg_len);
            crate::println!("[ipc-v82] mq_timedsend prio = {}", msg_prio);
            crate::println!("[ipc-v82] mq_timedsend timeout = {:#x}", abs_timeout);
            let mut tmp = [0u8; 96];
            let cap = if msg_len < tmp.len() {
                msg_len
            } else {
                tmp.len()
            };
            let ret = match copy_user_bytes_to_kernel(msg_ptr, cap, &mut tmp) {
                Ok(copied) => crate::fs::runtime::mq_send(mqdes, &tmp[..copied], msg_prio),
                Err(err) => err,
            };
            crate::println!("[ipc-v157] mq_timedsend canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MqTimedreceive {
            mqdes,
            msg_ptr,
            msg_len,
            msg_prio,
            abs_timeout,
        } => {
            crate::println!("[ipc-v82] mq_timedreceive mqdes = {}", mqdes);
            crate::println!("[ipc-v82] mq_timedreceive msg = {:#x}", msg_ptr);
            crate::println!("[ipc-v82] mq_timedreceive len = {}", msg_len);
            crate::println!("[ipc-v82] mq_timedreceive prio = {:#x}", msg_prio);
            crate::println!("[ipc-v82] mq_timedreceive timeout = {:#x}", abs_timeout);
            let mut tmp = [0u8; 96];
            let cap = if msg_len < tmp.len() {
                msg_len
            } else {
                tmp.len()
            };
            let ret = crate::fs::runtime::mq_receive(mqdes, &mut tmp[..cap]);
            if ret > 0 {
                let _ = copy_kernel_bytes_to_user(msg_ptr, &tmp[..ret as usize]);
                if msg_prio != 0 {
                    with_sum_enabled(|| unsafe {
                        core::ptr::write_volatile(msg_prio as *mut u32, 0);
                    });
                }
            }
            crate::println!("[ipc-v157] mq_timedreceive canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MqNotify { mqdes, sevp } => {
            crate::println!("[ipc-v82] mq_notify mqdes = {}", mqdes);
            crate::println!("[ipc-v82] mq_notify sevp = {:#x}", sevp);
            crate::println!("[ipc-v82] mq_notify ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MqGetsetattr {
            mqdes,
            newattr,
            oldattr,
        } => {
            crate::println!("[ipc-v82] mq_getsetattr mqdes = {}", mqdes);
            crate::println!("[ipc-v82] mq_getsetattr newattr = {:#x}", newattr);
            zero_user_bytes(oldattr, 64);
            let ret = if crate::fs::runtime::fd_kind(mqdes) == Some(crate::fs::runtime::FdKind::Mq)
            {
                0
            } else {
                crate::fs::runtime::EBADF
            };
            crate::println!("[ipc-v157] mq_getsetattr canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Msgget { key, msgflg } => {
            crate::println!("[ipc-v82] msgget key = {}", key);
            crate::println!("[ipc-v82] msgget flags = {:#x}", msgflg);
            let ret = crate::fs::runtime::msgget(key);
            crate::println!("[ipc-v157] msgget canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Msgctl { msqid, cmd, buf } => {
            crate::println!("[ipc-v82] msgctl id = {}", msqid);
            crate::println!("[ipc-v82] msgctl cmd = {}", cmd);
            let ret = crate::fs::runtime::msgctl(msqid, cmd);
            if ret == 0 && cmd == crate::fs::runtime::IPC_STAT {
                zero_user_bytes(buf, 128);
            }
            crate::println!("[ipc-v157] msgctl canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Msgrcv {
            msqid,
            msgp,
            msgsz,
            msgtyp,
            msgflg,
        } => {
            crate::println!("[ipc-v82] msgrcv id = {}", msqid);
            crate::println!("[ipc-v82] msgrcv msgp = {:#x}", msgp);
            crate::println!("[ipc-v82] msgrcv size = {}", msgsz);
            crate::println!("[ipc-v82] msgrcv type = {}", msgtyp);
            crate::println!("[ipc-v82] msgrcv flags = {:#x}", msgflg);
            let mut tmp = [0u8; 96];
            let cap = if msgsz < tmp.len() { msgsz } else { tmp.len() };
            let ret = crate::fs::runtime::msgrcv(msqid, &mut tmp[..cap]);
            if ret > 0 {
                let _ = copy_kernel_bytes_to_user(msgp, &tmp[..ret as usize]);
            }
            crate::println!("[ipc-v157] msgrcv canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Msgsnd {
            msqid,
            msgp,
            msgsz,
            msgflg,
        } => {
            crate::println!("[ipc-v82] msgsnd id = {}", msqid);
            crate::println!("[ipc-v82] msgsnd msgp = {:#x}", msgp);
            crate::println!("[ipc-v82] msgsnd size = {}", msgsz);
            crate::println!("[ipc-v82] msgsnd flags = {:#x}", msgflg);
            let mut tmp = [0u8; 96];
            let cap = if msgsz < tmp.len() { msgsz } else { tmp.len() };
            let ret = match copy_user_bytes_to_kernel(msgp, cap, &mut tmp) {
                Ok(copied) => crate::fs::runtime::msgsnd(msqid, &tmp[..copied]),
                Err(err) => err,
            };
            crate::println!("[ipc-v157] msgsnd canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Semget { key, nsems, semflg } => {
            crate::println!("[ipc-v82] semget key = {}", key);
            crate::println!("[ipc-v82] semget nsems = {}", nsems);
            crate::println!("[ipc-v82] semget flags = {:#x}", semflg);
            let ret = crate::fs::runtime::semget(key, nsems);
            crate::println!("[ipc-v157] semget canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Semctl {
            semid,
            semnum,
            cmd,
            arg,
        } => {
            crate::println!("[ipc-v82] semctl id = {}", semid);
            crate::println!("[ipc-v82] semctl num = {}", semnum);
            crate::println!("[ipc-v82] semctl cmd = {}", cmd);
            crate::println!("[ipc-v82] semctl arg = {:#x}", arg);
            let ret = crate::fs::runtime::semctl(semid, cmd);
            crate::println!("[ipc-v157] semctl canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Semtimedop {
            semid,
            sops,
            nsops,
            timeout,
        } => {
            crate::println!("[ipc-v82] semtimedop id = {}", semid);
            crate::println!("[ipc-v82] semtimedop sops = {:#x}", sops);
            crate::println!("[ipc-v82] semtimedop nsops = {}", nsops);
            crate::println!("[ipc-v82] semtimedop timeout = {:#x}", timeout);
            let delta: isize = if nsops == 0 { 0 } else { 1 };
            let ret = crate::fs::runtime::semop(semid, delta);
            crate::println!("[ipc-v157] semtimedop canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Semop { semid, sops, nsops } => {
            crate::println!("[ipc-v82] semop id = {}", semid);
            crate::println!("[ipc-v82] semop sops = {:#x}", sops);
            crate::println!("[ipc-v82] semop nsops = {}", nsops);
            let delta: isize = if nsops == 0 { 0 } else { 1 };
            let ret = crate::fs::runtime::semop(semid, delta);
            crate::println!("[ipc-v157] semop canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Shmget { key, size, shmflg } => {
            crate::println!("[ipc-v82] shmget key = {}", key);
            crate::println!("[ipc-v82] shmget size = {}", size);
            crate::println!("[ipc-v82] shmget flags = {:#x}", shmflg);
            let ret = crate::fs::runtime::shmget(key, size);
            crate::println!("[ipc-v157] shmget canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Shmctl { shmid, cmd, buf } => {
            crate::println!("[ipc-v82] shmctl id = {}", shmid);
            crate::println!("[ipc-v82] shmctl cmd = {}", cmd);
            let ret = crate::fs::runtime::shmctl(shmid, cmd);
            if ret == 0 && cmd == crate::fs::runtime::IPC_STAT {
                zero_user_bytes(buf, 128);
            }
            crate::println!("[ipc-v157] shmctl canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Shmat {
            shmid,
            shmaddr,
            shmflg,
        } => {
            crate::println!("[ipc-v82] shmat id = {}", shmid);
            crate::println!("[ipc-v82] shmat addr = {:#x}", shmaddr);
            crate::println!("[ipc-v82] shmat flags = {:#x}", shmflg);
            let ret = crate::fs::runtime::shmat(shmid);
            crate::println!("[ipc-v157] shmat canonical ret = {:#x}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Shmdt { shmaddr } => {
            crate::println!("[ipc-v82] shmdt addr = {:#x}", shmaddr);
            let ret = crate::fs::runtime::shmdt(shmaddr);
            crate::println!("[ipc-v157] shmdt canonical ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        _ => RuntimeDispatchOutcome::Unhandled,
    }
}
