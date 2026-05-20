# AIO_TIMER_KEY_STATUS

## v81 - AIO/timer/key/scheduler scaffold

Implemented as smoke-level compatibility scaffolds:
- Linux AIO: io_setup/io_destroy/io_submit/io_cancel/io_getevents
- waitid/set_tid_address/unshare/set_robust_list/get_robust_list
- POSIX timers: timer_create/timer_gettime/timer_getoverrun/timer_settime/timer_delete
- clock_settime
- sched_setparam/sched_setscheduler/sched_setaffinity/sched_rr_get_interval/restart_syscall
- key management: add_key/request_key/keyctl

Still TODO:
- real AIO contexts and completion queues
- real POSIX timer queue
- real scheduler mutation
- real keyring permission model
- real waitid and robust futex semantics
