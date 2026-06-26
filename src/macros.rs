pub enum DebugType {
    Info,
    Error,
}

impl DebugType {
    pub fn as_tag(&self) -> &'static str {
        match self {
            DebugType::Info => "[*]",
            DebugType::Error => "[!]",
        }
    }
}

#[macro_export]
macro_rules! debug {
    // match any expression and an undefined number of parameters
    ($type:expr, $($s:expr),*) => {
        {
            use core::fmt::Write;

            let mut mini_uart = $crate::drivers::mini_uart::MiniUart;

            // send formatted message to mini uart
            let _ = write!(mini_uart, "{} ", $crate::macros::DebugType::as_tag(&$type)); // send formatted type tag to mini uart
            let _ = core::writeln!(mini_uart, $($s),*); // send the expression formatted with the parameters to mini uart
        }
    };
}
