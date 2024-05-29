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
    pub instant_boot: bool,
    pub offer_shell: bool,
    pub edit_config: bool,
    pub menu_clear: bool,
    pub boot_entries: Vec<BootEntry>,
}

impl Config {
    const KEY_LOG_LEVEL: &'static str = "loglevel";
    const KEY_INSTANT_BOOT: &'static str = "instantboot";
    const KEY_OFFER_SHELL: &'static str = "offershell";
    const KEY_EDIT_CONFIG: &'static str = "editconfig";
    const KEY_MENU_CLEAR: &'static str = "menuclear";
    const KEY_BOOTENTRIES: &'static str = "bootentries";

    const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::NORMAL;
    const DEFAULT_INSTANT_BOOT: bool = false;
    const DEFAULT_OFFER_SHELL: bool = true;
    const DEFAULT_EDIT_CONFIG: bool = true;
    const DEFAULT_MENU_CLEAR: bool = true;

    pub fn is_instant_boot(&self) -> bool {
        self.instant_boot
    }
}

const fn default_config() -> Config {
    Config {
        log_level: Config::DEFAULT_LOG_LEVEL,
        instant_boot: Config::DEFAULT_INSTANT_BOOT,
        offer_shell: Config::DEFAULT_OFFER_SHELL,
        edit_config: Config::DEFAULT_EDIT_CONFIG,
        menu_clear: Config::DEFAULT_MENU_CLEAR,
        boot_entries: Vec::new(),
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
    DEBUG,
}

impl TryFrom<&str> for LogLevel {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return match value {
            "SILENT" => Ok(LogLevel::SILENT),
            "QUIET" => Ok(LogLevel::QUIET),
            "NORMAL" => Ok(LogLevel::NORMAL),
            "DEBUG" => Ok(LogLevel::DEBUG),
            _ => Err(()),
        };
    }
}

#[derive(Clone, Debug, Default)]
pub struct BootEntry {
    pub name: String,
    pub partition: u32,
}

impl BootEntry {
    const KEY_NAME: &'static str = "name";
    const KEY_PARTITION: &'static str = "partition";

    pub const MAX_NAME_LENGTH: usize = 64;
}

macro_rules! config_path {
    () => {
        cstr16!("wtconfig.json")
    };
}

pub fn load_config(system_table: &SystemTable<Boot>) -> Result<(), Status> {
    println_force!("Loading config...");

    let mut efifs = match system_table.boot_services().get_image_file_system(image_handle!()) {
        Ok(ok) => FileSystem::new(ok),
        Err(err) => return Err(err.status()),
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

fn parse_config(buffer: Vec<u8>) -> Result<(), Status> {
    let buffer_str = &String::from_utf8(buffer).unwrap();
    let json = match JSONValue::load_and_verify(&buffer_str) {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("Failed to parse config file: {}", err);
            return Err(Status::ABORTED);
        }
    };

    macro_rules! get_config_var {
        ($key:ident, $prop:ident, $default:ident, $required:ident, bool) => {
            if does_exist_key_of_type(json, Config::$key, JSONValueType::Bool) {
                let setter = json.get_key_value(Config::$key).unwrap().read_boolean().unwrap();
                let mut config = CONFIG.write();
                config.$prop = setter;
                drop(config);
                dprintln!("Set property {} to {}", Config::$key, setter);
            } else {
                if $required {
                    eprintln!("Could not locate required key \"{}\" while parsing", Config::$key);
                    return Err(Status::ABORTED);
                }

                dprintln!("Property {} defaulted to {}", Config::$key, Config::$default);
            }
        };
    }

    // Set loglevel
    if does_exist_key_of_type(json, Config::KEY_LOG_LEVEL, JSONValueType::String) {
        let loglevel_str = json
            .get_key_value(Config::KEY_LOG_LEVEL)
            .unwrap()
            .read_string()
            .unwrap();
        let mut config = CONFIG.write();
        config.log_level = match LogLevel::try_from(loglevel_str) {
            Ok(ok) => ok,
            Err(_) => {
                print_force!(
                    "Unknown log level \"{}\", entering debug mode...",
                    loglevel_str
                );
                LogLevel::DEBUG
            }
        };
        drop(config);
    } else {
        dprintln!("Using default loglevel: {:?}", Config::DEFAULT_LOG_LEVEL);
    }

    get_config_var!(KEY_INSTANT_BOOT, instant_boot, DEFAULT_INSTANT_BOOT, false, bool);
    get_config_var!(KEY_OFFER_SHELL, offer_shell, DEFAULT_OFFER_SHELL, false, bool);
    get_config_var!(KEY_EDIT_CONFIG, edit_config, KEY_EDIT_CONFIG, false, bool);
    get_config_var!(KEY_MENU_CLEAR, menu_clear, DEFAULT_MENU_CLEAR, false, bool);

    // Load boot entries
    if does_exist_required_key_of_type(json, Config::KEY_BOOTENTRIES, JSONValueType::Array) {
        let boot_entries = json
            .get_key_value(Config::KEY_BOOTENTRIES)
            .unwrap()
            .iter_array()
            .unwrap();
        for bootentry_json in boot_entries {
            let bootentry = match parse_bootentry(bootentry_json) {
                Ok(ok) => ok,
                Err(err) => {
                    eprintln!("Failed to parse config file: {}", err);
                    return Err(Status::ABORTED);
                }
            };

            dprintln!("Detected boot entry: {:?}", bootentry);

            let mut config = CONFIG.write();
            config.boot_entries.push(bootentry);
            drop(config);
        }
    } else {
        return Err(Status::ABORTED);
    }

    let config = CONFIG.read();
    let boot_entries_count = config.boot_entries.len();
    drop(config);
    
    if boot_entries_count == 0 {
        wprintln!("No boot entries provided, enabling UEFI shell, config editor, and halting for user input...");
        let mut config = CONFIG.write();
        config.offer_shell = true;
        config.edit_config = true;
        config.instant_boot = false;
        drop(config);
    }

    Ok(())
}

fn parse_bootentry(json: JSONValue) -> Result<BootEntry, JSONParsingError> {
    if json.value_type != JSONValueType::Object {
        eprintln!("Non-object in boot entries");
        return Err(JSONParsingError::CannotParseObject);
    }

    let mut bootentry: BootEntry = BootEntry::default();

    // Boot entry name
    if !does_exist_required_key_of_type(json, BootEntry::KEY_NAME, JSONValueType::String) {
        return Err(JSONParsingError::KeyNotFound);
    }
    bootentry.name = String::from(
        json.get_key_value(BootEntry::KEY_NAME)
            .unwrap()
            .read_string()
            .unwrap(),
    );

    if bootentry.name.len() > BootEntry::MAX_NAME_LENGTH {
        eprintln!("Boot entry name exceeds 64 characters");
        return Err(JSONParsingError::EndOfStream);
    }

    // Boot entry partition
    if !does_exist_required_key_of_type(json, BootEntry::KEY_PARTITION, JSONValueType::Number) {
        return Err(JSONParsingError::KeyNotFound);
    }
    bootentry.partition = json
        .get_key_value(BootEntry::KEY_PARTITION)
        .unwrap()
        .read_integer()
        .unwrap() as u32;

    Ok(bootentry)
}

#[allow(dead_code)]
fn does_exist_key(json: JSONValue, key: &str) -> bool {
    return match json.get_key_value(key) {
        Ok(_) => true,
        Err(_) => false,
    };
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
            JSONValueType::Error => panic!("Unreachable statement"),
        };
        eprintln!(
            "Could not locate required {} \"{}\" while parsing",
            value_type_pretty, key
        );
    }
    ret
}
