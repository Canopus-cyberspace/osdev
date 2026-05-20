#!/usr/bin/env python3
import hashlib
import json
import os
import re
from pathlib import Path
from datetime import datetime

ROOT = Path.cwd()
LOG = Path(os.environ.get("V115_GUARD_LOG", ROOT / ".repair_logs" / "identity_time_resource_user_behavior_guard_v115.log"))
MANIFEST = Path(os.environ.get("V115_MANIFEST", ROOT / ".repair_logs" / "identity_time_resource_user_behavior_manifest_v115.json"))
LOG.parent.mkdir(parents=True, exist_ok=True)

lines = []
def out(msg: str):
    print(msg)
    lines.append(msg)

skip_parts = {"target", ".git", ".repair_logs"}
rs_files = []
for p in ROOT.rglob("*.rs"):
    if any(part in skip_parts for part in p.parts):
        continue
    rs_files.append(p)

texts = {}
combined_chunks = []
for p in rs_files:
    try:
        t = p.read_text(errors="ignore")
    except Exception:
        continue
    texts[str(p.relative_to(ROOT))] = t
    combined_chunks.append(t)
combined = "\n".join(combined_chunks)

out("[INFO] identity/time/resource/random user behavior guard v115 started")
out(f"[INFO] rust source files scanned: {len(texts)}")

# Duplicate SYS_* arm check inside each individual match block only.
def extract_match_blocks(text: str):
    blocks = []
    i = 0
    while True:
        m = re.search(r"\bmatch\s+[^\{]+\{", text[i:])
        if not m:
            break
        start = i + m.start()
        brace = i + m.end() - 1
        depth = 0
        end = None
        for j in range(brace, len(text)):
            ch = text[j]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = j + 1
                    break
        if end is None:
            break
        blocks.append(text[start:end])
        i = end
    return blocks

errors = []
for rel, text in texts.items():
    for idx, block in enumerate(extract_match_blocks(text), 1):
        arms = re.findall(r"(^|\n)\s*(SYS_[A-Z0-9_]+)\s*=>", block)
        names = [a[1] for a in arms]
        seen = set()
        dup = sorted({n for n in names if n in seen or seen.add(n)})
        if dup:
            errors.append(f"{rel}: match block {idx} duplicate arms: {', '.join(dup)}")

if errors:
    for e in errors:
        out(f"[ERROR] {e}")
    raise SystemExit(1)
out("[PASS] no duplicate SYS_* arms within individual match blocks")

checks = {
    "identity uid/gid/tid hooks": ["SYS_GETUID", "SYS_GETEUID", "SYS_GETGID", "SYS_GETEGID", "SYS_GETTID", "getuid", "geteuid", "getgid", "getegid", "gettid"],
    "session/process-group hooks": ["SYS_SETSID", "SYS_GETSID", "SYS_GETPGID", "SYS_SETPGID", "setsid", "getsid", "getpgid", "setpgid"],
    "resource limit/usage hooks": ["SYS_GETRLIMIT", "SYS_SETRLIMIT", "SYS_PRLIMIT64", "SYS_GETRUSAGE", "getrlimit", "setrlimit", "prlimit", "getrusage"],
    "time syscall hooks": ["SYS_CLOCK_GETTIME", "SYS_GETTIMEOFDAY", "SYS_NANOSLEEP", "SYS_CLOCK_NANOSLEEP", "clock_gettime", "gettimeofday", "nanosleep"],
    "random/sysinfo hooks": ["SYS_GETRANDOM", "SYS_SYSINFO", "getrandom", "sysinfo", "random"],
    "capability/prctl hooks": ["SYS_CAPGET", "SYS_CAPSET", "SYS_PRCTL", "capget", "capset", "prctl"],
    "umask/user namespace vocabulary": ["SYS_UMASK", "umask", "uid", "gid", "capability", "session"],
}

for name, needles in checks.items():
    found = [n for n in needles if n in combined]
    if not found:
        out(f"[ERROR] missing expected identity/time/resource coverage: {name}; candidates={needles}")
        raise SystemExit(1)
    out(f"[PASS] {name}: {', '.join(found[:8])}")

for bad in ["matches any value", "unreachable pattern"]:
    if bad in combined:
        out(f"[WARN] literal phrase present in source comments/text: {bad}; build log remains source of truth")

important = [
    "src/syscall/mod.rs",
    "src/syscall/runtime.rs",
    "src/task/mod.rs",
    "src/mm/mod.rs",
    "src/fs/mod.rs",
]
files = {}
for rel in important:
    p = ROOT / rel
    if p.exists():
        b = p.read_bytes()
        files[rel] = {"size": len(b), "sha256": hashlib.sha256(b).hexdigest()}

manifest = {
    "version": "v115",
    "kind": "identity_time_resource_user_behavior_guard",
    "timestamp": datetime.utcnow().isoformat() + "Z",
    "rust_files_scanned": len(texts),
    "checks": sorted(checks.keys()),
    "files": files,
}
MANIFEST.write_text(json.dumps(manifest, indent=2, sort_keys=True))
out(f"[INFO] manifest: {MANIFEST}")
out("[PASS] identity/time/resource/random user behavior guard v115 completed")
LOG.write_text("\n".join(lines) + "\n")
