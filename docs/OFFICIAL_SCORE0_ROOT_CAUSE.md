# Official Score 0 Root Cause

## Summary

The current official result has verdict `Accpted` and score `0` because the harness completed, but the kernel did not run any official test group output that the judges can score.

The RISC-V kernel now boots under the official QEMU command and shuts down cleanly. That fixed the previous silent QEMU runtime hang. However, it currently runs the internal v194/v214 compatibility path and exits after the external init smoke path. It does not discover the official sdcard filesystem, execute the official test scripts, or print official group markers.

## Current Evidence

The latest official `console_log` shows:

- `make all` completed.
- RISC-V QEMU launched with `kernel-rv`, `sdcard-rv.img`, virtio block, and virtio net.
- LoongArch QEMU launched with `kernel-la` and `sdcard-la.img`.
- The parser then evaluated every suite for `os_serial_out_rv.txt` and `os_serial_out_la.txt`.

The latest `docker_evaluate.log` shows:

```text
"verdict": "Accpted"
"score": "0"
```

and all rank entries are zero, including entries such as:

```text
basic-musl-rv: 0
busybox-musl-rv: 0
ltp-musl-rv: 0
basic-musl-la: 0
```

The RISC-V serial evidence contains internal compatibility markers and the clean shutdown line:

```text
[external-init-v82] smoke passed
[external-init-v82] kernel idle after external init ELF smoke
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

It does not contain any official group marker such as:

```text
#### OS COMP TEST GROUP START basic-musl ####
```

The LoongArch serial evidence reports that the current `kernel-la` is not loadable by LoongArch QEMU:

```text
qemu-system-loongarch64: could not load kernel 'kernel-la': Failed to load ELF
```

## Exact Root Cause

The official parser only feeds lines inside group markers to the judges:

```text
#### OS COMP TEST GROUP START <group> ####
...
#### OS COMP TEST GROUP END <group> ####
```

When no group marker appears, the parser still invokes each judge script, but each judge receives empty input. Empty input yields zero score for every suite.

Therefore, the immediate RISC-V root cause is:

```text
kernel-rv does not yet run the official sdcard test scripts or otherwise produce genuine official test group output.
```

The immediate LoongArch root cause is:

```text
kernel-la is not a real LoongArch ELF and cannot be loaded by qemu-system-loongarch64.
```

## Why Verdict Accpted Can Still Score 0

`postwork.py` reports verdict `Accpted` after the harness finishes and writes the score table. It does not require any test score to be nonzero.

This explains the apparent contradiction:

- `Accpted`: official wrapper completed without an infrastructure-level failure.
- `score 0`: no judge suite saw expected official output.

## Non-Causes

The current score 0 is not caused by the previous missing `make all` target. The compile stage now completes.

It is not caused by the old silent RISC-V QEMU hang. The current RISC-V path reaches a visible clean shutdown.

It is not caused by `docker_evaluate.log` being empty. The latest successful wrapper run produced a populated `docker_evaluate.log`.

It is not caused by the official parser failing to run. `console_log` shows each suite being evaluated.

## Minimal Condition For Nonzero RISC-V Score

The minimum official RISC-V requirement for nonzero score is:

1. Boot `kernel-rv` under the exact official QEMU command.
2. Read and execute real official test content from `sdcard-rv.img`.
3. Print at least one valid official test group:

```text
#### OS COMP TEST GROUP START basic-musl ####
...
#### OS COMP TEST GROUP END basic-musl ####
```

4. Inside that group, produce genuine output matching at least one judge expectation.

For example, the `basic-musl` judge awards points per subtest. A correctly bracketed run of the real `/musl/basic/write` program can earn points if it prints the expected basic section and body text:

```text
========== START write ==========
Hello operating system contest.
========== END write ==========
```

This output must come from the real compatibility path or real executed official program behavior. It must not be fabricated solely to satisfy the judge.

## LoongArch Impact

LoongArch is included in the official rank table and affects the maximum possible total score. However, RISC-V and LoongArch scores are independent entries. A real RISC-V official path can produce a nonzero score while LoongArch remains zero.

The next practical milestone is therefore nonzero RISC-V score. Real LoongArch support can be planned later unless the submission target requires full dual-architecture scoring.

