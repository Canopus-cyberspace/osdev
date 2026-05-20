#!/usr/bin/env python3
from pathlib import Path
import json
import re
import sys

MOD = Path('src/syscall/mod.rs')
REQUIRED = [
    # v83 modern pidfd / io_uring / fsconfig / landlock family
    'SYS_PIDFD_OPEN','SYS_IO_URING_SETUP','SYS_IO_URING_ENTER','SYS_IO_URING_REGISTER',
    'SYS_OPEN_TREE','SYS_MOVE_MOUNT','SYS_FSOPEN','SYS_FSCONFIG','SYS_FSMOUNT','SYS_FSPICK',
    'SYS_PIDFD_SEND_SIGNAL','SYS_PIDFD_GETFD','SYS_OPENAT2','SYS_CLOSE_RANGE',
    'SYS_LANDLOCK_CREATE_RULESET','SYS_LANDLOCK_ADD_RULE','SYS_LANDLOCK_RESTRICT_SELF',
    'SYS_FUTEX_WAITV',
    # v84 rseq/membarrier/statx/pkey/copy_file_range family
    'SYS_BPF','SYS_EXECVEAT','SYS_USERFAULTFD','SYS_MEMBARRIER','SYS_MLOCK2',
    'SYS_COPY_FILE_RANGE','SYS_PREADV2','SYS_PWRITEV2','SYS_PKEY_MPROTECT','SYS_PKEY_ALLOC',
    'SYS_PKEY_FREE','SYS_STATX','SYS_IO_PGETEVENTS','SYS_RSEQ','SYS_KEXEC_FILE_LOAD',
    # v85 mount/process-memory/memfd family
    'SYS_FACCESSAT2','SYS_PROCESS_MADVISE','SYS_EPOLL_PWAIT2','SYS_MOUNT_SETATTR',
    'SYS_QUOTACTL_FD','SYS_MEMFD_SECRET','SYS_PROCESS_MRELEASE','SYS_SET_MEMPOLICY_HOME_NODE',
    'SYS_CACHESTAT','SYS_FCHMODAT2','SYS_MAP_SHADOW_STACK',
    # v86 mount/LSM/futex/xattr-at/mseal family
    'SYS_FUTEX_WAKE','SYS_FUTEX_WAIT','SYS_FUTEX_REQUEUE','SYS_STATMOUNT','SYS_LISTMOUNT',
    'SYS_LSM_GET_SELF_ATTR','SYS_LSM_SET_SELF_ATTR','SYS_LSM_LIST_MODULES','SYS_MSEAL',
    'SYS_SETXATTRAT','SYS_GETXATTRAT','SYS_LISTXATTRAT','SYS_REMOVEXATTRAT',
    'SYS_OPEN_TREE_ATTR','SYS_FILE_GETATTR','SYS_FILE_SETATTR','SYS_LISTNS','SYS_RSEQ_SLICE_YIELD',
]

if not MOD.exists():
    print('[ERROR] missing src/syscall/mod.rs')
    sys.exit(2)

text = MOD.read_text(errors='ignore')
lines = text.splitlines()

consts = set(re.findall(r'\b(?:pub\s+)?const\s+(SYS_[A-Z0-9_]+)\b', text))
all_arm_symbols = set(re.findall(r'(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>', text))

missing_consts = [s for s in REQUIRED if s not in consts]
missing_arms = [s for s in REQUIRED if s not in all_arm_symbols]

# Brace-aware extraction of match blocks, grouped by selector expression.
def find_match_blocks(src: str):
    out = []
    pat = re.compile(r'match\s+([^\{]+?)\s*\{')
    for m in pat.finditer(src):
        selector = ' '.join(m.group(1).split())
        start = m.end() - 1
        depth = 0
        end = None
        for i in range(start, len(src)):
            ch = src[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    end = i
                    break
        if end is None:
            continue
        body = src[start+1:end]
        start_line = src.count('\n', 0, m.start()) + 1
        out.append((selector, start_line, body))
    return out

duplicates = []
blocks = []
for selector, start_line, body in find_match_blocks(text):
    arms = []
    for mm in re.finditer(r'(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>', body):
        sym = mm.group(1)
        line = start_line + body.count('\n', 0, mm.start()) + 1
        arms.append((sym, line))
    if not arms:
        continue
    seen = {}
    dups = []
    for sym, line in arms:
        if sym in seen:
            dups.append({'symbol': sym, 'first_line': seen[sym], 'duplicate_line': line})
        else:
            seen[sym] = line
    if dups:
        duplicates.append({'selector': selector, 'start_line': start_line, 'duplicates': dups})
    blocks.append({'selector': selector, 'start_line': start_line, 'arm_count': len(arms), 'unique_arm_count': len(seen)})

# Catch classic variable-binding fallback from missing constants: bare all-caps identifiers in match arms
# are expected to be constants; build warnings also gate this, but this reports before build.
unknown_arm_symbols = sorted(sym for sym in all_arm_symbols if sym not in consts)

report = {
    'mod': str(MOD),
    'required_count': len(REQUIRED),
    'missing_consts': missing_consts,
    'missing_arms': missing_arms,
    'unknown_arm_symbols_without_const': unknown_arm_symbols,
    'match_blocks': blocks,
    'duplicate_arms_within_same_match': duplicates,
}
print(json.dumps(report, indent=2, sort_keys=True))

failed = False
if missing_consts:
    print('[ERROR] missing syscall constants:', ', '.join(missing_consts))
    failed = True
if missing_arms:
    print('[ERROR] missing dispatcher arms:', ', '.join(missing_arms))
    failed = True
if unknown_arm_symbols:
    print('[ERROR] dispatcher arms without const definitions:', ', '.join(unknown_arm_symbols))
    failed = True
if duplicates:
    for item in duplicates:
        print(f"[ERROR] duplicate arms in match {item['selector']} starting line {item['start_line']}:")
        for d in item['duplicates']:
            print(f"  - {d['symbol']}: first line {d['first_line']}, duplicate line {d['duplicate_line']}")
    failed = True

if failed:
    sys.exit(1)
print('[PASS] v88 syscall dispatch diagnostics guard passed: constants, arms, and per-match duplicate checks OK')
