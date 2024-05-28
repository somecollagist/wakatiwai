extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use microjson::*;
use spin::RwLock;
use uefi::fs::FileSystem;
use uefi::prelude::*;

use crate::*;

pub static CONFIG: RwLock<Config> = RwLock::new(default_config());

#[derive(Debug)]
pub struct Config {
	pub log_level: LogLevel,
	pub timeout: i32,
	pub boot_order: Vec<BootEntry>
}

const DEFAULT_TIMEOUT: i32			= 5;
const fn default_config() -> Config {
	Config {
		log_level: LogLevel::NORMAL,
		timeout: DEFAULT_TIMEOUT,
		boot_order: Vec::new()
	}
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
	// Will produce no output bar critical failures
	SILENT,
	// Will output critical failures and warnings
	QUIET,
	// Will produce normal output
	NORMAL,
	// Will produce debug output
	DEBUG
}

impl TryFrom<&str> for LogLevel {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		return match value {
			"SILENT" => Ok(LogLevel::SILENT),
			"QUIET" => Ok(LogLevel::QUIET),
			"NORMAL" => Ok(LogLevel::NORMAL),
			"DEBUG" => Ok(LogLevel::DEBUG),
			_ => Err(())
		}
	}
}

#[derive(Debug, Default)]
pub struct BootEntry {
	pub name: String,
	pub partition: u32
}

macro_rules! config_path {
	() => {
		cstr16!("wtconfig.json")
	};
}

pub fn load_config(image_handle: Handle, system_table: &SystemTable<Boot>) -> Result<(), Status> {
	println_force!("Loading config...");

	let mut efifs = match system_table.boot_services().get_image_file_system(image_handle) {
		Ok(ok) => FileSystem::new(ok),
		Err(err) => return Err(err.status())
	};

	if !efifs.try_exists(config_path!()).unwrap() {
		eprintln!("No config file found");
		return Err(Status::ABORTED);
	}

	let wtconfig_info = efifs.metadata(config_path!()).unwrap();
	if wtconfig_info.is_directory() {
		eprintln!("Directory found instead of wtconfig file");
		return Err(Status::ABORTED);
	}

	let wtconfig_buffer = match efifs.read(config_path!()) {
		Ok(ok) => ok,
		Err(_) => {
			eprintln!("Failed to read config");
			return Err(Status::ABORTED);
		}
	};

	parse_config(wtconfig_buffer)
}

const CFG_KEY_LOGLEVEL: &str = "loglevel";
const CFG_KEY_TIMEOUT: &str = "timeout";
const CFG_KEY_BOOTENTRIES: &str = "bootentries";

fn parse_config(buffer: Vec<u8>) -> Result<(), Status> {
	let buffer_str = &String::from_utf8(buffer).unwrap();
	let json = match JSONValue::load_and_verify(&buffer_str) {
		Ok(ok) => ok,
		Err(err) => {
			eprintln!("Failed to parse config file: {}", err);
			return Err(Status::ABORTED);
		}
	};

	// Set loglevel
	if does_exist_key_of_type(json, CFG_KEY_LOGLEVEL, JSONValueType::String) {
		let loglevel_str = json.get_key_value(CFG_KEY_LOGLEVEL).unwrap().read_string().unwrap();
		let mut config = CONFIG.write();
		config.log_level = match LogLevel::try_from(loglevel_str) {
			Ok(ok) => ok,
			Err(_) => {
				print_force!("Unknown log level \"{}\", entering debug mode...", loglevel_str);
				LogLevel::DEBUG
			}
		};
		drop(config);
	}

	// Set timeout
	if does_exist_key_of_type(json, CFG_KEY_TIMEOUT, JSONValueType::Number) {
		let timeout = match json.get_key_value(CFG_KEY_TIMEOUT).unwrap().read_integer() {
			Ok(ok) => ok as i32,
			Err(_) => {
				wprintln!("Timeout length is not an integer, using default...");
				DEFAULT_TIMEOUT
			}
		};
		let mut config = CONFIG.write();
		config.timeout = timeout;
		drop(config);
		match timeout {
			0				=> { dprintln!("Bootloader will immediately boot into preferred entry"); },
			1				=> { dprintln!("Bootloader will wait 1 second before booting into preferred entry unless stopped"); },
			2..=i32::MAX	=> { dprintln!("Bootloader will wait {} seconds before booting into preferred entry unless stopped", timeout); },
			i32::MIN..=-1	=> { dprintln!("Bootloader will wait for user input before booting into an entry"); }
		}
	}

	// Load boot entries
	if does_exist_required_key_of_type(json, CFG_KEY_BOOTENTRIES, JSONValueType::Array) {
		let bootentries = json.get_key_value(CFG_KEY_BOOTENTRIES).unwrap().iter_array().unwrap();
		for bootentry_json in bootentries {
			let bootentry = match parse_bootentry(bootentry_json) {
				Ok(ok) => ok,
				Err(err) => {
					eprintln!("Failed to parse config file: {}", err);
					return Err(Status::ABORTED);
				}
			};

			dprintln!("Detected boot entry: {:?}", bootentry);

			let mut config = CONFIG.write();
			config.boot_order.push(bootentry);
			drop(config);
		}
	}
	else {
		return Err(Status::ABORTED);
	}

	Ok(())
}

const BE_KEY_NAME: &str = "name";
const BE_KEY_PARTITION: &str = "partition";

fn parse_bootentry<'a>(json: JSONValue) -> Result<BootEntry, JSONParsingError> {
	if json.value_type != JSONValueType::Object {
		eprintln!("Non-object in boot order");
		return Err(JSONParsingError::CannotParseObject);
	}

	let mut bootentry: BootEntry = BootEntry::default();

	// Boot entry name
	if !does_exist_required_key_of_type(json, BE_KEY_NAME, JSONValueType::String) {
		return Err(JSONParsingError::KeyNotFound);
	}
	bootentry.name = String::from(json.get_key_value(BE_KEY_NAME).unwrap().read_string().unwrap());

	// Boot entry partition
	if !does_exist_required_key_of_type(json, BE_KEY_PARTITION, JSONValueType::Number){
		return Err(JSONParsingError::KeyNotFound)
	}
	bootentry.partition = json.get_key_value(BE_KEY_PARTITION).unwrap().read_integer().unwrap() as u32;

	Ok(bootentry)
}

#[allow(dead_code)]
fn does_exist_key(json: JSONValue, key: &str) -> bool {
	return match json.get_key_value(key) {
		Ok(_) => true,
		Err(_) => false
	}
}

#[allow(dead_code)]
fn does_exist_required_key(json: JSONValue, key: &str) -> bool {
	let ret = does_exist_key(json, key);
	if !ret {
		eprintln!("Could not locate required key \"{}\" while parsing", key);
	}
	ret
}

#[allow(dead_code)]
fn does_exist_key_of_type(json: JSONValue, key: &str, value_type: JSONValueType) -> bool {
	does_exist_key(json, key) && json.get_key_value(key).unwrap().value_type == value_type
}

#[allow(dead_code)]
fn does_exist_required_key_of_type(json: JSONValue, key: &str, value_type: JSONValueType) -> bool {
	let ret = does_exist_key_of_type(json, key, value_type);
	if !ret {
		let value_type_pretty = match value_type {
			JSONValueType::Array => "array",
			JSONValueType::Bool => "boolean",
			JSONValueType::Null => "null",
			JSONValueType::Number => "numerical",
			JSONValueType::Object => "object",
			JSONValueType::String => "string",
			JSONValueType::Error => panic!("Unreachable statement")
		};
		eprintln!("Could not locate required {} \"{}\" while parsing", value_type_pretty, key);
	}
	ret
}