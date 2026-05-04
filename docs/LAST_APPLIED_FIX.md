# LAST_APPLIED_FIX

Version: v50b

Goal:
- Fix external `init.elf` execution by replacing the minimal broken trap path with the robust TrapContext save/restore path.

Key fix:
- `sscratch` is restored to the trap stack before every `sret`, preventing repeated trap corruption after the first user ecall.
