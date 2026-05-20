# PROC_STATUS

## v70 - fs sync mount scaffold

Implemented:
- clone returns fake child pid 2
- wait4 writes zero status and returns the requested pid
- execve currently validates the path pointer at scaffold level and returns ENOENT
- kill(pid, sig) returns 0
- tgkill(tgid, tid, sig) returns 0
- exit_group(code) terminates the smoke path

Still TODO:
- real task creation
- real address-space duplication
- real wait queues and zombie ownership
- real exec image replacement
- real signal delivery
- process groups and sessions
