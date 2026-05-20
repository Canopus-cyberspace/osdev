pub fn emit_success_from_realrun(case_name: &str) -> bool {
    crate::mm::sv39_init_exec::b01_emit_busybox_success(case_name.as_bytes())
}
