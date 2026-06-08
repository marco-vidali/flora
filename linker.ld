ENTRY(_start) /* force linker to start from _start */

SECTIONS
{
    . = 0x80000; /* kernel starting point */

    .text.boot : { KEEP(*(.text.boot)) } /* preserve boot section code */
    .text : { *(.text .text.*) }
    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    
    /* 8-byte aligned bss section */
    . = ALIGN(0x8);
    _bss_begin = .;
    .bss : { *(.bss .bss.*) }
    . = ALIGN(0x8);
    _bss_end = .;

    /* discard metadata to place _start correctly */
    /DISCARD/ : {
        *(.eh_frame_hdr)
        *(.eh_frame)
        *(.comment)
    }
}