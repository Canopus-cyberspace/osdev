#!/usr/bin/env python3
import hashlib, json, os, re, sys
from pathlib import Path
ROOT = Path.cwd()
LOG = Path(os.environ.get("V97_GUARD_LOG", ".repair_logs/mmap_brk_semantic_guard_v97.log"))
MANIFEST = Path(os.environ.get("V97_MANIFEST", ".repair_logs/mmap_brk_semantic_manifest_v97.json"))
LOG.parent.mkdir(parents=True, exist_ok=True)
MANIFEST.parent.mkdir(parents=True, exist_ok=True)
rs_files = [p for p in ROOT.rglob("*.rs") if "/target/" not in str(p) and "/.backup" not in str(p)]
texts = {}
for p in rs_files:
    try: texts[p] = p.read_text(errors="ignore")
    except Exception: texts[p] = ""
modrs = ROOT / "src" / "syscall" / "mod.rs"
mod_text = modrs.read_text(errors="ignore") if modrs.exists() else ""
def blocks(text):
    out=[]; i=0
    while True:
        m=re.search(r"\bmatch\s+[^\{]+\{", text[i:])
        if not m: break
        start=i+m.start(); brace=i+m.end()-1; depth=0; end=None
        for j in range(brace,len(text)):
            if text[j]=='{': depth+=1
            elif text[j]=='}':
                depth-=1
                if depth==0: end=j+1; break
        if end is None: break
        out.append(text[start:end]); i=end
    return out
consts=set(re.findall(r"\b(?:pub\s+)?(?:const|static)\s+(SYS_[A-Z0-9_]+)\b", mod_text))
dups=[]; missing=[]
for n,b in enumerate(blocks(mod_text),1):
    arms=re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>", b, flags=re.M)
    seen=set(); local=[]
    for a in arms:
        if a in seen and a not in local: local.append(a)
        seen.add(a)
    if local: dups.append({"match_block":n,"duplicates":sorted(local)})
    for a in sorted(set(arms)):
        if a not in consts and not re.search(rf"\buse\b[^;]*\b{re.escape(a)}\b", mod_text):
            missing.append({"match_block":n,"symbol":a})
keywords=["SYS_MMAP","SYS_MUNMAP","SYS_MPROTECT","SYS_BRK","SYS_MREMAP","mmap","munmap","mprotect","brk","MemorySet","MapArea","PageTable","MapPermission","copy_from_user","copy_to_user","EFAULT"]
hits={k:[] for k in keywords}
for p,t in texts.items():
    for k in keywords:
        if k in t: hits[k].append(str(p.relative_to(ROOT)))
def sha(p):
    try: return hashlib.sha256(p.read_bytes()).hexdigest()
    except Exception: return None
files={}
for rel in ["src/syscall/mod.rs","src/mm/mod.rs","src/trap/mod.rs","user/build_init_elf.py","user/init.elf","Cargo.toml","build.rs"]:
    p=ROOT/rel
    if p.exists(): files[rel]={"size":p.stat().st_size,"sha256":sha(p)}
manifest={"version":"v97","guard":"mmap/brk memory-management semantic guard","rs_file_count":len(rs_files),"syscall_const_count":len(consts),"match_block_duplicate_errors":dups,"missing_sys_const_errors":missing[:200],"keyword_hits":{k:v[:12] for k,v in hits.items()},"files":files,"note":"Keyword hits are diagnostic; duplicate arms and missing SYS const/import are hard failures."}
MANIFEST.write_text(json.dumps(manifest,indent=2,ensure_ascii=False))
with LOG.open("w",encoding="utf-8") as f:
    f.write("[INFO] mmap/brk semantic guard v97\n")
    f.write(f"[INFO] rust source files scanned: {len(rs_files)}\n")
    for k in keywords: f.write(f"[INFO] keyword {k}: {hits[k][:8]}\n")
    if dups: f.write(f"[ERROR] duplicate SYS_* arms inside single match block: {dups}\n")
    if missing: f.write(f"[ERROR] SYS_* arms without visible const/static/import: {missing[:20]}\n")
    f.write(f"[INFO] manifest: {MANIFEST}\n")
if dups or missing:
    print(f"[ERROR] mmap/brk semantic guard v97 failed; see {LOG}"); sys.exit(1)
print(f"[PASS] mmap/brk semantic guard v97 passed; manifest: {MANIFEST}")
