//! Official marker output stays isolated here.
//!
//! User stdout/stderr bytes from syscall `write(64)` are routed through
//! `official::user_output`, not through this marker-facing module.
