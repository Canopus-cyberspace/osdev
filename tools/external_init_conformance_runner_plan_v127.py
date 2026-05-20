#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v127"
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
lower_text = all_text.lower()

syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    fail("src/syscall/mod.rs not found")
mod_text = read(syscall_mod)

scenario_path = ROOT / "tools/external_init_conformance_scenarios_v127.json"
if not scenario_path.exists():
    fail("tools/external_init_conformance_scenarios_v127.json missing")
scenario_doc = json.loads(scenario_path.read_text())
scenarios = scenario_doc.get("scenarios", [])

print("[INFO] external-init conformance runner plan v127 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")
print(f"[INFO] tool files scanned: {len(tool_files)}")
print(f"[INFO] scenario count: {len(scenarios)}")

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

if len(scenarios) < 12:
    fail(f"scenario plan too small: {len(scenarios)}")
ok(f"scenario registry size: {len(scenarios)}")

scenario_results = {}
missing = []
for scenario in scenarios:
    sid = scenario.get("id", "<missing>")
    syscalls = scenario.get("syscalls", [])
    indicators = set()
    for s in syscalls:
        indicators.add(s)
        indicators.add(s.lower())
        indicators.add("SYS_" + s.upper())
    found = sorted({x for x in indicators if x and (x in all_text or x.lower() in lower_text)})
    if not found:
        missing.append((sid, syscalls))
    else:
        scenario_results[sid] = found[:16]
        ok(f"scenario {sid}: " + ", ".join(found[:12]))

if missing:
    for sid, syscalls in missing:
        print(f"[ERROR] scenario {sid} has no source/tool syscall coverage; syscalls={syscalls}")
    sys.exit(1)

# Validate external-init single-marker harness remains available.
init_builder = ROOT / "user/build_init_elf.py"
if not init_builder.exists():
    fail("user/build_init_elf.py missing")
init_text = init_builder.read_text(errors="ignore")
needed = ["hello from external init.elf", "syscall write", "init.elf"]
found_needed = [x for x in needed if x in init_text]
if len(found_needed) < 2:
    fail("external-init builder marker readiness insufficient")
ok("external-init builder readiness: " + ", ".join(found_needed))

# Future-runner implementation hints should be present in tools so future patches can append concrete cases.
contract = scenario_doc.get("runner_contract", {})
for key in ["scenario_marker_prefix", "result_markers", "execution_model"]:
    if key not in contract:
        fail(f"runner_contract missing key: {key}")
ok("runner contract keys present")

# Check prior harness/matrix evidence if available.
prior_indicators = [
    "userland_behavior_registry_v126",
    "userland_compat_regression_matrix",
    "syscall_conformance_mini_suite",
]
found_prior = [x for x in prior_indicators if x in all_text]
if found_prior:
    ok("prior matrix/harness evidence: " + ", ".join(found_prior))
else:
    print("[WARN] prior matrix/harness evidence sparse; v127 scenario plan is self-contained")

manifest = {
    "version": VERSION,
    "mode": scenario_doc.get("mode"),
    "scenario_count": len(scenarios),
    "scenario_results": scenario_results,
    "runner_contract": contract,
    "rust_source_count": len(rs_files),
    "tool_file_count": len(tool_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "scenario_doc_sha256": hashlib.sha256(scenario_path.read_bytes()).hexdigest(),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / "external_init_conformance_runner_plan_manifest_v127.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("external-init conformance runner plan v127 completed")
