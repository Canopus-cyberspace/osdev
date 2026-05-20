#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import os
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v116"
EXCLUDE_PREFIXES = (
    ".git",
    "target",
    ".repair_logs",
)
EXCLUDE_NAME_PREFIXES = (
    ".backup_repair_",
)

def excluded(path: Path) -> bool:
    parts = path.parts
    for part in parts:
        if part in EXCLUDE_PREFIXES:
            return True
        if any(part.startswith(p) for p in EXCLUDE_NAME_PREFIXES):
            return True
    return False

def read(path: Path) -> str:
    return path.read_text(errors="ignore")

rs_files = [p for p in ROOT.glob("src/**/*.rs") if not excluded(p)]
if not rs_files:
    print("[ERROR] no Rust source files found under src/")
    sys.exit(1)

texts = {str(p): read(p) for p in rs_files}
all_src = "\n".join(texts.values())
syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    print("[ERROR] src/syscall/mod.rs not found")
    sys.exit(1)

mod_text = read(syscall_mod)

def fail(msg: str):
    print(f"[ERROR] {msg}")
    sys.exit(1)

def ok(msg: str):
    print(f"[PASS] {msg}")

print("[INFO] fcntl/tty/fdflags user behavior guard v116 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")

# Duplicate SYS_* arms only within individual match blocks, not globally.
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

# Catch-all style mistakes: missing constants often appear as lowercase-ish variable binding or any-value warning later,
# but this guard checks obvious self-binding arms.
bad_self = re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>\s*\1\b", mod_text, flags=re.M)
if bad_self:
    fail("suspicious SYS_* self-binding arms: " + ", ".join(sorted(set(bad_self))))
ok("no suspicious SYS_* self-binding arms")

required_groups = {
    "fcntl constants/arms": ["SYS_FCNTL", "fcntl"],
    "ioctl/tty hooks": ["SYS_IOCTL", "ioctl"],
    "dup/fd duplication": ["SYS_DUP", "SYS_DUP3", "dup"],
    "fd close/open flags": ["SYS_CLOSE", "SYS_OPENAT", "O_CLOEXEC"],
    "read/write fd ops": ["SYS_READ", "SYS_WRITE", "read", "write"],
    "pollable fd interop": ["SYS_POLL", "SYS_PPOLL", "epoll"],
}

missing_groups = []
for name, needles in required_groups.items():
    missing = [n for n in needles if n not in all_src]
    if missing:
        missing_groups.append((name, missing))
    else:
        ok(f"{name}: " + ", ".join(needles))

if missing_groups:
    for name, missing in missing_groups:
        print(f"[ERROR] {name} missing symbols: {', '.join(missing)}")
    sys.exit(1)

vocab_groups = {
    "fd flags vocabulary": ["cloexec", "nonblock", "append", "flags"],
    "tty vocabulary": ["tty", "termios", "winsize"],
    "errno/usercopy vocabulary": ["EFAULT", "EINVAL", "ENOSYS"],
}

for name, needles in vocab_groups.items():
    found = [n for n in needles if n.lower() in all_src.lower()]
    if not found:
        fail(f"{name} not found; expected one of: {', '.join(needles)}")
    ok(f"{name}: " + ", ".join(found))

manifest = {
    "version": VERSION,
    "rust_source_count": len(rs_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "checked_groups": list(required_groups.keys()),
    "excluded": {
        "dirs": list(EXCLUDE_PREFIXES),
        "prefixes": list(EXCLUDE_NAME_PREFIXES),
    },
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / f"fcntl_tty_fdflags_user_behavior_manifest_v116.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("fcntl/tty/fdflags user behavior guard v116 completed")
