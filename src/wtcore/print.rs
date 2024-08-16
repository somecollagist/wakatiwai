/// Prints an error message.
/// Note: This is functionally equal to `eprint_force!`.
#[macro_export]
macro_rules! eprint {
	($($arg:tt)*) => {
		crate::eprint_force!($($arg)*);
	}
}

/// Prints an error message with a newline.
/// Note: This is functionally equal to `eprintln_force!`.
#[macro_export]
macro_rules! eprintln {
	($($arg:tt)*) => {
		crate::eprintln_force!($($arg)*);
	}
}

/// Prints a warning message, if allowed by the current log level.
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
			crate::wtcore::config::LogLevel::DEBUG |
			crate::wtcore::config::LogLevel::NORMAL |
			crate::wtcore::config::LogLevel::QUIET => {
				crate::wprint_force!($($arg)*);
			}
			crate::wtcore::config::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

/// Prints a warning message with a newline, if allowed by the current log level.
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
			crate::wtcore::config::LogLevel::DEBUG |
			crate::wtcore::config::LogLevel::NORMAL |
			crate::wtcore::config::LogLevel::QUIET => {
				crate::wprintln_force!($($arg)*);
			}
			crate::wtcore::config::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

/// Prints a message, if allowed by the current log level.
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
			crate::wtcore::config::LogLevel::DEBUG |
			crate::wtcore::config::LogLevel::NORMAL => {
				crate::print_force!($($arg)*);
			}
			crate::wtcore::config::LogLevel::QUIET |
			crate::wtcore::config::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

/// Prints a message with a newline, if allowed by the current log level.
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
			crate::wtcore::config::LogLevel::DEBUG |
			crate::wtcore::config::LogLevel::NORMAL => {
				crate::println_force!($($arg)*);
			}
			crate::wtcore::config::LogLevel::QUIET |
			crate::wtcore::config::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

/// Prints a debug message, if allowed by the current log level.
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
			crate::wtcore::config::LogLevel::DEBUG => {
				crate::dprint_force!($($arg)*);
			}
			crate::wtcore::config::LogLevel::NORMAL |
			crate::wtcore::config::LogLevel::QUIET |
			crate::wtcore::config::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

/// Prints a debug message with a newline, if allowed by the current log level.
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
			crate::wtcore::config::LogLevel::DEBUG => {
				crate::dprintln_force!($($arg)*);
			}
			crate::wtcore::config::LogLevel::NORMAL |
			crate::wtcore::config::LogLevel::QUIET |
			crate::wtcore::config::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

/// Forcibly prints an error message.
#[macro_export]
macro_rules! eprint_force {
	() => {};
	($($arg:tt)*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightRed,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($($arg)*);
	}
}

/// Forcibly prints an error message with a newline.
#[macro_export]
macro_rules! eprintln_force {
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

/// Forcibly prints a warning message.
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

/// Forcibly prints a warning message with a newline.
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

/// Forcibly prints a message.
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

/// Forcibly prints a message with a newline.
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

/// Forcibly prints a debug message.
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

/// Forcibly prints a debug message with a newline.
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
