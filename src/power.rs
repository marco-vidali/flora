use uefi::{Status, runtime::ResetType};

pub struct Power;

impl Power {
    pub fn shut_down() {
        uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
    }
}
