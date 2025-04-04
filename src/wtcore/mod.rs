use core::str::FromStr;

pub mod config;
pub mod menu;
pub mod panic;
pub mod print;

/// Shorthand to get the loaded image handle.
#[macro_export]
macro_rules! image_handle {
	() => {
		uefi::boot::image_handle()
	};
}

/// Shorthand to get the standard input.
#[macro_export]
macro_rules! stdin {
	() => {
		unsafe {
            core::mem::transmute::
                <
                    *mut uefi_raw::protocol::console::SimpleTextInputProtocol,
                    *mut uefi::proto::console::text::Input
                >
            (uefi::table::system_table_raw().unwrap().as_mut().stdin)
            .as_mut().unwrap()
        }
	};
}

/// Shorthand to get the standard output.
#[macro_export]
macro_rules! stdout {
	() => {
		unsafe {
            core::mem::transmute::
                <
                    *mut uefi_raw::protocol::console::SimpleTextOutputProtocol,
                    *mut uefi::proto::console::text::Output
                >
            (uefi::table::system_table_raw().unwrap().as_mut().stdout)
            .as_mut().unwrap()
        }
	};
}

/// Shorthand to get the current output mode.
#[macro_export]
macro_rules! current_output_mode {
	() => {
		crate::stdout!().current_mode().unwrap().unwrap()
	};
}

fn get_unix_time() -> i64 {
    let start_time_uefi = uefi::runtime::get_time().unwrap();
    chrono::NaiveDate::from_ymd_opt(
        start_time_uefi.year() as i32,
        start_time_uefi.month() as u32,
        start_time_uefi.day() as u32
    ).unwrap().and_hms_opt(
        start_time_uefi.hour() as u32,
        start_time_uefi.minute() as u32,
        start_time_uefi.second() as u32
    ).unwrap().and_utc().timestamp()
}