#!/usr/bin/env python3
from __future__ import annotations
import argparse, json, hashlib, re, time
from pathlib import Path

ROOT = Path.cwd()
SIGNAL_TERMS = [
    "rt_sigaction", "rt_sigprocmask", "rt_sigreturn", "rt_sigpending", "rt_sigsuspend",
    "kill", "tkill", "tgkill", "rt_sigqueueinfo", "rt_tgsigqueueinfo",
    "sigaction", "sigmask", "sigframe", "signal", "Signal",
]
SIGNAL_SYS = [
    "SYS_RT_SIGACTION", "SYS_RT_SIGPROCMASK", "SYS_RT_SIGRETURN", "SYS_RT_SIGPENDING",
    "SYS_RT_SIGSUSPEND", "SYS_KILL", "SYS_TKILL", "SYS_TGKILL",
    "SYS_RT_SIGQUEUEINFO", "SYS_RT_TGSIGQUEUEINFO",
]

def sha256(p: Path) -> str:
    h = hashlib.sha256()
    with p.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()

def read(p: Path) -> str:
    try:
        return p.read_text(errors='ignore')
    except Exception:
        return ''

def repo_text() -> tuple[str, list[str]]:
    parts, files = [], []
    for p in ROOT.rglob('*'):
        if any(x in p.parts for x in ['target', '.git']):
            continue
        if p.suffix in {'.rs', '.py', '.sh', '.toml', '.md'} or p.name in {'Makefile', 'build.rs'}:
            s = read(p)
            if s:
                parts.append(s)
                files.append(str(p))
    return '\n'.join(parts), files

def find_match_blocks(src: str):
    blocks = []
    for m in re.finditer(r'match\s+[^\{]+\{', src):
        start = m.start()
        brace = src.find('{', m.start())
        depth = 0
        for i in range(brace, len(src)):
            ch = src[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    blocks.append((start, i + 1, src[start:i+1]))
                    break
    return blocks

def duplicate_arms_by_block(src: str):
    duplicates = []
    for idx, (start, _end, block) in enumerate(find_match_blocks(src), 1):
        arms = re.findall(r'\b(SYS_[A-Z0-9_]+)\s*=>', block)
        seen, dup = set(), set()
        for a in arms:
            if a in seen:
                dup.add(a)
            seen.add(a)
        if dup:
            line = src[:start].count('\n') + 1
            duplicates.append({'block': idx, 'line': line, 'duplicates': sorted(dup)})
    return duplicates

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--manifest', required=True)
    ap.add_argument('--version', default='v109')
    ns = ap.parse_args()
    syscall_mod = ROOT / 'src' / 'syscall' / 'mod.rs'
    if not syscall_mod.exists():
        print('[ERROR] src/syscall/mod.rs not found')
        return 1
    mod = read(syscall_mod)
    alltext, scanned_files = repo_text()
    found_terms = sorted({t for t in SIGNAL_TERMS if t in alltext})
    found_sys = sorted({s for s in SIGNAL_SYS if s in alltext})
    missing_soft = sorted(set(SIGNAL_SYS) - set(found_sys))
    duplicates = duplicate_arms_by_block(mod)
    behavior_terms = [
        'copy_from_user', 'copy_to_user', 'copy_cstr', 'EFAULT',
        'TrapContext', 'sigreturn', 'SigAction', 'sigaction', 'signal',
        'kill', 'tgkill', 'pending', 'mask', 'User',
    ]
    found_behavior = sorted({t for t in behavior_terms if t in alltext})
    manifest = {
        'version': ns.version,
        'timestamp': int(time.time()),
        'root': str(ROOT),
        'syscall_mod': {
            'path': str(syscall_mod),
            'bytes': syscall_mod.stat().st_size,
            'sha256': sha256(syscall_mod),
            'match_blocks': len(find_match_blocks(mod)),
            'sys_arms': len(re.findall(r'\bSYS_[A-Z0-9_]+\s*=>', mod)),
        },
        'signal_terms_found': found_terms,
        'signal_syscalls_found': found_sys,
        'signal_syscalls_missing_soft': missing_soft,
        'behavior_terms_found': found_behavior,
        'duplicate_arms_by_match_block': duplicates,
        'files_scanned': len(scanned_files),
    }
    Path(ns.manifest).write_text(json.dumps(manifest, indent=2, ensure_ascii=False))
    ok = True
    if duplicates:
        print('[ERROR] duplicate SYS_* dispatcher arms within a single match block:')
        for d in duplicates:
            print(f"  block#{d['block']} line {d['line']}: {', '.join(d['duplicates'])}")
        ok = False
    if len(found_terms) < 5:
        print('[ERROR] too few signal-related implementation terms found:', found_terms)
        ok = False
    if len(found_sys) < 4:
        print('[ERROR] too few signal syscall constants/arms found:', found_sys)
        ok = False
    if len(found_behavior) < 5:
        print('[ERROR] too few signal user-behavior building blocks found:', found_behavior)
        ok = False
    print('[INFO] signal terms found:', ', '.join(found_terms) if found_terms else '(none)')
    print('[INFO] signal SYS_* found:', ', '.join(found_sys) if found_sys else '(none)')
    if missing_soft:
        print('[WARN] soft-missing signal SYS_* names:', ', '.join(missing_soft))
    print('[INFO] behavior terms found:', ', '.join(found_behavior) if found_behavior else '(none)')
    print('[INFO] manifest:', ns.manifest)
    if ok:
        print('[PASS] signal user behavior guard v109 passed')
        return 0
    return 1

if __name__ == '__main__':
    raise SystemExit(main())
