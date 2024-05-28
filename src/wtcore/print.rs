#[macro_export]
macro_rules! eprint {
	() => {
		uefi::print!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightRed,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($( $payload, )*);
	}
}

#[macro_export]
macro_rules! eprintln {
	() => {
		uefi::println!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightRed,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($( $payload, )*);
	}
}

#[macro_export]
macro_rules! wprint {
	() => {};
	( $( $payload:expr ),*) => {
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
				uefi::helpers::system_table().stdout().set_color(
					uefi::proto::console::text::Color::Yellow,
					uefi::proto::console::text::Color::Black
				).unwrap();
				uefi::print!($( $payload, )*);
			}
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! wprintln {
	() => {
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
				uefi::println!("");
			}
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	};
	( $( $payload:expr ),*) => {
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
				uefi::helpers::system_table().stdout().set_color(
					uefi::proto::console::text::Color::Yellow,
					uefi::proto::console::text::Color::Black
				).unwrap();
				uefi::println!($( $payload, )*);
			}
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! print {
	() => {};
	( $( $payload:expr ),*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};
	
		match config.log_level {
			crate::LogLevel::DEBUG |
			crate::LogLevel::NORMAL => {
				uefi::helpers::system_table().stdout().set_color(
					uefi::proto::console::text::Color::LightGray,
					uefi::proto::console::text::Color::Black
				).unwrap();
				uefi::print!($( $payload, )*);
			}
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! println {
	() => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};
	
		match config.log_level {
			crate::LogLevel::DEBUG |
			crate::LogLevel::NORMAL => {
				uefi::println!("");
			}
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	};
	( $( $payload:expr ),*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};
	
		match config.log_level {
			crate::LogLevel::DEBUG |
			crate::LogLevel::NORMAL => {
				uefi::helpers::system_table().stdout().set_color(
					uefi::proto::console::text::Color::LightGray,
					uefi::proto::console::text::Color::Black
				).unwrap();
				uefi::println!($( $payload, )*);
			}
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	}
}

#[macro_export]
macro_rules! dprint {
	() => {};
	( $( $payload:expr ),*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};
	
		match config.log_level {
			crate::LogLevel::DEBUG => {
				uefi::helpers::system_table().stdout().set_color(
					uefi::proto::console::text::Color::LightCyan,
					uefi::proto::console::text::Color::Black
				).unwrap();
				uefi::print!($( $payload, )*);
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
	() => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};
	
		match config.log_level {
			crate::LogLevel::DEBUG => {
				uefi::println!("");
			}
			crate::LogLevel::NORMAL |
			crate::LogLevel::QUIET |
			crate::LogLevel::SILENT => {}
		}

		drop(config);
	};
	( $( $payload:expr ),*) => {
		let config = match crate::wtcore::config::CONFIG.try_read() {
			Some(lock) => lock,
			None => {
				panic!("Attempted to read locked config");
			}
		};
	
		match config.log_level {
			crate::LogLevel::DEBUG => {
				uefi::helpers::system_table().stdout().set_color(
					uefi::proto::console::text::Color::LightCyan,
					uefi::proto::console::text::Color::Black
				).unwrap();
				uefi::println!($( $payload, )*);
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
	() => {
		uefi::print!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::Yellow,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($( $payload, )*);
	}
}

#[macro_export]
macro_rules! wprintln_force {
	() => {
		uefi::println!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::Yellow,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($( $payload, )*);
	}
}

#[macro_export]
macro_rules! print_force {
	() => {
		uefi::print!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightGray,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($( $payload, )*);
	}
}

#[macro_export]
macro_rules! println_force {
	() => {
		uefi::println!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightGray,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($( $payload, )*);
	}
}

#[macro_export]
macro_rules! dprint_force {
	() => {
		uefi::print!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightCyan,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::print!($( $payload, )*);
	}
}

#[macro_export]
macro_rules! dprintln_force {
	() => {
		uefi::println!("");
	};
	( $( $payload:expr ),*) => {
		uefi::helpers::system_table().stdout().set_color(
			uefi::proto::console::text::Color::LightCyan,
			uefi::proto::console::text::Color::Black
		).unwrap();
		uefi::println!($( $payload, )*);
	}
}