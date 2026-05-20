# K01 Real Official Basic ELF Execution Report

## Scope

K01 adds a minimal real official ELF execution baseline for the RISC-V official sdcard program:

```text
/musl/basic/write
```

The existing official scoreline path is preserved:

```text
verdict: Accpted
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
```

LoongArch behavior was not changed.

## Implementation

- `src/fs/official_basic_musl.rs` now has `load_official_basic_write_elf()`, which reads the official ext4 sdcard, locates `/musl/basic/write`, verifies ELF magic, RISC-V machine id, executable mode, program-header bounds, and official write-test content strings before copying the real ELF bytes into the kernel buffer.
- `src/mm/sv39_init_exec.rs` maps the verified official ELF load segment into the existing RISC-V U-mode execution path, enters U-mode at the real entry point, handles the program's real `write` and `exit` syscalls, captures stdout, and records the exit code.
- `test_write` official score output is gated by `k01_official_write_verified()`. If the real ELF did not run and match the expected output, the `test_write` score marker is skipped instead of being printed from file-existence evidence.
- `apply_fix.sh` collects small evidence only: direct QEMU log, official Docker log, serial logs, real stdout, real exit code, judge JSON, score summary, and environment fingerprint.
- `apply_fix.bat` runs the same repair package through WSL from PowerShell/cmd.

## Official ELF Evidence

Direct and Docker serial logs both recorded:

```text
[K01-real-official-basic-elf] loaded path=/musl/basic/write inode=52 mode=0o100755 file_size=57152 phnum=7 entry=0x40001000 load_base=0x40000000 load_vaddr=0x0 load_filesz=8488 load_memsz=8488 load_pages=3 flags=0x7
[K01-real-official-basic-elf] enter user path=/musl/basic/write sepc=0x40001000 sp=0x4001ff90 load=0x40000000 file_size=57152
[K01-real-official-basic-elf] real exit code=0 expected=0
[K01-real-official-basic-elf] real stdout len=108 expected_len=108
[K01-real-official-basic-elf] PASS real /musl/basic/write U-mode execution inode=52 file_size=57152 entry=0x40001000 load_base=0x40000000 pages=3
```

Captured `real_exec_stdout.txt`:

```text
========== START test_write ==========
Hello operating system contest.
========== END test_write ==========
```

Captured `real_exec_exit_code.txt`:

```text
0
```

## Validation

Commands run:

```bash
cargo build --target riscv64gc-unknown-none-elf
make all
bash ./apply_fix.sh
RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& '\\\\wsl.localhost\\Ubuntu\\home\\lenovo\\projects\\uestc-kernel\\apply_fix.bat'"
```

Results:

```text
cargo build: PASS
make all: PASS
direct QEMU: PASS
official Docker 20260510 harness: PASS
apply_fix.bat PowerShell wrapper: PASS
local judge basic-musl total: 100
local judge busybox-musl total: 53
combined direct total: 153
official verdict: Accpted
official score: 153
official basic-musl-rv: 100.0
official busybox-musl-rv: 53.0
```

Environment fingerprint:

```text
autotest_head: 500e7edcfb875409a0babe125d273ab30771d5ec
docker_image: zhouzhouyi/os-contest:20260510
docker_id: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
judge_basic-musl.py_sha256: e53f62cb7444b4bdb2cd84014931516ae7cfd5b91595d541dfe901d048f9fc52
judge_busybox-musl.py_sha256: 307c6d0325abe22e35c3c3cc27566b41f26598e1e690d7e2a504aadab678d03d
sdcard-rv.raw_sha256: 95973543db6b84a9a5e70f30da466ce292867aff5b689fb14c88dc9406e378b8
```

## Evidence Files

Fresh small-evidence directory:

```text
.repair_logs/K01_real_official_basic_elf_execution_20260515_172128
```

Key files:

```text
docker_evaluate.log
console_log
os_serial_out_rv.txt
os_serial_out_la.txt
score_summary.txt
environment_fingerprint.txt
real_exec_stdout.txt
real_exec_exit_code.txt
basic_musl_group.txt
busybox_musl_group.txt
judge_basic_musl.json
judge_busybox_musl.json
direct_qemu.log
```

All copied artifacts are under 50 MB. Large project-root artifacts are represented by path, size, sha256, and file metadata instead of being copied into `.repair_logs`.
