#!/usr/bin/env python3
from __future__ import annotations
import hashlib, json, re, sys, time
from pathlib import Path

PROJECT = Path.cwd()
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else PROJECT / '.repair_logs' / 'socket_loopback_semantic_manifest_v103.json'
SRC = PROJECT / 'src'
SYSCALL_MOD = PROJECT / 'src' / 'syscall' / 'mod.rs'
TEXTS = []
for p in SRC.rglob('*.rs') if SRC.exists() else []:
    try:
        TEXTS.append((p, p.read_text(errors='ignore')))
    except Exception:
        pass
ALL = '\n'.join(t for _, t in TEXTS)

def sha256_path(p: Path) -> str | None:
    if not p.exists():
        return None
    h = hashlib.sha256()
    h.update(p.read_bytes())
    return h.hexdigest()

def count(pattern: str, text: str = ALL) -> int:
    return len(re.findall(pattern, text))

def extract_match_blocks(text: str):
    blocks = []
    idx = 0
    while True:
        m = re.search(r'\bmatch\s+[^\{]+\{', text[idx:])
        if not m:
            break
        start = idx + m.start()
        brace = idx + m.end() - 1
        depth = 0
        end = None
        for i in range(brace, len(text)):
            ch = text[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end is None:
            break
        header = text[start:brace+1]
        body = text[brace+1:end-1]
        line = text[:start].count('\n') + 1
        blocks.append({'line': line, 'header': header[:120], 'body': body})
        idx = end
    return blocks

mod_text = SYSCALL_MOD.read_text(errors='ignore') if SYSCALL_MOD.exists() else ''
blocks = extract_match_blocks(mod_text)
duplicate_blocks = []
for b in blocks:
    arms = re.findall(r'\b(SYS_[A-Z0-9_]+)\b\s*=>', b['body'])
    seen = {}
    dups = []
    for a in arms:
        seen[a] = seen.get(a, 0) + 1
    for a, n in sorted(seen.items()):
        if n > 1:
            dups.append({'symbol': a, 'count': n})
    if dups:
        duplicate_blocks.append({'line': b['line'], 'header': b['header'], 'duplicates': dups})

# Socket/loopback semantic surface. Hard-fail only on structural hazards; report semantic debt as manifest.
expected_terms = {
    'SYS_SOCKETPAIR': r'\bSYS_SOCKETPAIR\b|\bsocketpair\b',
    'SYS_SENDMSG': r'\bSYS_SENDMSG\b|\bsendmsg\b',
    'SYS_RECVMSG': r'\bSYS_RECVMSG\b|\brecvmsg\b',
    'SYS_SENDTO': r'\bSYS_SENDTO\b|\bsendto\b',
    'SYS_RECVFROM': r'\bSYS_RECVFROM\b|\brecvfrom\b',
    'SYS_SOCKET': r'\bSYS_SOCKET\b|\bsys_socket\b',
    'SYS_BIND_CONNECT': r'\bSYS_BIND\b|\bSYS_CONNECT\b|\bbind\b|\bconnect\b',
    'POLL_WAKEUP_SURFACE': r'poll|epoll|wait_queue|wakeup|wake_up|block_current|sleep',
    'USER_COPY_SURFACE': r'copy_from_user|copy_to_user|copy.*user|UserBuffer|iovec|msghdr|IoVec|MsgHdr',
    'FD_SURFACE': r'Fd|File|file_table|fd_table|FileDescriptor|FileOps|read_at|write_at',
}
term_counts = {name: count(pattern) for name, pattern in expected_terms.items()}
semantic_debt = []
for name, c in term_counts.items():
    if c == 0:
        semantic_debt.append(f'missing visible surface for {name}')

catch_all_risks = []
for lineno, line in enumerate(mod_text.splitlines(), 1):
    if re.search(r'\bSYS_[A-Z0-9_]+\s*=>\s*SYS_[A-Z0-9_]+\b', line):
        catch_all_risks.append({'line': lineno, 'text': line.strip()})

manifest = {
    'version': 'v103',
    'timestamp': int(time.time()),
    'project': str(PROJECT),
    'syscall_mod_exists': SYSCALL_MOD.exists(),
    'syscall_mod_sha256': sha256_path(SYSCALL_MOD),
    'rust_file_count': len(TEXTS),
    'match_block_count': len(blocks),
    'duplicate_match_blocks': duplicate_blocks,
    'catch_all_risks': catch_all_risks,
    'socket_loopback_term_counts': term_counts,
    'semantic_debt_notes': semantic_debt,
    'hard_failures': [],
}
if not SYSCALL_MOD.exists():
    manifest['hard_failures'].append('src/syscall/mod.rs missing')
if duplicate_blocks:
    manifest['hard_failures'].append('duplicate SYS_* arms inside a single match block')
if catch_all_risks:
    manifest['hard_failures'].append('suspicious SYS_* => SYS_* arm may indicate catch-all risk')

OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(json.dumps(manifest, indent=2, sort_keys=True))
if manifest['hard_failures']:
    print('[ERROR] socket loopback semantic guard v103 hard failures: ' + '; '.join(manifest['hard_failures']))
    sys.exit(1)
print('[PASS] socket loopback semantic guard v103 structural checks passed')
if semantic_debt:
    print('[INFO] semantic debt notes:')
    for item in semantic_debt:
        print(f'  - {item}')
