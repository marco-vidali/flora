extern crate alloc;

use crate::{
    config::{self, ERROR_FLAG},
    file_system::FileSystem,
    power::Power,
    print,
    screen::Screen,
};

use alloc::string::String;
use alloc::vec::Vec;
use const_format::concatcp;
use spin::{LazyLock, Mutex};
use uefi::proto::console::text::Key;
static COMMAND: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

static COMMANDS_TABLE: [(&str, fn(&[&str])); 3] = [
    ("clear", |_| Screen::clear()),
    ("off", |_| Power::shut_down()),
    ("ls", |args| FileSystem::list_directory_entries(args)),
];

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
        uefi::system::with_stdin(|stdin| stdin.read_key())
            .expect(concatcp!(ERROR_FLAG, " Failed to access keyboard."))
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

            Key::Special(_) => {
                // Handle special keys
            }
        };
    }

    fn execute_command() {
        let mut command = COMMAND.lock();

        if command.is_empty() {
            print!("\r\n");
        } else {
            print!("\r\n");

            let mut command_found = false;
            let mut command_parts = command.as_str().split_whitespace();

            let command_name = command_parts
                .next()
                .expect(concatcp!("{} Failed to get command name.", ERROR_FLAG));

            // Find corresponding function and execute it
            for (name, func) in COMMANDS_TABLE {
                if name == command_name {
                    let args: Vec<&str> = command_parts.collect();
                    func(&args);

                    command_found = true;
                    break;
                }
            }

            if !command_found {
                print!("{} Command not found.", ERROR_FLAG);
            }

            if *command != "clear" {
                print!("\r\n");
            }

            *command = String::new();
        }

        Self::print_prompt();
    }
}
