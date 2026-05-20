#!/usr/bin/env python3
from __future__ import annotations
from pathlib import Path
import re
import sys

root = Path.cwd()
mod = root / 'src' / 'syscall' / 'mod.rs'
log = Path(sys.argv[1]) if len(sys.argv) > 1 else root / '.repair_logs' / 'syscall_external_init_guard_v89.log'
marker = 'hello from external init.elf v89 syscall write'
required_symbols = [
    'SYS_PIDFD_OPEN', 'SYS_IO_URING_SETUP', 'SYS_IO_URING_ENTER', 'SYS_IO_URING_REGISTER',
    'SYS_OPENAT2', 'SYS_CLOSE_RANGE', 'SYS_LANDLOCK_CREATE_RULESET', 'SYS_LANDLOCK_ADD_RULE',
    'SYS_LANDLOCK_RESTRICT_SELF', 'SYS_FUTEX_WAITV', 'SYS_RSEQ', 'SYS_MEMBARRIER', 'SYS_STATX',
    'SYS_COPY_FILE_RANGE', 'SYS_PKEY_MPROTECT', 'SYS_FSPICK', 'SYS_PIDFD_GETFD', 'SYS_MEMFD_SECRET',
    'SYS_FUTEX_WAKE', 'SYS_FUTEX_WAIT', 'SYS_FUTEX_REQUEUE', 'SYS_STATMOUNT', 'SYS_LISTMOUNT',
    'SYS_MSEAL', 'SYS_SETXATTRAT', 'SYS_GETXATTRAT', 'SYS_LISTXATTRAT', 'SYS_REMOVEXATTRAT',
    'SYS_LISTNS', 'SYS_RSEQ_SLICE_YIELD'
]

out: list[str] = []
def emit(s: str) -> None:
    print(s)
    out.append(s)

if not mod.exists():
    emit(f'[ERROR] missing {mod}')
    log.write_text('\n'.join(out) + '\n')
    sys.exit(1)
text = mod.read_text(errors='ignore')

all_src = '\n'.join(p.read_text(errors='ignore') for p in (root/'src').rglob('*.rs'))
missing = [s for s in required_symbols if s not in all_src]
if missing:
    emit('[ERROR] required modern generic syscall symbols missing: ' + ', '.join(missing))
    log.write_text('\n'.join(out) + '\n')
    sys.exit(1)
emit(f'[PASS] v83-v86 modern generic syscall symbols present: {len(required_symbols)}')

def match_blocks(src: str):
    idx = 0
    while True:
        m = re.search(r'\bmatch\s+[^\{]+\{', src[idx:])
        if not m:
            return
        start = idx + m.start()
        brace = idx + m.end() - 1
        depth = 0
        end = None
        for i in range(brace, len(src)):
            c = src[i]
            if c == '{':
                depth += 1
            elif c == '}':
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end is None:
            return
        yield start, end, src[start:end]
        idx = end

dups = []
for n, (start, end, block) in enumerate(match_blocks(text), 1):
    arms = re.findall(r'(^|[\n\s])(?P<sym>SYS_[A-Z0-9_]+)\s*=>', block)
    seen: dict[str, int] = {}
    for _, sym in arms:
        seen[sym] = seen.get(sym, 0) + 1
    repeated = sorted(k for k, v in seen.items() if v > 1)
    if repeated:
        dups.append((n, start, repeated))
if dups:
    for n, start, repeated in dups:
        emit(f'[ERROR] duplicate dispatcher arms in match block #{n} near byte {start}: ' + ', '.join(repeated))
    log.write_text('\n'.join(out) + '\n')
    sys.exit(1)
emit('[PASS] no duplicate SYS_* dispatcher arms detected inside match blocks')

if re.search(r'\bSYS_[A-Z0-9_]+\s*=>\s*SYS_[A-Z0-9_]+\b', text):
    emit('[ERROR] suspicious SYS_* => SYS_* arm found')
    log.write_text('\n'.join(out) + '\n')
    sys.exit(1)
emit('[PASS] no suspicious SYS_* catch-all style arm found')

init_gen = root / 'user' / 'build_init_elf.py'
init_elf = root / 'user' / 'init.elf'
if init_gen.exists() and marker in init_gen.read_text(errors='ignore'):
    emit('[PASS] exact v89 marker present in user/build_init_elf.py')
else:
    emit('[WARN] exact v89 marker not visible in user/build_init_elf.py')
if init_elf.exists() and marker.encode() in init_elf.read_bytes():
    emit('[PASS] exact v89 marker present in user/init.elf')
else:
    emit('[WARN] exact v89 marker not visible in user/init.elf; QEMU smoke will require compatible marker or exact marker elsewhere')

emit('[PASS] syscall external-init consistency guard v89 checks completed')
log.write_text('\n'.join(out) + '\n')
