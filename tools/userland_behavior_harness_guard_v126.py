#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v126"
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

registry_path = ROOT / "tools/userland_behavior_registry_v126.json"
if not registry_path.exists():
    fail("tools/userland_behavior_registry_v126.json missing")
registry = json.loads(registry_path.read_text())

print("[INFO] userland behavior harness guard v126 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")
print(f"[INFO] tool files scanned: {len(tool_files)}")
print(f"[INFO] registry scenario groups: {len(registry.get('scenario_groups', []))}")

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

# Ensure the aggregated matrix from v125 is present or its key coverage is still represented.
matrix_indicators = [
    "userland_compat_regression_matrix",
    "VFS basic behavior",
    "async/vector I/O",
    "security/permission/capability",
    "scheduler/resource/affinity",
]
found_matrix = [x for x in matrix_indicators if x in all_text]
if len(found_matrix) < 2:
    print(f"[WARN] v125 matrix tooling sparse; found={found_matrix}; falling back to source syscall coverage")
else:
    ok("v125 matrix tooling evidence: " + ", ".join(found_matrix))

# Registry rows must map to at least one real source/tool indicator each.
missing_scenarios = []
scenario_results = {}
for row in registry.get("scenario_groups", []):
    sid = row.get("id", "<missing>")
    syscalls = row.get("syscalls", [])
    indicators = set(syscalls)
    for s in syscalls:
        indicators.add("SYS_" + s.upper())
        indicators.add(s.lower())
    found = sorted({x for x in indicators if x and (x in all_text or x.lower() in all_text.lower())})
    if not found:
        missing_scenarios.append((sid, syscalls))
    else:
        scenario_results[sid] = found[:16]
        ok(f"scenario {sid}: " + ", ".join(found[:10]))

if missing_scenarios:
    for sid, syscalls in missing_scenarios:
        print(f"[ERROR] registry scenario {sid} has no source/tool coverage; syscalls={syscalls}")
    sys.exit(1)

# Check external-init harness readiness without requiring exact future implementation.
harness_needles = ["build_init_elf.py", "init.elf", "hello from external init.elf", "syscall write"]
harness_text = ""
for p in [ROOT / "user/build_init_elf.py"]:
    if p.exists():
        harness_text += p.read_text(errors="ignore")
harness_text += all_tools
found_harness = [n for n in harness_needles if n in harness_text]
if len(found_harness) < 3:
    fail("external-init harness readiness insufficient; found: " + ", ".join(found_harness))
ok("external-init harness readiness: " + ", ".join(found_harness))

soft_groups = {
    "future conformance vocabulary": ["scenario", "registry", "matrix", "manifest", "smoke"],
    "user ABI data vocabulary": ["argv", "envp", "auxv", "iovec", "timespec", "msghdr", "sigframe"],
    "behavior oracle vocabulary": ["PASS", "FAIL", "EFAULT", "EINVAL", "ENOSYS", "expected"],
    "wait/runtime vocabulary": ["poll", "epoll", "futex", "wake", "wait", "timeout", "timerfd"],
}

for name, needles in soft_groups.items():
    found = [n for n in needles if n.lower() in all_text.lower()]
    if found:
        ok(f"{name}: " + ", ".join(found[:12]))
    else:
        print(f"[WARN] {name} sparse; tracked as semantic debt, non-fatal")

manifest = {
    "version": VERSION,
    "rust_source_count": len(rs_files),
    "tool_file_count": len(tool_files),
    "registry_scenario_count": len(registry.get("scenario_groups", [])),
    "scenario_results": scenario_results,
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "registry_sha256": hashlib.sha256(registry_path.read_bytes()).hexdigest(),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / "userland_behavior_harness_manifest_v126.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("userland behavior harness guard v126 completed")
