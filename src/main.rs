#![no_std]
#![no_main]

use core::{hint, panic::PanicInfo};

use flora::{cpu, drivers::mini_uart::MiniUart};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // print welcome message
    let mini_uart = MiniUart::new();
    mini_uart.send_str("Welcome to flora.\n");

    // print current exception level
    let el = cpu::get_current_el();

    mini_uart.send_str("Current exception level: ");
    mini_uart.send_char((el + b'0') as char);

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
