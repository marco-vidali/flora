use uefi::proto::console::text::Output;

pub struct Commands;

impl Commands {
    pub fn clear_screen() {
        uefi::system::with_stdout(Output::clear).unwrap();
    }
}
