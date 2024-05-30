#![no_std]
#![no_main]
#![feature(alloc_error_handler, const_type_id, panic_info_message)]

mod blockdev;
mod wtcore;

use uefi::prelude::*;

use wtcore::boot::attempt_boot;
use wtcore::config::CONFIG;
use wtcore::config::load::load_config;
use wtcore::menu::{BootMenu, MenuOption};
use wtcore::shell::shell_return;

/// Entry point for the Wakatiwai bootloader.
#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Init the boot services
    uefi::helpers::init(&mut system_table).unwrap();

    // Clear the screen (logo, uefi traces, etc.) and print the name
    system_table.stdout().clear().unwrap();
    println_force!("Starting Wakatiwai Bootloader");

    // Load config file
    match load_config(&system_table) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            return shell_return();
        }
    };

    // Config is locked behind an RwLock, so obtain a read lock - it shouldn't be changed hereafter
    let config = CONFIG.read();
    dprintln!("Loaded config: {}", config);

    // Display the menu for the user to select boot options
    let boot_option = BootMenu::select_option();
    match boot_option {
        MenuOption::BootOption(entry) => {
            attempt_boot(&entry);
        }
        MenuOption::UEFIShell => {
            return Status::ABORTED;
        }
        MenuOption::EditConfig => {
            todo!();
        }
    }

    shell_return()
}
