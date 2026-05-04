.RECIPEPREFIX := >

KERNEL_ELF := target/riscv64gc-unknown-none-elf/debug/uestc-kernel

.PHONY: build run clean objdump

build:
>cargo +nightly build

run: build
>bash tools/run-qemu.sh $(KERNEL_ELF)

objdump: build
>rust-objdump -d $(KERNEL_ELF) | less

clean:
>cargo clean
