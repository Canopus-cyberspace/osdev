# v194 Official RISC-V QEMU Runtime Compatibility Report

## Scope

This batch fixes official OSComp RISC-V QEMU runtime compatibility only.

Baseline entering the batch:

- v194 plus the official `make all` compatibility batch.
- Root `make all` already compiled successfully and produced `kernel-rv`.
- The official Docker wrapper entered the RISC-V QEMU phase but could leave `qemu-system-riscv64` running for a long time.

This batch does not implement a new v195-v200 milestone, does not fake official PASS markers, and does not claim LoongArch runtime support.

## Diagnosis

The official RISC-V QEMU command was reproduced with a timeout:

```text
qemu-system-riscv64 -machine virt -kernel kernel-rv -m 1G -nographic -smp 1 -bios default -drive file=sdcard-rv.img,if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -no-reboot -device virtio-net-device,netdev=net -netdev user,id=net -rtc base=utc
```

The kernel did boot visibly and preserved the historical runtime path, but after the external init smoke completed it printed:

```text
[external-init-v82] smoke passed
[external-init-v82] kernel idle after external init ELF smoke
```

and then entered an infinite `wfi` loop.

The official harness starts QEMU through `subprocess.Popen(...).communicate(timeout=...)`. If QEMU does not exit by itself, the harness can kill only the shell process and leave the QEMU child behind. That made the official run look like a silent hang even though the kernel had reached the end of its current smoke path.

## Change

- Added an SBI System Reset shutdown path in `src/sbi.rs`.
- Added a QEMU virt finisher fallback at `0x00100000` for older or unusual QEMU/OpenSBI combinations where the SBI reset call returns.
- Replaced the post-smoke infinite idle loops in `src/mm/sv39_init_exec.rs` with a shared terminal path:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

After that diagnostic, the kernel requests shutdown so the official RISC-V QEMU process exits cleanly.

## Preserved Behavior

- The existing boot marker path is preserved.
- The existing real U-mode external init path is preserved.
- The v151k7 through v194 runtime markers remain present in fresh RISC-V QEMU output.
- `kernel-rv` is still produced by `make all`.
- LoongArch runtime support was not implemented or claimed. The official LoongArch run still reports that `kernel-la` is not a loadable LoongArch kernel.

## Validation Results

Local validation on 2026-05-10:

- `cargo build --target riscv64gc-unknown-none-elf`: PASS.
- `make all`: PASS.
- Local official RISC-V QEMU command in a scratch directory: PASS, QEMU exited without timeout.
- `bash ./apply_fix.sh`: PASS.
- `cmd.exe /C \\wsl.localhost\Ubuntu\home\lenovo\projects\uestc-kernel\apply_fix.bat`: PASS.

Official Docker validation on 2026-05-10:

- Command:

```text
timeout 20m ./scripts/run_official_autotest.sh /home/lenovo/oscomp-official-env /home/lenovo/projects/uestc-kernel
```

- Result: wrapper completed without the previous RISC-V QEMU hang.
- `docker_evaluate.log` reported:

```text
{"verdict": "Accpted", "score": "0", ...}
```

- `console_log` progressed beyond QEMU launch into official scoring/parsing lines for `os_serial_out_rv.txt`.
- No stale `qemu-system-riscv64` process remained after the run.

## Evidence Files

Fresh copied evidence for this batch:

```text
.repair_logs/v194_official_qemu_runtime_evidence/docker_evaluate.log
.repair_logs/v194_official_qemu_runtime_evidence/console_log
.repair_logs/v194_official_qemu_runtime_evidence/os_serial_out_rv.txt
.repair_logs/v194_official_qemu_runtime_evidence/os_serial_out_la.txt
```

The RISC-V serial evidence ends with:

```text
[ucompat-v194] userland abi hardening PASS
[external-init-v82] smoke passed
[external-init-v82] kernel idle after external init ELF smoke
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

The local `apply_fix.sh` validator creates a fresh per-run QEMU log under:

```text
.repair_logs/v194_official_qemu_runtime_compat_<timestamp>/official_riscv_qemu.log
```

and checks the required v151k7 through v194 markers plus the official shutdown diagnostic.
