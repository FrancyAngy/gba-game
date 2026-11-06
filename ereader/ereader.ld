ENTRY(_start)
SECTIONS
{
  . = 0x02000000;
  .text   : { KEEP(*(.text._start)) *(.text .text.*) . = ALIGN(4); } = 0xff
  .rodata : { *(.rodata .rodata.*) . = ALIGN(4); } = 0xff
  .data   : { *(.data .data.*) . = ALIGN(4); } = 0xff
  .bss    : { __bss_start = .; *(.bss .bss.*) *(COMMON) . = ALIGN(4); __bss_end = .; }
  __end = .;
  /DISCARD/ : { *(.ARM.exidx*) *(.ARM.attributes) *(.comment) }
}
