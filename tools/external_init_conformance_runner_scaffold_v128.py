#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v128"
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

def load_scenarios() -> dict:
    preferred = ROOT / "tools/external_init_conformance_scenarios_v127.json"
    fallback = {
        "version": VERSION,
        "mode": "runner-scaffold",
        "runner_contract": {
            "scenario_marker_prefix": "[ucompat-v128]",
            "result_markers": ["PASS", "FAIL", "SKIP"],
            "execution_model": "v128 validates runner scaffold and marker contract; future versions emit per-scenario result lines from external init."
        },
        "scenarios": [
            {"id": "vfs_open_rw_lseek", "group": "vfs", "syscalls": ["openat", "write", "lseek", "read", "close"], "oracle": "roundtrip bytes and offset"},
            {"id": "mmap_brk_basic", "group": "memory", "syscalls": ["mmap", "munmap", "mprotect", "brk"], "oracle": "mapped page access"},
            {"id": "process_fork_wait", "group": "process", "syscalls": ["clone", "exit", "wait4", "getpid"], "oracle": "child status"},
            {"id": "pipe_poll_eventfd", "group": "waitable_fd", "syscalls": ["pipe2", "poll", "eventfd2", "read", "write"], "oracle": "readiness and payload"},
            {"id": "futex_wait_wake", "group": "sync", "syscalls": ["futex", "sched_yield"], "oracle": "waiter wakes"},
            {"id": "signal_delivery_return", "group": "signal", "syscalls": ["rt_sigaction", "tgkill", "rt_sigreturn"], "oracle": "handler returns"},
            {"id": "socketpair_msg", "group": "socket", "syscalls": ["socketpair", "sendmsg", "recvmsg"], "oracle": "message roundtrip"},
            {"id": "identity_time_random", "group": "libc_base", "syscalls": ["clock_gettime", "getrandom", "getuid", "gettid"], "oracle": "result shapes"}
        ]
    }
    if preferred.exists():
        doc = json.loads(preferred.read_text())
        doc["loaded_from"] = str(preferred)
        return doc
    fallback_path = ROOT / "tools/external_init_conformance_scenarios_v128_fallback.json"
    fallback_path.write_text(json.dumps(fallback, indent=2, sort_keys=True))
    fallback["loaded_from"] = str(fallback_path)
    return fallback

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

scenario_doc = load_scenarios()
scenarios = scenario_doc.get("scenarios", [])

print("[INFO] external-init conformance runner scaffold v128 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")
print(f"[INFO] tool files scanned: {len(tool_files)}")
print(f"[INFO] scenario source: {scenario_doc.get('loaded_from', '<inline>')}")
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

if len(scenarios) < 8:
    fail(f"runner scenario set too small: {len(scenarios)}")
ok(f"runner scenario registry size: {len(scenarios)}")

contract = scenario_doc.get("runner_contract", {})
prefix = contract.get("scenario_marker_prefix") or "[ucompat-v128]"
result_markers = contract.get("result_markers") or []
if not prefix.startswith("[ucompat-"):
    fail(f"invalid scenario marker prefix: {prefix}")
if not {"PASS", "FAIL", "SKIP"}.issubset(set(result_markers)):
    fail(f"runner result markers incomplete: {result_markers}")
ok("runner marker protocol: prefix + PASS/FAIL/SKIP")

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
        scenario_results[sid] = {
            "group": scenario.get("group"),
            "oracle": scenario.get("oracle"),
            "matched_indicators": found[:16],
            "planned_marker_pass": f"{prefix} {sid} PASS",
            "planned_marker_fail": f"{prefix} {sid} FAIL"
        }
        ok(f"scenario {sid}: " + ", ".join(found[:10]))

if missing:
    for sid, syscalls in missing:
        print(f"[ERROR] scenario {sid} has no source/tool syscall coverage; syscalls={syscalls}")
    sys.exit(1)

# Produce an explicit runner plan artifact for future external init generator patches.
plan = {
    "version": VERSION,
    "mode": "scaffold-no-exec",
    "contract": contract,
    "scenario_results": scenario_results,
    "future_external_init_output_examples": [
        f"{prefix} vfs_open_rw_lseek PASS",
        f"{prefix} mmap_brk_basic PASS",
        f"{prefix} signal_delivery_return SKIP reason=not-yet-implemented"
    ],
    "notes": [
        "v128 intentionally does not replace the single-marker external init smoke path.",
        "Next phases can teach user/build_init_elf.py to emit these scenario result lines one group at a time.",
        "Guard scanning excludes backup/log/target directories."
    ],
}

out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
plan_path = ROOT / "tools/external_init_conformance_runner_plan_v128.json"
plan_path.write_text(json.dumps(plan, indent=2, sort_keys=True))

# Validate external-init builder still has single-marker smoke path.
init_builder = ROOT / "user/build_init_elf.py"
if not init_builder.exists():
    fail("user/build_init_elf.py missing")
init_text = init_builder.read_text(errors="ignore")
needed = ["hello from external init.elf", "syscall write", "init.elf"]
found_needed = [x for x in needed if x in init_text]
if len(found_needed) < 2:
    fail("external-init builder marker readiness insufficient")
ok("external-init single-marker smoke remains available: " + ", ".join(found_needed))

manifest = {
    "version": VERSION,
    "mode": "runner-scaffold-no-exec",
    "scenario_count": len(scenarios),
    "scenario_results": scenario_results,
    "runner_contract": contract,
    "plan_path": str(plan_path),
    "rust_source_count": len(rs_files),
    "tool_file_count": len(tool_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "scenario_source": scenario_doc.get("loaded_from"),
    "plan_sha256": hashlib.sha256(plan_path.read_bytes()).hexdigest(),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
manifest_path = out_dir / "external_init_conformance_runner_scaffold_manifest_v128.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] plan: {plan_path}")
print(f"[INFO] manifest: {manifest_path}")
ok("external-init conformance runner scaffold v128 completed")
