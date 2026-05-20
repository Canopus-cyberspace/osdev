# IPC_MSG_NETIO_STATUS

## v82 - IPC/message/net I/O scaffold

Implemented as smoke-level compatibility scaffolds:
- POSIX message queues: mq_open/mq_unlink/mq_timedsend/mq_timedreceive/mq_notify/mq_getsetattr
- SysV IPC: msgget/msgctl/msgrcv/msgsnd/semget/semctl/semtimedop/semop/shmget/shmctl/shmat/shmdt
- Socket message APIs: recvmsg/sendmsg/recvmmsg/sendmmsg
- Advisory I/O: readahead/fadvise64
- Signal queue: rt_tgsigqueueinfo

Still TODO:
- real POSIX mqueue object table
- real SysV IPC namespace/object tables
- real socket msghdr/iovec parsing
- real advisory readahead/fadvise semantics
- real queued signal delivery
