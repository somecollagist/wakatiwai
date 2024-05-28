use core::panic::PanicInfo;

use uefi::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!(
		"[PANIC] @ {}: {}",
		info.location().unwrap(),
		info.message().unwrap()
	);

	loop {}
}