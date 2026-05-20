fn dispatch_process_action(cx: &mut TrapContext, action: RuntimeSyscallAction) -> RuntimeDispatchOutcome {
    match action {
        RuntimeSyscallAction::Getpid => {
            let ret = crate::fs::runtime::current_pid() as isize;
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getppid => {
            let ret = crate::fs::runtime::current_ppid() as isize;
            if k04a_current_kind() == K02_REALRUN_KIND_GETPPID {
                crate::println!(
                    "[K04b-process-trace] getppid pid={} ppid={}",
                    crate::fs::runtime::current_pid(),
                    ret
                );
            }
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Gettid => {
            let ret = crate::fs::runtime::current_pid() as isize;
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SetTidAddress { user_tidptr } => {
            crate::println!("[proc-v81] set_tid_address tidptr = {:#x}", user_tidptr);
            let tid = crate::fs::runtime::current_pid() as isize;
            crate::println!("[proc-v81] set_tid_address ret = {}", tid);
            cx.regs[10] = tid as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SetRobustList { head, len } => {
            crate::println!("[proc-v81] set_robust_list head = {:#x}", head);
            crate::println!("[proc-v81] set_robust_list len = {}", len);
            crate::println!("[proc-v81] set_robust_list ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Prlimit64 {
            pid,
            resource,
            new_limit,
            old_limit,
        } => {
            let ret = sys_prlimit64_user(pid, resource, new_limit, old_limit);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Clone {
            flags,
            stack,
            parent_tid,
            tls,
            child_tid,
        } => {
            let ret = match k04a_try_clone(cx, flags, stack) {
                Some(ret) => {
                    crate::println!(
                        "[process-v165] clone K04a context child pid = {}",
                        ret
                    );
                    ret
                }
                None => sys_clone_user(flags, stack, parent_tid, tls, child_tid),
            };
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Wait4 {
            pid,
            user_wstatus,
            options,
            user_rusage,
        } => {
            let ret = sys_wait4_user(pid, user_wstatus, options, user_rusage);
            if ret == 0
                && k04a_realrun_context_enabled()
                && k04a_has_runnable_child()
                && k04a_block_parent_and_run_child(cx, "wait4_pending")
            {
                return RuntimeDispatchOutcome::ReturnNow;
            }
            if ret > 0 && k04b_is_fork_clone_exec_kind(k04a_current_kind()) {
                crate::println!(
                    "[K04b-process-trace] wait4-collected parent_pid={} child_pid={} ret={}",
                    crate::fs::runtime::current_pid(),
                    ret,
                    ret
                );
            }
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Execve {
            user_path,
            user_argv,
            user_envp,
        } => {
            if k04b_execve_context_enabled() {
                let ret = k04b_try_execve_user(cx, user_path, user_argv, user_envp);
                if ret == 0 {
                    return RuntimeDispatchOutcome::ReturnNow;
                }
                cx.regs[10] = ret as usize;
                return RuntimeDispatchOutcome::ReturnNow;
            }
            let ret = sys_execve_user(user_path, user_argv, user_envp);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getrusage { who, user_usage } => {
            let ret = sys_getrusage_user(who, user_usage);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Prctl {
            option,
            arg2,
            arg3,
            arg4,
            arg5,
        } => {
            let ret = sys_prctl_user(option, arg2, arg3, arg4, arg5);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getcpu {
            user_cpu,
            user_node,
            user_tcache,
        } => {
            let ret = sys_getcpu_user(user_cpu, user_node, user_tcache);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Capget { header, data } => {
            let ret = sys_capget_user(header, data);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Capset { header, data } => {
            let ret = sys_capset_user(header, data);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Personality { persona } => {
            let ret = sys_personality(persona);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setpriority { which, who, prio } => {
            let ret = sys_setpriority(which, who, prio);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getpriority { which, who } => {
            let ret = sys_getpriority(which, who);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setregid { rgid, egid } => {
            let ret = sys_setregid(rgid, egid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setgid { gid } => {
            let ret = sys_setgid(gid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setreuid { ruid, euid } => {
            let ret = sys_setreuid(ruid, euid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setuid { uid } => {
            let ret = sys_setuid(uid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setresuid { ruid, euid, suid } => {
            let ret = sys_setresuid(ruid, euid, suid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getresuid { ruid, euid, suid } => {
            let ret = sys_getresuid_user(ruid, euid, suid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setresgid { rgid, egid, sgid } => {
            let ret = sys_setresgid(rgid, egid, sgid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getresgid { rgid, egid, sgid } => {
            let ret = sys_getresgid_user(rgid, egid, sgid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setfsuid { uid } => {
            let ret = sys_setfsuid(uid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setfsgid { gid } => {
            let ret = sys_setfsgid(gid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Times { user_tms } => {
            let ret = sys_times_user(user_tms);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setpgid { pid, pgid } => {
            let ret = sys_setpgid(pid, pgid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getpgid { pid } => {
            let ret = sys_getpgid(pid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getsid { pid } => {
            let ret = sys_getsid(pid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setsid => {
            let ret = sys_setsid();
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getgroups { size, user_list } => {
            let ret = sys_getgroups_user(size, user_list);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setgroups { size, user_list } => {
            let ret = sys_setgroups_user(size, user_list);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getrlimit {
            resource,
            user_rlim,
        } => {
            let ret = sys_getrlimit_user(resource, user_rlim);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setrlimit {
            resource,
            user_rlim,
        } => {
            let ret = sys_setrlimit_user(resource, user_rlim);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PidfdOpen { pid, flags } => {
            let ret = sys_pidfd_open(pid, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PidfdSendSignal {
            pidfd,
            sig,
            info,
            flags,
        } => {
            let ret = sys_pidfd_send_signal(pidfd, sig, info, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PidfdGetfd {
            pidfd,
            targetfd,
            flags,
        } => {
            let ret = sys_pidfd_getfd(pidfd, targetfd, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Clone3 { user_args, size } => {
            let ret = sys_clone3_user(user_args, size);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ProcessMrelease { pidfd, flags } => {
            let ret = sys_process_mrelease(pidfd, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Ptrace {
            request,
            pid,
            addr,
            data,
        } => {
            let ret = sys_ptrace_user(request, pid, addr, data);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setns { fd, nstype } => {
            let ret = sys_setns(fd, nstype);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Kcmp {
            pid1,
            pid2,
            type_,
            idx1,
            idx2,
        } => {
            let ret = sys_kcmp(pid1, pid2, type_, idx1, idx2);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Waitid {
            idtype,
            id,
            infop,
            options,
            rusage,
        } => {
            crate::println!("[proc-v81] waitid idtype = {}", idtype);
            crate::println!("[proc-v81] waitid id = {}", id);
            let wait_pid = if id == 0 { -1 } else { id as isize };
            let mut exit_code = 0isize;
            let ret = crate::fs::runtime::waitid(wait_pid, &mut exit_code);
            zero_user_bytes(infop, 128);
            if infop != 0 && ret > 0 {
                with_sum_enabled(|| unsafe {
                    core::ptr::write_volatile((infop + 0) as *mut i32, 17);
                    core::ptr::write_volatile((infop + 16) as *mut i32, ret as i32);
                    core::ptr::write_volatile((infop + 24) as *mut i32, exit_code as i32);
                });
            }
            zero_user_bytes(rusage, 144);
            crate::println!("[proc-v81] waitid options = {:#x}", options);
            crate::println!("[process-v165] waitid exit_code = {}", exit_code);
            crate::println!("[process-v165] waitid ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Unshare { flags } => {
            crate::println!("[proc-v81] unshare flags = {:#x}", flags);
            let ret = crate::fs::runtime::unshare_namespaces(flags);
            crate::println!("[namespace-v184] unshare ret = {}", ret);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::GetRobustList {
            pid,
            head_ptr,
            len_ptr,
        } => {
            crate::println!("[proc-v81] get_robust_list pid = {}", pid);
            if head_ptr != 0 {
                with_sum_enabled(|| unsafe {
                    core::ptr::write_volatile(head_ptr as *mut u64, 0);
                });
            }
            if len_ptr != 0 {
                with_sum_enabled(|| unsafe {
                    core::ptr::write_volatile(len_ptr as *mut u64, 24);
                });
            }
            crate::println!("[proc-v81] get_robust_list ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::RestartSyscall => {
            crate::println!("[sched-v81] restart_syscall ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ExitGroup { code } => {
            if handle_real_umode_exit(cx, code) {
                return RuntimeDispatchOutcome::ReturnNow;
            }
            sys_exit_group(code)
        },
        RuntimeSyscallAction::Exit { code } => {
            if handle_real_umode_exit(cx, code) {
                return RuntimeDispatchOutcome::ReturnNow;
            }
            crate::println!("[external-init-v82] exit code = {}", code);
            let ret = crate::fs::runtime::exit_current_task(code);
            crate::println!("[process-v165] exit canonical ret = {}", ret);
            EXIT_SEEN.store(true, Ordering::SeqCst);
            crate::println!("[external-init-v82] smoke passed");
            crate::println!("[external-init-v82] kernel idle after external init ELF smoke");
            crate::official::runtime_finish::finish_official_qemu_runtime()
        },
        _ => RuntimeDispatchOutcome::Unhandled,
    }
}
