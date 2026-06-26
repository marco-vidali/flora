#[macro_export]
macro_rules! debug {
    // match any expression and an undefined number of parameters
    ($($s:expr), *) => {
        let mut mini_uart = $crate::drivers::mini_uart::MiniUart::new();
        let _ = write!(mini_uart, $($s), *); // send the expression formatted with the parameters to the mini_uart
    };
}
