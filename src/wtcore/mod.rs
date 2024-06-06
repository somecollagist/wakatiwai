pub mod config;
pub mod menu;
pub mod panic;
pub mod print;

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

/// Shorthand to get the standard input.
#[macro_export]
macro_rules! stdin {
	() => {
		crate::system_table!().stdin()
	};
}

/// Shorthand to get the standard output.
#[macro_export]
macro_rules! stdout {
	() => {
		crate::system_table!().stdout()
	};
}

/// Shorthand to get the current output mode.
#[macro_export]
macro_rules! current_output_mode {
	() => {
		crate::stdout!().current_mode().unwrap().unwrap()
	};
}