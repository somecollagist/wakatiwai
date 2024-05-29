#![no_std]
#![no_main]
#![feature(alloc_error_handler, panic_info_message)]

mod blockdev;
mod wtcore;

use wtcore::config::*;
use wtcore::menu::display_menu;
use wtcore::shell::shell_return;

use uefi::prelude::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    system_table.stdout().clear().unwrap();
    println_force!("Starting Wakatiwai Bootloader");

    match load_config(&system_table) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            return shell_return();
        }
    };

    let config = CONFIG.read();
    dprintln!("Loaded config: {:?}", config);

    match display_menu() {
        Ok(_) => {
            // Signifies UEFI shell
            return Status::ABORTED;
        }
        Err(err) => {
            eprintln!("Menu error: {}", err);
        }
    }

    shell_return()
}
