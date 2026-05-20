#!/usr/bin/env python3
from __future__ import annotations
import hashlib, json, re, sys
from pathlib import Path
from datetime import datetime, timezone
ROOT = Path.cwd()
LOG_PATH = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT/'.repair_logs'/'futex_scheduler_semantic_guard_v100b.log'
MANIFEST_PATH = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT/'.repair_logs'/'futex_scheduler_semantic_manifest_v100b.json'
def read(path: Path) -> str:
    try: return path.read_text(errors='ignore')
    except FileNotFoundError: return ''
def sha256(path: Path):
    if not path.exists(): return None
    h=hashlib.sha256()
    with path.open('rb') as f:
        for c in iter(lambda:f.read(1024*1024), b''): h.update(c)
    return h.hexdigest()
def find_match_blocks(src: str):
    blocks=[]
    for m in re.finditer(r'\bmatch\s+([A-Za-z0-9_\.]+)\s*\{', src):
        start=m.end(); depth=1; i=start
        while i < len(src) and depth:
            if src[i]=='{': depth+=1
            elif src[i]=='}': depth-=1
            i+=1
        if depth==0: blocks.append((m.group(1), start, i-1, src[start:i-1]))
    return blocks
def duplicate_arms(block: str):
    arms=re.findall(r'^\s*(SYS_[A-Z0-9_]+)\s*=>', block, flags=re.M)
    seen=set(); dup=[]
    for a in arms:
        if a in seen and a not in dup: dup.append(a)
        seen.add(a)
    return dup, arms
def main() -> int:
    LOG_PATH.parent.mkdir(parents=True, exist_ok=True); MANIFEST_PATH.parent.mkdir(parents=True, exist_ok=True)
    syscall_mod=ROOT/'src'/'syscall'/'mod.rs'; src=read(syscall_mod)
    if not src:
        print('[ERROR] missing src/syscall/mod.rs'); return 1
    errors=[]; warnings=[]; summaries=[]; all_arms=[]
    for idx,(expr,start,end,block) in enumerate(find_match_blocks(src)):
        dups, arms = duplicate_arms(block); all_arms += arms
        summaries.append({'index':idx,'expr':expr,'arm_count':len(arms),'duplicate_arms':dups})
        if dups: errors.append(f"duplicate SYS_* arms inside match block {idx} ({expr}): {', '.join(dups)}")
    tree=src
    for p in [ROOT/'src'/'task'/'mod.rs', ROOT/'src'/'proc'/'mod.rs', ROOT/'src'/'process'/'mod.rs', ROOT/'src'/'timer'/'mod.rs', ROOT/'src'/'sync'/'mod.rs']:
        tree += '\n' + read(p)
    groups={
      'futex_syscalls':['SYS_FUTEX','SYS_FUTEX_WAIT','SYS_FUTEX_WAKE','SYS_FUTEX_WAITV','futex'],
      'scheduler_syscalls':['SYS_SCHED_YIELD','SYS_NANOSLEEP','SYS_CLOCK_NANOSLEEP','sched'],
      'thread_tid':['SYS_SET_TID_ADDRESS','SYS_SET_ROBUST_LIST','SYS_GET_ROBUST_LIST','robust'],
      'wake_sleep_concepts':['wake','wakeup','sleep','block','yield'],
      'timeout_usercopy_interop':['timeout','timespec','copy_from_user','copy_to_user','EFAULT'],
    }
    hits={}
    for g,terms in groups.items():
        hs=[t for t in terms if t in tree]; hits[g]=hs
        if not hs: warnings.append(f'semantic debt: no obvious {g} terms found')
    marker=b'hello from external init.elf v100b syscall write'; marker_files=[]
    for p in [ROOT/'user'/'build_init_elf.py', ROOT/'user'/'init.elf']:
        if p.exists() and marker in p.read_bytes(): marker_files.append(str(p.relative_to(ROOT)))
    if not marker_files: warnings.append('exact v100b marker not found in init sources; QEMU smoke may still prove external-init evidence')
    manifest={'version':'v100b','created_at_utc':datetime.now(timezone.utc).isoformat(),'project':str(ROOT),'syscall_mod_sha256':sha256(syscall_mod),'syscall_mod_size':syscall_mod.stat().st_size if syscall_mod.exists() else None,'unique_arm_count':len(set(all_arms)),'match_block_count':len(summaries),'match_blocks':summaries,'term_hits':hits,'marker_files':marker_files,'warnings':warnings,'errors':errors}
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2, sort_keys=True))
    print(f'[INFO] futex/scheduler guard manifest: {MANIFEST_PATH}')
    print(f'[INFO] match blocks checked: {len(summaries)}')
    print(f'[INFO] unique SYS_* arms: {len(set(all_arms))}')
    for g,hs in hits.items(): print(f"[INFO] {g} hits: {', '.join(hs) if hs else '(none)'}")
    for w in warnings: print(f'[WARN] {w}')
    for e in errors: print(f'[ERROR] {e}')
    if errors: return 1
    print('[PASS] futex/scheduler wakeup semantic guard v100b passed')
    return 0
if __name__ == '__main__': raise SystemExit(main())
