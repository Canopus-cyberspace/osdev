#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RuntimeDispatchOutcome {
    Unhandled,
    Handled,
    ReturnNow,
}

fn write_runtime_return(cx: &mut TrapContext, value: isize) {
    cx.regs[10] = value as usize;
}

fn dispatch_runtime_trap(cx: &mut TrapContext, args: RuntimeSyscallArgs) {
    let id = args.id;

    crate::println!("[external-init-v82] syscall id = {}", id);
    crate::println!("[syscall-dispatch-v54] central dispatch id = {}", id);
    k02_record_syscall(id);
    b01_record_syscall(id);

    let action = crate::syscall::dispatch_runtime_syscall(args);
    let mut outcome = match action {
        RuntimeSyscallAction::Return(value) => {
            write_runtime_return(cx, value);
            RuntimeDispatchOutcome::Handled
        }
        _ => RuntimeDispatchOutcome::Unhandled,
    };

    if outcome == RuntimeDispatchOutcome::Unhandled {
        outcome = dispatch_fs_action(cx, action);
    }
    if outcome == RuntimeDispatchOutcome::Unhandled {
        outcome = dispatch_process_action(cx, action);
    }
    if outcome == RuntimeDispatchOutcome::Unhandled {
        outcome = dispatch_mm_action(cx, action);
    }
    if outcome == RuntimeDispatchOutcome::Unhandled {
        outcome = dispatch_time_action(cx, action);
    }
    if outcome == RuntimeDispatchOutcome::Unhandled {
        outcome = dispatch_signal_action(cx, action);
    }
    if outcome == RuntimeDispatchOutcome::Unhandled {
        outcome = dispatch_net_action(cx, action);
    }
    if outcome == RuntimeDispatchOutcome::Unhandled {
        outcome = dispatch_misc_action(cx, action);
    }

    match outcome {
        RuntimeDispatchOutcome::Handled => maybe_deliver_pending_signal_user(cx),
        RuntimeDispatchOutcome::ReturnNow => return,
        RuntimeDispatchOutcome::Unhandled => {
            write_runtime_return(cx, crate::syscall::ENOSYS);
            maybe_deliver_pending_signal_user(cx);
        }
    }
}

include!("fs_actions.rs");
include!("process_actions.rs");
include!("mm_actions.rs");
include!("time_actions.rs");
include!("signal_actions.rs");
include!("net_actions.rs");
include!("misc_actions.rs");
