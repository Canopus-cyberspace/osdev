OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

PHDRS
{
  text PT_LOAD FLAGS(5);
  rodata PT_LOAD FLAGS(4);
  data PT_LOAD FLAGS(6);
}

SECTIONS
{
  . = BASE_ADDRESS;

  skernel = .;

  .text : {
    stext = .;
    *(.text.entry)
    *(.text .text.*)
    etext = .;
  } :text

  . = ALIGN(4096);
  .rodata : {
    srodata = .;
    *(.rodata .rodata.*)
    erodata = .;
  } :rodata

  . = ALIGN(4096);
  .data : {
    sdata = .;
    *(.data .data.*)
    edata = .;
  } :data

  .bss : {
    sbss = .;
    *(.bss .bss.*)
    ebss = .;
  } :data

  . = ALIGN(4096);
  .boot_stack (NOLOAD) : {
    *(.boot_stack)
  } :data

  . = ALIGN(4096);
  .trap_stack (NOLOAD) : {
    *(.trap_stack)
  } :data

  ekernel = .;
}
