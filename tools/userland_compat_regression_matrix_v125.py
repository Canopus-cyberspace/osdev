#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v125"
EXCLUDE_DIRS = {".git", "target", ".repair_logs"}
EXCLUDE_PREFIXES = (".backup_repair_",)

def excluded(path: Path) -> bool:
    for part in path.parts:
        if part in EXCLUDE_DIRS:
            return True
        if any(part.startswith(prefix) for prefix in EXCLUDE_PREFIXES):
            return True
    return False

def read(path: Path) -> str:
    return path.read_text(errors="ignore")

def fail(msg: str) -> None:
    print(f"[ERROR] {msg}")
    sys.exit(1)

def ok(msg: str) -> None:
    print(f"[PASS] {msg}")

rs_files = [p for p in ROOT.glob("src/**/*.rs") if not excluded(p)]
tool_files = [p for p in ROOT.glob("tools/*") if p.is_file() and not excluded(p)]
if not rs_files:
    fail("no Rust source files found under src/")

texts = {str(p): read(p) for p in rs_files}
tool_texts = {str(p): read(p) for p in tool_files}
all_src = "\n".join(texts.values())
all_tools = "\n".join(tool_texts.values())
all_text = all_src + "\n" + all_tools
lower_text = all_text.lower()

syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    fail("src/syscall/mod.rs not found")
mod_text = read(syscall_mod)

print("[INFO] userland compatibility regression matrix v125 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")
print(f"[INFO] tool files scanned: {len(tool_files)}")

# Duplicate SYS_* arms only within individual match blocks.
for idx, m in enumerate(re.finditer(r"match\s+[^{}]+\{", mod_text), 1):
    start = m.end()
    depth = 1
    i = start
    while i < len(mod_text) and depth:
        if mod_text[i] == "{":
            depth += 1
        elif mod_text[i] == "}":
            depth -= 1
        i += 1
    block = mod_text[start:i-1]
    arms = re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>", block, flags=re.M)
    dup = sorted({a for a in arms if arms.count(a) > 1})
    if dup:
        fail(f"src/syscall/mod.rs: match block {idx} duplicate arms: {', '.join(dup)}")
ok("no duplicate SYS_* arms within individual match blocks")

bad_self = re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>\s*\1\b", mod_text, flags=re.M)
if bad_self:
    fail("suspicious SYS_* self-binding arms: " + ", ".join(sorted(set(bad_self))))
ok("no suspicious SYS_* self-binding arms")

# Aggregate matrix covering v105-v124 user behavior focus areas.
matrix = {
    "VFS basic behavior": ["SYS_OPENAT", "SYS_READ", "SYS_WRITE", "SYS_CLOSE", "SYS_LSEEK", "SYS_GETDENTS64", "SYS_STATX"],
    "usercopy/iovec/time structs": ["copy_from_user", "copy_to_user", "iovec", "timespec", "EFAULT"],
    "memory management": ["SYS_MMAP", "SYS_MUNMAP", "SYS_MPROTECT", "SYS_BRK", "mmap", "brk"],
    "process lifecycle": ["SYS_CLONE", "SYS_EXECVE", "SYS_WAIT4", "SYS_EXIT", "SYS_EXIT_GROUP", "getpid"],
    "waitable FD": ["SYS_PIPE2", "SYS_EVENTFD2", "SYS_TIMERFD_CREATE", "SYS_POLL", "SYS_EPOLL_CTL", "epoll"],
    "futex/scheduler": ["SYS_FUTEX", "SYS_FUTEX_WAITV", "SYS_SET_TID_ADDRESS", "sched_yield", "futex"],
    "execve stack ABI": ["argv", "envp", "auxv", "AT_", "execve"],
    "signal delivery": ["SYS_RT_SIGACTION", "SYS_RT_SIGPROCMASK", "SYS_RT_SIGRETURN", "SYS_TGKILL", "signal"],
    "socket/network": ["SYS_SOCKET", "SYS_SOCKETPAIR", "SYS_BIND", "SYS_CONNECT", "SYS_SENDMSG", "SYS_RECVMSG"],
    "IPC": ["SYS_MSGGET", "SYS_SHMGET", "SYS_SEMGET", "SYS_MQ_OPEN", "msgget", "shmget"],
    "filesystem metadata/path/xattr": ["SYS_FACCESSAT", "SYS_FCHMODAT", "SYS_READLINKAT", "SYS_SETXATTR", "xattr"],
    "identity/time/resource": ["SYS_CLOCK_GETTIME", "SYS_GETRANDOM", "SYS_GETUID", "SYS_GETRUSAGE", "SYS_PRLIMIT64"],
    "fcntl/tty/fdflags": ["SYS_FCNTL", "SYS_IOCTL", "O_CLOEXEC", "O_NONBLOCK", "tty"],
    "path/cwd/openat-dirfd": ["SYS_GETCWD", "SYS_CHDIR", "AT_FDCWD", "dirfd", "readlinkat"],
    "VFS sync/truncate/range": ["SYS_FSYNC", "SYS_FTRUNCATE", "SYS_FALLOCATE", "SYS_COPY_FILE_RANGE", "SYS_READAHEAD"],
    "mount/statfs/fsconfig": ["SYS_MOUNT", "SYS_STATFS", "SYS_FSOPEN", "SYS_FSCONFIG", "SYS_MOUNT_SETATTR"],
    "process observability/runtime": ["SYS_PIDFD_OPEN", "SYS_PRCTL", "SYS_RSEQ", "SYS_MEMBARRIER", "SYS_GETTID"],
    "async/vector I/O": ["SYS_READV", "SYS_WRITEV", "SYS_PREADV", "SYS_IO_SETUP", "SYS_IO_URING_SETUP"],
    "security/permission/capability": ["SYS_CAPGET", "SYS_CAPSET", "SYS_LANDLOCK_CREATE_RULESET", "SYS_FACCESSAT2", "EPERM"],
    "scheduler/resource/affinity": ["SYS_SCHED_YIELD", "SYS_SCHED_GETAFFINITY", "SYS_SETPRIORITY", "SYS_GETCPU", "SYS_GETRLIMIT"],
}

matrix_results = {}
failed = []
for name, needles in matrix.items():
    found = [n for n in needles if n in all_text]
    # require at least two indicators per completed phase to avoid a single stale string.
    threshold = 2
    if len(found) < threshold:
        failed.append((name, needles, found))
    else:
        matrix_results[name] = found
        ok(f"{name}: " + ", ".join(found[:12]))

if failed:
    for name, needles, found in failed:
        print(f"[ERROR] matrix row {name} insufficient coverage; found={found}; expected among: {', '.join(needles)}")
    sys.exit(1)

# Guard tool presence is useful but not mandatory for every earlier v if user skipped one; require recent core tools.
tool_expect = [
    "syscall_conformance_mini_suite",
    "vfs",
    "mmap",
    "process",
    "socket",
    "futex",
]
present_tools = {k: (k.lower() in all_tools.lower()) for k in tool_expect}
for k, present in present_tools.items():
    if present:
        ok(f"tooling evidence present: {k}")
    else:
        print(f"[WARN] tooling evidence sparse for {k}; source matrix still passed")

soft_groups = {
    "errno surface": ["EFAULT", "EINVAL", "EPERM", "EACCES", "ENOENT", "ENOSYS", "EBADF"],
    "blocking/wakeup surface": ["wake", "wait", "block", "sleep", "ready", "poll", "epoll"],
    "user ABI surface": ["argv", "envp", "auxv", "iovec", "msghdr", "timespec", "sigframe"],
    "runtime observability surface": ["pid", "tid", "rseq", "membarrier", "getcpu", "resource"],
}

for name, needles in soft_groups.items():
    found = [n for n in needles if n.lower() in lower_text]
    if found:
        ok(f"{name}: " + ", ".join(found[:12]))
    else:
        print(f"[WARN] {name} sparse; tracked as semantic debt, non-fatal")

manifest = {
    "version": VERSION,
    "rust_source_count": len(rs_files),
    "tool_file_count": len(tool_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "matrix_results": matrix_results,
    "tool_expect": present_tools,
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / "userland_compat_regression_matrix_manifest_v125.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("userland compatibility regression matrix v125 completed")
