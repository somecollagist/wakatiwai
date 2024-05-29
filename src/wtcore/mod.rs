mod boot;
pub mod config;
pub mod menu;
pub mod panic;
pub mod print;
pub mod shell;

#[macro_export]
macro_rules! system_table {
	() => {
		uefi::helpers::system_table()
	};
}

#[macro_export]
macro_rules! boot_services {
	() => {
		uefi::helpers::system_table().boot_services()
	};
}

#[macro_export]
macro_rules! image_handle {
	() => {
		crate::boot_services!().image_handle()
	};
}