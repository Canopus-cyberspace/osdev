use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    emit("cargo:rerun-if-changed=build/riscv64.ld");
    emit("cargo:rerun-if-changed=build/loongarch64.ld");

    let target = env::var("TARGET").unwrap_or_default();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    match target.as_str() {
        "riscv64gc-unknown-none-elf" => link_kernel(
            "source-riscv64-kernel",
            manifest_dir.join("build").join("riscv64.ld"),
        ),
        "loongarch64-unknown-none" => link_kernel(
            "source-loongarch64-kernel",
            manifest_dir.join("build").join("loongarch64.ld"),
        ),
        _ => {}
    }
}

fn link_kernel(bin: &str, linker_script: PathBuf) {
    emit(&format!(
        "cargo:rustc-link-arg-bin={bin}=-T{}",
        linker_script.display(),
    ));
}

fn emit(line: &str) {
    let mut out = io::stdout().lock();
    out.write_all(line.as_bytes()).unwrap();
    out.write_all(b"\n").unwrap();
}
