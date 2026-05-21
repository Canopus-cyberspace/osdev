# AI Usage

AI was used for code inspection, implementation planning, LoongArch-local syscall changes, and validation summarization. All enabled user-visible basic-musl cases remain real PLV3 ELF executions loaded from the official sdcard image.

## Iteration 02

AI was used to inspect the LoongArch trap entry, implement a dedicated trap stack switch, preserve user register state across the new entry sequence, and document validation results.

No AI-suggested fake output, parser-shaped success text, or broad runtime dispatcher rewrite was accepted. Official score refresh still requires human or environment validation because Docker was unavailable.
