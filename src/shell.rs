use crate::config;
use uefi::{print, proto::console::text::Key};

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
        uefi::system::with_stdin(|stdin| stdin.read_key()).expect("Expected input")
    }

    fn handle_key(key: Key) {
        match key {
            Key::Printable(p) => {
                print!("{}", p);
            }

            Key::Special(s) => {
                // handle special keys
            }
        };
    }
}
