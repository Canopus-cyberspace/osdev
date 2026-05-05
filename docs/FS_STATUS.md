# FS_STATUS

## v65 - path/tty/fcntl scaffold

Implemented:
- getcwd returns `/`
- chdir accepts `/` and returns 0
- readlinkat returns `/init.elf`
- fcntl handles common no-op commands
- ioctl handles `TIOCGWINSZ` with 24x80 winsize
- umask returns old mask 0

Still TODO:
- real cwd per process
- real path resolution
- real symlink nodes
- real file flags
- real terminal driver
