#![no_std]
#![no_main]

use flora::{commands::Commands, screen::Screen};
use uefi::Status;

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap(); // Init UEFI boot services

    Commands::clear_screen();

    Screen::init();

    loop {
        core::hint::spin_loop(); // Loop without overheating
    }
}
