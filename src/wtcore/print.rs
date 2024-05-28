#[macro_export]
macro_rules! eprint {
	() => {};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightRed,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($($arg)*);
	}
}

#[macro_export]
macro_rules! eprintln {
	() => {
		uefi::println!("");
	};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightRed,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($($arg)*);
	}
}

#[macro_export]
macro_rules! wprint {
	($($arg:tt)*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};

		match config.log_level {
			crate::LogLevel::DEBUG |
			crate::LogLevel::NORMAL |
			crate::LogLevel::QUIET => {
				crate::wprint_force!($($arg)*);
			}
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! wprintln {
	($($arg:tt)*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};

		match config.log_level {
			crate::LogLevel::DEBUG |
			crate::LogLevel::NORMAL |
			crate::LogLevel::QUIET => {
				crate::wprintln_force!($($arg)*);
			}
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};

		match config.log_level {
			crate::LogLevel::DEBUG |
			crate::LogLevel::NORMAL => {
				crate::print_force!($($arg)*);
			}
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! println {
	($($arg:tt)*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};

		match config.log_level {
			crate::LogLevel::DEBUG |
			crate::LogLevel::NORMAL => {
				crate::println_force!($($arg)*);
			}
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! dprint {
	($($arg:tt)*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};

		match config.log_level {
			crate::LogLevel::DEBUG => {
				crate::dprint_force!($($arg)*);
			}
			crate::LogLevel::NORMAL |
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! dprintln {
	($($arg:tt)*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};

		match config.log_level {
			crate::LogLevel::DEBUG => {
				crate::dprintln_force!($($arg)*);
			}
			crate::LogLevel::NORMAL |
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

// print_error and println_error have no force as they will always be emitted.

#[macro_export]
macro_rules! wprint_force {
	() => {};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::Yellow,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($($arg)*);
	}
}

#[macro_export]
macro_rules! wprintln_force {
	() => {
		uefi::println!("");
	};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::Yellow,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($($arg)*);
	}
}

#[macro_export]
macro_rules! print_force {
	() => {};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightGray,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($($arg)*);
	}
}

#[macro_export]
macro_rules! println_force {
	() => {
		uefi::println!("");
	};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightGray,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($($arg)*);
	}
}

#[macro_export]
macro_rules! dprint_force {
	() => {};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightCyan,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($($arg)*);
	}
}

#[macro_export]
macro_rules! dprintln_force {
	() => {
		uefi::println!("");
	};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightCyan,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($($arg)*);
	}
}
