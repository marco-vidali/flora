use uefi::{Status, proto::console::text::Output, runtime::ResetType};

pub struct Commands;

impl Commands {
    pub fn clear_screen() {
        uefi::system::with_stdout(Output::clear).unwrap();
    }

    pub fn shut_down() {
        uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
    }
}
