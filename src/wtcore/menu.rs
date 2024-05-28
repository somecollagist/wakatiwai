use uefi::helpers::system_table;
use uefi::prelude::Status;
use uefi::proto::console::text::*;

use crate::println_force;

pub fn menu_return() -> Status {
	println_force!("");
	println_force!("Press ESC to continue to UEFI shell...");

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