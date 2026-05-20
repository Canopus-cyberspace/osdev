# v158 历史实现接入设计

本报告不截断。v158 的目标是把当前仓库中已经稳定通过的历史实现模块接入一个统一 runtime bus，同时保留 v151k7/v154/v155/v156 回归，并对失败实验残留做显式清单化。

原则：

1. 稳定通过的历史模块接入 `src/fs/ucompat_history_v158.rs`。
2. v152/v152b 等已知失败实验不会被盲目启用，避免破坏当前基线。
3. `active_once()` 只作为验证入口，不承担主要内核逻辑。
4. 不重写 dispatcher/trap/主 syscall 路径。
5. 每次 build 后都做 forbidden warning gate 和 QEMU fresh runtime scan。
