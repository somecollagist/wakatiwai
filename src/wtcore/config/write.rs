extern crate alloc;

use alloc::vec::Vec;

use uefi::fs::FileSystem;
use uefi::Status;

use crate::*;
use crate::wtcore::config::*;

/// Writes to a byte array to the bootloader configuration file, overwriting the existing contents.
pub fn write_config(buffer: &Vec<u8>) -> Result<(), Status> {
    // Attempt to get the file system containing the bootloader - the config file should be in the same file system
    let mut efifs = match uefi::boot::get_image_file_system(image_handle!()) {
        Ok(ok) => FileSystem::new(ok),
        Err(err) => return Err(err.status()),
    };

    // If there's a directory at the config path, delete it
    if efifs.try_exists(CONFIG_PATH).unwrap() && efifs.metadata(CONFIG_PATH).unwrap().is_directory() {
        efifs.remove_dir_all(CONFIG_PATH).unwrap();
    }

    // Overwrites the config file with the buffer's content
    efifs.write(CONFIG_PATH, buffer).unwrap();

    Ok(())
}