pub mod boot;
pub mod config;
pub mod menu;
pub mod panic;
pub mod print;
pub mod shell;

/// Shorthand to get the current system table.
#[macro_export]
macro_rules! system_table {
	() => {
		uefi::helpers::system_table()
	};
}

/// Shorthand to get the current system's boot services.
#[macro_export]
macro_rules! boot_services {
	() => {
		uefi::helpers::system_table().boot_services()
	};
}

/// Shorthand to get the loaded image handle.
#[macro_export]
macro_rules! image_handle {
	() => {
		crate::boot_services!().image_handle()
	};
}