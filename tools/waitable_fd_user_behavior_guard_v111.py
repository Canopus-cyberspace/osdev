#!/usr/bin/env python3
from __future__ import annotations
from pathlib import Path
import hashlib, json, re, sys, time

ROOT = Path.cwd()
manifest_path = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / '.repair_logs' / f'waitable_fd_user_behavior_manifest_v111_{int(time.time())}.json'
mod = ROOT / 'src' / 'syscall' / 'mod.rs'
all_rs = list((ROOT / 'src').rglob('*.rs')) if (ROOT / 'src').exists() else []
texts = {}
for p in all_rs:
    try:
        texts[str(p.relative_to(ROOT))] = p.read_text(errors='ignore')
    except Exception:
        pass
combined = '\n'.join(texts.values())

required_terms = {
    'pipe': r'(?i)\bpipe2?\b|SYS_PIPE2?',
    'eventfd': r'(?i)eventfd|SYS_EVENTFD2?',
    'timerfd': r'(?i)timerfd|SYS_TIMERFD_(CREATE|SETTIME|GETTIME)',
    'poll': r'(?i)\bpoll\b|ppoll|SYS_POLL|SYS_PPOLL',
    'epoll': r'(?i)epoll|SYS_EPOLL_(CREATE1|CTL|PWAIT|PWAIT2)',
    'readiness': r'(?i)waitable|readiness|ready|pollable|wakeup|wake_up|wait_queue|WaitQueue|block_current|sleep',
    'timespec_timeout': r'(?i)timespec|timeout|nanosleep|clock_gettime|itimerspec',
    'fd_file_ops': r'(?i)FileOps|file_ops|fd_table|FdTable|readable|writable|read\(|write\(',
    'usercopy_interop': r'(?i)copy_from_user|copy_to_user|user.*copy|copy.*user|EFAULT',
}
term_hits = {k: bool(re.search(v, combined)) for k, v in required_terms.items()}

def match_blocks_with_duplicate_sys_arms(text: str):
    results = []
    idx = 0
    while True:
        m = re.search(r'\bmatch\b[^\{]*\{', text[idx:])
        if not m:
            break
        start = idx + m.start()
        brace = idx + m.end() - 1
        depth = 0
        end = brace
        for j in range(brace, len(text)):
            if text[j] == '{':
                depth += 1
            elif text[j] == '}':
                depth -= 1
                if depth == 0:
                    end = j + 1
                    break
        block = text[start:end]
        arms = re.findall(r'(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>', block)
        seen, dup = set(), []
        for a in arms:
            if a in seen and a not in dup:
                dup.append(a)
            seen.add(a)
        if dup:
            line = text[:start].count('\n') + 1
            results.append({'line': line, 'duplicates': dup})
        idx = max(end, brace + 1)
    return results

duplicates = []
if mod.exists():
    duplicates = match_blocks_with_duplicate_sys_arms(mod.read_text(errors='ignore'))

files = {}
for rel in ['src/syscall/mod.rs', 'tools/waitable_fd_user_behavior_guard_v111.py', 'tools/run_waitable_fd_user_behavior_smoke_v111.sh', 'user/init.elf', 'user/build_init_elf.py']:
    p = ROOT / rel
    if p.exists():
        b = p.read_bytes()
        files[rel] = {'size': len(b), 'sha256': hashlib.sha256(b).hexdigest()}

manifest = {
    'version': 'v111',
    'purpose': 'waitable FD pipe/eventfd/timerfd/poll/epoll user behavior regression guard',
    'generated_at_unix': int(time.time()),
    'term_hits': term_hits,
    'duplicate_sys_arms_by_match_block': duplicates,
    'files': files,
    'notes': [
        'Blocks duplicate SYS_* arms inside a single match block.',
        'Records waitable-FD/timer/event readiness markers for true user-mode conformance tests.'
    ],
}
manifest_path.parent.mkdir(parents=True, exist_ok=True)
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))

print(f"[INFO] v110 waitable FD user behavior manifest: {manifest_path}")
print('[INFO] term hits:')
for k, v in sorted(term_hits.items()):
    print(f"  - {k}: {'yes' if v else 'no'}")
if duplicates:
    print('[ERROR] duplicate SYS_* arms inside individual match blocks:')
    for item in duplicates:
        print(f"  - match block near line {item['line']}: {', '.join(item['duplicates'])}")
    sys.exit(1)

critical = ['pipe', 'eventfd', 'poll', 'epoll']
missing_critical = [k for k in critical if not term_hits.get(k)]
if missing_critical:
    print('[ERROR] missing critical waitable FD behavior markers: ' + ', '.join(missing_critical))
    sys.exit(1)

print('[PASS] waitable FD user behavior guard v111 passed')
