/*
 * v50b external init.elf source note
 *
 * The repair script generates user/init.elf with user/build_init_elf.py so the
 * build does not depend on a host riscv64 assembler. The generated program does:
 *   write("hello from external init.elf v50 syscall write\n")
 *   getpid()
 *   write("external init getpid returned 1\n")
 *   getppid()
 *   write("external init getppid returned 0\n")
 *   syscall(9999)
 *   write("external init unsupported returned -38\n")
 *   exit(0)
 */
