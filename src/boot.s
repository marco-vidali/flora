.section ".text.boot"

.global _start
_start:
    // halt all the cores except core 0
    mrs x0, mpidr_el1
    and x0, x0, #0xFF
    cbnz x0, halt

    // set stack pointer address to 0x80000
    ldr x0, =_start
    mov sp, x0

    // load bss begin and end addresses
    ldr x0, =_bss_begin
    ldr x1, =_bss_end

bss_clear:
    cmp x0, x1 // check if current address is the same as end
    b.hs bss_done // if is same or higher, branch to bss_done
    str xzr, [x0], #8 // else, store a 0 in the current address and increment it by 8 bytes
    b bss_clear // branch again to bss_clear

bss_done:
    // branch to kernel_main
    bl kernel_main

halt:
    wfe // wait for event
    b halt