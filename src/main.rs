#![no_std]
#![no_main]
#![feature(
	alloc_error_handler,
	panic_info_message
)]

mod wtcore;

use wtcore::config::*;
use wtcore::menu::menu_return;

use uefi::prelude::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
	uefi::helpers::init(&mut system_table).unwrap();

	system_table.stdout().clear().unwrap();
	println_force!("Wakatiwai Bootloader");

	match load_config(image_handle, &system_table) {
		Ok(config) => config,
		Err(err) => {
			eprintln!("Failed to load config: {}", err);
			return menu_return();
		}
	};

	let config = CONFIG.read();

	dprintln!("Loaded config: {:?}", config);

	menu_return()
}
