# PROC_STATUS

## v64 - process/resource/random scaffold

Implemented:
- set_tid_address -> returns tid 1
- set_robust_list -> returns 0
- getuid/geteuid/getgid/getegid -> returns 0
- gettid -> returns 1
- sysinfo writes a minimal sysinfo structure
- prlimit64 writes a minimal rlimit64 structure
- getrandom fills a deterministic byte stream

Still TODO:
- real task IDs
- real credentials
- real robust futex list
- real resource accounting
- real entropy source
