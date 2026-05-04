.RECIPEPREFIX := >

KERNEL_ELF := target/riscv64gc-unknown-none-elf/debug/uestc-kernel
USER_ELF := user/user.elf
USER_BIN := user/user.bin

.PHONY: build run clean objdump user

user: $(USER_BIN)

$(USER_ELF): user/user.S user/user.ld
>riscv64-linux-gnu-gcc -nostdlib -static -mcmodel=medany -fno-pic -march=rv64gc -mabi=lp64d -T user/user.ld -o $(USER_ELF) user/user.S

$(USER_BIN): $(USER_ELF)
>riscv64-linux-gnu-objcopy -O binary $(USER_ELF) $(USER_BIN)

build: $(USER_BIN)
>cargo +nightly build

run: build
>bash tools/run-qemu.sh $(KERNEL_ELF)

objdump: build
>rust-objdump -d $(KERNEL_ELF) | less

clean:
>cargo clean
>rm -f $(USER_ELF) $(USER_BIN)
