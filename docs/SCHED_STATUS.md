# SCHED_STATUS

## v71b - scheduler/resource/prctl scaffold

Implemented:
- sched_getscheduler returns SCHED_OTHER
- sched_getparam writes priority 0
- sched_getaffinity writes CPU0 mask and returns 8
- sched_get_priority_max/min return 0
- clock_getres writes 1ms resolution
- clock_nanosleep returns immediately
- getrusage writes zeroed rusage
- prctl(PR_GET_NAME) writes `init`
- getcpu writes CPU 0 / NUMA node 0
- riscv_flush_icache returns 0
- membarrier query returns a minimal supported mask

Still TODO:
- real scheduler policy and priorities
- real CPU affinity mask
- real resource usage counters
- real prctl option matrix
- real cross-hart icache flush
