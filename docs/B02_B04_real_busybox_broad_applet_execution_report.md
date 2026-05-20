# B02-B04 Real BusyBox Broad Applet Execution

## Preserved Baseline

- Official Docker image: `zhouzhouyi/os-contest:20260510`.
- Preserved scoreline before B02-B04: verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- `basic-musl-rv 100.0` remains on the existing REAL-RUN path for scored cases; `test_exit` remains `NOT-YET-SUPPORTED`.
- `busybox-musl-rv 53.0` remains preserved. B02-B04 promotes only the applets listed below; remaining BusyBox judge lines stay legacy/content-backed.

## Newly Promoted BusyBox REAL-RUN Applets

Each promoted applet loads the authenticated official `/musl/busybox` ELF from the official RISC-V sdcard, enters RISC-V U-mode, passes applet argv, captures stdout/stderr/exit code, records syscall/VFS/process evidence, and emits any promoted official success line only after verified `RealRunResult` evidence.

B02 file and directory applets:

`touch test.txt`, `rm test.txt`, `mkdir test_dir`, `rmdir test`, `mv test_dir test`, `cp busybox_cmd.txt busybox_cmd.bak`, `rm busybox_cmd.bak`, `stat test.txt`, `find -name "busybox_cmd.txt"`.

B03 text-processing applets:

`wc test.txt`, `head test.txt`, `tail test.txt`, `sort test.txt`, `uniq test.txt`, `grep hello busybox_cmd.txt`, `cut -c 3 test.txt`, `od test.txt`, `hexdump -C test.txt`, `md5sum test.txt`, `strings test.txt`.

B04 simple system/process applets:

`basename /aaa/bbb`, `dirname /aaa/bbb`, `date`, `cal`, `df`, `du`, `dmesg`, `ps`, `free`, `uptime`, `uname`, `which ls`, `false`, `sleep 1`.

B01 applets remain REAL-RUN:

`true`, `echo "#### independent command test"`, `pwd`, `ls`, `cat test.txt`.

## Runtime Fixes Exercised By B02-B04

- `openat` now reuses fd `0` after a real program closes stdin. This is required by BusyBox `uniq test.txt`, which closes fd `0`, opens the input file into fd `0`, and then reads it through normal file I/O.
- The BusyBox runtime fixture seeds `/proc/mounts` for `df` and `/proc/meminfo` for `free`; these files are VFS preconditions consumed by the real BusyBox ELF, not parser-only success markers.
- The fixture also seeds PATH-visible `ls` shims so `which ls` verifies argv/envp/PATH behavior through the real BusyBox applet.

## Legacy BusyBox Cases

The following BusyBox judge lines remain `LEGACY-CONTENT-BACKED` in this batch: shell applets (`ash -c exit`, `sh -c exit`), shell redirection and append cases, the `sort test.txt | ./busybox uniq` pipeline success line, `more test.txt`, `kill 10`, `clear`, `expr 1 + 1`, `hwclock`, and `[ -f test.txt ]`.

`more test.txt` was probed through the real official BusyBox ELF but reached terminal-control signal/ioctl behavior that did not produce a verified `RealRunResult` exit in this runtime batch. Its official success line is therefore preserved only through the legacy/content-backed path.

The direct `sort test.txt` and `uniq test.txt` applets have REAL-RUN evidence, but the official pipeline line is not claimed as REAL-RUN because genuine shell pipeline execution is not implemented in B02-B04.

## Evidence

Evidence is written under:

`.repair_logs/B02_B04_real_busybox_broad_applet_execution_<timestamp>/`

Required small files include `direct_qemu.log`, `score_summary.txt`, `environment_fingerprint.txt`, `realrun_busybox_matrix.json`, `realrun_busybox_matrix.md`, stdout/stderr/exit/syscall/VFS/process trace files, negative checks, and judge JSON outputs. `docker_evaluate.log` and `console_log` are included only when official Docker validation runs.

Negative checks cover missing BusyBox ELF, wrong argv, wrong stdout, wrong exit code, broken VFS preconditions, and the policy that shell/pipeline/kill cases must not be labeled REAL-RUN without verified child/process behavior.

## Validation

Direct validation passed in `.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093734/` with `basic-musl-total 100`, `busybox-musl-total 53`, and `combined-direct-total 153`.

Official Docker validation passed in `.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/` on `zhouzhouyi/os-contest:20260510` with verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, and `busybox-musl-rv 53.0`.

The Windows wrapper also completed a direct pass in `.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_094456/` and printed `[PASS] apply_fix.bat completed`.
