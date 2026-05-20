# SIGNAL_STATUS

## v66 - signal scaffold

Implemented:
- `rt_sigaction(sig, NULL, oldact, 8)` zeroes old action and returns 0
- `rt_sigprocmask(how, NULL, oldset, 8)` zeroes old mask and returns 0

Still TODO:
- real signal delivery
- signal pending queues
- user signal trampoline
- rt_sigreturn restore path
