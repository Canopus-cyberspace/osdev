fn dispatch_time_action(cx: &mut TrapContext, action: RuntimeSyscallAction) -> RuntimeDispatchOutcome {
    match action {
        RuntimeSyscallAction::ClockGettime { clock_id, user_ts } => {
            let ret = sys_clock_gettime_user(clock_id, user_ts);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Gettimeofday { user_tv, user_tz } => {
            let ret = sys_gettimeofday_user(user_tv, user_tz);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedYield => {
            let ret = sys_sched_yield();
            cx.regs[10] = ret as usize;
            if ret == 0 && k04a_yield_to_next(cx) {
                return RuntimeDispatchOutcome::ReturnNow;
            }
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Nanosleep { req, rem } => {
            let ret = sys_nanosleep_user(req, rem);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Futex {
            uaddr,
            op,
            val,
            timeout,
            uaddr2,
            val3,
        } => {
            let ret = sys_futex_user(uaddr, op, val, timeout, uaddr2, val3);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Eventfd2 { initval, flags } => {
            let ret = sys_eventfd2(initval, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::EpollCreate1 { flags } => {
            let ret = sys_epoll_create1(flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::EpollCtl {
            epfd,
            op,
            fd,
            event,
        } => {
            let ret = sys_epoll_ctl_user(epfd, op, fd, event);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::EpollPwait {
            epfd,
            events,
            maxevents,
            timeout,
            sigmask,
            sigsetsize,
        } => {
            let ret = sys_epoll_pwait_user(epfd, events, maxevents, timeout, sigmask, sigsetsize);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Ppoll {
            fds,
            nfds,
            timeout,
            sigmask,
            sigsetsize,
        } => {
            let ret = sys_ppoll_user(fds, nfds, timeout, sigmask, sigsetsize);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Pselect6 {
            nfds,
            readfds,
            writefds,
            exceptfds,
            timeout,
            sigmask,
        } => {
            let ret = sys_pselect6_user(nfds, readfds, writefds, exceptfds, timeout, sigmask);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedGetScheduler { pid } => {
            let ret = sys_sched_getscheduler(pid);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedGetParam { pid, user_param } => {
            let ret = sys_sched_getparam_user(pid, user_param);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedGetAffinity {
            pid,
            len,
            user_mask,
        } => {
            let ret = sys_sched_getaffinity_user(pid, len, user_mask);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedGetPriorityMax { policy } => {
            let ret = sys_sched_get_priority_max(policy);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedGetPriorityMin { policy } => {
            let ret = sys_sched_get_priority_min(policy);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ClockGetres { clock_id, user_ts } => {
            let ret = sys_clock_getres_user(clock_id, user_ts);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ClockNanosleep {
            clock_id,
            flags,
            req,
            rem,
        } => {
            let ret = sys_clock_nanosleep_user(clock_id, flags, req, rem);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::InotifyInit1 { flags } => {
            let ret = sys_inotify_init1(flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::InotifyAddWatch {
            fd,
            user_path,
            mask,
        } => {
            let ret = sys_inotify_add_watch_user(fd, user_path, mask);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::InotifyRmWatch { fd, wd } => {
            let ret = sys_inotify_rm_watch(fd, wd);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoprioSet { which, who, ioprio } => {
            let ret = sys_ioprio_set(which, who, ioprio);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::IoprioGet { which, who } => {
            let ret = sys_ioprio_get(which, who);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Flock { fd, op } => {
            let ret = sys_flock(fd, op);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Signalfd4 {
            fd,
            user_mask,
            sizemask,
            flags,
        } => {
            let ret = sys_signalfd4_user(fd, user_mask, sizemask, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SyncFileRange {
            fd,
            offset,
            nbytes,
            flags,
        } => {
            let ret = sys_sync_file_range(fd, offset, nbytes, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerfdCreate { clockid, flags } => {
            let ret = sys_timerfd_create(clockid, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerfdSettime {
            fd,
            flags,
            new_value,
            old_value,
        } => {
            let ret = sys_timerfd_settime_user(fd, flags, new_value, old_value);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerfdGettime { fd, curr_value } => {
            let ret = sys_timerfd_gettime_user(fd, curr_value);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getitimer { which, curr_value } => {
            let ret = sys_getitimer_user(which, curr_value);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setitimer {
            which,
            new_value,
            old_value,
        } => {
            let ret = sys_setitimer_user(which, new_value, old_value);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::EpollPwait2 {
            epfd,
            events,
            maxevents,
            timeout,
            sigmask,
            sigsetsize,
        } => {
            let ret = sys_epoll_pwait2_user(epfd, events, maxevents, timeout, sigmask, sigsetsize);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::FutexWaitv {
            waiters,
            nr_futexes,
            flags,
            timeout,
        } => {
            let ret = sys_futex_waitv_user(waiters, nr_futexes, flags, timeout);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerCreate {
            clockid,
            sevp,
            timerid,
        } => {
            crate::println!("[timer-v81] timer_create clockid = {}", clockid);
            crate::println!("[timer-v81] timer_create sevp = {:#x}", sevp);
            if timerid != 0 {
                with_sum_enabled(|| unsafe {
                    core::ptr::write_volatile(timerid as *mut i32, 1);
                });
            }
            crate::println!("[timer-v81] timer_create ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerGettime {
            timerid,
            curr_value,
        } => {
            crate::println!("[timer-v81] timer_gettime timerid = {}", timerid);
            zero_user_bytes(curr_value, 32);
            crate::println!("[timer-v81] timer_gettime ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerGetoverrun { timerid } => {
            crate::println!("[timer-v81] timer_getoverrun timerid = {}", timerid);
            crate::println!("[timer-v81] timer_getoverrun ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerSettime {
            timerid,
            flags,
            new_value,
            old_value,
        } => {
            crate::println!("[timer-v81] timer_settime timerid = {}", timerid);
            crate::println!("[timer-v81] timer_settime flags = {:#x}", flags);
            crate::println!("[timer-v81] timer_settime new = {:#x}", new_value);
            zero_user_bytes(old_value, 32);
            crate::println!("[timer-v81] timer_settime ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::TimerDelete { timerid } => {
            crate::println!("[timer-v81] timer_delete timerid = {}", timerid);
            crate::println!("[timer-v81] timer_delete ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ClockSettime { clockid, tp } => {
            crate::println!("[timer-v81] clock_settime clockid = {}", clockid);
            crate::println!("[timer-v81] clock_settime tp = {:#x}", tp);
            crate::println!("[timer-v81] clock_settime ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedSetparam { pid, param } => {
            crate::println!("[sched-v81] sched_setparam pid = {}", pid);
            crate::println!("[sched-v81] sched_setparam param = {:#x}", param);
            crate::println!("[sched-v81] sched_setparam ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedSetscheduler { pid, policy, param } => {
            crate::println!("[sched-v81] sched_setscheduler pid = {}", pid);
            crate::println!("[sched-v81] sched_setscheduler policy = {}", policy);
            crate::println!("[sched-v81] sched_setscheduler param = {:#x}", param);
            crate::println!("[sched-v81] sched_setscheduler ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedSetaffinity { pid, len, mask } => {
            crate::println!("[sched-v81] sched_setaffinity pid = {}", pid);
            crate::println!("[sched-v81] sched_setaffinity len = {}", len);
            crate::println!("[sched-v81] sched_setaffinity mask = {:#x}", mask);
            crate::println!("[sched-v81] sched_setaffinity ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SchedRrGetInterval { pid, tp } => {
            crate::println!("[sched-v81] sched_rr_get_interval pid = {}", pid);
            zero_user_bytes(tp, 16);
            crate::println!("[sched-v81] sched_rr_get_interval ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        _ => RuntimeDispatchOutcome::Unhandled,
    }
}
