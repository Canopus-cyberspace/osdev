#!/usr/bin/env python3
import hashlib
import json
import re
import sys
from pathlib import Path
from datetime import datetime, timezone

project = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()
out = Path(sys.argv[2]) if len(sys.argv) > 2 else project / '.repair_logs' / 'identity_time_resource_manifest_v115b.json'
project = project.resolve()

print('[INFO] identity/time/resource/random/session user behavior guard v115b started')
print('[INFO] scan policy: current source tree only; excluding .backup_repair_*, .repair_logs, target, .git')

src = project / 'src'
if not src.exists():
    print('[ERROR] src directory not found')
    sys.exit(1)

rs_files = sorted(src.rglob('*.rs'))
texts = []
for p in rs_files:
    # rglob under src avoids .backup_repair_* by construction.
    try:
        texts.append((p, p.read_text(encoding='utf-8', errors='ignore')))
    except Exception as exc:
        print(f'[WARN] failed to read {p}: {exc}')
combined = '\n'.join(t for _, t in texts)
combined_lower = combined.lower()
print(f'[INFO] rust source files scanned: {len(rs_files)}')

# Duplicate arm check is intentionally scoped to each individual match block.
mod = project / 'src' / 'syscall' / 'mod.rs'
if not mod.exists():
    print('[ERROR] src/syscall/mod.rs not found')
    sys.exit(1)
text = mod.read_text(encoding='utf-8', errors='ignore')

def find_matching_brace(s, open_idx):
    depth = 0
    in_line = False
    in_block = 0
    in_str = False
    esc = False
    i = open_idx
    while i < len(s):
        ch = s[i]
        nxt = s[i:i+2]
        if in_line:
            if ch == '\n':
                in_line = False
            i += 1
            continue
        if in_block:
            if nxt == '/*':
                in_block += 1; i += 2; continue
            if nxt == '*/':
                in_block -= 1; i += 2; continue
            i += 1
            continue
        if in_str:
            if esc:
                esc = False
            elif ch == '\\':
                esc = True
            elif ch == '"':
                in_str = False
            i += 1
            continue
        if nxt == '//':
            in_line = True; i += 2; continue
        if nxt == '/*':
            in_block = 1; i += 2; continue
        if ch == '"':
            in_str = True; i += 1; continue
        if ch == '{':
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1

dups = []
for m in re.finditer(r'\bmatch\b[^\{]*\{', text):
    open_idx = text.find('{', m.start())
    close_idx = find_matching_brace(text, open_idx)
    if close_idx < 0:
        continue
    block = text[open_idx+1:close_idx]
    arms = re.findall(r'^\s*(SYS_[A-Z0-9_]+)\s*=>', block, flags=re.M)
    seen = set()
    repeated = []
    for a in arms:
        if a in seen and a not in repeated:
            repeated.append(a)
        seen.add(a)
    if repeated:
        line = text.count('\n', 0, m.start()) + 1
        dups.append({'line': line, 'duplicates': repeated})

if dups:
    for d in dups:
        print(f"[ERROR] current src/syscall/mod.rs match block at line {d['line']} duplicate arms: {', '.join(d['duplicates'])}")
    sys.exit(1)
print('[PASS] no duplicate SYS_* arms within individual match blocks in current src/syscall/mod.rs')

checks = [
    ('time syscalls', ['SYS_CLOCK_GETTIME', 'clock_gettime', 'SYS_GETTIMEOFDAY', 'gettimeofday', 'SYS_NANOSLEEP', 'nanosleep']),
    ('random/sysinfo syscalls', ['SYS_GETRANDOM', 'getrandom', 'SYS_SYSINFO', 'sysinfo']),
    ('identity syscalls', ['SYS_GETUID', 'getuid', 'SYS_GETEUID', 'geteuid', 'SYS_GETGID', 'getgid', 'SYS_GETTID', 'gettid']),
    ('session/process-group syscalls', ['SYS_SETSID', 'setsid', 'SYS_GETSID', 'getsid', 'SYS_GETPGID', 'getpgid', 'SYS_SETPGID', 'setpgid']),
    ('resource syscalls', ['SYS_GETRLIMIT', 'getrlimit', 'SYS_SETRLIMIT', 'setrlimit', 'SYS_PRLIMIT64', 'prlimit', 'SYS_GETRUSAGE', 'getrusage']),
    ('capability/prctl/umask syscalls', ['SYS_CAPGET', 'capget', 'SYS_CAPSET', 'capset', 'SYS_PRCTL', 'prctl', 'SYS_UMASK', 'umask']),
]
missing = []
for name, toks in checks:
    found = [t for t in toks if t.lower() in combined_lower]
    if found:
        print(f"[PASS] {name}: {', '.join(found[:8])}")
    else:
        print(f"[ERROR] missing expected vocabulary for {name}: any of {', '.join(toks)}")
        missing.append(name)
if missing:
    sys.exit(1)

manifest = {
    'version': 'v115b',
    'created_at': datetime.now(timezone.utc).isoformat(),
    'project': str(project),
    'scan_policy': 'src/**/*.rs only; excludes backup/log/target directories',
    'rust_files_scanned': len(rs_files),
    'src_syscall_mod_sha256': hashlib.sha256(text.encode('utf-8')).hexdigest(),
    'checks': [name for name, _ in checks],
}
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding='utf-8')
print(f'[INFO] manifest: {out}')
print('[PASS] identity/time/resource/random/session user behavior guard v115b completed')
