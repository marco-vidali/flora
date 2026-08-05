use const_format::concatcp;
use uefi::{
    CStr16, CString16,
    boot::ScopedProtocol,
    fs::{Error, FileSystem as UefiFileSystem, UefiDirectoryIter},
    proto::media::fs::SimpleFileSystem,
};

use crate::{config::ERROR_FLAG, println};

pub struct FileSystem;

impl FileSystem {
    fn get_directory_entries(path: &CStr16) -> Result<UefiDirectoryIter, Error> {
        let fs_protocol: ScopedProtocol<SimpleFileSystem> =
            uefi::boot::get_image_file_system(uefi::boot::image_handle()).unwrap();

        let mut fs = UefiFileSystem::new(fs_protocol);

        fs.read_dir(path)
    }

    pub fn list_directory_entries(args: &[&str]) {
        let path = args.first().copied().unwrap_or("\\");
        let path = CString16::try_from(path).expect(concatcp!(
            "{} Failed to convert path to a UTF-16 string.",
            ERROR_FLAG
        ));

        let entries = match Self::get_directory_entries(&path) {
            Ok(e) => e,
            Err(_) => {
                println!("{} Invalid directory.", ERROR_FLAG);
                return;
            }
        };

        for entry in entries {
            let entry = entry.unwrap();

            if entry.is_directory() {
                println!("d {}", entry.file_name());
            } else {
                println!("f {} {}", entry.file_name(), entry.file_size());
            }
        }
    }
}
