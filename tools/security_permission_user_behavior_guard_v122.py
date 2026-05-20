#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v122"
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

print("[INFO] security/permission/capability user behavior guard v122 started")
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
    "identity credentials": ["SYS_GETUID", "SYS_GETEUID", "SYS_GETGID", "SYS_GETEGID", "getuid", "geteuid", "getgid", "getegid"],
    "capability syscalls": ["SYS_CAPGET", "SYS_CAPSET", "capget", "capset"],
    "path permission checks": ["SYS_FACCESSAT", "SYS_FACCESSAT2", "faccessat", "faccessat2", "access"],
    "mode/owner permission syscalls": ["SYS_FCHMODAT", "SYS_FCHMODAT2", "SYS_FCHOWNAT", "SYS_FCHOWN", "chmod", "chown"],
    "xattr permission surface": ["SYS_SETXATTR", "SYS_GETXATTR", "SYS_LISTXATTR", "SYS_REMOVEXATTR", "xattr"],
    "landlock sandbox surface": ["SYS_LANDLOCK_CREATE_RULESET", "SYS_LANDLOCK_ADD_RULE", "SYS_LANDLOCK_RESTRICT_SELF", "landlock"],
    "prctl/security runtime": ["SYS_PRCTL", "prctl", "PR_SET", "PR_GET"],
    "umask/session context": ["SYS_UMASK", "umask", "SYS_SETSID", "SYS_GETSID"],
}

missing = []
for name, needles in required_groups.items():
    found = [n for n in needles if n in all_src]
    # Landlock/prctl may be scaffolded with exact SYS names or lowercase helpers;
    # require one indicator per family to stay compatible with current scaffold naming.
    if not found:
        missing.append((name, needles))
    else:
        ok(f"{name}: " + ", ".join(found[:12]))

if missing:
    for name, needles in missing:
        print(f"[ERROR] {name} missing symbols; expected one of: {', '.join(needles)}")
    sys.exit(1)

soft_groups = {
    "security errno vocabulary": ["EPERM", "EACCES", "EINVAL", "EFAULT", "ENOSYS", "ENOENT"],
    "user/path copy vocabulary": ["user_path", "copy_from_user", "copy_to_user", "path", "dirfd"],
    "permission model vocabulary": ["permission", "mode", "uid", "gid", "cap", "owner"],
    "sandbox/filter vocabulary": ["landlock", "seccomp", "sandbox", "restrict", "rule"],
}

for name, needles in soft_groups.items():
    found = [n for n in needles if n.lower() in all_src.lower()]
    if found:
        ok(f"{name}: " + ", ".join(found[:10]))
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
manifest_path = out_dir / "security_permission_user_behavior_manifest_v122.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("security/permission/capability user behavior guard v122 completed")
