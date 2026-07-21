#![no_std]
#![no_main]

use uefi::{prelude::*, println, proto::console::text::Output};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap(); // Init UEFI boot services

    system::with_stdout(Output::clear).unwrap(); // Clear screen

    println!("Welcome to flora.");

    loop {
        core::hint::spin_loop(); // Loop without overheating
    }
}
