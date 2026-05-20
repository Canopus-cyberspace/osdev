#!/usr/bin/env python3
import argparse, hashlib, json, re, sys, time
from pathlib import Path

GROUPS = {
    "pid_identity": ["SYS_GETPID", "SYS_GETPPID"],
    "process_create": ["SYS_CLONE", "SYS_CLONE3", "fork", "clone"],
    "process_exec": ["SYS_EXECVE", "SYS_EXECVEAT", "execve"],
    "process_exit": ["SYS_EXIT", "SYS_EXIT_GROUP", "ExitGroup", "sys_exit"],
    "process_wait": ["SYS_WAIT4", "wait4"],
    "task_model": ["Task", "Process", "pid", "tid"],
    "fd_inheritance_context": ["fd", "File", "Fd", "close_on_exec", "cloexec"],
    "user_memory_context": ["copy_from_user", "copy_to_user", "argv", "envp", "auxv"],
}
SYS_ARM_RE = re.compile(r"\b(SYS_[A-Z0-9_]+)\s*=>")
MATCH_RE = re.compile(r"\bmatch\s+[^\{]+\{")
CONST_RE = re.compile(r"\b(?:pub\s+)?(?:const|static)\s+(SYS_[A-Z0-9_]+)\b|\buse\s+[^;]*\b(SYS_[A-Z0-9_]+)\b")

def read(path):
    try:
        return path.read_text(errors="ignore")
    except Exception:
        return ""

def sha256(path):
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except Exception:
        return None

def iter_rs(project):
    skip = {"target", ".git", ".repair_logs"}
    for p in project.rglob("*.rs"):
        if any(part in skip for part in p.parts):
            continue
        yield p

def extract_match_blocks(text):
    blocks = []
    for m in MATCH_RE.finditer(text):
        start = m.start()
        brace = text.find("{", m.end()-1)
        if brace < 0:
            continue
        depth = 0
        end = None
        for i in range(brace, len(text)):
            ch = text[i]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end:
            blocks.append(text[start:end])
    return blocks

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--project", default=".")
    ap.add_argument("--log", required=True)
    ap.add_argument("--manifest", required=True)
    args = ap.parse_args()
    project = Path(args.project).resolve()
    log_path = Path(args.log)
    manifest_path = Path(args.manifest)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    rs_files = list(iter_rs(project))
    combined = "\n".join(read(p) for p in rs_files)
    syscall_mod = project / "src/syscall/mod.rs"
    syscall_text = read(syscall_mod)
    missing_groups = {}
    found_groups = {}
    for group, needles in GROUPS.items():
        found = sorted({n for n in needles if n in combined})
        found_groups[group] = found
        if not found:
            missing_groups[group] = needles
    duplicate_blocks = []
    for idx, block in enumerate(extract_match_blocks(syscall_text)):
        arms = SYS_ARM_RE.findall(block)
        seen = set()
        dup = []
        for arm in arms:
            if arm in seen and arm not in dup:
                dup.append(arm)
            seen.add(arm)
        if dup:
            duplicate_blocks.append({"match_block_index": idx, "duplicates": sorted(dup)})
    consts = {a or b for a, b in CONST_RE.findall(combined) if (a or b)}
    arms_all = sorted(set(SYS_ARM_RE.findall(combined)))
    missing_consts = [a for a in arms_all if a not in consts and a not in {"SYS_UNKNOWN"}]
    manifest = {
        "version": "v107",
        "timestamp": int(time.time()),
        "project": str(project),
        "syscall_mod": str(syscall_mod),
        "syscall_mod_sha256": sha256(syscall_mod),
        "rust_file_count": len(rs_files),
        "sys_arm_unique_count": len(arms_all),
        "sys_const_unique_count": len(consts),
        "found_groups": found_groups,
        "missing_groups": missing_groups,
        "duplicate_blocks": duplicate_blocks,
        "missing_consts_sample": missing_consts[:50],
        "focus": "process user behavior: clone/fork/execve/exit/wait4/getpid/getppid with fd/user-memory context",
    }
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
    lines = ["[INFO] process user behavior guard v107", f"[INFO] project: {project}"]
    for group, found in found_groups.items():
        lines.append(f"[INFO] group {group}: {found if found else 'MISSING'}")
    lines += [f"[INFO] unique SYS arms: {len(arms_all)}", f"[INFO] unique SYS const/imports: {len(consts)}", f"[INFO] manifest: {manifest_path}"]
    errors = []
    for g in ["pid_identity", "process_exec", "process_exit", "process_wait"]:
        if g in missing_groups:
            errors.append(f"missing required process behavior group: {g} candidates={missing_groups[g]}")
    if duplicate_blocks:
        errors.append(f"duplicate SYS_* arms inside individual match blocks: {duplicate_blocks}")
    if missing_consts:
        errors.append(f"SYS_* arms without visible const/import declarations: {missing_consts[:20]}")
    if errors:
        for e in errors:
            lines.append(f"[ERROR] {e}")
        log_path.write_text("\n".join(lines) + "\n")
        print("\n".join(lines))
        return 1
    lines.append("[PASS] process user behavior semantic guard v107 passed")
    log_path.write_text("\n".join(lines) + "\n")
    print("\n".join(lines))
    return 0
if __name__ == "__main__":
    sys.exit(main())
