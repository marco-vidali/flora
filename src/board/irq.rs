use core::ptr;
use core::{arch::asm, fmt::Write};

use crate::debug;
use crate::drivers::MMIO_BASE;

const IRQ_BASE: usize = MMIO_BASE + 0x0000_B200;

const IRQ_ENABLE_0: *mut u32 = (IRQ_BASE + 0x10) as *mut u32;
const IRQ_PENDING_0: *mut u32 = (IRQ_BASE + 0x00) as *mut u32;

// error messages array
pub const ENTRY_ERROR_MESSAGES: [&str; 16] = [
    "SYNC_INVALID_EL1t",
    "IRQ_INVALID_EL1t",
    "FIQ_INVALID_EL1t",
    "ERROR_INVALID_EL1T",
    "SYNC_INVALID_EL1h",
    "IRQ_INVALID_EL1h",
    "FIQ_INVALID_EL1h",
    "ERROR_INVALID_EL1h",
    "SYNC_INVALID_EL0_64",
    "IRQ_INVALID_EL0_64",
    "FIQ_INVALID_EL0_64",
    "ERROR_INVALID_EL0_64",
    "SYNC_INVALID_EL0_32",
    "IRQ_INVALID_EL0_32",
    "FIQ_INVALID_EL0_32",
    "ERROR_INVALID_EL0_32",
];

#[unsafe(no_mangle)]
pub extern "C" fn show_invalid_entry_message(error_type: u32, esr: u64, address: u64) {
    let msg = ENTRY_ERROR_MESSAGES
        .get(error_type as usize)
        .unwrap_or(&"Unknown error");

    debug!(
        "[!] Error caught: {} - ESR: {:X} - Address: {:X}\n",
        msg, esr, address
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn handle_irq() {
    let irq: u32 = unsafe { ptr::read_volatile(IRQ_PENDING_0) };

    while irq != 0 {
        // ...
        break;
    }
}

pub struct IrqManager;

impl IrqManager {
    pub fn init() {
        // store vectors table address on vbar_el1
        unsafe {
            asm!(
                r"
                adr x0, vectors // store vectors table address in x0
                msr vbar_el1, x0 // write x0 to vector base address register
                ",
                out("x0") _
            );
        }

        // initialize hardware controller
        let mask = 0; // set which peripherals can generate interrupts

        unsafe {
            ptr::write_volatile(IRQ_ENABLE_0, mask); // write enabled peripheral bits to register
        }

        // enable interrupts
        unsafe {
            asm!("msr daifclr, #0b10"); // remove irq mask
        }
    }
}
