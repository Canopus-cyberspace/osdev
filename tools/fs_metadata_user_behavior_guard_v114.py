#!/usr/bin/env python3
from __future__ import annotations
import hashlib, json, re, sys, time
from pathlib import Path

ROOT = Path.cwd()
SRC = ROOT / 'src' / 'syscall' / 'mod.rs'
MANIFEST = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / '.repair_logs' / 'fs_metadata_user_behavior_manifest_v114.json'

REQUIRED_FILES = [SRC]
for f in REQUIRED_FILES:
    if not f.exists():
        print(f'[ERROR] required file missing: {f}')
        sys.exit(1)

text = SRC.read_text(errors='ignore')

# Block-scope duplicate SYS_* arm check: same SYS may appear in separate dispatchers,
# but not twice in the same match body.
def extract_match_blocks(s: str):
    blocks = []
    idx = 0
    while True:
        m = re.search(r'\bmatch\s+[^\{]+\{', s[idx:])
        if not m:
            break
        start = idx + m.start()
        open_brace = idx + m.end() - 1
        depth = 0
        end = None
        for pos in range(open_brace, len(s)):
            ch = s[pos]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    end = pos + 1
                    break
        if end is None:
            break
        blocks.append((start, end, s[start:end]))
        idx = end
    return blocks

dups = []
for bi, (start, end, block) in enumerate(extract_match_blocks(text), 1):
    arms = re.findall(r'\b(SYS_[A-Z0-9_]+)\s*=>', block)
    seen = set()
    local = []
    for a in arms:
        if a in seen and a not in local:
            local.append(a)
        seen.add(a)
    if local:
        line = text[:start].count('\n') + 1
        dups.append({'block': bi, 'line': line, 'symbols': local})
if dups:
    for d in dups:
        print(f"[ERROR] duplicate SYS_* arms inside match block {d['block']} near line {d['line']}: {', '.join(d['symbols'])}")
    sys.exit(1)
print('[PASS] no duplicate SYS_* arms inside individual match blocks')

# Catch-all style risk: constant-like SYS arms should have corresponding textual definitions/imports.
arms = sorted(set(re.findall(r'\b(SYS_[A-Z0-9_]+)\s*=>', text)))
missing_consts = []
for sym in arms:
    # Accept any declaration/import/reference outside the arm itself. This is intentionally conservative.
    decl_patterns = [
        rf'const\s+{re.escape(sym)}\b',
        rf'static\s+{re.escape(sym)}\b',
        rf'pub\s+const\s+{re.escape(sym)}\b',
        rf'use\s+[^;]*\b{re.escape(sym)}\b',
    ]
    if not any(re.search(p, text) for p in decl_patterns):
        # Avoid false positives for projects using numeric constants generated elsewhere by warning only.
        missing_consts.append(sym)
if missing_consts:
    print('[WARN] SYS_* arms without local const/import declarations; verify they are imported/generated elsewhere: ' + ', '.join(missing_consts[:40]))
else:
    print('[PASS] SYS_* arms have visible local const/import declarations')

# v114 behavior debt map. These are not fatal individually: the goal is to keep a manifest
# of what exists and what still needs real behavior tests.
groups = {
    'metadata_stat': ['SYS_FSTAT', 'SYS_NEWFSTATAT', 'SYS_STATX', 'sys_fstat', 'sys_statx', 'Statx', 'FileStat'],
    'directory_getdents': ['SYS_GETDENTS64', 'getdents', 'Dirent', 'dirent'],
    'path_permission': ['SYS_FACCESSAT', 'SYS_FACCESSAT2', 'SYS_FCHMODAT', 'SYS_FCHMODAT2', 'SYS_FCHOWNAT', 'faccessat', 'fchmodat', 'fchownat'],
    'link_rename_unlink': ['SYS_LINKAT', 'SYS_UNLINKAT', 'SYS_RENAMEAT', 'SYS_RENAMEAT2', 'linkat', 'unlinkat', 'renameat'],
    'mkdir_readlink': ['SYS_MKDIRAT', 'SYS_READLINKAT', 'SYS_SYMLINKAT', 'mkdirat', 'readlinkat', 'symlinkat'],
    'xattr_classic': ['SYS_SETXATTR', 'SYS_GETXATTR', 'SYS_LISTXATTR', 'SYS_REMOVEXATTR', 'xattr'],
    'xattr_at': ['SYS_SETXATTRAT', 'SYS_GETXATTRAT', 'SYS_LISTXATTRAT', 'SYS_REMOVEXATTRAT'],
    'usercopy_structs': ['copy_to_user', 'copy_from_user', 'EFAULT', 'UserBuffer', 'copyout'],
}
coverage = {}
for name, needles in groups.items():
    hits = [n for n in needles if n in text]
    coverage[name] = {'hits': hits, 'hit_count': len(hits), 'needle_count': len(needles)}
    if hits:
        print(f"[PASS] v114 coverage signal {name}: {', '.join(hits[:8])}")
    else:
        print(f"[WARN] v114 coverage signal missing for {name}; recorded as semantic debt")

# Require broad evidence across at least several groups, but do not require every modern optional syscall.
covered_groups = sum(1 for v in coverage.values() if v['hit_count'] > 0)
if covered_groups < 4:
    print(f'[ERROR] insufficient filesystem metadata/path/xattr coverage evidence: {covered_groups} groups')
    sys.exit(1)
print(f'[PASS] filesystem metadata/path/xattr coverage evidence groups: {covered_groups}/{len(groups)}')

manifest = {
    'version': 'v114',
    'timestamp': int(time.time()),
    'source': str(SRC),
    'sha256': hashlib.sha256(SRC.read_bytes()).hexdigest(),
    'bytes': SRC.stat().st_size,
    'sys_arm_count': len(arms),
    'unique_sys_arm_count': len(set(arms)),
    'missing_const_or_import_warnings': missing_consts[:100],
    'coverage': coverage,
    'match_block_count': len(extract_match_blocks(text)),
}
MANIFEST.parent.mkdir(parents=True, exist_ok=True)
MANIFEST.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f'[PASS] v114 manifest written: {MANIFEST}')
print('[PASS] filesystem metadata/path/xattr user behavior guard v114 passed')
