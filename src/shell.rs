extern crate alloc;

use crate::config;
use alloc::string::String;
use spin::{LazyLock, Mutex};
use uefi::{print, proto::console::text::Key};

static COMMAND: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

pub struct Shell;

impl Shell {
    pub fn init() {
        Self::print_prompt();

        loop {
            let key = Self::read_key();

            if let Some(key) = key {
                Self::handle_key(key);
            }
        }
    }

    fn print_prompt() {
        print!("{} ", config::SHELL_PROMPT);
    }

    fn read_key() -> Option<Key> {
        uefi::system::with_stdin(|stdin| stdin.read_key()).expect("[!] Failed to access keyboard.")
    }

    fn handle_key(key: Key) {
        match key {
            Key::Printable(p) => match char::from(p) {
                '\r' => Self::execute_command(),
                '\x08' => {
                    if COMMAND.lock().pop().is_some() {
                        print!("\x08 \x08"); // Remove last character on display
                    }
                }
                _ => {
                    COMMAND.lock().push(char::from(p));
                    print!("{}", p);
                }
            },

            Key::Special(s) => {
                // handle special keys
            }
        };
    }

    fn execute_command() {
        let mut command = COMMAND.lock();

        if !command.is_empty() {
            print!("\r\n");
            print!("{}", command);

            *command = String::new();
        }

        print!("\r\n");
        Self::print_prompt();
    }
}
