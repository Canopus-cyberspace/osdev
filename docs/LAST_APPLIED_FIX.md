# LAST_APPLIED_FIX

Version: v63

Goal:
- Add uname/clock_gettime/gettimeofday scaffolds.

Verified:
- uname writes utsname to user buffer
- clock_gettime writes timespec to user buffer
- gettimeofday writes timeval to user buffer
- prior mm/fd syscall regressions remain passing
