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

  . = ALIGN(8);
  .rodata : {
    srodata = .;
    *(.rodata .rodata.*)
    erodata = .;
  } :rodata

  . = ALIGN(8);
  .data : {
    sdata = .;
    *(.data .data.*)
    edata = .;
  } :data

  . = ALIGN(8);
  .bss (NOLOAD) : {
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
