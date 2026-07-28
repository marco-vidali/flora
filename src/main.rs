#![no_std]
#![no_main]

use const_format::concatcp;
use flora::{config::ERROR_FLAG, print, println, screen::Screen};
use uefi::Status;

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().expect(concatcp!(ERROR_FLAG, "Failed to init UEFI boot services.")); // Init UEFI boot services

    Screen::clear();
    Screen::init();

    println!("Welcome to flora.\n\r");

    loop {
        core::hint::spin_loop(); // Loop without overheating
    }
}
