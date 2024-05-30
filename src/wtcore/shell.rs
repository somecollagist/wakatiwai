use uefi::prelude::Status;
use uefi::proto::console::text::*;
use uefi::ResultExt;

use crate::{boot_services, println_force, system_table};

/// Prompts the user to press the Escape key and then enters the UEFI shell
pub fn shell_return() -> Status {
    println_force!("");
    println_force!("Press ESC to continue to UEFI shell...");

    loop {
        system_table!().stdin().reset(false).unwrap();
        boot_services!()
            .wait_for_event([system_table!().stdin().wait_for_key_event().unwrap()].as_mut())
            .discard_errdata()
            .unwrap();

        match system_table!().stdin().read_key().unwrap() {
            Some(Key::Special(ScanCode::ESCAPE)) => {
                return Status::ABORTED;
            }
            _ => {
                continue;
            }
        }
    }
}
