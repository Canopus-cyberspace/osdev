# Roadmap from v158 to v167

## Overall direction

The project has completed the first historical integration pass in v157. Future versions should stop doing broad "integration claims" and instead deepen one real kernel subsystem per version.

## v158: Event / pipe / socket / poll / epoll readiness

Goal:

- Implement real readiness semantics for canonical fd objects.

Scope:

- pipe buffer
- eventfd counter
- socketpair loopback buffer
- poll readiness
- epoll interest list
- O_NONBLOCK behavior
- close release and readiness updates

Out of scope:

- full TCP/IP
- real hardware network driver
- full timer expiration
- full scheduler blocking

Marker:

```text
[ucompat-v158] event pipe socket readiness PASS
```

## v159: Process / wait / exit / fork lifecycle

Goal:

- Build a real task lifecycle model.

Scope:

- task table
- fork/clone child creation
- parent-child relation
- exit -> zombie
- wait4/waitid recovery
- per-task fdtable snapshot or sharing rules
- initial SIGCHLD bookkeeping

Marker:

```text
[ucompat-v159] process wait exit fork lifecycle PASS
```

## v160: execve from VFS + argv/envp/auxv

Goal:

- Execute ELF programs loaded from canonical VFS rather than only embedded external init.

Scope:

- VFS-backed ELF read
- PT_LOAD mapping
- argv/envp/auxv user stack
- CLOEXEC fd handling
- process image replacement

Marker:

```text
[ucompat-v160] execve from vfs user stack PASS
```

## v161: VMA / page fault / lazy allocation

Goal:

- Add realistic memory region management.

Scope:

- VMA list/tree
- page fault handler
- lazy brk/mmap allocation
- munmap VMA split
- mprotect permissions
- usercopy validation through user mappings

Marker:

```text
[ucompat-v161] vma page fault lazy allocation PASS
```

## v162: Signal delivery + rt_sigreturn

Goal:

- Implement real user-visible signal delivery.

Scope:

- pending signal queue
- blocked mask
- signal frame on user stack
- signal trampoline
- rt_sigreturn trap context restoration
- kill/tkill/tgkill delivery
- SIGCHLD integration

Marker:

```text
[ucompat-v162] signal delivery rt_sigreturn PASS
```

## v163: mount / rootfs / devfs / procfs deepening

Goal:

- Make the runtime user environment more complete.

Scope:

- ramfs/tmpfs root
- devfs nodes
- procfs files
- mount tree basics
- statfs/fstatfs
- cwd/root path constraints

Marker:

```text
[ucompat-v163] mount rootfs devfs procfs PASS
```

## v164: futex / scheduler sleep-wakeup

Goal:

- Connect blocking syscalls to scheduler wait queues.

Scope:

- futex wait/wake queue
- pipe/eventfd/poll sleeping
- timer-driven wakeup
- run queue and sleep queue integration

Marker:

```text
[ucompat-v164] futex scheduler sleep wakeup PASS
```

## v165: IPC blocking and lifecycle

Goal:

- Deepen POSIX mq and SysV IPC.

Scope:

- message queue capacity
- typed msgrcv
- semaphore wait/wake
- shared memory mapping lifecycle
- object permissions simplified but consistent

Marker:

```text
[ucompat-v165] ipc blocking lifecycle PASS
```

## v166: credentials / permissions / capability

Goal:

- Add a coherent identity and permission model.

Scope:

- real/effective/saved uid/gid
- file mode permission checks
- directory execute permission
- capability bitsets
- simplified namespace-aware permission checks

Marker:

```text
[ucompat-v166] credentials permissions capability PASS
```

## v167: Userland compatibility matrix

Goal:

- Move beyond single embedded external init smoke.

Scope:

- multiple ELF test programs
- VFS/rootfs test runner
- per-test result collection
- return-code reporting
- compatibility matrix report

Marker:

```text
[ucompat-v167] userland compatibility matrix PASS
```
