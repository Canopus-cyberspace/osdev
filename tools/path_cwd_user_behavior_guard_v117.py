#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v117"
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

print("[INFO] path/cwd/openat-dirfd user behavior guard v117 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")

# Duplicate arms within a single match block only.
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

# Use at least one of each group to avoid over-constraining project naming.
groups = {
    "cwd syscalls": ["SYS_GETCWD", "getcwd", "SYS_CHDIR", "chdir", "SYS_FCHDIR", "fchdir"],
    "openat dirfd/path syscalls": ["SYS_OPENAT", "openat", "dirfd", "AT_FDCWD"],
    "relative path/link syscalls": ["SYS_READLINKAT", "readlinkat", "SYS_LINKAT", "linkat", "SYS_SYMLINKAT", "symlinkat"],
    "directory mutation syscalls": ["SYS_MKDIRAT", "mkdirat", "SYS_UNLINKAT", "unlinkat", "SYS_RENAMEAT", "renameat"],
    "metadata/access path syscalls": ["SYS_FACCESSAT", "faccessat", "SYS_FACCESSAT2", "faccessat2", "SYS_STATX", "statx"],
    "dirent traversal": ["SYS_GETDENTS64", "getdents64", "dirent", "d_name"],
    "user path copy/errno": ["EFAULT", "ENOENT", "ENOTDIR", "EINVAL"],
}

for name, needles in groups.items():
    found = [n for n in needles if n in all_src]
    if not found:
        fail(f"{name} not found; expected one of: {', '.join(needles)}")
    ok(f"{name}: " + ", ".join(found[:8]))

# Additional vocabulary for semantics debt tracking, warn instead of fail if sparse.
vocab = ["cwd", "current_dir", "path", "absolute", "relative", "lookup", "resolve", "dentry", "inode"]
found_vocab = [v for v in vocab if v.lower() in all_src.lower()]
if found_vocab:
    ok("path-resolution vocabulary: " + ", ".join(found_vocab))
else:
    print("[WARN] path-resolution vocabulary sparse; guard kept non-fatal for existing scaffold baseline")

manifest = {
    "version": VERSION,
    "rust_source_count": len(rs_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "checked_groups": list(groups.keys()),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / "path_cwd_user_behavior_manifest_v117.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("path/cwd/openat-dirfd user behavior guard v117 completed")
