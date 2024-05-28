#![no_main]
#![no_std]
#![feature(
	panic_info_message
)]

use core::panic::PanicInfo;

use uefi::helpers::system_table;
use uefi::prelude::*;
use uefi::println;
use uefi::proto::console::text::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
	uefi::helpers::init(&mut system_table).unwrap();

	system_table.stdout().clear().unwrap();
	println!("Wakatiwai Bootloader");

	menu_return()
}

pub fn menu_return() -> Status {
	println!("");
	println!("Press ESC to continue to UEFI shell...");

	loop {
		system_table().stdin().reset(false).unwrap();
		system_table().boot_services().wait_for_event(
			[system_table().stdin().wait_for_key_event().unwrap()].as_mut()
		).unwrap();

		match system_table().stdin().read_key().unwrap() {
			Some(key) => {
				if key == Key::Special(ScanCode::ESCAPE) {
					return Status::ABORTED;
				}
			}
			None => {
				continue;
			}
		}
	}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!(
		"[PANIC] @ {}: {}",
		info.location().unwrap(),
		info.message().unwrap()
	);

	loop {}
}
