use core::panic::PanicInfo;

use uefi::{Char16, Status};

use crate::{eprintln_force, image_handle, println_force, system_table};

/// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln_force!(
        "[PANIC] @ {:?}: {:?}",
        info.location(),
        info.message()
    );
    println_force!("Exiting in 5 seconds...");

    system_table!().boot_services().stall(5_000_000);
    unsafe {
        system_table!().boot_services().exit(image_handle!(), Status::ABORTED, 0, 0 as *mut Char16);
    }
}
