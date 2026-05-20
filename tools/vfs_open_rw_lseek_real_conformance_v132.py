#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v132"
SCENARIO_PREFIX = "[ucompat-v132]"
SCENARIO_ID = "vfs_open_rw_lseek"
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

def has_any(text: str, needles: list[str]) -> list[str]:
    low = text.lower()
    found = []
    for n in needles:
        if n in text or n.lower() in low:
            found.append(n)
    return found

rs_files = [p for p in ROOT.glob("src/**/*.rs") if not excluded(p)]
tool_files = [p for p in ROOT.glob("tools/*") if p.is_file() and not excluded(p)]
if not rs_files:
    fail("no Rust source files found under src/")

texts = {str(p): read(p) for p in rs_files}
tool_texts = {str(p): read(p) for p in tool_files}
all_src = "\n".join(texts.values())
all_tools = "\n".join(tool_texts.values())
all_text = all_src + "\n" + all_tools

syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    fail("src/syscall/mod.rs not found")
mod_text = read(syscall_mod)

print("[INFO] VFS open/write/lseek/read/close real-conformance guard v132 started")
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

required = {
    "syscall constants/arms": ["SYS_OPENAT", "SYS_WRITE", "SYS_LSEEK", "SYS_READ", "SYS_CLOSE"],
    "file operation helpers": ["openat", "sys_openat", "write", "sys_write", "lseek", "sys_lseek", "read", "sys_read", "close", "sys_close"],
    "fd table / file object vocabulary": ["fd", "File", "file", "Fd", "descriptor"],
    "offset/content vocabulary": ["offset", "pos", "seek", "len", "size", "buffer"],
    "usercopy/error surface": ["copy_from_user", "copy_to_user", "EFAULT", "EBADF", "EINVAL"],
}
missing = []
coverage = {}
for name, needles in required.items():
    found = has_any(all_src, needles)
    coverage[name] = found
    threshold = 3 if name == "syscall constants/arms" else 2
    if len(found) < threshold:
        missing.append((name, needles, found))
    else:
        ok(f"{name}: " + ", ".join(found[:12]))

if missing:
    for name, needles, found in missing:
        print(f"[ERROR] {name} insufficient for real conformance scenario; found={found}; expected among: {', '.join(needles)}")
    sys.exit(1)

# Do not accept fake PASS injection in build_init_elf.py unless scenario text also has syscall-like vocabulary.
init_builder = ROOT / "user/build_init_elf.py"
if not init_builder.exists():
    fail("user/build_init_elf.py missing")
init_text = read(init_builder)
fake_pass = f"{SCENARIO_PREFIX} {SCENARIO_ID} PASS"
scenario_vocab = ["openat", "write", "lseek", "read", "close"]
scenario_vocab_found = has_any(init_text, scenario_vocab)
if fake_pass in init_text and len(scenario_vocab_found) < 4:
    fail("possible fake scenario PASS detected in user/build_init_elf.py without syscall vocabulary; refusing to validate")
ok("no fake v132 scenario PASS injection detected")

# Record the scenario contract. The actual PASS must come from QEMU serial logs after a future/user-init implementation emits it.
contract = {
    "version": VERSION,
    "scenario_prefix": SCENARIO_PREFIX,
    "scenario_id": SCENARIO_ID,
    "required_runtime_pass_marker": f"{SCENARIO_PREFIX} {SCENARIO_ID} PASS",
    "required_runtime_fail_marker": f"{SCENARIO_PREFIX} {SCENARIO_ID} FAIL",
    "required_runtime_skip_marker": f"{SCENARIO_PREFIX} {SCENARIO_ID} SKIP",
    "syscall_sequence": ["openat", "write", "lseek", "read", "close"],
    "oracle": "write known bytes to a file, seek to start, read back same bytes, close fd",
    "policy": "Do not report conformance PASS unless QEMU serial logs contain the scenario PASS marker from external init.",
}
contract_path = ROOT / "tools/vfs_open_rw_lseek_conformance_contract_v132.json"
contract_path.write_text(json.dumps(contract, indent=2, sort_keys=True))

out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest = {
    "version": VERSION,
    "phase": "first-real-conformance-scenario-readiness",
    "coverage": coverage,
    "contract": contract,
    "scenario_vocab_found_in_init_builder": scenario_vocab_found,
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "rust_source_count": len(rs_files),
    "tool_file_count": len(tool_files),
    "contract_path": str(contract_path),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
manifest_path = out_dir / "vfs_open_rw_lseek_real_conformance_guard_manifest_v132.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] contract: {contract_path}")
print(f"[INFO] manifest: {manifest_path}")
ok("VFS open/write/lseek/read/close real-conformance readiness guard v132 completed")
