#!/usr/bin/env python3
from pathlib import Path
import json, re, sys
ROOT=Path.cwd()
s=(ROOT/"src/mm/sv39_init_exec.rs").read_text(errors="ignore")
print("[INFO] v137c source guard started")
for token in ["UCOMPAT_V137C_OPENAT_OCREAT_RUNTIME_REGULAR_FILE","UCOMPAT_V137C_REG_FD","ucompat_v137c_regular_reset","sys_openat_user","sys_write_user","sys_read_user","sys_lseek_user","sys_close_user"]:
    if token not in s:
        print(f"[ERROR] missing v137c source token: {token}")
        sys.exit(1)
    print(f"[PASS] source token present: {token}")
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
out=ROOT/".repair_logs/openat_ocreat_runtime_regular_file_guard_manifest_v137c.json"
out.write_text(json.dumps({"version":"v137c","status":"ok"},indent=2))
print(f"[INFO] manifest: {out}")
print("[PASS] v137c source guard completed")
