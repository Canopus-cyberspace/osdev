# Roadmap Index v158-v190

This index is a compact version-by-version map.

For details, read the phase files under `docs/phases/`.

| Version | Main target | New marker |
|---|---|---|
| v158 | event/pipe/socket/poll/epoll readiness | `[ucompat-v158] event pipe socket readiness PASS` |
| v159 | timerfd deterministic timer readiness | `[ucompat-v159] timerfd deterministic readiness PASS` |
| v160 | close_range/CLOEXEC/fd lifecycle | `[ucompat-v160] fd lifecycle cloexec close_range PASS` |
| v161 | unified iovec IO path | `[ucompat-v161] unified iovec io path PASS` |
| v162 | POSIX mq and SysV IPC lifecycle | `[ucompat-v162] ipc registry lifecycle PASS` |
| v163 | futex wait/wake object model | `[ucompat-v163] futex wait wake object model PASS` |
| v164 | scheduler wait queue foundation | `[ucompat-v164] scheduler wait queue foundation PASS` |
| v165 | task table/process lifecycle | `[ucompat-v165] task table process lifecycle PASS` |
| v166 | fork/clone child creation | `[ucompat-v166] fork clone child task PASS` |
| v167 | exit/zombie/wait lifecycle | `[ucompat-v167] exit zombie wait lifecycle PASS` |
| v168 | per-task fdtable/cwd/root/signal snapshot | `[ucompat-v168] per task runtime snapshot PASS` |
| v169 | execve from canonical VFS | `[ucompat-v169] execve from canonical vfs PASS` |
| v170 | argv/envp/auxv and CLOEXEC | `[ucompat-v170] execve user stack cloexec PASS` |
| v171 | VMA/page fault foundation | `[ucompat-v171] vma page fault foundation PASS` |
| v172 | lazy mmap/brk/munmap/mprotect | `[ucompat-v172] lazy mmap brk munmap mprotect PASS` |
| v173 | signal frame and rt_sigreturn | `[ucompat-v173] signal frame rt_sigreturn PASS` |
| v174 | SIGCHLD/process-group signal basics | `[ucompat-v174] sigchld process group signal PASS` |
| v175 | rootfs/tmpfs backend | `[ucompat-v175] rootfs tmpfs backend PASS` |
| v176 | devfs core devices | `[ucompat-v176] devfs core devices PASS` |
| v177 | procfs process/status/maps | `[ucompat-v177] procfs process status maps PASS` |
| v178 | mount tree and statfs/fstatfs | `[ucompat-v178] mount tree statfs PASS` |
| v179 | permissions and credentials | `[ucompat-v179] permissions credentials PASS` |
| v180 | capability/identity model | `[ucompat-v180] capability identity model PASS` |
| v181 | AF_UNIX/socket loopback deepening | `[ucompat-v181] unix socket loopback PASS` |
| v182 | local datagram socket | `[ucompat-v182] local datagram socket PASS` |
| v183 | IPC blocking and scheduler integration | `[ucompat-v183] ipc blocking scheduler integration PASS` |
| v184 | namespace basics | `[ucompat-v184] namespace basics PASS` |
| v185 | multi-ELF rootfs runner | `[ucompat-v185] multi elf rootfs runner PASS` |
| v186 | libc syscall matrix | `[ucompat-v186] libc syscall matrix PASS` |
| v187 | fs/process/memory suite | `[ucompat-v187] fs process memory suite PASS` |
| v188 | signal/pipe/poll/ipc suite | `[ucompat-v188] signal pipe poll ipc suite PASS` |
| v189 | stress/error-path hardening | `[ucompat-v189] stress error path hardening PASS` |
| v190 | final competition readiness | `[ucompat-v190] final competition kernel readiness PASS` |
