# LAST_APPLIED_FIX

Version: v53f

Goal:
- Fix external init ELF regression after v53 scaffold additions.

Diagnosis:
- Serial log reached:
  - `[external-init-v50b] enter user sepc = 0x40000000`
  - `[external-init-v50b] enter user sp   = 0x40020000`
- Then the system re-entered `boot.S`.
- No user syscall trap handler output appeared.

Fix:
- Add `.balign 4` before `__sv39_init_v50b_alltraps`.
- Mask low two bits before writing `stvec` as a defensive measure.
- Keep external init ELF Sv39 U-mode smoke as regression.
