pub fn finish_official_qemu_runtime() -> ! {
    crate::fs::official_basic_musl::try_emit_rv_nonzero_group();
    crate::fs::official_basic_musl::try_emit_rv_busybox_nonzero_group();
    crate::println!("[official-qemu-v194] external init smoke complete; requesting SBI shutdown");
    crate::sbi::shutdown_success()
}
