#![no_std]
#![no_main]

use flora::shell::Shell;
use uefi::{Status, println, proto::console::text::Output};

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap(); // Init UEFI boot services

    uefi::system::with_stdout(Output::clear).unwrap(); // Clear screen

    println!("Welcome to flora.\n");

    Shell::init();

    loop {
        core::hint::spin_loop(); // Loop without overheating
    }
}
