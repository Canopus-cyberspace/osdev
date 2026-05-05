# LAST_APPLIED_FIX

Version: v65

Goal:
- Add common path/tty/fcntl syscall scaffolds used by libc startup, shells, and utilities.

Verified:
- getcwd
- fcntl
- ioctl TIOCGWINSZ
- readlinkat
- umask
- chdir
- prior fd/mm/time/proc regressions remain passing
