# Full-Score Master Plan

## Principle

Use official score as the main feedback loop. Implement the next official-scoring capability, not an abstract "whole Linux" in one run.

## P01: RISC-V basic-musl expansion 1

Focus:

```text
getpid
getppid
uname
getcwd
close
dup
dup2
open
read
openat
fstat
getdents
```

Goal: raise `basic-musl-rv` above 2.0.

## P02: RISC-V basic-musl filesystem expansion

Focus:

```text
writev
readv
lseek
stat
lstat
newfstatat
mkdir
chdir
unlink
rename
link
symlink
readlink
access
faccessat
```

Goal: broader VFS correctness.

## P03: RISC-V basic-musl process/time expansion

Focus:

```text
fork
clone
execve
wait4
waitpid
yield
sleep
nanosleep
times
clock_gettime
gettimeofday
pipe
poll
```

## P04: RISC-V basic-musl memory expansion

Focus:

```text
brk
mmap
munmap
mprotect
invalid pointer checks
```

This may require real MM work.

## P05: General Ext4/virtio-blk execution

Goal: move from selected official files to a general read-only official sdcard filesystem path.

Work:

```text
virtio-blk request path
block cache
ext4 superblock/group/inode/dirent/block map
open/read/stat/getdents from sdcard
execve official ELF from image
```

## P06: BusyBox musl RISC-V

Goal: nonzero and increasing `busybox-musl-rv`.

Commands:

```text
sh
ls
cat
echo
pwd
mkdir
rm
cp
mv
ps
```

## P07: libc-test / libcbench RISC-V

Focus:

```text
mmap/brk/page fault
futex
clone/thread basics
signal
clock/time
errno correctness
```

## P08: LTP musl RISC-V subset

Focus:

```text
error paths
permissions
fork/exec/wait
mmap/mprotect/munmap
signal
pipe/poll/epoll
futex
ipc
namespace basics
```

## P09: Performance suites

Focus:

```text
lmbench
iozone
cyclictest
netperf/iperf if network exists
```

## P10: LoongArch64 bring-up

Only after RISC-V score path is strong.

Work:

```text
HAL extraction
kernel-la real ELF
LoongArch boot
trap/syscall
page table/TLB
console/timer
LoongArch basic-musl nonzero score
```

## P11: LoongArch64 score expansion

Mirror high-value RISC-V phases on LoongArch.

## P12: final full-score submission hardening

Work:

```text
syscall support matrix
suite compatibility matrix
known limitations
performance evidence
clean build/run scripts
final technical report
official full harness logs
release package
```
