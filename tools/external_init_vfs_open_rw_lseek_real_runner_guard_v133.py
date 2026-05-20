#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v133"
SCENARIO_PASS = "[ucompat-v132] vfs_open_rw_lseek PASS"
SCENARIO_FAIL = "[ucompat-v132] vfs_open_rw_lseek FAIL"
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

print("[INFO] external-init VFS real runner guard v133 started")
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

required_groups = {
    "syscall dispatch": ["SYS_OPENAT", "SYS_WRITE", "SYS_LSEEK", "SYS_READ", "SYS_CLOSE"],
    "VFS operation names": ["openat", "write", "lseek", "read", "close"],
    "FD/file state vocabulary": ["fd", "file", "offset", "pos", "seek"],
    "user buffer/error vocabulary": ["copy_from_user", "copy_to_user", "EFAULT", "EBADF", "EINVAL"],
}
coverage = {}
for name, needles in required_groups.items():
    found = [n for n in needles if n in all_src or n.lower() in all_src.lower()]
    coverage[name] = found
    if len(found) < 2:
        fail(f"{name} insufficient coverage: found={found}, expected among={needles}")
    ok(f"{name}: " + ", ".join(found[:12]))

builder = ROOT / "user/build_init_elf.py"
if not builder.exists():
    fail("user/build_init_elf.py missing")
builder_text = read(builder)
for token in ["SYS_OPENAT", "SYS_WRITE", "SYS_LSEEK", "SYS_READ", "SYS_CLOSE", "vfs-roundtrip-133", SCENARIO_PASS, SCENARIO_FAIL]:
    if token not in builder_text:
        fail(f"external init real runner builder missing token: {token}")
ok("external init builder contains real syscall sequence and pass/fail markers")

init_elf = ROOT / "user/init.elf"
if not init_elf.exists():
    fail("user/init.elf missing")
blob = init_elf.read_bytes()
for token in [b"hello from external init.elf v133 syscall write", SCENARIO_PASS.encode(), SCENARIO_FAIL.encode(), b"vfs-roundtrip-133"]:
    if token not in blob:
        fail(f"user/init.elf missing required embedded token: {token!r}")
ok("user/init.elf embeds base marker, scenario markers, and roundtrip payload")

manifest = {
    "version": VERSION,
    "phase": "external-init-real-vfs-open-rw-lseek-runner",
    "runtime_required_pass_marker": SCENARIO_PASS,
    "runtime_fail_marker": SCENARIO_FAIL,
    "coverage": coverage,
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
manifest_path = out_dir / "external_init_vfs_open_rw_lseek_real_runner_guard_manifest_v133.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("external-init VFS real runner guard v133 completed")
