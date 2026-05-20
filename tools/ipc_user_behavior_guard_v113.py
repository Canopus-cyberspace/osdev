#!/usr/bin/env python3
from pathlib import Path
import json, re, hashlib, sys, time
ROOT = Path(__file__).resolve().parents[1]
LOG = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT/'.repair_logs'/'ipc_user_behavior_guard_v113.log'
MANIFEST = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT/'.repair_logs'/'ipc_user_behavior_manifest_v113.json'
LOG.parent.mkdir(parents=True, exist_ok=True)

def read(path):
    try:
        return path.read_text(encoding='utf-8', errors='ignore')
    except Exception:
        return ''

rs_files = list((ROOT/'src').rglob('*.rs')) if (ROOT/'src').exists() else []
all_text = '\n'.join(read(p) for p in rs_files)
mod = ROOT/'src'/'syscall'/'mod.rs'
mod_text = read(mod)

lines, errors, warnings = [], [], []
def log(s): lines.append(s)

log('[INFO] IPC SysV/POSIX user behavior guard v113 started')
log(f'[INFO] rust source files scanned: {len(rs_files)}')

def find_match_blocks(text):
    blocks = []
    for m in re.finditer(r'\bmatch\s+[^\{]+\{', text):
        start = m.end() - 1
        depth = 0
        end = None
        for i in range(start, len(text)):
            ch = text[i]
            if ch == '{': depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end: blocks.append((m.start(), end, text[start:end]))
    return blocks

for idx, (_, _, block) in enumerate(find_match_blocks(mod_text), 1):
    arms = re.findall(r'\b(SYS_[A-Z0-9_]+)\b\s*=>', block)
    seen, dup = set(), []
    for arm in arms:
        if arm in seen and arm not in dup:
            dup.append(arm)
        seen.add(arm)
    if dup:
        errors.append(f'duplicate SYS_* dispatcher arms inside match block #{idx}: {", ".join(sorted(dup))}')
if errors:
    for e in errors: log('[ERROR] ' + e)
else:
    log('[PASS] no duplicate SYS_* arms within individual match blocks')

groups = {
    'SysV message queue syscall coverage': ['SYS_MSGGET', 'SYS_MSGCTL', 'SYS_MSGRCV', 'SYS_MSGSND', 'msgget', 'msgctl', 'msgrcv', 'msgsnd'],
    'SysV shared-memory syscall coverage': ['SYS_SHMGET', 'SYS_SHMCTL', 'SYS_SHMAT', 'SYS_SHMDT', 'shmget', 'shmctl', 'shmat', 'shmdt'],
    'SysV semaphore syscall coverage': ['SYS_SEMGET', 'SYS_SEMCTL', 'SYS_SEMTIMEDOP', 'SYS_SEMOP', 'semget', 'semctl', 'semtimedop', 'semop'],
    'POSIX mqueue syscall coverage': ['SYS_MQ_OPEN', 'SYS_MQ_UNLINK', 'SYS_MQ_TIMEDSEND', 'SYS_MQ_TIMEDRECEIVE', 'SYS_MQ_NOTIFY', 'SYS_MQ_GETSETATTR', 'mq_open', 'mq_unlink'],
    'IPC object/id vocabulary': ['ipc', 'Ipc', 'msg_queue', 'shm', 'semaphore', 'id_alloc', 'key_t', 'qid', 'shmid', 'semid'],
    'permission/lifecycle vocabulary': ['permission', 'mode', 'uid', 'gid', 'creator', 'destroy', 'unlink', 'remove', 'IPC_RMID'],
    'blocking/wakeup vocabulary': ['wait_queue', 'WaitQueue', 'wake', 'wakeup', 'block', 'sleep', 'timeout'],
    'user memory structure interop': ['copy_from_user', 'copy_to_user', 'EFAULT', 'timespec', 'iovec', 'msghdr'],
}
coverage = {}
for name, toks in groups.items():
    hits = [t for t in toks if t in all_text]
    coverage[name] = hits
    if hits:
        log(f'[PASS] {name}: ' + ', '.join(hits[:10]))
    else:
        warnings.append(f'missing/debt: {name}')
        log(f'[WARN] {name}: no obvious token found; recorded as semantic debt')

ipc_any = any(coverage[k] for k in [
    'SysV message queue syscall coverage',
    'SysV shared-memory syscall coverage',
    'SysV semaphore syscall coverage',
    'POSIX mqueue syscall coverage',
])
if not ipc_any:
    errors.append('no SysV/POSIX IPC syscall vocabulary found after v82 baseline')

sha = hashlib.sha256(mod_text.encode()).hexdigest() if mod_text else None
manifest = {
    'version': 'v113',
    'kind': 'ipc_sysv_posix_user_behavior_guard',
    'timestamp': int(time.time()),
    'src_syscall_mod_sha256': sha,
    'rust_files_scanned': len(rs_files),
    'coverage': coverage,
    'warnings': warnings,
    'errors': errors,
}
MANIFEST.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding='utf-8')
for w in warnings:
    log('[INFO] semantic debt: ' + w)
log(f'[INFO] manifest: {MANIFEST}')
LOG.write_text('\n'.join(lines) + '\n', encoding='utf-8')
print('\n'.join(lines))
if errors:
    sys.exit(1)
