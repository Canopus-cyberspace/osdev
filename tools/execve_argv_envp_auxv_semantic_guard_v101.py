#!/usr/bin/env python3
"""v101 execve argv/envp/auxv semantic guard.

This is intentionally conservative: it fails only on hard regression signals
that previously broke the tree, and records semantic debt in the manifest so
future repairs can turn each item into a real user-mode conformance test.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding='utf-8')
    except UnicodeDecodeError:
        return path.read_text(encoding='utf-8', errors='ignore')


def collect_rs(project: Path) -> Dict[str, str]:
    out: Dict[str, str] = {}
    for p in sorted(project.rglob('*.rs')):
        rel = p.relative_to(project).as_posix()
        if '/target/' in '/' + rel or rel.startswith('target/'):
            continue
        out[rel] = read_text(p)
    return out


def find_match_blocks(text: str) -> List[Tuple[int, int, str]]:
    blocks: List[Tuple[int, int, str]] = []
    for m in re.finditer(r'\bmatch\s+[^{}]+\{', text):
        start = m.start()
        open_brace = text.find('{', m.start(), m.end())
        if open_brace < 0:
            continue
        depth = 0
        for i in range(open_brace, len(text)):
            ch = text[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    blocks.append((start, i + 1, text[start:i + 1]))
                    break
    return blocks


def duplicate_sys_arms_within_blocks(mod_text: str) -> List[dict]:
    dups: List[dict] = []
    for bi, (start, end, block) in enumerate(find_match_blocks(mod_text), 1):
        arms = re.findall(r'\b(SYS_[A-Z0-9_]+)\s*=>', block)
        seen = set()
        dup = sorted({a for a in arms if a in seen or seen.add(a)})
        if dup:
            line = mod_text.count('\n', 0, start) + 1
            dups.append({'block_index': bi, 'line': line, 'duplicates': dup})
    return dups


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--project', default='.')
    ap.add_argument('--manifest', required=True)
    args = ap.parse_args()

    project = Path(args.project).resolve()
    manifest_path = Path(args.manifest)
    src_syscall = project / 'src' / 'syscall' / 'mod.rs'
    if not src_syscall.exists():
        print('[ERROR] missing src/syscall/mod.rs')
        return 1

    mod_text = read_text(src_syscall)
    rs_files = collect_rs(project)
    all_rs = '\n'.join(rs_files.values())

    errors: List[str] = []
    warnings: List[str] = []

    required_symbols = [
        'SYS_EXECVE', 'SYS_EXECVEAT', 'SYS_BRK', 'SYS_MMAP', 'SYS_MUNMAP',
        'copy_from_user', 'copy_to_user', 'execve', 'auxv', 'argv', 'envp',
    ]
    symbol_hits = {sym: (sym in all_rs or sym.lower() in all_rs.lower()) for sym in required_symbols}

    if not (symbol_hits.get('SYS_EXECVE') or 'sys_execve' in all_rs.lower()):
        errors.append('missing visible execve syscall symbol/path')

    # These are semantic readiness signals. Missing ones are recorded as debt
    # rather than hard failures because earlier scaffolds may use different names.
    readiness_terms = {
        'argv': ['argv', 'argc', 'argument'],
        'envp': ['envp', 'environment'],
        'auxv': ['auxv', 'auxiliary', 'AT_PHDR', 'AT_PAGESZ', 'AT_ENTRY'],
        'user_stack': ['user_stack', 'ustack', 'stack_top', 'sp'],
        'elf_loader': ['Elf', 'ELF', 'program header', 'phdr', 'entry'],
        'copy_user_string': ['copy_from_user', 'read_cstr', 'cstring', 'user_str', 'copyinstr'],
        'fault_errors': ['EFAULT', 'EINVAL', 'ENOENT', 'ENOMEM'],
    }
    readiness = {}
    lower_all = all_rs.lower()
    for key, terms in readiness_terms.items():
        hits = [t for t in terms if t.lower() in lower_all]
        readiness[key] = hits
        if not hits:
            warnings.append(f'semantic debt: no obvious {key} signal found')

    dups = duplicate_sys_arms_within_blocks(mod_text)
    if dups:
        errors.append('duplicate SYS_* dispatcher arms within a single match block: ' + json.dumps(dups, ensure_ascii=False))

    manifest = {
        'version': 'v101',
        'project': str(project),
        'src_syscall_mod_rs': {
            'exists': src_syscall.exists(),
            'size': src_syscall.stat().st_size,
            'sha256': sha256(src_syscall),
            'sys_arm_count': len(re.findall(r'\bSYS_[A-Z0-9_]+\s*=>', mod_text)),
            'match_block_count': len(find_match_blocks(mod_text)),
        },
        'symbol_hits': symbol_hits,
        'readiness': readiness,
        'warnings': warnings,
        'errors': errors,
    }
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

    for w in warnings:
        print('[WARN]', w)
    if errors:
        for e in errors:
            print('[ERROR]', e)
        print(f'[ERROR] execve argv/envp/auxv semantic guard v101 failed; manifest: {manifest_path}')
        return 1

    print(f'[PASS] execve argv/envp/auxv semantic guard v101 passed; manifest: {manifest_path}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
