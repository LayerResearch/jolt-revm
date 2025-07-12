MEMORY {
  program (rwx) : ORIGIN = 0x80000000, LENGTH = 10485760
}

ENTRY(_start)

SECTIONS {

  .text.boot : {
    *(.text.boot)
  } > program

  .text : {
    *(.text .text.*)
  } > program

  .rodata : {
    *(.rodata .rodata.*)
  } > program

  .data : {
    *(.data .data.*)
  } > program

  .bss : {
    *(.bss .bss.*)
    *(COMMON)
  } > program

  .tohost : {
    *(.tohost)
  } > program

  .fromhost : {
    *(.fromhost)
  } > program

  . = ALIGN(8);
  _end = .;

  . = . + 4096;
  _STACK_PTR = .;

  . = ALIGN(8);
  _HEAP_PTR = .;
}
