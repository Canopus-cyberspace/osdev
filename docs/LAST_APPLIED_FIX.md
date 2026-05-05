# LAST_APPLIED_FIX

Version: v64

Goal:
- Add process/resource/random syscall scaffolds frequently used by libc startup and utilities.

Verified:
- set_tid_address
- set_robust_list
- getuid/geteuid/getgid/getegid/gettid
- sysinfo
- prlimit64
- getrandom
- prior fd/mm/time regressions remain passing
