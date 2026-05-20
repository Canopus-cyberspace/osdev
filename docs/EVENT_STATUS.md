# EVENT_STATUS

## v75 - event/timer/misc fd scaffold

Implemented:
- inotify_init1 returns fixed fd 18
- inotify_add_watch returns wd 1
- inotify_rm_watch returns 0
- ioprio_set/ioprio_get return 0
- flock returns 0
- signalfd4 returns fixed fd 19
- sync_file_range returns 0
- timerfd_create returns fixed fd 20
- timerfd_settime/timerfd_gettime return 0 and zero output buffers
- getitimer/setitimer return 0 and zero output buffers

Still TODO:
- real event queues
- real timer wheel
- blocking reads from timerfd/signalfd/inotify
- file lock ownership and conflict checks
- real ioprio storage
