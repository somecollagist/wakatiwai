pub mod load;
mod parse;
pub mod write;

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Display;
use core::str::FromStr;

use spin::RwLock;
use uefi::{cstr16, CStr16, Guid};

/// The loaded configuration for the bootloader.
pub static CONFIG: RwLock<Config> = RwLock::new(Config::new());

/// Configuration variables for the bootloader.
#[derive(Debug)]
pub struct Config {
    /// The log level to be used. This value determines which kinds of messages can be printed to the screen.
    pub log_level: LogLevel,
    /// Determines if the default boot option should be booted immediately.
    pub instant_boot: bool,
    /// Determines if the option to exit to the UEFI shell should be offered.
    pub offer_shell: bool,
    /// Determines if the option to edit the bootloader configuration file should be offered.
    pub edit_config: bool,
    /// Determines if the screen should be cleared before the boot option menu is drawn.
    pub menu_clear: bool,
    /// An array describing all the boot entries in the bootloader configuration file.
    pub boot_entries: Vec<BootEntry>,
}

impl Config {
    #[doc(hidden)]
    const KEY_LOG_LEVEL: &'static str = "loglevel";
    #[doc(hidden)]
    const KEY_INSTANT_BOOT: &'static str = "instantboot";
    #[doc(hidden)]
    const KEY_OFFER_SHELL: &'static str = "offershell";
    #[doc(hidden)]
    const KEY_EDIT_CONFIG: &'static str = "editconfig";
    #[doc(hidden)]
    const KEY_MENU_CLEAR: &'static str = "menuclear";
    #[doc(hidden)]
    const KEY_BOOT_ENTRIES: &'static str = "bootentries";

    #[doc(hidden)]
    const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::NORMAL;
    #[doc(hidden)]
    const DEFAULT_INSTANT_BOOT: bool = false;
    #[doc(hidden)]
    const DEFAULT_OFFER_SHELL: bool = true;
    #[doc(hidden)]
    const DEFAULT_EDIT_CONFIG: bool = true;
    #[doc(hidden)]
    const DEFAULT_MENU_CLEAR: bool = true;

    /// Returns a default (i.e. empty) configuration.
    pub const fn new() -> Self {
        Config {
            log_level: Config::DEFAULT_LOG_LEVEL,
            instant_boot: Config::DEFAULT_INSTANT_BOOT,
            offer_shell: Config::DEFAULT_OFFER_SHELL,
            edit_config: Config::DEFAULT_EDIT_CONFIG,
            menu_clear: Config::DEFAULT_MENU_CLEAR,
            boot_entries: Vec::new(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
"{{
    {log_level_key}: {log_level_val:?},
    {instant_boot_key}: {instant_boot_val},
    {offer_shell_key}: {offer_shell_val},
    {edit_config_key}: {edit_config_val},
    {menu_clear_key}: {menu_clear_val}
}}",
            log_level_key = Config::KEY_LOG_LEVEL,
            log_level_val = self.log_level,
            instant_boot_key = Config::KEY_INSTANT_BOOT,
            instant_boot_val = self.instant_boot,
            offer_shell_key = Config::KEY_OFFER_SHELL,
            offer_shell_val = self.offer_shell,
            edit_config_key = Config::KEY_EDIT_CONFIG,
            edit_config_val = self.edit_config,
            menu_clear_key = Config::KEY_MENU_CLEAR,
            menu_clear_val = self.menu_clear
        )
    }
}

/// The logging levels to be used by the bootloader.
/// These will determine which messages can and cannot be printed.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum LogLevel {
    // Will produce no output bar critical failures
    SILENT,
    // Will output critical failures and warnings
    QUIET,
    // Will produce normal output
    NORMAL,
    // Will produce debug output
    #[default]
    DEBUG,
}

impl FromStr for LogLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SILENT" => Ok(LogLevel::SILENT),
            "QUIET" => Ok(LogLevel::QUIET),
            "NORMAL" => Ok(LogLevel::NORMAL),
            "DEBUG" => Ok(LogLevel::DEBUG),
            _ => Ok(LogLevel::default()),
        }
    }
}

/// Describes the properties of a boot option.
#[derive(Clone, Debug, Default)]
pub struct BootEntry {
    /// The name of the boot option, displayed to the user.
    pub name: String,
    /// The GUID of the disk containing this boot option.
    pub disk_guid: Guid,
    /// The partition of the disk containing this boot option.
    pub partition: u32,
    /// The type of file system upon which this boot option resides.
    pub fs: FS,
    /// The type of program this boot option points to.
    pub progtype: Progtype
}

impl BootEntry {
    #[doc(hidden)]
    const KEY_NAME: &'static str = "name";
    #[doc(hidden)]
    const KEY_DISK :&'static str = "diskguid";
    #[doc(hidden)]
    const KEY_PARTITION: &'static str = "partition";
    #[doc(hidden)]
    const KEY_FS :&'static str = "fs";
    #[doc(hidden)]
    const KEY_PROGTYPE :&'static str = "progtype";

    /// The maximum name length for a boot entry.
    pub const MAX_NAME_LENGTH: usize = 64;
}

impl Display for BootEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
"{{
    {name_key}: {name_val},
    {disk_key}: {disk_val}
    {partition_key}: {partition_val}
    {fs_key}: {fs_val:?}
    {progtype_key}: {progtype_val:?}
}}",
            name_key = BootEntry::KEY_NAME, name_val = self.name,
            disk_key = BootEntry::KEY_DISK, disk_val = self.disk_guid,
            partition_key = BootEntry::KEY_PARTITION, partition_val = self.partition,
            fs_key = BootEntry::KEY_FS, fs_val = self.fs,
            progtype_key = BootEntry::KEY_PROGTYPE, progtype_val = self.progtype
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum FS {
    #[default]
    UNKNOWN,
    FAT12,
    FAT16,
    FAT32
}

impl FromStr for FS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fat12" => Ok(FS::FAT12),
            "fat16" => Ok(FS::FAT16),
            "fat32" => Ok(FS::FAT32),
            _ => Ok(FS::default())
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Progtype {
    #[default]
    UNKNOWN,
    UEFI,
    ELF
}

impl FromStr for Progtype {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UEFI" => Ok(Progtype::UEFI),
            "ELF" => Ok(Progtype::ELF),
            _ => Ok(Progtype::default())
        }
    }
}

/// Path to the bootloader configuration file.
const CONFIG_PATH: &CStr16 = cstr16!("wtconfig.json");
