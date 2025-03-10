use core::{panic::PanicInfo, ptr::null_mut};

use uefi::Status;

use crate::{eprintln_force, image_handle, println_force};

/// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln_force!(
        "[PANIC] @ {:?}: {:?}",
        info.location(),
        info.message()
    );
    println_force!("Exiting in 5 seconds...");

    uefi::boot::stall(5_000_000);
    unsafe {
        uefi::boot::exit(image_handle!(), Status::ABORTED, 0, null_mut());
    }
}
