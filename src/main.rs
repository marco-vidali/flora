#![no_std]
#![no_main]

use flora::{commands::Commands, shell::Shell};
use uefi::{Status, println};

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap(); // Init UEFI boot services

    Commands::clear_screen();

    println!("Welcome to flora.\n");

    Shell::init();

    loop {
        core::hint::spin_loop(); // Loop without overheating
    }
}
