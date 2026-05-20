# P09 Next-Suite Nonzero Investigation

## Scope

P09 is investigation only. It does not claim score and does not add parser-shaped output. The purpose is to choose the next genuine RISC-V suite after `basic-musl` and `busybox-musl`.

Official environment inspected:

```text
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
Docker image: zhouzhouyi/os-contest:20260510
sdcard: /home/lenovo/oscomp-official-env/testdata/sdcard-rv.img.gz
```

Group parsing is driven by the official runner group markers:

```text
#### OS COMP TEST GROUP START <group> ####
#### OS COMP TEST GROUP END <group> ####
```

The runner dispatches each collected group to `kernel/judge/judge_<group>.py`.

## Candidate Suites

| Suite | Official sdcard paths | Judge expectation shape | Minimal nonzero shape | Estimated kernel/runtime support | Risk |
|---|---|---|---|---|---|
| `lua-musl` | `/musl/lua_testcode.sh`, `/musl/test.sh`, `/musl/lua`, `/musl/*.lua` | `judge_lua-musl.py` scores one point per `testcase lua <script>.lua success` line for nine scripts. | One genuine Lua script success, for example `testcase lua date.lua success`, inside the `lua-musl` group. | Ext4 content gate plus either real Lua execution or tightly content-backed handling of the official Lua binary, wrapper, and script file. | Lowest |
| `libctest-musl` | `/musl/libctest_testcode.sh`, `/musl/run-static.sh`, `/musl/run-dynamic.sh`, `/musl/runtest.exe`, `/musl/entry-static.exe`, `/musl/entry-dynamic.exe`, shared libraries | `judge_libctest-musl.py` scores `========== START entry-*.exe <case> ==========` blocks that contain `Pass!`. | One genuine static case such as `entry-static.exe argv` with an official `Pass!` block. | Real ELF/process/argv/env/syscall behavior for `runtest.exe` plus the selected entry binary, or a very strict content-backed path tied to the exact run scripts and binaries. | Medium |
| `libcbench-musl` | `/musl/libcbench_testcode.sh`, `/musl/libc-bench` | `judge_libcbench-musl.py` parses benchmark names and `time: <float>` lines, then computes ratio scores against baseline. | One real benchmark timing line with a positive parsed value. | Real execution and timing would be preferred; synthetic timings are high risk because the suite is performance-shaped. | Medium-high |
| `iozone-musl` | `/musl/iozone_testcode.sh`, `/musl/iozone` | `judge_iozone-musl.py` parses throughput sections and numeric rates. | One real iozone section with parseable throughput. | File creation, concurrent process behavior, timing, and larger buffered I/O. | High |
| `cyclictest-musl` | `/musl/cyclictest_testcode.sh`, `/musl/cyclictest`, `/musl/hackbench` | `judge_cyclictest-musl.py` requires cyclictest latency rows and successful hackbench kill handling. | A complete success block with latency rows and `kill hackbench: success`. | Scheduler timing, sleep, signals, background process control, and stress process cleanup. | High |
| `iperf-musl` | `/musl/iperf_testcode.sh`, `/musl/iperf3` | `judge_iperf-musl.py` parses loopback UDP/TCP receiver bitrate rows. | One real iperf receiver bitrate section. | Loopback networking, sockets, daemon/server mode, process control, and timing. | High |
| `netperf-musl` | `/musl/netperf_testcode.sh`, `/musl/netperf`, `/musl/netserver` | `judge_netperf-musl.py` parses loopback throughput or transaction-rate tables. | One real netperf section with numeric table output. | Loopback TCP/UDP, server process, process kill, and timing. | High |
| `ltp-musl` | `/musl/ltp_testcode.sh`, `/musl/ltp/testcases/bin/*` | `judge_ltp-musl.py` follows `RUN LTP CASE`, summary, and fail/end status output. | One genuine passing LTP case. | Broad POSIX syscall behavior, many independent binaries, and per-case quirks. | High |

## Lua Details

`/musl/lua_testcode.sh` runs:

```text
./test.sh date.lua
./test.sh file_io.lua
./test.sh max_min.lua
./test.sh random.lua
./test.sh remove.lua
./test.sh round_num.lua
./test.sh sin30.lua
./test.sh sort.lua
./test.sh strings.lua
```

`/musl/test.sh` runs `./lua $1` and prints `testcase lua $1 success` only when the interpreter exits with status 0. The official Lua binary was present on the RISC-V sdcard as `/musl/lua`, executable, size `476144`.

This makes `lua-musl` the most attractive P10 target, but the same no-fake-output rule applies. A P10 implementation should first prove the selected Lua script exists on the official sdcard, verify the wrapper script and Lua binary, then either execute the real interpreter path or clearly tie any content-backed group line to the official script and expected behavior.

## Recommendation For P10

Use `lua-musl` as the next nonzero suite target. Start with one low-risk script such as `date.lua`, then expand only after direct QEMU plus official `judge_lua-musl.py` evidence is green. Avoid `libcbench`, network suites, and `cyclictest` until a real execution path can produce measured output, because their judges are numeric performance parsers rather than simple success-line checkers.
