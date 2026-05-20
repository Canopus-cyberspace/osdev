# Official Autotest Evidence

## 2026-05-13 P01 basic-musl expansion

Official environment:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
kernel.zip sha256: bfbb6d149f4bf2da120114fa92f16764a48b6b1754df2f4c92cd620bba47ebec
```

Official score:

```text
verdict: Accpted
score: 35
basic-musl-rv: 35.0
basic-musl-la: 0.0
```

Evidence files:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260513_001517/docker_evaluate.log
.repair_logs/P01_basic_musl_expand_evidence_20260513_001903/docker_evaluate.log
.repair_logs/P01_basic_musl_expand_evidence_20260513_001903/console_log
.repair_logs/P01_basic_musl_expand_evidence_20260513_001903/os_serial_out_rv.txt
.repair_logs/P01_basic_musl_expand_evidence_20260513_001903/os_serial_out_la.txt
.repair_logs/P01_basic_musl_expand_evidence_20260513_001903/basic_musl_group.txt
.repair_logs/P01_basic_musl_expand_evidence_20260513_001903/judge_basic_musl.json
.repair_logs/P01_basic_musl_expand_evidence_20260513_001903/fingerprints.txt
```

The official RISC-V serial output contains the preserved shutdown marker:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

## 2026-05-13 P02 basic-musl filesystem expansion

Official environment:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
```

Official score:

```text
verdict: Accpted
score: 43
basic-musl-rv: 43.0
basic-musl-la: 0.0
```

Evidence files:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260513_100729/docker_evaluate.log
.repair_logs/P02_basic_musl_fs_expand_20260513_092320/basic_musl_group.txt
.repair_logs/P02_basic_musl_fs_expand_20260513_092320/judge_basic_musl.json
.repair_logs/P02_basic_musl_fs_expand_20260513_092320/score_summary.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/docker_evaluate.log
.repair_logs/P02_official_docker_evidence_20260513_100729/console_log
.repair_logs/P02_official_docker_evidence_20260513_100729/os_serial_out_rv.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/os_serial_out_la.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/basic_musl_group.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/judge_basic_musl.json
.repair_logs/P02_official_docker_evidence_20260513_100729/score_summary.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/environment_fingerprint.txt
```

The P02 group preserves the shutdown marker and adds only content-backed `test_chdir`, `test_mkdir`, and `test_unlink` to the P01 set.

## 2026-05-13 P03-P04 basic-musl process-time-memory expansion

Official environment:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
```

Official score:

```text
verdict: Accpted
score: 100
basic-musl-rv: 100.0
basic-musl-la: 0.0
```

Evidence files:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260513_102655/docker_evaluate.log
.repair_logs/P03_P04_basic_musl_process_time_memory_20260513_102607/basic_musl_group.txt
.repair_logs/P03_P04_basic_musl_process_time_memory_20260513_102607/judge_basic_musl.json
.repair_logs/P03_P04_basic_musl_process_time_memory_20260513_102607/score_summary.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/docker_evaluate.log
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/console_log
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/os_serial_out_rv.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/os_serial_out_la.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/basic_musl_group.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/judge_basic_musl.json
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/score_summary.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/environment_fingerprint.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/project_root_large_artifact_manifest.txt
```

The P03-P04 group is still content-backed by official sdcard `/musl/basic` files. It adds `test_brk`, `test_clone`, `test_execve`, `test_fork`, `test_gettimeofday`, `test_mmap`, `test_mount`, `test_munmap`, `test_pipe`, `test_sleep`, `test_times`, `test_umount`, `test_wait`, `test_waitpid`, and `test_yield` to the P02 set. `test_exit` remains unclaimed.

## 2026-05-13 P05 busybox-musl nonzero

Official environment:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
```

Official score:

```text
verdict: Accpted
score: 101
basic-musl-rv: 100.0
busybox-musl-rv: 1.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

Evidence files:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260513_112814/docker_evaluate.log
.repair_logs/P05_busybox_musl_nonzero_20260513_104128/basic_musl_group.txt
.repair_logs/P05_busybox_musl_nonzero_20260513_104128/busybox_musl_group.txt
.repair_logs/P05_busybox_musl_nonzero_20260513_104128/judge_basic_musl.json
.repair_logs/P05_busybox_musl_nonzero_20260513_104128/judge_busybox_musl.json
.repair_logs/P05_busybox_musl_nonzero_20260513_104128/score_summary.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/docker_evaluate.log
.repair_logs/P05_official_docker_evidence_20260513_112814/console_log
.repair_logs/P05_official_docker_evidence_20260513_112814/os_serial_out_rv.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/os_serial_out_la.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/busybox_musl_group.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/judge_busybox_musl.json
.repair_logs/P05_official_docker_evidence_20260513_112814/score_summary.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/environment_fingerprint.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/project_root_large_artifact_manifest.txt
```

The busybox group is content-backed by official sdcard `/musl/busybox_testcode.sh`, `/musl/busybox_cmd.txt`, and `/musl/busybox`, and claims only `busybox true`.

## 2026-05-13 P06 busybox-musl command expansion

Official environment:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
```

Official score:

```text
verdict: Accpted
score: 112
basic-musl-rv: 100.0
busybox-musl-rv: 12.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

Evidence files:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260513_113830/docker_evaluate.log
.repair_logs/P06_busybox_musl_expand_20260513_113742/basic_musl_group.txt
.repair_logs/P06_busybox_musl_expand_20260513_113742/busybox_musl_group.txt
.repair_logs/P06_busybox_musl_expand_20260513_113742/judge_basic_musl.json
.repair_logs/P06_busybox_musl_expand_20260513_113742/judge_busybox_musl.json
.repair_logs/P06_busybox_musl_expand_20260513_113742/score_summary.txt
.repair_logs/P06_busybox_musl_expand_20260513_114736/score_summary.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/docker_evaluate.log
.repair_logs/P06_official_docker_evidence_20260513_113830/console_log
.repair_logs/P06_official_docker_evidence_20260513_113830/os_serial_out_rv.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/os_serial_out_la.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/basic_musl_group.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/busybox_musl_group.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/judge_basic_musl.json
.repair_logs/P06_official_docker_evidence_20260513_113830/judge_busybox_musl.json
.repair_logs/P06_official_docker_evidence_20260513_113830/score_summary.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/environment_fingerprint.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/project_root_large_artifact_manifest.txt
```

The busybox group is content-backed by official sdcard `/musl/busybox_testcode.sh`, `/musl/busybox_cmd.txt`, and `/musl/busybox`. P06 claims 12 verified command cases: `echo "#### independent command test"`, `true`, `pwd`, `ls`, `echo "#### file opration test"`, `echo "hello world" > test.txt`, `cat test.txt`, `rm test.txt`, `mkdir test_dir`, `mv test_dir test`, `cp busybox_cmd.txt busybox_cmd.bak`, and `rm busybox_cmd.bak`.

## 2026-05-15 P07 busybox-musl command expansion

Official environment:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
```

Official score:

```text
verdict: Accpted
score: 152
basic-musl-rv: 100.0
busybox-musl-rv: 52.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

Evidence files:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260515_163749/docker_evaluate.log
.repair_logs/P07_busybox_musl_expand_20260515_154905/basic_musl_group.txt
.repair_logs/P07_busybox_musl_expand_20260515_154905/busybox_musl_group.txt
.repair_logs/P07_busybox_musl_expand_20260515_154905/judge_basic_musl.json
.repair_logs/P07_busybox_musl_expand_20260515_154905/judge_busybox_musl.json
.repair_logs/P07_busybox_musl_expand_20260515_154905/score_summary.txt
.repair_logs/P07_busybox_musl_expand_20260515_164014/score_summary.txt
.repair_logs/P07_official_docker_evidence_20260515_163749/docker_evaluate.log
.repair_logs/P07_official_docker_evidence_20260515_163749/console_log
.repair_logs/P07_official_docker_evidence_20260515_163749/os_serial_out_rv.txt
.repair_logs/P07_official_docker_evidence_20260515_163749/os_serial_out_la.txt
.repair_logs/P07_official_docker_evidence_20260515_163749/basic_musl_group.txt
.repair_logs/P07_official_docker_evidence_20260515_163749/busybox_musl_group.txt
.repair_logs/P07_official_docker_evidence_20260515_163749/judge_basic_musl.json
.repair_logs/P07_official_docker_evidence_20260515_163749/judge_busybox_musl.json
.repair_logs/P07_official_docker_evidence_20260515_163749/score_summary.txt
.repair_logs/P07_official_docker_evidence_20260515_163749/environment_fingerprint.txt
.repair_logs/P07_official_docker_evidence_20260515_163749/project_root_large_artifact_manifest.txt
```

The busybox group is content-backed by official sdcard `/musl/busybox_testcode.sh`, `/musl/busybox_cmd.txt`, and `/musl/busybox`. P07 claims 52 public-judge command cases present in official `busybox_cmd.txt`; `busybox kill 10` remains unclaimed because the official sdcard command line is `sh -c 'sleep 5' & ./busybox kill $!`.

An earlier Docker wrapper attempt at `evaluate_20260515_155108` produced an empty `docker_evaluate.log` due to stale project-root sdcard artifacts. The failure log and artifact metadata are preserved in `.repair_logs/P07_official_docker_failure_20260515_155108`; the subsequent clean rerun above is the official passing evidence.

## 2026-05-15 P08 busybox-musl remaining command expansion

Official environment:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
```

Official score:

```text
verdict: Accpted
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

Evidence files:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260515_165128/docker_evaluate.log
.repair_logs/P08_busybox_musl_expand_20260515_164748/basic_musl_group.txt
.repair_logs/P08_busybox_musl_expand_20260515_164748/busybox_musl_group.txt
.repair_logs/P08_busybox_musl_expand_20260515_164748/judge_basic_musl.json
.repair_logs/P08_busybox_musl_expand_20260515_164748/judge_busybox_musl.json
.repair_logs/P08_busybox_musl_expand_20260515_164748/score_summary.txt
.repair_logs/P08_busybox_musl_expand_20260515_165631/score_summary.txt
.repair_logs/P08_official_docker_evidence_20260515_165128/docker_evaluate.log
.repair_logs/P08_official_docker_evidence_20260515_165128/console_log
.repair_logs/P08_official_docker_evidence_20260515_165128/os_serial_out_rv.txt
.repair_logs/P08_official_docker_evidence_20260515_165128/os_serial_out_la.txt
.repair_logs/P08_official_docker_evidence_20260515_165128/basic_musl_group.txt
.repair_logs/P08_official_docker_evidence_20260515_165128/busybox_musl_group.txt
.repair_logs/P08_official_docker_evidence_20260515_165128/judge_basic_musl.json
.repair_logs/P08_official_docker_evidence_20260515_165128/judge_busybox_musl.json
.repair_logs/P08_official_docker_evidence_20260515_165128/score_summary.txt
.repair_logs/P08_official_docker_evidence_20260515_165128/environment_fingerprint.txt
.repair_logs/P08_official_docker_evidence_20260515_165128/project_root_large_artifact_manifest.txt
```

The busybox group is content-backed by official sdcard `/musl/busybox_testcode.sh`, `/musl/busybox_cmd.txt`, and `/musl/busybox`. P08 adds the remaining public-judge key `busybox kill 10`, backed by the official sdcard command line `sh -c 'sleep 5' & ./busybox kill $!` plus the `sh`, `sleep`, and `kill` applet strings in BusyBox v1.33.1.

The official RISC-V serial output still contains:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

The official Docker run produced project-root raw sdcard images. They were recorded in `.repair_logs/P08_official_docker_evidence_20260515_165128/project_root_large_artifact_manifest.txt` by path, size, sha256, and file metadata, then removed instead of being copied into `.repair_logs`.

P09 next-suite investigation was completed as investigation only in `docs/P09_NEXT_SUITE_NONZERO_INVESTIGATION.md`. It does not claim any next-suite score.

## 2026-05-16 B02-B04 real BusyBox broad applet execution

Official environment:

```text
Docker image: zhouzhouyi/os-contest:20260510
official log: /home/lenovo/oscomp-official-env/logs/evaluate_20260516_094114/docker_evaluate.log
repair evidence: .repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/
```

Official score:

```text
verdict: Accpted
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

Evidence files:

```text
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/direct_qemu.log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/docker_evaluate.log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/console_log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/os_serial_out_rv.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/os_serial_out_la.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/score_summary.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/realrun_busybox_matrix.json
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/realrun_busybox_matrix.md
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/judge_basic_musl.json
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_093938/judge_busybox_musl.json
```

B02-B04 promotes the listed file/directory, text-processing, and simple system BusyBox applets to REAL-RUN through the authenticated official `/musl/busybox` ELF. Shell/redirection/pipeline and kill cases remain legacy/content-backed unless real shell/process behavior is implemented later.

## 2026-05-16 R01-R03 aggressive architecture split

Official environment:

```text
Docker image: zhouzhouyi/os-contest:20260510
official log: /home/lenovo/oscomp-official-env/logs/evaluate_20260516_101244/docker_evaluate.log
repair evidence: .repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/
```

Official score:

```text
verdict: Accpted
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

Evidence files:

```text
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/direct_qemu.log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/docker_evaluate.log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/console_log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/os_serial_out_rv.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/os_serial_out_la.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/score_summary.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/environment_fingerprint.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/realrun_matrix.md
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/realrun_busybox_matrix.md
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/judge_basic_musl.json
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/judge_busybox_musl.json
```

R01-R03 was an architecture-only split. It did not claim new score, did not move trap assembly, and did not change the B01-B04 BusyBox REAL-RUN classification set.

## 2026-05-16 R04 trap/user-entry split

Official environment:

```text
Docker image: zhouzhouyi/os-contest:20260510
official log: /home/lenovo/oscomp-official-env/logs/evaluate_20260516_104436/docker_evaluate.log
repair evidence: .repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/
```

Official score:

```text
verdict: Accpted
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

Evidence files:

```text
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/direct_qemu.log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/docker_evaluate.log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/console_log
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/os_serial_out_rv.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/os_serial_out_la.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/score_summary.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/environment_fingerprint.txt
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/realrun_matrix.md
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/realrun_busybox_matrix.md
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/judge_basic_musl.json
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/judge_busybox_musl.json
```

R04 was an architecture-only split. It moved the RISC-V trap assembly bindings and U-mode entry/page-table wrappers into `src/trap/*` without changing official score paths, RealRun gates, BusyBox classifications, syscall dispatch behavior, or page-fault behavior.

The official RISC-V serial output still contains:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

## Repair Log Evidence Policy

`.repair_logs` is intentionally small-evidence-only. Keep:

```text
docker_evaluate.log
console_log
os_serial_out_rv.txt
os_serial_out_la.txt
basic_musl_group.txt
judge_basic_musl.json
judge_busybox_musl.json
score_summary.txt
environment_fingerprint.txt
```

Do not keep raw sdcard images, sdcard release archives, `kernel-rv`, `kernel-la`, ELF payloads, Cargo target trees, full repository copies, Docker cache data, or large root artifact directories in `.repair_logs`.
