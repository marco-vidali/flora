use uefi::{Status, runtime::ResetType};

pub struct Commands;

impl Commands {
    pub fn shut_down() {
        uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
    }
}
