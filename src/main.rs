#![no_std]
#![no_main]

use const_format::concatcp;
use flora::{config::ERROR_FLAG, println, screen::Screen, shell::Shell};
use uefi::Status;

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().expect(concatcp!(ERROR_FLAG, "Failed to init UEFI boot services.")); // Init UEFI boot services

    Screen::init();
    Screen::clear();

    println!("Welcome to flora.\n\r");

    Shell::init();

    loop {
        core::hint::spin_loop(); // Loop without overheating
    }
}
