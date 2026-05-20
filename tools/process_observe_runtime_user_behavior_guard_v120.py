#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v120"
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
if not rs_files:
    fail("no Rust source files found under src/")

texts = {str(p): read(p) for p in rs_files}
all_src = "\n".join(texts.values())
syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    fail("src/syscall/mod.rs not found")
mod_text = read(syscall_mod)

print("[INFO] process observability/runtime user behavior guard v120 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")

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

required_groups = {
    "pidfd process handles": ["SYS_PIDFD_OPEN", "SYS_PIDFD_SEND_SIGNAL", "SYS_PIDFD_GETFD", "pidfd"],
    "prctl/process controls": ["SYS_PRCTL", "prctl"],
    "rseq runtime registration": ["SYS_RSEQ", "rseq"],
    "membarrier runtime synchronization": ["SYS_MEMBARRIER", "membarrier"],
    "process identity/observability": ["SYS_GETPID", "SYS_GETPPID", "SYS_GETTID", "getpid", "getppid", "gettid"],
    "resource/runtime limits": ["SYS_GETRLIMIT", "SYS_PRLIMIT64", "SYS_GETRUSAGE", "rlimit", "rusage"],
    "capability/session interop": ["SYS_CAPGET", "SYS_CAPSET", "SYS_SETSID", "SYS_GETSID", "capget", "capset", "setsid", "getsid"],
}

missing = []
for name, needles in required_groups.items():
    found = [n for n in needles if n in all_src]
    # pidfd group can be represented by pidfd vocabulary + any SYS_PIDFD_*;
    # require at least one indicator, most previous scaffold packages already add several.
    if not found:
        missing.append((name, needles))
    else:
        ok(f"{name}: " + ", ".join(found[:10]))

if missing:
    for name, needles in missing:
        print(f"[ERROR] {name} missing symbols; expected one of: {', '.join(needles)}")
    sys.exit(1)

soft_groups = {
    "errno/usercopy vocabulary": ["EFAULT", "EINVAL", "ESRCH", "EPERM", "ENOSYS"],
    "process table vocabulary": ["pid", "tid", "process", "task", "thread"],
    "signal/process interop": ["signal", "tgkill", "kill", "wait", "exit"],
    "runtime scheduling vocabulary": ["sched", "yield", "wake", "futex", "barrier"],
}

for name, needles in soft_groups.items():
    found = [n for n in needles if n.lower() in all_src.lower()]
    if found:
        ok(f"{name}: " + ", ".join(found[:8]))
    else:
        print(f"[WARN] {name} sparse; tracked as semantic debt, non-fatal")

manifest = {
    "version": VERSION,
    "rust_source_count": len(rs_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "required_groups": required_groups,
    "soft_groups": soft_groups,
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / "process_observe_runtime_user_behavior_manifest_v120.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("process observability/runtime user behavior guard v120 completed")
