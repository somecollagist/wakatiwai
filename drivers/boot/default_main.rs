#![no_main]
#![no_std]

use springboard::driver;
use springboard::io;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    // Locate callback registry
    let locate_callback_registry_status = driver::find_driver_call_buffer();
    if !locate_callback_registry_status.is_success() {
        return locate_callback_registry_status;
    }

    let args = driver::get_args().unwrap();
    let returns = driver::get_mut_returns().unwrap();

    Status::SUCCESS
}