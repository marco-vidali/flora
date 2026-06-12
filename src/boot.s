.section ".text.boot"

// processor status configuration
.equ SPSR_MASK_ALL, (0b1111 << 6) // disable DAIF bits to prevent crashes
.equ SPSR_EL1H, (0b0100 << 0) // set EL1 as exception level and its stack pointer
.equ SPSR_VALUE, (SPSR_MASK_ALL | SPSR_EL1H)

// hypervisor configuration
.equ HCR_RW, (1 << 31) // set EL2 into 64 bit mode
.equ HCR_VALUE, HCR_RW

// EL2, EL1 and EL0 configuration
.equ SCR_RESERVED, (0b11 << 4) // set reserved bits
.equ SCR_RW, (1 << 10) // set EL2 and EL1 into 64 bit mode
.equ SCR_NS, (1 << 0) // enable non-secure mode
.equ SCR_VALUE, (SCR_RESERVED | SCR_RW | SCR_NS)

.global _start
_start:
    // halt all the cores except core 0
    mrs x0, mpidr_el1
    and x0, x0, #0xFF
    cbnz x0, halt

    // load calculated exception level registers to boot into EL1
    ldr x0, =SPSR_VALUE
    msr spsr_el3, x0

    ldr x0, =HCR_VALUE
    msr hcr_el2, x0

    ldr x0, =SCR_VALUE
    msr scr_el3, x0

    // configure EL1 return address
    adr x0, el1_entry
    msr elr_el3, x0

    // change exception level
    eret

el1_entry:
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