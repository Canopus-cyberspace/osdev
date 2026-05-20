# IPC_STATUS

## v67 - event/pipe/dup/poll scaffold

Implemented:
- eventfd2 returns fixed fd 6
- epoll_create1 returns fixed fd 7
- epoll_ctl returns 0
- epoll_pwait returns 0 events
- ppoll returns 0
- pselect6 returns 0
- pipe2 writes fixed fds 8 and 9
- dup returns fixed fd 10
- dup3 returns requested new fd

Still TODO:
- real fd table allocation
- real eventfd counters
- real epoll interest lists
- real pipe buffers
- real poll readiness
