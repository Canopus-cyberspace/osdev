fn dispatch_net_action(cx: &mut TrapContext, action: RuntimeSyscallAction) -> RuntimeDispatchOutcome {
    match action {
        RuntimeSyscallAction::Socket {
            domain,
            sock_type,
            protocol,
        } => {
            let ret = sys_socket_user(domain, sock_type, protocol);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Socketpair {
            domain,
            sock_type,
            protocol,
            user_sv,
        } => {
            let ret = sys_socketpair_user(domain, sock_type, protocol, user_sv);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Bind {
            fd,
            user_addr,
            addrlen,
        } => {
            let ret = sys_bind_user(fd, user_addr, addrlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Listen { fd, backlog } => {
            let ret = sys_listen(fd, backlog);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Accept4 {
            fd,
            user_addr,
            user_addrlen,
            flags,
        } => {
            let ret = sys_accept4_user(fd, user_addr, user_addrlen, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Connect {
            fd,
            user_addr,
            addrlen,
        } => {
            let ret = sys_connect_user(fd, user_addr, addrlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getsockname {
            fd,
            user_addr,
            user_addrlen,
        } => {
            let ret = sys_getsockname_user(fd, user_addr, user_addrlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getpeername {
            fd,
            user_addr,
            user_addrlen,
        } => {
            let ret = sys_getpeername_user(fd, user_addr, user_addrlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Sendto {
            fd,
            user_buf,
            len,
            flags,
            user_dest,
            addrlen,
        } => {
            let ret = sys_sendto_user(fd, user_buf, len, flags, user_dest, addrlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Recvfrom {
            fd,
            user_buf,
            len,
            flags,
            user_src,
            user_addrlen,
        } => {
            let ret = sys_recvfrom_user(fd, user_buf, len, flags, user_src, user_addrlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Setsockopt {
            fd,
            level,
            optname,
            user_optval,
            optlen,
        } => {
            let ret = sys_setsockopt_user(fd, level, optname, user_optval, optlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Getsockopt {
            fd,
            level,
            optname,
            user_optval,
            user_optlen,
        } => {
            let ret = sys_getsockopt_user(fd, level, optname, user_optval, user_optlen);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Shutdown { fd, how } => {
            let ret = sys_shutdown(fd, how);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Recvmsg { fd, msg, flags } => {
            let ret = sys_recvmsg_user(fd, msg, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Sendmsg { fd, msg, flags } => {
            let ret = sys_sendmsg_user(fd, msg, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Recvmmsg {
            fd,
            msgvec,
            vlen,
            flags,
            timeout,
        } => {
            crate::println!("[netmsg-v82] recvmmsg fd = {}", fd);
            crate::println!("[netmsg-v82] recvmmsg msgvec = {:#x}", msgvec);
            crate::println!("[netmsg-v82] recvmmsg vlen = {}", vlen);
            crate::println!("[netmsg-v82] recvmmsg flags = {:#x}", flags);
            crate::println!("[netmsg-v82] recvmmsg timeout = {:#x}", timeout);
            crate::println!("[netmsg-v82] recvmmsg ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Sendmmsg {
            fd,
            msgvec,
            vlen,
            flags,
        } => {
            crate::println!("[netmsg-v82] sendmmsg fd = {}", fd);
            crate::println!("[netmsg-v82] sendmmsg msgvec = {:#x}", msgvec);
            crate::println!("[netmsg-v82] sendmmsg vlen = {}", vlen);
            crate::println!("[netmsg-v82] sendmmsg flags = {:#x}", flags);
            crate::println!("[netmsg-v82] sendmmsg ret = 0");
            cx.regs[10] = 0;
            RuntimeDispatchOutcome::Handled
        },
        _ => RuntimeDispatchOutcome::Unhandled,
    }
}
