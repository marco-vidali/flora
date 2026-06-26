#![no_std]
#![no_main]

use core::{hint, panic::PanicInfo};

use flora::{
    board::{cpu, irq::IrqManager},
    debug,
    drivers::mini_uart::MiniUart,
    macros::DebugType,
};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // init mini uart
    MiniUart::init();
    debug!(DebugType::Info, "Mini UART initialized.");

    // print current exception level
    let el = cpu::get_current_el();
    let el = (el + b'0') as char;
    debug!(DebugType::Info, "Current exception level: {}.", el);

    // init interrupt requests manager
    IrqManager::init();
    debug!(DebugType::Info, "Interrupt requests manager enabled.");

    loop {
        hint::spin_loop(); // wait without overheating
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hint::spin_loop();
    }
}
