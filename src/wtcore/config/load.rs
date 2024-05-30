use uefi::fs::FileSystem;
use uefi::prelude::*;

use crate::*;
use crate::wtcore::config::*;
use crate::wtcore::config::parse::parse_config;

/// Loads the bootloader configuration file.
///
/// Returns `Ok(())` on a successful load and `Err(uefi_raw::Status)` on a failure.
pub fn load_config(system_table: &SystemTable<Boot>) -> Result<(), Status> {
    println_force!("Loading config...");

    // Attempt to get the file system containing the bootloader - the config file should be in the same file system
    let mut efifs = match system_table.boot_services().get_image_file_system(image_handle!()) {
        Ok(ok) => FileSystem::new(ok),
        Err(err) => return Err(err.status()),
    };

    // Check if the config file exists - this ONLY checks if a directory entry exists at the path
    if !efifs.try_exists(CONFIG_PATH).unwrap() {
        eprintln!("No config file found");
        return Err(Status::ABORTED);
    }

    // Check if the entry at the config file's path is a directory and fail if so
    let wtconfig_info = efifs.metadata(CONFIG_PATH).unwrap();
    if wtconfig_info.is_directory() {
        eprintln!("Directory found instead of wtconfig file");
        return Err(Status::ABORTED);
    }

    // Config file path contains a file - parse it as a config file
    parse_config(match efifs.read(CONFIG_PATH) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("Failed to read config");
            return Err(Status::ABORTED);
        }
    })
}