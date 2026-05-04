OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x10000000;

SECTIONS
{
  . = BASE_ADDRESS;

  .text : {
    *(.text .text.*)
  }

  .rodata : {
    *(.rodata .rodata.*)
  }

  .data : {
    *(.data .data.*)
  }

  .bss : {
    *(.bss .bss.*)
  }
}
