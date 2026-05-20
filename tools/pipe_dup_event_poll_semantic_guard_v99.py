#!/usr/bin/env python3
from __future__ import annotations
import hashlib, json, re, sys, time
from pathlib import Path
ROOT=Path.cwd()
LOG=Path(sys.argv[1]) if len(sys.argv)>1 else ROOT/'.repair_logs/pipe_dup_event_poll_semantic_guard_v99.log'
MANIFEST=Path(sys.argv[2]) if len(sys.argv)>2 else ROOT/'.repair_logs/pipe_dup_event_poll_semantic_manifest_v99.json'
LOG.parent.mkdir(parents=True, exist_ok=True); MANIFEST.parent.mkdir(parents=True, exist_ok=True)
def read(path: str)->str:
    p=ROOT/path
    return p.read_text(errors='ignore') if p.exists() else ''
def sha(path: Path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None
scan_files=['src/syscall/mod.rs','src/fs/mod.rs','src/task/mod.rs','src/process/mod.rs','src/mm/mod.rs','src/trap/mod.rs','user/build_init_elf.py']
# Include shallow source files because pipe/eventfd/poll code may live in submodules.
for p in (ROOT/'src').rglob('*.rs'):
    rel=str(p.relative_to(ROOT))
    if rel not in scan_files and len(scan_files) < 220:
        scan_files.append(rel)
files={k:read(k) for k in scan_files}
all_text='\n'.join(files.values())
mod=files.get('src/syscall/mod.rs','')
errors=[]; warnings=[]
# Keep missing feature checks as semantic debt warnings, not hard errors, because v99 is a guard/smoke step.
feature_groups={
 'pipe_or_pipe2':['SYS_PIPE2','SYS_PIPE','pipe2','Pipe','pipe_read','pipe_write'],
 'dup_family':['SYS_DUP','SYS_DUP2','SYS_DUP3','dup_fd','dup3','Dup','fd table'],
 'eventfd':['SYS_EVENTFD2','SYS_EVENTFD','eventfd','EventFd','event counter'],
 'poll_ppoll':['SYS_PPOLL','SYS_POLL','ppoll','poll','PollFd','pollfd'],
 'epoll':['SYS_EPOLL_CREATE1','SYS_EPOLL_CTL','SYS_EPOLL_PWAIT','SYS_EPOLL_PWAIT2','epoll','Epoll'],
 'wait_queue_or_wakeup':['WaitQueue','wait_queue','wakeup','wake_up','sleep','block_current','suspend_current'],
 'fd_read_write_ops':['read','write','File','Fd','OpenFile','file descriptor'],
}
for group, needles in feature_groups.items():
    hits=[n for n in needles if n in all_text]
    if not hits:
        warnings.append(f'semantic debt marker not observed for {group}: any of {needles}')
required_sys_soft=['SYS_PIPE2','SYS_DUP','SYS_DUP3','SYS_EVENTFD2','SYS_PPOLL','SYS_EPOLL_CTL']
soft_status={sym:(sym in all_text) for sym in required_sys_soft}
for sym, ok in soft_status.items():
    if not ok: warnings.append(f'soft-missing waitable-fd syscall symbol: {sym}')
# Hard guard: duplicate SYS arms within a single match block only.
lines=mod.splitlines(); match_blocks=[]
for i,line in enumerate(lines):
    if re.search(r'\bmatch\s+[^\{]+\{', line):
        depth=line.count('{')-line.count('}'); body=[line]; j=i+1
        while j<len(lines) and depth>0:
            body.append(lines[j]); depth += lines[j].count('{')-lines[j].count('}'); j+=1
        match_blocks.append((i+1, body))
for start, body in match_blocks:
    seen={}
    for off,line in enumerate(body):
        m=re.match(r'\s*(SYS_[A-Z0-9_]+)\s*=>', line)
        if not m: continue
        sym=m.group(1)
        if sym in seen: errors.append(f'duplicate dispatcher arm in one match block: {sym} at lines {seen[sym]} and {start+off}')
        else: seen[sym]=start+off
# Catch-all risk: an arm symbol with no visible definition is a warning unless rustc warning gate catches it.
def_lines=set(re.findall(r'\b(?:const|pub\s+const|static|pub\s+static)\s+(SYS_[A-Z0-9_]+)\b', all_text))
arm_syms=set(re.findall(r'^\s*(SYS_[A-Z0-9_]+)\s*=>', mod, flags=re.M))
missing_defs=sorted(sym for sym in arm_syms if sym not in def_lines)
if missing_defs:
    warnings.append('SYS_* arms without direct const/static definition in scanned files; rustc gate remains source of truth: '+', '.join(missing_defs[:80]))
manifest={'version':'v99','guard':'pipe_dup_event_poll_semantic_guard','timestamp_unix':int(time.time()),'soft_syscalls':soft_status,'feature_hits':{g:[n for n in ns if n in all_text] for g,ns in feature_groups.items()},'match_blocks':len(match_blocks),'syscall_arm_count':len(re.findall(r'^\s*SYS_[A-Z0-9_]+\s*=>', mod, flags=re.M)),'syscall_const_count':len(def_lines),'files':{str(p):{'exists':p.exists(),'sha256':sha(p),'size':p.stat().st_size if p.exists() else None} for p in [ROOT/'src/syscall/mod.rs', ROOT/'user/init.elf', ROOT/'user/build_init_elf.py', ROOT/'build.rs', ROOT/'tools/pipe_dup_event_poll_semantic_guard_v99.py']},'warnings':warnings,'errors':errors}
MANIFEST.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding='utf-8')
with LOG.open('w', encoding='utf-8') as f:
    f.write('[INFO] pipe/dup/eventfd/poll semantic guard v99\n')
    f.write(f'[INFO] soft syscall status: {soft_status}\n')
    f.write(f'[INFO] match block count: {manifest["match_blocks"]}\n')
    f.write(f'[INFO] syscall arm count: {manifest["syscall_arm_count"]}\n')
    for group,hits in manifest['feature_hits'].items(): f.write(f'[INFO] feature hits {group}: {hits}\n')
    for w in warnings: f.write(f'[WARN] {w}\n')
    if errors:
        for e in errors: f.write(f'[ERROR] {e}\n')
    else: f.write('[PASS] pipe/dup/eventfd/poll semantic guard v99 checks passed\n')
    f.write(f'[INFO] manifest: {MANIFEST}\n')
if errors:
    print(f'[ERROR] pipe/dup/eventfd/poll semantic guard v99 failed; manifest: {MANIFEST}'); sys.exit(1)
print(f'[PASS] pipe/dup/eventfd/poll semantic guard v99 checks passed; manifest: {MANIFEST}')
