ENTRY(_start) /* force linker to start from _start */

SECTIONS
{
    . = 0x80000; /* kernel starting point */

    .text.boot : { KEEP(*(.text.boot)) } /* preserve boot section code */
    .text : { *(.text) }
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    
    /* 8-byte aligned bss section */
    . = ALIGN(0x8);
    bss_begin = .;
    .bss : { *(.bss*) }
    . = ALIGN(0x8);
    bss_end = .;
}