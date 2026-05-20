#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
MOD = ROOT / "src" / "syscall" / "mod.rs"
EXPECTED = [
    ("BPF", 280), ("EXECVEAT", 281), ("USERFAULTFD", 282), ("MEMBARRIER", 283),
    ("MLOCK2", 284), ("COPY_FILE_RANGE", 285), ("PREADV2", 286), ("PWRITEV2", 287),
    ("PKEY_MPROTECT", 288), ("PKEY_ALLOC", 289), ("PKEY_FREE", 290), ("STATX", 291),
    ("IO_PGETEVENTS", 292), ("RSEQ", 293), ("KEXEC_FILE_LOAD", 294),
    ("PIDFD_SEND_SIGNAL", 424), ("IO_URING_SETUP", 425), ("IO_URING_ENTER", 426),
    ("IO_URING_REGISTER", 427), ("OPEN_TREE", 428), ("MOVE_MOUNT", 429),
    ("FSOPEN", 430), ("FSCONFIG", 431), ("FSMOUNT", 432), ("FSPICK", 433),
    ("PIDFD_OPEN", 434), ("CLONE3", 435), ("CLOSE_RANGE", 436), ("OPENAT2", 437),
    ("PIDFD_GETFD", 438), ("FACCESSAT2", 439), ("PROCESS_MADVISE", 440),
    ("EPOLL_PWAIT2", 441), ("MOUNT_SETATTR", 442), ("QUOTACTL_FD", 443),
    ("LANDLOCK_CREATE_RULESET", 444), ("LANDLOCK_ADD_RULE", 445),
    ("LANDLOCK_RESTRICT_SELF", 446), ("MEMFD_SECRET", 447), ("PROCESS_MRELEASE", 448),
    ("FUTEX_WAITV", 449), ("SET_MEMPOLICY_HOME_NODE", 450), ("CACHESTAT", 451),
    ("FCHMODAT2", 452), ("MAP_SHADOW_STACK", 453), ("FUTEX_WAKE", 454),
    ("FUTEX_WAIT", 455), ("FUTEX_REQUEUE", 456), ("STATMOUNT", 457),
    ("LISTMOUNT", 458), ("LSM_GET_SELF_ATTR", 459), ("LSM_SET_SELF_ATTR", 460),
    ("LSM_LIST_MODULES", 461), ("MSEAL", 462), ("SETXATTRAT", 463),
    ("GETXATTRAT", 464), ("LISTXATTRAT", 465), ("REMOVEXATTRAT", 466),
    ("OPEN_TREE_ATTR", 467), ("FILE_GETATTR", 468), ("FILE_SETATTR", 469),
    ("LISTNS", 470), ("RSEQ_SLICE_YIELD", 471),
]

def fail(msg: str):
    print(f"[ERROR] {msg}")
    sys.exit(1)

if not MOD.exists():
    fail(f"missing {MOD}")
src = MOD.read_text(encoding="utf-8", errors="ignore")
missing_consts, wrong_consts, missing_arms, duplicate_arms = [], [], [], []
for name, nr in EXPECTED:
    m = re.search(rf"\b(?:pub\s+)?const\s+SYS_{name}\s*:\s*(?:usize|isize|u64|i64)\s*=\s*([0-9]+)\s*;", src)
    if not m:
        missing_consts.append(f"SYS_{name}={nr}")
    elif int(m.group(1)) != nr:
        wrong_consts.append(f"SYS_{name}: expected {nr}, found {m.group(1)}")
    arm_count = len(re.findall(rf"\bSYS_{name}\s*=>", src))
    if arm_count == 0:
        missing_arms.append(f"SYS_{name}")
    elif arm_count > 1:
        duplicate_arms.append(f"SYS_{name} x{arm_count}")
soft_missing = {"SYS_KEXEC_FILE_LOAD", "SYS_BPF"}
hard_missing_arms = [x for x in missing_arms if x not in soft_missing]
print("[INFO] v87 syscall scaffold coverage guard")
print(f"[INFO] checked expected modern generic syscall constants/arms: {len(EXPECTED)}")
if missing_consts:
    fail("missing syscall constants: " + ", ".join(missing_consts))
if wrong_consts:
    fail("wrong syscall numbers: " + ", ".join(wrong_consts))
if hard_missing_arms:
    fail("missing dispatcher arms: " + ", ".join(hard_missing_arms))
if duplicate_arms:
    fail("duplicate dispatcher arms: " + ", ".join(duplicate_arms))
print("[PASS] v87 syscall coverage guard found required constants and non-duplicate dispatcher arms")
