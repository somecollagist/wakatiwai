use core::str::FromStr;

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

#[derive(Clone, Debug, Default, PartialEq)]
pub enum FS {
    #[default]
    UNKNOWN,
    EXT2,
    FAT12,
    FAT16,
    FAT32
}

impl FromStr for FS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ext2" => Ok(Self::EXT2),
            "fat12" => Ok(Self::FAT12),
            "fat16" => Ok(Self::FAT16),
            "fat32" => Ok(Self::FAT32),
            _ => Ok(Self::default())
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
            "UEFI" => Ok(Self::UEFI),
            "ELF" => Ok(Self::ELF),
            _ => Ok(Self::default())
        }
    }
}