# LAST_APPLIED_FIX

Version: v82

Goal:
- Add IPC/message/net I/O compatibility syscall scaffolds.

Verified:
- mq_open/mq_unlink/mq_timedsend/mq_timedreceive/mq_notify/mq_getsetattr
- msgget/msgctl/msgrcv/msgsnd
- semget/semctl/semtimedop/semop
- shmget/shmctl/shmat/shmdt
- recvmsg/sendmsg/recvmmsg/sendmmsg
- readahead/fadvise64
- rt_tgsigqueueinfo
