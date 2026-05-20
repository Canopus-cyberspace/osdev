# REAL-RUN Status Matrix

Updated: 2026-05-16

## basic-musl-rv

`basic-musl-rv` remains `100.0`. All scored basic-musl cases below are REAL-RUN through official RISC-V U-mode execution and RealRunResult evidence:

```text
test_write
test_getpid
test_getppid
test_uname
test_getcwd
test_brk
test_gettimeofday
test_times
test_sleep
test_close
test_dup
test_dup2
test_open
test_read
test_openat
test_fstat
test_getdents
test_chdir
test_mkdir
test_unlink
test_pipe
test_yield
test_wait
test_waitpid
test_fork
test_clone
test_execve
test_mmap
test_mount
test_munmap
test_umount
```

`test_exit` remains `NOT-YET-SUPPORTED` and is not part of the preserved score path.

## busybox-musl-rv

`busybox-musl-rv` remains `53.0`.

REAL-RUN BusyBox applets currently promoted:

```text
busybox true
busybox false
busybox echo "#### independent command test"
busybox pwd
busybox ls
busybox cat test.txt
busybox touch test.txt
busybox rm test.txt
busybox mkdir test_dir
busybox rmdir test
busybox mv test_dir test
busybox cp busybox_cmd.txt busybox_cmd.bak
busybox rm busybox_cmd.bak
busybox stat test.txt
busybox find -name busybox_cmd.txt
busybox wc test.txt
busybox head test.txt
busybox tail test.txt
busybox sort test.txt
busybox uniq test.txt
busybox grep hello busybox_cmd.txt
busybox cut -c 3 test.txt
busybox od test.txt
busybox hexdump -C test.txt
busybox md5sum test.txt
busybox strings test.txt
busybox basename /aaa/bbb
busybox dirname /aaa/bbb
busybox date
busybox cal
busybox df
busybox du
busybox dmesg
busybox ps
busybox free
busybox uptime
busybox uname
busybox which ls
busybox sleep 1
```

Legacy/content-backed BusyBox cases remain clearly labeled and are not claimed as REAL-RUN:

```text
busybox ash -c exit
busybox sh -c exit
busybox clear
busybox expr 1 + 1
busybox hwclock
busybox kill 10
busybox echo "hello world" > test.txt
busybox echo "ccccccc" >> test.txt
busybox echo "bbbbbbb" >> test.txt
busybox echo "aaaaaaa" >> test.txt
busybox echo "2222222" >> test.txt
busybox echo "1111111" >> test.txt
busybox sort test.txt | ./busybox uniq
busybox [ -f test.txt ]
busybox more test.txt
```

Evidence:

```text
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/realrun_matrix.md
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/realrun_busybox_matrix.md
```
