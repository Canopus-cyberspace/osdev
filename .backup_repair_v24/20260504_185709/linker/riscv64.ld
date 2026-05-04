OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
  . = BASE_ADDRESS;

  skernel = .;

  .text : {
    stext = .;
    *(.text.entry)
    *(.text .text.*)
    etext = .;
  }

  .rodata : {
    srodata = .;
    *(.rodata .rodata.*)
    erodata = .;
  }

  .data : {
    sdata = .;
    *(.data .data.*)
    edata = .;
  }

  .bss : {
    sbss = .;
    *(.bss .bss.*)
    ebss = .;
  }

  . = ALIGN(4096);
  .boot_stack (NOLOAD) : {
    *(.boot_stack)
  }

  . = ALIGN(4096);
  .trap_stack (NOLOAD) : {
    *(.trap_stack)
  }

  ekernel = .;
}
