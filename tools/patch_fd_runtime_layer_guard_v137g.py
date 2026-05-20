#!/usr/bin/env python3
from pathlib import Path
import json, re, sys
ROOT=Path.cwd()
report=Path(sys.argv[1]) if len(sys.argv)>1 else ROOT/".repair_logs/patch_fd_runtime_layer_report_v137g.json"
data=json.loads(report.read_text()) if report.exists() else {}
mm=(ROOT/"src/mm/sv39_init_exec.rs").read_text(errors="ignore")
fd_file=Path(data.get("fd_file",""))
fd=(fd_file.read_text(errors="ignore") if fd_file.exists() else "")
print("[INFO] v137g source guard started")
for token in ["UCOMPAT_V137G_OPENAT_OCREAT_BRIDGE","UCOMPAT_V137G_REG_FD","sys_openat_user"]:
    if token not in mm:
        print(f"[ERROR] missing v137g mm token: {token}")
        sys.exit(1)
    print(f"[PASS] mm token present: {token}")
for token in ["UCOMPAT_V137G_FD_RUNTIME_LAYER","UCOMPAT_V137G_REG_FD","ucompat_v137g_regular_reset"]:
    if token not in fd:
        print(f"[ERROR] missing v137g fd token: {token}")
        sys.exit(1)
    print(f"[PASS] fd token present: {token}")
patched=data.get("patched",{})
for k in ["write","read"]:
    if not patched.get(k):
        print(f"[ERROR] patch report has no fd-runtime {k} target")
        sys.exit(1)
    print(f"[PASS] fd-runtime {k} targets: {', '.join(patched[k])}")
if patched.get("seek"):
    print(f"[PASS] fd-runtime seek targets: {', '.join(patched['seek'])}")
else:
    print("[INFO] no fd-runtime seek target; syscall-layer seek patch may handle SYS_LSEEK")
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
out=ROOT/".repair_logs/patch_fd_runtime_layer_guard_manifest_v137g.json"
out.write_text(json.dumps({"version":"v137g","status":"ok","patched":patched,"fd_file":str(fd_file)},indent=2))
print(f"[INFO] manifest: {out}")
print("[PASS] v137g source guard completed")
