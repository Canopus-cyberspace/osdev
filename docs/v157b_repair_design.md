# v157b StatusSnap / warning repair design

This repair keeps the v156/v157 direction intact and fixes the build break introduced by the first v157 package.

## Real repaired kernel semantics

- Preserves the v157 process / task / signal semantic module instead of disabling it.
- Adds delivered-signal state to `StatusSnap`, so the scenario can verify SIGCHLD delivery to the parent snapshot.
- Keeps process / task / session / process-group / waitpid / tgkill / exec-close-on-exec checks alive through the existing v157 module.
- Preserves v151k7, v154, v155 and v156 regression markers.
- Suppresses pre-existing generated-module unused-assignment warnings in v155/v156 so the strict warning gate remains clean.

## Safety policy

- No dispatcher/trap rewrite.
- No broad syscall implementation replacement.
- Text-level repair is run before executing `user/build_init_elf.py`.
- QEMU validation uses fresh binary-safe runtime logs only.
