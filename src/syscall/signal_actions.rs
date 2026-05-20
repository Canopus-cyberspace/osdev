fn dispatch_signal_action(cx: &mut TrapContext, action: RuntimeSyscallAction) -> RuntimeDispatchOutcome {
    match action {
        RuntimeSyscallAction::RtSigaction {
            sig,
            act,
            oldact,
            sigsetsize,
        } => {
            let ret = sys_rt_sigaction_user(sig, act, oldact, sigsetsize);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::RtSigprocmask {
            how,
            set,
            oldset,
            sigsetsize,
        } => {
            let ret = sys_rt_sigprocmask_user(how, set, oldset, sigsetsize);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::RtSigreturn => {
            let ret = sys_rt_sigreturn_user(cx);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Kill { pid, sig } => {
            let ret = sys_kill(pid, sig);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Tkill { tid, sig } => {
            let ret = sys_tkill(tid, sig);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Tgkill { tgid, tid, sig } => {
            let ret = sys_tgkill(tgid, tid, sig);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::RtTgsigqueueinfo {
            tgid,
            tid,
            sig,
            uinfo,
        } => {
            crate::println!("[ipc-v82] rt_tgsigqueueinfo tgid = {}", tgid);
            crate::println!("[ipc-v82] rt_tgsigqueueinfo tid = {}", tid);
            crate::println!("[ipc-v82] rt_tgsigqueueinfo sig = {}", sig);
            crate::println!("[ipc-v82] rt_tgsigqueueinfo uinfo = {:#x}", uinfo);
            crate::println!("[ipc-v82] rt_tgsigqueueinfo ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        _ => RuntimeDispatchOutcome::Unhandled,
    }
}
