extern crate alloc;

use alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::{type_name, TypeId};
use core::fmt::Debug;

use microjson::*;
use uefi::prelude::*;

use crate::*;
use crate::wtcore::*;
use crate::wtcore::config::*;

#[doc(hidden)]
macro_rules! unwrap_json_var {
    ( $get_config_var:expr ) => {
        match $get_config_var {
            Ok(ok) => ok,
            Err(_) => {
                return Err(Status::ABORTED);
            }
        }
    };
}

/// Parses a buffer and sets the bootloader configuration accordingly.
pub fn parse_config(buffer: Vec<u8>) -> Result<(), Status> {
    // Converts the buffer to a string and checks if it's valid JSON
    let buffer_string = &String::from_utf8(buffer).unwrap();
    let json = match JSONValue::load_and_verify(&buffer_string) {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("Failed to parse config file: {}", err);
            return Err(Status::ABORTED);
        }
    };

    // Write log level earlier so conditional prints work properly
    let log_level = unwrap_json_var!(get_json_var::<LogLevel>(&json, Config::KEY_LOG_LEVEL, Config::DEFAULT_LOG_LEVEL, false, JSONValueType::String));
    let mut config = CONFIG.write();
    config.log_level = log_level.clone();
    drop(config);

    // Get config properties
    let timeout         = unwrap_json_var!(get_json_var::<i32>(&json, Config::KEY_TIMEOUT, Config::DEFAULT_TIMEOUT, false, JSONValueType::Number));
    let exit            = unwrap_json_var!(get_json_var::<bool>(&json, Config::KEY_EXIT, Config::DEFAULT_EXIT, false, JSONValueType::Bool));
    let edit_config     = unwrap_json_var!(get_json_var::<bool>(&json, Config::KEY_EDIT_CONFIG, Config::DEFAULT_EDIT_CONFIG, false, JSONValueType::Bool));
    let menu_clear      = unwrap_json_var!(get_json_var::<bool>(&json, Config::KEY_MENU_CLEAR, Config::DEFAULT_MENU_CLEAR, false, JSONValueType::Bool));

    // Get boot entries
    let mut boot_entries: Vec<BootEntry> = Vec::new();
    if match json.get_key_value(Config::KEY_BOOT_ENTRIES) {
        Ok(type_value) => { type_value.value_type == JSONValueType::Array }
        Err(_) => false
    } {
        let boot_entry_array = json.get_key_value(Config::KEY_BOOT_ENTRIES).unwrap().iter_array().unwrap();
        for bootentry_json in boot_entry_array {
            let bootentry = match parse_bootentry(bootentry_json) {
                Ok(ok) => ok,
                Err(err) => {
                    eprintln!("Failed to parse boot entry: {}", err);
                    return Err(Status::ABORTED);
                }
            };

            dprintln!("Detected boot entry: {}", bootentry);
            boot_entries.push(bootentry);
        }
    } else {
        wprintln!("No boot entries detected in config");
    }

    // Open writable lock on the config
    let mut config = CONFIG.write();
    *config = Config {
        log_level,
        timeout,
        exit,
        edit_config,
        menu_clear,
        boot_entries
    };
    if config.boot_entries.len() == 0 {
        // If no boot entries are available, offer exit, config edit, and wait for user input
        if config.log_level != LogLevel::SILENT {
            // wprintln! won't work here because it relies on a readable config lock, which will be blocked by the above writable lock
            wprintln_force!("No boot entries provided, enabling exit, config editor, and halting for user input...");
        }
        config.exit = true;
        config.edit_config = true;
        config.timeout = -1;
    }
    drop(config);

    Ok(())
}

/// Parses a JSON object and attempts to return a corresponding `BootEntry`.
fn parse_bootentry(json: JSONValue) -> Result<BootEntry, Status> {
    // Check if the JSON given is indeed an object
    if json.value_type != JSONValueType::Object {
        eprintln!("Non-object in boot entries");
        return Err(Status::COMPROMISED_DATA);
    }

    // Get boot entry properties
    let name            = unwrap_json_var!(get_json_var::<String>(&json, BootEntry::KEY_NAME, String::new(), true, JSONValueType::String));
    let mut disk_guid   = unwrap_json_var!(get_json_var::<Guid>(&json, BootEntry::KEY_DISK, Guid::ZERO, false, JSONValueType::String));
    let partition       = unwrap_json_var!(get_json_var::<u32>(&json, BootEntry::KEY_PARTITION, 0, true, JSONValueType::Number));
    let fs              = unwrap_json_var!(get_json_var::<FS>(&json, BootEntry::KEY_FS, FS::UNKNOWN, true, JSONValueType::String));
    let progtype        = unwrap_json_var!(get_json_var::<Progtype>(&json, BootEntry::KEY_PROGTYPE, Progtype::UNKNOWN, true, JSONValueType::String));
    let path            = unwrap_json_var!(get_json_var::<String>(&json, BootEntry::KEY_PATH, String::new(), true, JSONValueType::String));
    let initrd          = unwrap_json_var!(get_json_var::<String>(&json, BootEntry::KEY_INITRD, String::new(), false, JSONValueType::String));
    let args            = unwrap_json_var!(get_json_var::<String>(&json, BootEntry::KEY_ARGS, String::new(), false, JSONValueType::String));

    if disk_guid == Guid::ZERO {
        wprintln!("Disk property missing or malformed, assuming current...");
        disk_guid = *dev::BOOTLOADER_DISK_GUID;
    }

    if fs == FS::UNKNOWN {
        eprintln!("Unknown filesystem \"{}\" specified", json.get_key_value(BootEntry::KEY_FS).unwrap().read_string().unwrap());
        return Err(Status::ABORTED);
    }
    if progtype == Progtype::UNKNOWN {
        eprintln!("Unknown program type \"{}\" specified", json.get_key_value(BootEntry::KEY_PROGTYPE).unwrap().read_string().unwrap());
        return Err(Status::ABORTED);
    }

    Ok(BootEntry {
        name,
        disk_guid,
        partition,
        fs,
        progtype,
        path,
        initrd,
        args
    })
}

/// Gets a variable from a JSON object.
fn get_json_var<T: Default + Debug + FromStr + 'static>(json: &JSONValue, key: &str, default: T, required: bool, json_type: JSONValueType) -> Result<T, Status> {
    if !match json.get_key_value(key) {
        Ok(type_value) => { type_value.value_type == json_type }
        Err(_) => false
    } {
        // Enters if a key with the specified name and type was not found

        // If the key is required, exit
        if required {
            let json_type_pretty = match json_type {
                JSONValueType::Array => "array",
                JSONValueType::Bool => "boolean",
                JSONValueType::Null => "null",
                JSONValueType::Number => "numerical",
                JSONValueType::Object => "object",
                JSONValueType::String => "string",
                JSONValueType::Error => panic!("Unreachable statement"),
            };
            eprintln!(
                "Could not locate required {} key \"{}\" while parsing",
                json_type_pretty, key
            );
            return Err(Status::ABORTED);
        }

        // Otherwise, return the supplied default
        return Ok(default);
    }

    let value = json.get_key_value(key).unwrap();
    
    // We need to emit the variable in the specified type
    // ret is what will eventually be emitted, but how it's set depends on its type
    let ret: T;
    let t_type = TypeId::of::<T>();
    macro_rules! set_on_types {
        ( $( ($type_id:ident, $blk:block) ),+ ) => {
            /*
            Would be really cool if we could instead pass the type (bool, String, e.g.)
            and create the TypeId consts automatically - this would probably require
            treating the type parameter of the macro as a raw so a proper identifier can
            be made, but a map might be an alternative? Kudos if you find a solution <3
             */

            // This lets us create a dynamic else-if macro for as many types as we need
            if false {
                unreachable!()
            }
            $(
                // If T is of the specified type...
                else if t_type == $type_id {
                    // ...set ret according to $blk (unsafe because .unwrap_unchecked() requires it)
                    ret = unsafe { $blk }
                }
            )+
            // If the specified property cannot be deserialised (i.e. we haven't specified it), panic
            else {
                unimplemented!("Cannot obtain config var of type {}", type_name::<T>())
            }
        };
    }

    // Input supported types here
    const BOOL_TYPE: TypeId         = TypeId::of::<bool>();
    const FS_TYPE: TypeId           = TypeId::of::<FS>();
    const GUID_TYPE: TypeId         = TypeId::of::<Guid>();
    const I32_TYPE: TypeId          = TypeId::of::<i32>();
    const LOG_LEVEL_TYPE: TypeId    = TypeId::of::<LogLevel>();
    const PROGTYPE_TYPE: TypeId     = TypeId::of::<Progtype>();
    const STRING_TYPE: TypeId       = TypeId::of::<String>();
    const U32_TYPE: TypeId          = TypeId::of::<u32>();

    // Describe how to deserialise to given types here
    set_on_types!(
        (BOOL_TYPE, {
            T::from_str(
                &value.read_boolean().unwrap().to_string()
            ).unwrap_unchecked()
        }),
        (FS_TYPE, {
            T::from_str(
                &value.read_string().unwrap()
            ).unwrap_unchecked()
        }),
        (GUID_TYPE, {
            T::from_str(
                &value.read_string().unwrap()
            ).unwrap_or_else(|_| T::from_str(&Guid::ZERO.to_string()).unwrap_unchecked())
        }),
        (I32_TYPE, {
            T::from_str(
                &value.read_integer().unwrap().to_string()
            ).unwrap_unchecked()
        }),
        (LOG_LEVEL_TYPE, {
            T::from_str(
                value.read_string().unwrap()
            ).unwrap_unchecked()
        }),
        (PROGTYPE_TYPE, {
            T::from_str(
                &value.read_string().unwrap()
            ).unwrap_unchecked()
        }),
        (STRING_TYPE, {
            T::from_str(
                value.read_string().unwrap()
            ).unwrap_unchecked()
        }),
        (U32_TYPE, {
            T::from_str(
                &value.read_integer().unwrap().to_string()
            ).unwrap_unchecked()
        })
    );

    Ok(ret)
}