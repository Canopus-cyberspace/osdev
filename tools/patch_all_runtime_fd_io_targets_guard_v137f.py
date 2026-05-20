#!/usr/bin/env python3
from pathlib import Path
import json, re, sys
ROOT=Path.cwd()
report=Path(sys.argv[1]) if len(sys.argv)>1 else ROOT/".repair_logs/patch_all_runtime_fd_io_targets_report_v137f.json"
s=(ROOT/"src/mm/sv39_init_exec.rs").read_text(errors="ignore")
print("[INFO] v137f source guard started")
for token in ["UCOMPAT_V137F_OPENAT_OCREAT_RUNTIME_REGULAR_FILE","UCOMPAT_V137F_REG_FD","ucompat_v137f_regular_reset","sys_openat_user"]:
    if token not in s:
        print(f"[ERROR] missing v137f source token: {token}")
        sys.exit(1)
    print(f"[PASS] source token present: {token}")
data=json.loads(report.read_text()) if report.exists() else {}
patched=data.get("patched",{})
for k in ["write","read","seek"]:
    if not patched.get(k):
        print(f"[ERROR] patch report has no {k} target")
        sys.exit(1)
    print(f"[PASS] patched {k} targets: {', '.join(patched[k])}")
mod=(ROOT/"src/syscall/mod.rs").read_text(errors="ignore")
for idx,m in enumerate(re.finditer(r"match\s+[^{}]+\{",mod),1):
    start=m.end(); depth=1; i=start
    while i<len(mod) and depth:
        if mod[i]=="{": depth+=1
        elif mod[i]=="}": depth-=1
        i+=1
    block=mod[start:i-1]
    arms=re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>",block,flags=re.M)
    dup=sorted({a for a in arms if arms.count(a)>1})
    if dup:
        print(f"[ERROR] duplicate SYS_* arms in match block {idx}: {', '.join(dup)}")
        sys.exit(1)
print("[PASS] no duplicate SYS_* arms within individual match blocks")
out=ROOT/".repair_logs/patch_all_runtime_fd_io_targets_guard_manifest_v137f.json"
out.write_text(json.dumps({"version":"v137f","status":"ok","patched":patched},indent=2))
print(f"[INFO] manifest: {out}")
print("[PASS] v137f source guard completed")
