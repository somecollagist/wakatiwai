use core::panic::PanicInfo;

use crate::eprintln_force;

/// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln_force!(
        "[PANIC] @ {}: {}",
        info.location().unwrap(),
        info.message().unwrap()
    );

    loop {}
}
