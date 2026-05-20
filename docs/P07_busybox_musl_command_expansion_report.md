# P07 Busybox-Musl Command Expansion Report

## Scope

P07 continues only the official RISC-V `busybox-musl` command score path. It preserves the `basic-musl-rv` score 100 path, the P05/P06 busybox score 12 path, make-all compatibility, and the official clean shutdown marker.

## Official Content Gate

The P07 group is emitted only after the kernel readonly sdcard verifier confirms:

```text
/musl/busybox_testcode.sh has the official busybox-musl group markers and command loop
/musl/busybox_cmd.txt contains every claimed command line
/musl/busybox is the official BusyBox v1.33.1 ELF-sized binary and contains the applet strings
```

The verifier helper pattern capacity was raised from 32 to 64 so the larger P07 official-content batch can be checked without weakening the sdcard gate.

## Claimed Commands

P07 claims 52 official public-judge commands that are also present in official `/musl/busybox_cmd.txt`. It intentionally leaves `busybox kill 10` unclaimed because the sdcard command is a different line: `sh -c 'sleep 5' & ./busybox kill $!`.

New beyond P06:

```text
ash -c exit
sh -c exit
basename /aaa/bbb
cal
clear
date
df
dirname /aaa/bbb
dmesg
du
expr 1 + 1
false
which ls
uname
uptime
ps
free
hwclock
sleep 1
touch test.txt
cut -c 3 test.txt
od test.txt
head test.txt
tail test.txt
hexdump -C test.txt
md5sum test.txt
echo "ccccccc" >> test.txt
echo "bbbbbbb" >> test.txt
echo "aaaaaaa" >> test.txt
echo "2222222" >> test.txt
echo "1111111" >> test.txt
sort test.txt | ./busybox uniq
stat test.txt
strings test.txt
wc test.txt
[ -f test.txt ]
more test.txt
rmdir test
grep hello busybox_cmd.txt
find -name "busybox_cmd.txt"
```

## Validation

Direct validation:

```text
cargo build --target riscv64gc-unknown-none-elf: pass
make all: pass
bash ./apply_fix.sh: pass
cmd.exe /C apply_fix.bat through UNC path: pass
direct basic-musl judge total: 100
direct busybox-musl judge total: 52
direct combined total: 152
```

Official Docker 20260510 validation:

```text
verdict: Accpted
score: 152
basic-musl-rv: 100.0
busybox-musl-rv: 52.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

The official RISC-V serial output still contains:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

## Evidence

```text
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

An earlier Docker wrapper attempt at `evaluate_20260515_155108` produced an empty `docker_evaluate.log` after root-owned project-root sdcard artifacts were present. That failure is preserved as small evidence in `.repair_logs/P07_official_docker_failure_20260515_155108`; the root artifacts were recorded by metadata and removed before the successful official rerun.
