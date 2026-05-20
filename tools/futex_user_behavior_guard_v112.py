#!/usr/bin/env python3
from pathlib import Path
import json, re, hashlib, sys, time
ROOT = Path(__file__).resolve().parents[1]
LOG = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT/'.repair_logs'/'futex_user_behavior_guard_v112.log'
MANIFEST = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT/'.repair_logs'/'futex_user_behavior_manifest_v112.json'
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

lines = []
errors = []
warnings = []

def log(s):
    lines.append(s)

log('[INFO] futex/scheduler user behavior guard v112 started')
log(f'[INFO] rust source files scanned: {len(rs_files)}')

def find_match_blocks(text):
    blocks = []
    for m in re.finditer(r'\bmatch\s+[^\{]+\{', text):
        start = m.end() - 1
        depth = 0
        end = None
        for i in range(start, len(text)):
            ch = text[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end:
            blocks.append((m.start(), end, text[start:end]))
    return blocks

for idx, (start, end, block) in enumerate(find_match_blocks(mod_text), 1):
    arms = re.findall(r'\b(SYS_[A-Z0-9_]+)\b\s*=>', block)
    seen = set()
    dup = []
    for arm in arms:
        if arm in seen and arm not in dup:
            dup.append(arm)
        seen.add(arm)
    if dup:
        errors.append(f'duplicate SYS_* dispatcher arms inside match block #{idx}: {", ".join(sorted(dup))}')
if errors:
    for e in errors:
        log('[ERROR] ' + e)
else:
    log('[PASS] no duplicate SYS_* arms within individual match blocks')

groups = {
    'futex syscall constants/arms': ['SYS_FUTEX', 'SYS_FUTEX_WAIT', 'SYS_FUTEX_WAKE', 'futex_wait', 'futex_wake'],
    'futex_waitv coverage': ['SYS_FUTEX_WAITV', 'futex_waitv'],
    'tid cleanup hooks': ['SYS_SET_TID_ADDRESS', 'set_tid_address', 'clear_child_tid'],
    'robust futex list': ['SYS_SET_ROBUST_LIST', 'SYS_GET_ROBUST_LIST', 'robust'],
    'scheduler yield/sleep hooks': ['SYS_SCHED_YIELD', 'sched_yield', 'yield_current', 'suspend_current'],
    'timeout/timespec interop': ['timespec', 'TimeSpec', 'nanosleep', 'clock_gettime'],
    'wakeup/blocking vocabulary': ['wake', 'wakeup', 'block', 'sleep', 'wait_queue', 'WaitQueue'],
}
coverage = {}
for name, toks in groups.items():
    hits = [t for t in toks if t in all_text]
    coverage[name] = hits
    if hits:
        log(f'[PASS] {name}: ' + ', '.join(hits[:8]))
    else:
        warnings.append(f'missing/debt: {name}')
        log(f'[WARN] {name}: no obvious token found; recorded as semantic debt')

if not coverage['futex syscall constants/arms']:
    errors.append('no futex syscall/hook vocabulary found after v100b baseline')

sha = hashlib.sha256(mod_text.encode()).hexdigest() if mod_text else None
manifest = {
    'version': 'v112',
    'kind': 'futex_scheduler_user_behavior_guard',
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
