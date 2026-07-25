#![no_std]
#![no_main]

use flora::screen::Screen;
use uefi::Status;

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap(); // Init UEFI boot services

    Screen::clear();
    Screen::init();

    loop {
        core::hint::spin_loop(); // Loop without overheating
    }
}
