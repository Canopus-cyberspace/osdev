.RECIPEPREFIX := >

KERNEL_ELF := target/riscv64gc-unknown-none-elf/debug/uestc-kernel
KERNEL_BIN := target/riscv64gc-unknown-none-elf/debug/uestc-kernel.bin

.PHONY: build run clean objdump bin

build:
>cargo +nightly build

bin: build
>rust-objcopy --strip-all -O binary $(KERNEL_ELF) $(KERNEL_BIN)

run: build
>bash tools/run-qemu.sh $(KERNEL_ELF)

objdump: build
>rust-objdump -d $(KERNEL_ELF) | less

clean:
>cargo clean
