#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v135b"
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
all_src = "\n".join(read(p) for p in rs_files)

syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    fail("src/syscall/mod.rs not found")
mod_text = read(syscall_mod)

print("[INFO] openat errno guard repair v135b started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")

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

for name, needles in {
    "openat dispatch": ["SYS_OPENAT", "openat", "sys_openat"],
    "VFS path": ["path", "file", "fd"],
}.items():
    found = [n for n in needles if n in all_src or n.lower() in all_src.lower()]
    if len(found) < 2:
        fail(f"{name} insufficient: {found}")
    ok(f"{name}: " + ", ".join(found))

builder = ROOT / "user/build_init_elf.py"
if not builder.exists():
    fail("user/build_init_elf.py missing")
builder_text = read(builder)

# v135 builder generates RET_* strings dynamically via ERRNOS, so do not require literal RET_ENOSYS in source.
for token in ["ERRNOS", "ENOSYS", "ENOENT", "EINVAL", "EFAULT", "EBADF", "EACCES", "ENOTDIR", "EEXIST", "RET_{errno_name}", "openat errno classification"]:
    if token not in builder_text:
        fail(f"builder missing diagnostic generator token: {token}")
ok("builder contains errno classification generator")

init_elf = ROOT / "user/init.elf"
if not init_elf.exists():
    fail("user/init.elf missing")
blob = init_elf.read_bytes()
for token in [b"hello from external init.elf v135 syscall write", b"[ucompat-v135] openat_errno", b"RET_ENOSYS", b"RET_ENOENT", b"RET_EINVAL", b"RET_EFAULT", b"RET_EBADF", b"RET_OTHER_NEG"]:
    if token not in blob:
        fail(f"user/init.elf missing embedded token: {token!r}")
ok("user/init.elf embeds openat errno classification markers")

manifest = {
    "version": VERSION,
    "phase": "openat-errno-guard-repair",
    "diagnostic_prefix": "[ucompat-v135] openat_errno",
    "init_elf_sha256": hashlib.sha256(blob).hexdigest(),
    "init_elf_bytes": len(blob),
    "builder_sha256": hashlib.sha256(builder_text.encode()).hexdigest(),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "rust_source_count": len(rs_files),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / "openat_errno_guard_repair_manifest_v135b.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("openat errno guard repair v135b completed")
