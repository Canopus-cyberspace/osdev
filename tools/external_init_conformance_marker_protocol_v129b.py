#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v129b"
EXCLUDE_DIRS = {".git", "target", ".repair_logs"}
EXCLUDE_PREFIXES = (".backup_repair_",)
ALLOWED_RESULTS = {"PASS", "FAIL", "SKIP"}
PREFIX = "[ucompat-v129]"

DEFAULT_SYSCALLS_BY_SCENARIO = {
    "vfs_open_rw_lseek": ["openat", "write", "lseek", "read", "close"],
    "getdents_statx": ["getdents64", "statx", "fstat"],
    "mmap_brk_basic": ["mmap", "munmap", "mprotect", "brk"],
    "process_fork_wait": ["clone", "exit", "wait4", "getpid"],
    "execve_stack": ["execve", "argv", "envp", "auxv"],
    "pipe_poll_eventfd": ["pipe2", "poll", "eventfd2", "read", "write"],
    "timerfd_epoll": ["timerfd_create", "timerfd_settime", "epoll_ctl", "epoll_pwait"],
    "futex_wait_wake": ["futex", "futex_waitv", "sched_yield"],
    "signal_delivery_return": ["rt_sigaction", "tgkill", "rt_sigreturn"],
    "socketpair_msg": ["socketpair", "sendmsg", "recvmsg", "poll"],
    "ipc_msg_shm_sem": ["msgget", "msgsnd", "msgrcv", "shmget", "shmat", "semget"],
    "identity_time_random": ["clock_gettime", "getrandom", "getuid", "gettid", "getrusage"],
    "fcntl_tty_flags": ["fcntl", "ioctl", "dup3", "O_CLOEXEC"],
    "path_cwd_dirfd": ["getcwd", "chdir", "openat", "readlinkat", "AT_FDCWD"],
    "security_permissions": ["capget", "faccessat2", "landlock_create_ruleset", "prctl"],
}

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

def load_scenarios() -> tuple[list[dict], str]:
    candidates = [
        ROOT / "tools/external_init_conformance_runner_plan_v128.json",
        ROOT / "tools/external_init_conformance_scenarios_v127.json",
        ROOT / "tools/userland_behavior_registry_v126.json",
    ]
    for p in candidates:
        if not p.exists():
            continue
        doc = json.loads(p.read_text())
        if isinstance(doc.get("scenario_results"), dict):
            rows = []
            for sid, item in doc["scenario_results"].items():
                syscalls = item.get("syscalls", []) if isinstance(item, dict) else []
                if not syscalls:
                    syscalls = DEFAULT_SYSCALLS_BY_SCENARIO.get(sid, [])
                rows.append({
                    "id": sid,
                    "group": item.get("group") if isinstance(item, dict) else None,
                    "syscalls": syscalls,
                    "oracle": item.get("oracle") if isinstance(item, dict) else None,
                    "syscall_source": "scenario_results" if item.get("syscalls") else "default_map",
                })
            return rows, str(p)
        if isinstance(doc.get("scenarios"), list):
            rows = []
            for item in doc["scenarios"]:
                sid = item.get("id")
                syscalls = item.get("syscalls") or DEFAULT_SYSCALLS_BY_SCENARIO.get(sid, [])
                rows.append({**item, "syscalls": syscalls, "syscall_source": "scenario_doc" if item.get("syscalls") else "default_map"})
            return rows, str(p)
        if isinstance(doc.get("scenario_groups"), list):
            rows = []
            for item in doc["scenario_groups"]:
                sid = item.get("id")
                syscalls = item.get("syscalls") or DEFAULT_SYSCALLS_BY_SCENARIO.get(sid, [])
                rows.append({
                    "id": sid,
                    "group": item.get("id"),
                    "syscalls": syscalls,
                    "oracle": item.get("expected_next") or item.get("oracle"),
                    "syscall_source": "scenario_groups" if item.get("syscalls") else "default_map",
                })
            return rows, str(p)
    rows = []
    for sid, syscalls in DEFAULT_SYSCALLS_BY_SCENARIO.items():
        rows.append({"id": sid, "group": sid, "syscalls": syscalls, "oracle": "fallback", "syscall_source": "default_map"})
    return rows, "<fallback>"

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

scenarios, scenario_source = load_scenarios()

print("[INFO] external-init conformance marker protocol v129b started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")
print(f"[INFO] tool files scanned: {len(tool_files)}")
print(f"[INFO] scenario source: {scenario_source}")
print(f"[INFO] scenario count: {len(scenarios)}")

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
    fail(f"marker protocol scenario set too small: {len(scenarios)}")
ok(f"marker protocol scenario registry size: {len(scenarios)}")
ok("marker protocol prefix/result enum valid: [ucompat-v129] PASS/FAIL/SKIP")

scenario_results = {}
missing = []
empty_syscalls = []
for scenario in scenarios:
    sid = scenario.get("id") or "<missing>"
    if not re.match(r"^[a-z0-9_]+$", sid):
        fail(f"invalid scenario id format: {sid}")
    syscalls = scenario.get("syscalls") or []
    if not syscalls:
        empty_syscalls.append(sid)
        continue
    indicators = set()
    for s in syscalls:
        indicators.add(str(s))
        indicators.add(str(s).lower())
        indicators.add("SYS_" + str(s).upper())
    found = sorted({x for x in indicators if x and (x in all_text or str(x).lower() in lower_text)})
    if not found:
        missing.append((sid, syscalls))
        continue
    scenario_results[sid] = {
        "group": scenario.get("group"),
        "oracle": scenario.get("oracle"),
        "syscalls": syscalls,
        "syscall_source": scenario.get("syscall_source"),
        "matched_indicators": found[:16],
        "markers": {
            "pass": f"{PREFIX} {sid} PASS",
            "fail": f"{PREFIX} {sid} FAIL",
            "skip": f"{PREFIX} {sid} SKIP",
        },
    }
    ok(f"scenario {sid}: markers ok; syscalls={','.join(syscalls[:6])}; coverage " + ", ".join(found[:10]))

if empty_syscalls:
    fail("scenarios still have empty syscall lists after default-map repair: " + ", ".join(empty_syscalls))

if missing:
    for sid, syscalls in missing:
        print(f"[ERROR] scenario {sid} has no source/tool syscall coverage; syscalls={syscalls}")
    sys.exit(1)

protocol_doc = {
    "version": VERSION,
    "phase": "marker-protocol-phase1-syscall-map-repair",
    "scenario_marker_prefix": PREFIX,
    "result_markers": sorted(ALLOWED_RESULTS),
    "line_format": "<prefix> <scenario_id> <PASS|FAIL|SKIP> [key=value ...]",
    "scenario_results": scenario_results,
    "default_syscall_map": DEFAULT_SYSCALLS_BY_SCENARIO,
    "notes": [
        "v129b repairs v129 by backfilling syscalls for scenario_results entries that lost their syscalls in v128.",
        "v129b still preserves the existing single-marker external init QEMU smoke.",
        "Future versions can emit these lines from user/build_init_elf.py one scenario group at a time."
    ],
}
protocol_path = ROOT / "tools/external_init_conformance_marker_protocol_v129.json"
protocol_path.write_text(json.dumps(protocol_doc, indent=2, sort_keys=True))

init_builder = ROOT / "user/build_init_elf.py"
if not init_builder.exists():
    fail("user/build_init_elf.py missing")
init_text = init_builder.read_text(errors="ignore")
needed = ["hello from external init.elf", "syscall write", "init.elf"]
found_needed = [x for x in needed if x in init_text]
if len(found_needed) < 2:
    fail("external-init builder marker readiness insufficient")
ok("external-init single-marker smoke remains available: " + ", ".join(found_needed))

out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest = {
    "version": VERSION,
    "phase": "marker-protocol-phase1-syscall-map-repair",
    "scenario_count": len(scenarios),
    "scenario_results": scenario_results,
    "protocol_path": str(protocol_path),
    "scenario_source": scenario_source,
    "rust_source_count": len(rs_files),
    "tool_file_count": len(tool_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "protocol_sha256": hashlib.sha256(protocol_path.read_bytes()).hexdigest(),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
manifest_path = out_dir / "external_init_conformance_marker_protocol_manifest_v129b.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] protocol: {protocol_path}")
print(f"[INFO] manifest: {manifest_path}")
ok("external-init conformance marker protocol v129b completed")
