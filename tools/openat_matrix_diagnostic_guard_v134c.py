#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v134c"
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

print("[INFO] openat matrix diagnostic guard v134c started")
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
    "path flags": ["O_CREAT", "O_TRUNC", "AT_FDCWD", "flags"],
    "VFS path": ["path", "inode", "dentry", "file", "fd"],
}.items():
    found = [n for n in needles if n in all_src or n.lower() in all_src.lower()]
    if len(found) < 2:
        fail(f"{name} insufficient: {found}")
    ok(f"{name}: " + ", ".join(found))

builder = ROOT / "user/build_init_elf.py"
builder_text = read(builder)
for token in ["t1_prev_v133b_create_trunc", "t2_short_create_trunc", "t3_short_create_no_trunc", "t4_tiny_create_trunc", "t5_abs_create_trunc", "t6_readonly_base"]:
    if token not in builder_text:
        fail(f"builder missing matrix test token: {token}")
ok("builder contains openat matrix test cases")

init_elf = ROOT / "user/init.elf"
blob = init_elf.read_bytes()
for token in [b"hello from external init.elf v134c syscall write", b"[ucompat-v134c] openat_matrix", b"openat_matrix DONE"]:
    if token not in blob:
        fail(f"user/init.elf missing embedded token: {token!r}")
ok("user/init.elf embeds openat matrix markers")

manifest = {
    "version": VERSION,
    "phase": "openat-matrix-diagnostic",
    "matrix_prefix": "[ucompat-v134c] openat_matrix",
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
manifest_path = out_dir / "openat_matrix_diagnostic_manifest_v134c.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("openat matrix diagnostic guard v134c completed")
