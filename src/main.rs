#![no_std]
#![no_main]

use core::{hint, panic::PanicInfo};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
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
