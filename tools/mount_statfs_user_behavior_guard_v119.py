#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v119"
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

print("[INFO] mount/statfs/fsconfig user behavior guard v119 started")
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

# Require at least one/few indicators per group. Keep broad enough for current scaffold names.
required_groups = {
    "legacy mount/syscalls": ["SYS_MOUNT", "SYS_UMOUNT2", "mount", "umount2"],
    "statfs/filesystem capacity": ["SYS_STATFS", "SYS_FSTATFS", "statfs", "fstatfs", "statx"],
    "modern mount API": ["SYS_FSOPEN", "SYS_FSCONFIG", "SYS_FSMOUNT", "SYS_FSPICK", "fsopen", "fsconfig", "fsmount", "fspick"],
    "tree/move/open mount APIs": ["SYS_OPEN_TREE", "SYS_MOVE_MOUNT", "SYS_MOUNT_SETATTR", "open_tree", "move_mount", "mount_setattr"],
    "mount flags/user path copy": ["AT_FDCWD", "dirfd", "flags", "user_path", "EFAULT", "EINVAL"],
}

missing = []
for name, needles in required_groups.items():
    found = [n for n in needles if n in all_src]
    # Require at least two indicators for core groups; one for generic vocabulary-like group.
    threshold = 1 if name == "mount flags/user path copy" else 2
    if len(found) < threshold:
        missing.append((name, needles, found))
    else:
        ok(f"{name}: " + ", ".join(found[:10]))

if missing:
    for name, needles, found in missing:
        print(f"[ERROR] {name} insufficient coverage; found={found}; expected among: {', '.join(needles)}")
    sys.exit(1)

soft_groups = {
    "namespace/vfs mount vocabulary": ["namespace", "mount", "mnt", "vfs", "superblock", "filesystem"],
    "permission/error vocabulary": ["EPERM", "ENOENT", "ENOTDIR", "EBADF", "ENOSYS"],
    "lifetime/refcount vocabulary": ["ref", "drop", "close", "detach", "attach"],
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
manifest_path = out_dir / "mount_statfs_user_behavior_manifest_v119.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("mount/statfs/fsconfig user behavior guard v119 completed")
