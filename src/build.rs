// Auto-repaired by v82j: Cargo build scripts must define fn main().
fn main() {
    println!("cargo:rerun-if-changed=user/init.elf");
    println!("cargo:rerun-if-changed=user/build_init_elf.py");
}
