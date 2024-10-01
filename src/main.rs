#![no_std]
#![no_main]
#![feature(
    alloc_error_handler,
    const_option,
    const_type_id,
    iter_advance_by,
    slice_split_once,
    trait_upcasting
)]

mod dev;
mod boot;
mod editor;
mod fs;
mod wtcore;

#[macro_use]
extern crate alloc;
use alloc::string::String;

use uefi::prelude::*;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::table::runtime::ResetType;

use boot::attempt_boot;
use wtcore::config::*;
use wtcore::config::load::{load_config, read_config};
use wtcore::config::write::write_config;
use wtcore::menu::{BootMenu, MenuOption};

/// Entry point for the Wakatiwai bootloader.
#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Init the boot services
    uefi::helpers::init(&mut system_table).unwrap();

    // Initial stdout
    system_table.stdout().clear().unwrap();
    let mut best_mode = system_table.stdout().current_mode().unwrap().unwrap();
    for mode in system_table.stdout().modes() { 
        if mode.rows() > best_mode.rows() {
            best_mode = mode;
        }
    }
    system_table.stdout().set_mode(best_mode).unwrap();
    println_force!("Starting Wakatiwai Bootloader");

    // Load config file
    match load_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            println_force!("Opening editor in 5 seconds...");
            boot_services!().stall(5_000_000);
            edit_config();
            reboot();
        }
    };

    // Config is locked behind an RwLock, so obtain a read lock - it shouldn't be changed hereafter
    let config = CONFIG.read();
    dprintln!("Loaded config: {}", config);

    // Display the menu for the user to select boot options
    let boot_option = BootMenu::select_option();
    match boot_option {
        MenuOption::BootOption(entry) => {
            match attempt_boot(&entry) {
                Some(some) => {
                    eprintln!("Unable to boot: {:?}", some);
                }
                None => {}
            };
        }
        MenuOption::Exit => {
            // Set colours because it's good if exiting to EDKII shell
            stdout!().set_color(
                uefi::proto::console::text::Color::LightGray,
                uefi::proto::console::text::Color::Black
            ).unwrap();
            // What actually needs to be done
            return Status::ABORTED;
        }
        MenuOption::EditConfig => {
            edit_config();
            reboot();
        }
        MenuOption::Reboot => {
            reboot();
        }
        MenuOption::Poweroff => {
            poweroff();
        }
    }

    exit()
}

/// Reboots the system.
fn reboot() -> ! {
    println!("Rebooting...");
    system_table!().boot_services().stall(100_000);
    system_table!().runtime_services().reset(ResetType::COLD, Status::SUCCESS, None);
}

/// Powers off the system.
fn poweroff() -> ! {
    println!("Powering off...");
    system_table!().boot_services().stall(100_000);
    system_table!().runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

/// Prompts the user to press the Escape key and then exits the bootloader
fn exit() -> Status {
    println_force!("");
    println_force!("Press ESC to exit...");

    loop {
        stdin!().reset(false).unwrap();
        boot_services!()
            .wait_for_event([stdin!().wait_for_key_event().unwrap()].as_mut())
            .discard_errdata()
            .unwrap();

        match stdin!().read_key().unwrap() {
            Some(Key::Special(ScanCode::ESCAPE)) => {
                return Status::ABORTED;
            }
            _ => {
                continue;
            }
        }
    }
}

fn edit_config() {
    let mut editor = editor::Editor::new("wtconfig.json", &read_config().unwrap_or(vec![' ' as u8]));
    let editbuf = editor.edit();
    stdout!().clear().unwrap();

    dprintln!("Writing config:\n{}", String::from_utf8(editbuf.clone()).unwrap());
    write_config(&editbuf).unwrap();
}