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

    // branch to kernel_main
    bl kernel_main

halt:
    wfe // wait for event
    b halt