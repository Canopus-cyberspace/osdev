# SECURITY_OBSERVE_STATUS

## v78 - security/observability/compat scaffold

Implemented as smoke-level compatibility scaffolds:
- acct/syslog/ptrace/reboot
- swapon/swapoff
- perf_event_open
- fanotify_init/fanotify_mark
- name_to_handle_at/open_by_handle_at
- syncfs/setns
- process_vm_readv/process_vm_writev
- kcmp
- finit_module
- sched_setattr/sched_getattr
- seccomp/bpf
- execveat
- mlock2
- preadv2/pwritev2
- pkey_mprotect/pkey_alloc/pkey_free

Still TODO:
- real auditing/accounting
- real perf ring buffers
- real fanotify queues
- real namespace switching
- real module loader and verifier
- real seccomp/BPF verifier and enforcement
- real pkey support
