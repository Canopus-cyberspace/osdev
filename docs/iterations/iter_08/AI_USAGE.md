# Iteration 08 AI Usage

AI assistance was used to inspect the existing LoongArch ELF/user-memory paths, implement the focused LoongArch MMU/TLB module, wire fixed ET_EXEC mappings into the BusyBox probe, run local validation, and summarize the evidence.

Accepted AI changes:

- New `user_mmu.rs` module for LoongArch CSR, DMW, TLB flush, and huge-page TLB install helpers.
- Fixed-address ET_EXEC user virtual mapping metadata in `real_elf.rs`.
- Mapping activation and deactivation helpers around the non-scoring BusyBox probe.
- BusyBox probe evidence showing real `/musl/busybox true` exits through the LoongArch trap path.
- Iteration documentation and concise validation excerpts.

Rejected or deferred suggestions:

- Official BusyBox group markers were not emitted.
- BusyBox command success text was not hard-coded.
- Broader BusyBox commands were not kept enabled after development probing showed they need more bounded execution work.
- A broad `runtime_dispatch.rs` rewrite was not made.

Human/environment verification is still required for an updated official score because Docker was unavailable locally.
