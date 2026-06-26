#![no_std]
#![no_main]

use core::{fmt::Write, hint, panic::PanicInfo};

use flora::{
    board::{cpu, irq::IrqManager},
    debug,
    drivers::mini_uart::MiniUart,
};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // init mini uart
    MiniUart::init();

    // print current exception level
    let el = cpu::get_current_el();
    let el = (el + b'0') as char;
    debug!("Current exception level: {}.\n", el);

    // enable interrupt requests manager
    IrqManager::new();
    debug!("[*] Interrupt requests manager enabled.\n");

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
