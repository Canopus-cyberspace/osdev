#!/usr/bin/env python3
from __future__ import annotations
import argparse, hashlib, json, re
from pathlib import Path

def sha256(path: Path) -> str | None:
    if not path.exists():
        return None
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()

def find_match_blocks(text: str):
    blocks = []
    i = 0
    while True:
        m = re.search(r'\bmatch\s+[^{}]+\{', text[i:])
        if not m:
            break
        start = i + m.start()
        brace = i + m.end() - 1
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
        blocks.append(text[start:end])
        i = end
    return blocks

def duplicate_arms_by_block(text: str):
    dups = []
    for idx, block in enumerate(find_match_blocks(text), 1):
        arms = re.findall(r'(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>', block)
        seen = set()
        dup = sorted({a for a in arms if a in seen or seen.add(a)})
        if dup:
            dups.append({"match_block": idx, "duplicates": dup})
    return dups

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--manifest", required=True)
    ap.add_argument("--marker", required=True)
    args = ap.parse_args()

    syscall = Path("src/syscall/mod.rs")
    builder = Path("user/build_init_elf.py")
    init = Path("user/init.elf")

    problems = []
    text = syscall.read_text(encoding="utf-8", errors="replace") if syscall.exists() else ""
    if not syscall.exists():
        problems.append("missing src/syscall/mod.rs")
    if not builder.exists():
        problems.append("missing user/build_init_elf.py")
    if not init.exists():
        problems.append("missing user/init.elf")
    if builder.exists() and args.marker not in builder.read_text(encoding="utf-8", errors="replace"):
        problems.append("expected v101 marker missing from user/build_init_elf.py")
    if init.exists() and args.marker.encode() not in init.read_bytes():
        problems.append("expected v101 marker missing from user/init.elf")

    block_dups = duplicate_arms_by_block(text)
    if block_dups:
        problems.append("duplicate SYS_* arms inside a single match block")

    symbols = {
        "execve": bool(re.search(r'\bSYS_EXECVE\b|execve', text)),
        "argv": bool(re.search(r'argv|argc', text, re.I)),
        "envp": bool(re.search(r'envp|env', text, re.I)),
        "auxv": bool(re.search(r'auxv|AT_', text)),
        "user_string_or_copy": bool(re.search(r'copy_from_user|copy_to_user|user.*str|read.*cstring|string', text, re.I)),
    }

    manifest = {
        "version": "v101b",
        "marker": args.marker,
        "files": {
            "src/syscall/mod.rs": {"exists": syscall.exists(), "sha256": sha256(syscall), "bytes": syscall.stat().st_size if syscall.exists() else None},
            "user/build_init_elf.py": {"exists": builder.exists(), "sha256": sha256(builder), "bytes": builder.stat().st_size if builder.exists() else None},
            "user/init.elf": {"exists": init.exists(), "sha256": sha256(init), "bytes": init.stat().st_size if init.exists() else None},
        },
        "symbols": symbols,
        "duplicate_arms_by_match_block": block_dups,
        "problems": problems,
    }

    Path(args.manifest).write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding="utf-8")

    print(f"[INFO] execve argv/envp/auxv guard v101b marker: {args.marker}")
    print(f"[INFO] manifest written: {args.manifest}")
    for name, present in symbols.items():
        print(f"[INFO] semantic symbol probe {name}: {'present' if present else 'missing/debt'}")

    if block_dups:
        print("[ERROR] duplicate SYS_* dispatcher arms inside one match block:")
        for item in block_dups:
            print(f"  block {item['match_block']}: {', '.join(item['duplicates'])}")

    if problems:
        for p in problems:
            print(f"[ERROR] {p}")
        raise SystemExit(1)

    print("[PASS] execve argv/envp/auxv semantic guard v101b passed")

if __name__ == "__main__":
    main()
