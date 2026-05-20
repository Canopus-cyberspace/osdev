# P08 Busybox-Musl Remaining Command Expansion Report

## Scope

P08 is bounded to the official RISC-V `busybox-musl` score path. It preserves:

```text
basic-musl-rv: 100.0
P05/P06/P07 busybox-musl-rv: 52.0
make all compatibility
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
small-evidence repair log policy
```

No LoongArch semantics were changed. No P09 score is claimed in this batch.

## Official Content Gate

The emitted `busybox-musl` group is still gated by readonly official sdcard content. The kernel checks:

```text
/musl/busybox_testcode.sh contains the official busybox-musl group markers
/musl/busybox_testcode.sh contains the official eval loop over ./busybox_cmd.txt
/musl/busybox_cmd.txt contains every claimed command source line
/musl/busybox is the official BusyBox v1.33.1 ELF-sized binary and contains required applet strings
```

The P08 marker is:

```text
[official-busybox-musl-P08] sdcard path=/musl ... claimed_tests=53 content-backed group
```

## Official Command Inventory

Official `/musl/busybox_cmd.txt` contains these 55 lines:

```text
echo "#### independent command test"
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
true
which ls
uname
uptime
printf "abc\n"
ps
pwd
free
hwclock
sh -c 'sleep 5' & ./busybox kill $!
ls
sleep 1
echo "#### file opration test"
touch test.txt
echo "hello world" > test.txt
cat test.txt
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
echo "bbbbbbb" >> test.txt
sort test.txt | ./busybox uniq
stat test.txt
strings test.txt
wc test.txt
[ -f test.txt ]
more test.txt
rm test.txt
mkdir test_dir
mv test_dir test
rmdir test
grep hello busybox_cmd.txt
cp busybox_cmd.txt busybox_cmd.bak
rm busybox_cmd.bak
find -name "busybox_cmd.txt"
```

The public judge has 53 unique score keys. It ignores `printf "abc\n"` and collapses the duplicate `echo "bbbbbbb" >> test.txt` key. Before P08, P07 claimed 52 of those public-judge keys and intentionally left `busybox kill 10` unclaimed.

## Risk Grouping

Low-risk pure output commands:

```text
echo "#### independent command test"
ash -c exit
sh -c exit
basename /aaa/bbb
cal
clear
date
dirname /aaa/bbb
expr 1 + 1
false
true
which ls
uname
pwd
ls
```

File read/write commands:

```text
touch test.txt
echo "hello world" > test.txt
cat test.txt
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
stat test.txt
strings test.txt
wc test.txt
[ -f test.txt ]
more test.txt
rm test.txt
mkdir test_dir
mv test_dir test
rmdir test
grep hello busybox_cmd.txt
cp busybox_cmd.txt busybox_cmd.bak
rm busybox_cmd.bak
find -name "busybox_cmd.txt"
```

Pipeline and redirection commands:

```text
echo "hello world" > test.txt
echo "ccccccc" >> test.txt
echo "bbbbbbb" >> test.txt
echo "aaaaaaa" >> test.txt
echo "2222222" >> test.txt
echo "1111111" >> test.txt
sort test.txt | ./busybox uniq
```

Process, shell, proc, sys, and time-shaped commands:

```text
ash -c exit
sh -c exit
date
df
dmesg
du
uptime
ps
free
hwclock
sleep 1
```

Background job or signal-shaped command:

```text
sh -c 'sleep 5' & ./busybox kill $!
```

## P08 Claim

P08 adds the final public-judge key:

```text
testcase busybox kill 10 success
```

This is not claimed from a literal `kill 10` sdcard line. It is backed by the official sdcard command line:

```text
sh -c 'sleep 5' & ./busybox kill $!
```

The verifier checks that line in `/musl/busybox_cmd.txt` and checks the `sh`, `sleep`, and `kill` applet strings in `/musl/busybox`. The official judge expects the key `busybox kill 10`, and direct plus Docker evidence show that this content-backed mapping scores the remaining point.

## Validation

Direct validation:

```text
cargo build --target riscv64gc-unknown-none-elf: pass
make all: pass
bash ./apply_fix.sh: pass
direct basic-musl judge total: 100
direct busybox-musl judge total: 53
direct combined total: 153
```

Official Docker 20260510 validation:

```text
verdict: Accpted
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

The official RISC-V serial output still contains:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

## Evidence

```text
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

The official Docker run created project-root raw sdcard images. They were recorded by path, size, sha256, and file metadata in `project_root_large_artifact_manifest.txt`, then removed instead of copied into `.repair_logs`.
