.macro kernel_entry
	sub	sp, sp, #256 // lower stack pointer by 256 bytes

	// back up all registers in lowered stack
	stp	x0, x1, [sp, #16 * 0]
	stp	x2, x3, [sp, #16 * 1]
	stp	x4, x5, [sp, #16 * 2]
	stp	x6, x7, [sp, #16 * 3]
	stp	x8, x9, [sp, #16 * 4]
	stp	x10, x11, [sp, #16 * 5]
	stp	x12, x13, [sp, #16 * 6]
	stp	x14, x15, [sp, #16 * 7]
	stp	x16, x17, [sp, #16 * 8]
	stp	x18, x19, [sp, #16 * 9]
	stp	x20, x21, [sp, #16 * 10]
	stp	x22, x23, [sp, #16 * 11]
	stp	x24, x25, [sp, #16 * 12]
	stp	x26, x27, [sp, #16 * 13]
	stp	x28, x29, [sp, #16 * 14]
    str x30, [sp, #16 * 15]
.endm

.macro kernel_exit
	// reset all registers
	ldp	x0, x1, [sp, #16 * 0]
	ldp	x2, x3, [sp, #16 * 1]
	ldp	x4, x5, [sp, #16 * 2]
	ldp	x6, x7, [sp, #16 * 3]
	ldp	x8, x9, [sp, #16 * 4]
	ldp	x10, x11, [sp, #16 * 5]
	ldp	x12, x13, [sp, #16 * 6]
	ldp	x14, x15, [sp, #16 * 7]
	ldp	x16, x17, [sp, #16 * 8]
	ldp	x18, x19, [sp, #16 * 9]
	ldp	x20, x21, [sp, #16 * 10]
	ldp	x22, x23, [sp, #16 * 11]
	ldp	x24, x25, [sp, #16 * 12]
	ldp	x26, x27, [sp, #16 * 13]
	ldp	x28, x29, [sp, #16 * 14]
    ldr x30, [sp, #16 * 15]

	add	sp, sp, #256 // reset stack pointer raising it by 256 bytes

	eret // go back and continue the interrupted task
.endm

.macro handle_invalid_entry type
    kernel_entry // save kernel state
	
    mov x0, #\type // store entry type
    mrs x1, esr_el1 // store error
    mrs x2, elr_el1 // store the instruction address that crashed the kernel

    bl show_invalid_entry_message // branch to rust to manage invalid entry

    b err_halt // branch to err_halt
.endm

.macro ventry label
.align 7 // align table entries by 128 bytes
    b \label // branch to label
.endm

.align 11 // align table by 2048 bytes
.global vectors 
vectors:
	// exception in EL1 with EL10 stack
	ventry sync_invalid_el1t // Synchronous EL1t
	ventry irq_invalid_el1t // IRQ EL1t
	ventry fiq_invalid_el1t // FIQ EL1t
	ventry error_invalid_el1t // Error EL1t

	// exception in EL1 with EL1 stack
	ventry sync_invalid_el1h // Synchronous EL1h
	ventry handle_el1_irq // IRQ EL1h
	ventry fiq_invalid_el1h // FIQ EL1h
	ventry error_invalid_el1h // Error EL1h

	// exception in 64 bit user space
	ventry sync_invalid_el0_64 // Synchronous 64-bit EL0
	ventry irq_invalid_el0_64 // IRQ 64-bit EL0
	ventry fiq_invalid_el0_64 // FIQ 64-bit EL0
	ventry error_invalid_el0_64 // Error 64-bit EL0

	// exception in 32 bit user space
	ventry sync_invalid_el0_32 // Synchronous 32-bit EL0
	ventry irq_invalid_el0_32 // IRQ 32-bit EL0
	ventry fiq_invalid_el0_32 // FIQ 32-bit EL0
	ventry error_invalid_el0_32 // Error 32-bit EL0

sync_invalid_el1t:
	handle_invalid_entry 0

irq_invalid_el1t:
	handle_invalid_entry 1

fiq_invalid_el1t:
	handle_invalid_entry 2

error_invalid_el1t:
	handle_invalid_entry 3

sync_invalid_el1h:
	handle_invalid_entry 4

handle_el1_irq:
    kernel_entry // save kernel state
    bl handle_irq // branch to rust interrupt handler
    kernel_exit // restore kernel state

fiq_invalid_el1h:
	handle_invalid_entry 6

error_invalid_el1h:
	handle_invalid_entry 7

sync_invalid_el0_64:
	handle_invalid_entry 8

irq_invalid_el0_64:
	handle_invalid_entry 9

fiq_invalid_el0_64:
	handle_invalid_entry 10

error_invalid_el0_64:
	handle_invalid_entry 11

sync_invalid_el0_32:
	handle_invalid_entry 12

irq_invalid_el0_32:
	handle_invalid_entry 13

fiq_invalid_el0_32:
	handle_invalid_entry 14

error_invalid_el0_32:
	handle_invalid_entry 15

.global err_halt
err_halt:
	b err_halt // error loop