#!/usr/bin/env python3
from __future__ import annotations

import argparse, hashlib, json, re, time
from pathlib import Path
from typing import Any

ROOT = Path.cwd()
VERSION = "v95"
EXPECTED_MARKER = "hello from external init.elf v95 fd-vfs syscall write"
CORE_SYSCALLS = ["SYS_OPENAT", "SYS_CLOSE", "SYS_READ", "SYS_WRITE", "SYS_FSTAT", "SYS_LSEEK", "SYS_GETDENTS64", "SYS_DUP", "SYS_DUP3", "SYS_PIPE2"]
FD_VFS_KEYWORDS = {
    "fd_table_or_allocator": ["fd_table", "FdTable", "alloc_fd", "alloc_fd_at", "insert_fd", "get_fd", "close_fd"],
    "file_object": ["trait File", "struct File", "FileLike", "FileHandle", "OpenFile", "Arc<dyn"],
    "read_write_ops": ["fn read", "sys_read", "read_at", "fn write", "sys_write", "write_at"],
    "seek_offset": ["SEEK_SET", "SEEK_CUR", "SEEK_END", "offset", "seek", "lseek"],
    "path_open": ["openat", "OpenFlags", "O_CREAT", "O_TRUNC", "path", "lookup"],
    "stat_metadata": ["fstat", "statx", "Kstat", "Stat", "metadata", "inode"],
    "dirent": ["getdents64", "Dirent", "linux_dirent64", "d_reclen", "d_name"],
    "user_copy": ["copy_from_user", "copy_to_user", "translated_byte_buffer", "UserBuffer", "read_cstr", "write_user"],
}
FILES_OF_INTEREST = ["src/syscall/mod.rs", "src/fs/mod.rs", "src/vfs/mod.rs", "src/task/mod.rs", "src/mm/mod.rs", "user/build_init_elf.py", "user/init.elf", "build.rs", "tools/fd_vfs_semantic_guard_v95.py", "tools/run_fd_vfs_semantic_smoke_v95.sh"]

def sha256_file(p: Path) -> str | None:
    if not p.exists() or not p.is_file(): return None
    h = hashlib.sha256()
    with p.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()

def file_info(rel: str) -> dict[str, Any]:
    p = ROOT / rel
    info: dict[str, Any] = {"path": rel, "exists": p.exists()}
    if p.exists() and p.is_file():
        st = p.stat(); info.update({"size": st.st_size, "mtime": int(st.st_mtime), "sha256": sha256_file(p)})
    return info

def read_all_rs() -> tuple[str, dict[str, str]]:
    texts: dict[str, str] = {}
    for p in sorted((ROOT / "src").rglob("*.rs")) if (ROOT / "src").exists() else []:
        try: texts[str(p.relative_to(ROOT))] = p.read_text(encoding="utf-8", errors="ignore")
        except Exception: pass
    return "\n".join(texts.values()), texts

def strip_comments_and_strings(src: str) -> str:
    src = re.sub(r"//.*", "", src)
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    src = re.sub(r'"(?:\\.|[^"\\])*"', '""', src)
    return src

def find_match_blocks(src: str) -> list[dict[str, Any]]:
    blocks: list[dict[str, Any]] = []
    for m in re.finditer(r"\bmatch\s+([^{}]+)\{", src):
        selector = " ".join(m.group(1).strip().split())
        brace = src.find("{", m.start())
        if brace < 0: continue
        depth = 0; end = None
        for i in range(brace, len(src)):
            if src[i] == "{": depth += 1
            elif src[i] == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1; break
        if end is None: continue
        body = src[brace + 1:end - 1]
        arms = re.findall(r"(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>", body)
        blocks.append({"selector": selector, "arms": arms, "start": m.start(), "end": end})
    return blocks

def analyze_dispatcher() -> dict[str, Any]:
    p = ROOT / "src/syscall/mod.rs"
    out: dict[str, Any] = {"path": "src/syscall/mod.rs", "exists": p.exists(), "errors": [], "warnings": []}
    if not p.exists():
        out["errors"].append("src/syscall/mod.rs missing"); return out
    raw = p.read_text(encoding="utf-8", errors="ignore")
    src = strip_comments_and_strings(raw)
    mentions = set(re.findall(r"\bSYS_[A-Z0-9_]+\b", src))
    blocks = find_match_blocks(src)
    block_summaries = []; duplicate_errors = []
    for i, b in enumerate(blocks, start=1):
        seen: dict[str, int] = {}; dups = []
        for a in b["arms"]:
            seen[a] = seen.get(a, 0) + 1
            if seen[a] == 2: dups.append(a)
        if dups: duplicate_errors.append(f"match block #{i} ({b['selector']}): {', '.join(sorted(dups))}")
        block_summaries.append({"index": i, "selector": b["selector"], "arm_count": len(b["arms"]), "unique_arm_count": len(set(b["arms"])), "duplicates": sorted(dups)})
    missing_core = [s for s in CORE_SYSCALLS if s not in mentions]
    if duplicate_errors: out["errors"].append("duplicate SYS_* dispatcher arms inside a single match block: " + " | ".join(duplicate_errors))
    if missing_core: out["warnings"].append("core FD/VFS syscall symbols not observed: " + ", ".join(missing_core))
    out.update({"core_syscalls_observed": sorted([s for s in CORE_SYSCALLS if s in mentions]), "core_syscalls_missing": missing_core, "match_block_count": len(blocks), "match_blocks": block_summaries})
    return out

def analyze_fd_vfs_keywords() -> dict[str, Any]:
    corpus, per_file = read_all_rs()
    out: dict[str, Any] = {"categories": {}, "score": 0, "max_score": len(FD_VFS_KEYWORDS), "warnings": []}
    for cat, needles in FD_VFS_KEYWORDS.items():
        hits = [n for n in needles if n in corpus]
        file_hits: list[str] = []
        if hits:
            for rel, text in per_file.items():
                if any(h in text for h in hits): file_hits.append(rel)
            out["score"] += 1
        else:
            out["warnings"].append(f"FD/VFS semantic keyword category not observed yet: {cat}")
        out["categories"][cat] = {"observed": bool(hits), "hits": hits[:20], "files": sorted(set(file_hits))[:20]}
    return out

def latest_logs() -> list[dict[str, Any]]:
    d = ROOT / ".repair_logs"
    if not d.exists(): return []
    items = sorted([p for p in d.iterdir() if p.is_file()], key=lambda p: p.stat().st_mtime, reverse=True)[:50]
    return [{"path": str(p.relative_to(ROOT)), "size": p.stat().st_size, "mtime": int(p.stat().st_mtime), "sha256": sha256_file(p)} for p in items]

def main() -> int:
    ap = argparse.ArgumentParser(); ap.add_argument("--manifest", required=True); ap.add_argument("--guard-log", required=True); args = ap.parse_args()
    dispatcher = analyze_dispatcher(); fdvfs = analyze_fd_vfs_keywords()
    manifest: dict[str, Any] = {"version": VERSION, "created_at_unix": int(time.time()), "project": str(ROOT), "expected_marker": EXPECTED_MARKER, "files": [file_info(x) for x in FILES_OF_INTEREST], "dispatcher": dispatcher, "fd_vfs_semantic_readiness": fdvfs, "latest_logs": latest_logs()}
    mp = Path(args.manifest); mp.parent.mkdir(parents=True, exist_ok=True); mp.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding="utf-8")
    lines = [f"[INFO] FD/VFS semantic guard {VERSION}", f"[INFO] project: {ROOT}", f"[INFO] expected marker: {EXPECTED_MARKER}", f"[INFO] manifest: {mp}", f"[INFO] core FD/VFS syscall symbols observed: {', '.join(dispatcher.get('core_syscalls_observed', []))}", f"[INFO] FD/VFS semantic readiness score: {fdvfs['score']}/{fdvfs['max_score']}"]
    for b in dispatcher.get("match_blocks", []): lines.append(f"[INFO] match block #{b['index']} selector={b['selector']} arms={b['arm_count']} unique={b['unique_arm_count']}")
    for w in dispatcher.get("warnings", []): lines.append(f"[WARN] {w}")
    for w in fdvfs.get("warnings", []): lines.append(f"[WARN] {w}")
    if dispatcher.get("errors"):
        for e in dispatcher["errors"]: lines.append(f"[ERROR] {e}")
        lines.append("[ERROR] FD/VFS semantic guard v95 failed")
        Path(args.guard_log).write_text("\n".join(lines)+"\n", encoding="utf-8"); print("\n".join(lines)); return 1
    lines.append("[PASS] FD/VFS semantic guard v95 passed")
    Path(args.guard_log).write_text("\n".join(lines)+"\n", encoding="utf-8"); print("\n".join(lines)); return 0
if __name__ == "__main__": raise SystemExit(main())
