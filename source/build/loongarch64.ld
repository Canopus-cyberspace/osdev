OUTPUT_ARCH(loongarch64)
ENTRY(_start)

SECTIONS
{
  . = 0x90000000;
  __kernel_start = .;

  .text : ALIGN(4K) {
    __text_start = .;
    *(.text.entry)
    *(.text .text.*)
    __text_end = .;
  }

  .rodata : ALIGN(4K) {
    __rodata_start = .;
    *(.rodata .rodata.*)
    __rodata_end = .;
  }

  .data : ALIGN(4K) {
    __data_start = .;
    *(.sdata .sdata.*)
    *(.data .data.*)
    __data_end = .;
  }

  .bss : ALIGN(4K) {
    __bss_start = .;
    *(.sbss .sbss.*)
    *(.bss .bss.*)
    *(COMMON)
    __bss_end = .;
  }

  __kernel_end = .;

  /DISCARD/ : {
    *(.eh_frame)
    *(.comment)
  }
}
