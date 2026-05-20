fn dispatch_mm_action(cx: &mut TrapContext, action: RuntimeSyscallAction) -> RuntimeDispatchOutcome {
    match action {
        RuntimeSyscallAction::Brk { addr } => {
            let ret = sys_brk(addr);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mmap {
            addr,
            len,
            prot,
            flags,
            fd,
            offset,
        } => {
            let ret = sys_mmap(addr, len, prot, flags, fd, offset);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Munmap { addr, len } => {
            let ret = sys_munmap(addr, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mprotect { addr, len, prot } => {
            let ret = sys_mprotect(addr, len, prot);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Madvise { addr, len, advice } => {
            let ret = sys_madvise(addr, len, advice);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mremap {
            old_addr,
            old_size,
            new_size,
            flags,
            new_addr,
        } => {
            let ret = sys_mremap_user(old_addr, old_size, new_size, flags, new_addr);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Msync { addr, len, flags } => {
            let ret = sys_msync(addr, len, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mlock { addr, len } => {
            let ret = sys_mlock(addr, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Munlock { addr, len } => {
            let ret = sys_munlock(addr, len);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mlockall { flags } => {
            let ret = sys_mlockall(flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Munlockall => {
            let ret = sys_munlockall();
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mincore { addr, len, vec } => {
            let ret = sys_mincore_user(addr, len, vec);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::RemapFilePages {
            start,
            size,
            prot,
            pgoff,
            flags,
        } => {
            let ret = sys_remap_file_pages(start, size, prot, pgoff, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mbind {
            start,
            len,
            mode,
            nodemask,
            maxnode,
            flags,
        } => {
            let ret = sys_mbind(start, len, mode, nodemask, maxnode, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::GetMempolicy {
            mode,
            nodemask,
            maxnode,
            addr,
            flags,
        } => {
            let ret = sys_get_mempolicy_user(mode, nodemask, maxnode, addr, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SetMempolicy {
            mode,
            nodemask,
            maxnode,
        } => {
            let ret = sys_set_mempolicy(mode, nodemask, maxnode);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Userfaultfd { flags } => {
            let ret = sys_userfaultfd(flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ProcessMadvise {
            pidfd,
            iov,
            vlen,
            advice,
            flags,
        } => {
            let ret = sys_process_madvise_user(pidfd, iov, vlen, advice, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::LandlockCreateRuleset { attr, size, flags } => {
            let ret = sys_landlock_create_ruleset_user(attr, size, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::LandlockAddRule {
            ruleset_fd,
            rule_type,
            rule_attr,
            flags,
        } => {
            let ret = sys_landlock_add_rule_user(ruleset_fd, rule_type, rule_attr, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::LandlockRestrictSelf { ruleset_fd, flags } => {
            let ret = sys_landlock_restrict_self(ruleset_fd, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::MemfdSecret { flags } => {
            let ret = sys_memfd_secret(flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::SetMempolicyHomeNode {
            start,
            len,
            home_node,
            flags,
        } => {
            let ret = sys_set_mempolicy_home_node(start, len, home_node, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ProcessVmReadv {
            pid,
            local_iov,
            liovcnt,
            remote_iov,
            riovcnt,
            flags,
        } => {
            let ret =
                sys_process_vm_readv_user(pid, local_iov, liovcnt, remote_iov, riovcnt, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::ProcessVmWritev {
            pid,
            local_iov,
            liovcnt,
            remote_iov,
            riovcnt,
            flags,
        } => {
            let ret =
                sys_process_vm_writev_user(pid, local_iov, liovcnt, remote_iov, riovcnt, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::Mlock2 { addr, len, flags } => {
            let ret = sys_mlock2(addr, len, flags);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PkeyMprotect {
            addr,
            len,
            prot,
            pkey,
        } => {
            let ret = sys_pkey_mprotect(addr, len, prot, pkey);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PkeyAlloc {
            flags,
            access_rights,
        } => {
            let ret = sys_pkey_alloc(flags, access_rights);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        RuntimeSyscallAction::PkeyFree { pkey } => {
            let ret = sys_pkey_free(pkey);
            cx.regs[10] = ret as usize;
            RuntimeDispatchOutcome::Handled
        },
        _ => RuntimeDispatchOutcome::Unhandled,
    }
}
