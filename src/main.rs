#![no_std]
#![no_main]

use core::{hint, panic::PanicInfo};

use flora::drivers::mini_uart::MiniUart;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mini_uart = MiniUart::new();

    mini_uart.send_str("Welcome to flora.");

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
