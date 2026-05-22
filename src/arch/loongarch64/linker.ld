OUTPUT_ARCH(loongarch)
ENTRY(_start)

SECTIONS
{
  . = 0x90000000;

  .text : {
    *(.text.entry)
    *(.text .text.*)
  }

  . = 0x90010000;
  .user : {
    *(.user.text .user.text.*)
    *(.user.rodata .user.rodata.*)
    . = ALIGN(4096);
    *(.user.bss .user.bss.*)
  }

  . = ALIGN(4096);
  .rodata : {
    *(.rodata .rodata.*)
  }

  . = ALIGN(4096);
  .bss : {
    *(.bss .bss.* COMMON)
  }
}
