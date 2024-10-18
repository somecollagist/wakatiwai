extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::{CStr16, CString16, Char16};

use crate::fs::fat;

#[derive(Clone, Debug, Default)]
pub struct DirectoryEntry {
    pub long_name: Vec<DirectoryEntryLongFileName>,
    pub metadata: DirectoryEntryMetadata
}

impl DirectoryEntry {
    pub fn name(&self) -> String {
        if self.long_name.len() == 0 {
            unsafe {
                return if self.metadata.extension != [0x20, 0x20, 0x20] { // If the extension isn't just spaces
                    format!(
                        "{}.{}",
                        String::from_utf8_unchecked(self.metadata.name.to_vec()).trim(),
                        String::from_utf8_unchecked(self.metadata.extension.to_vec()).trim()
                    ).to_uppercase()
                }
                else {
                    String::from_utf8_unchecked(self.metadata.name.to_vec()).trim().to_owned().to_uppercase()
                }
            }
        }
        
        let mut name_builder = Vec::new();
        for long_name in self.long_name.iter() {
            name_builder.append(&mut Vec::from(long_name.name_high));
            name_builder.append(&mut Vec::from(long_name.name_mid));
            name_builder.append(&mut Vec::from(long_name.name_low));
        }

        name_builder.split_off(
            name_builder
            .iter()
            .position(
                |t| unsafe { *t == Char16::from_u16_unchecked(0x0000) }
            ).unwrap_or_else(|| name_builder.len())
        ).truncate(name_builder.len());
        name_builder.push(unsafe { Char16::from_u16_unchecked(0x0000) });

        let mut ret_cstr16 = CString16::new();
        ret_cstr16.push_str(CStr16::from_char16_with_nul(&name_builder).unwrap());
        ret_cstr16.to_string().to_uppercase()
    }

    pub fn is_file(&self) -> bool {
        !self.is_directory()
    }

    pub fn is_directory(&self) -> bool {
        self.metadata.attributes & DirectoryEntryMetadata::ATTRIBUTE_DIRECTORY != 0
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct DirectoryEntryMetadata {
    name: [u8; 8],
    extension: [u8; 3],
    attributes: u8,
    reserved: u8,
    creation_time_cs: u8,
    creation_time: fat::Time,
    creation_date: fat::Date,
    accessed_date: fat::Date,
    zero: u16,
    modified_time: fat::Time,
    modified_date: fat::Date,
    pub first_cluster: u16,
    pub file_size: u32
}

impl DirectoryEntryMetadata {
    const ATTRIBUTE_READ_ONLY: u8   = 0x01;
    const ATTRIBUTE_HIDDEN: u8      = 0x02;
    const ATTRIBUTE_SYSTEM: u8      = 0x04;
    const ATTRIBUTE_VOLUME_ID: u8   = 0x08;
    const ATTRIBUTE_DIRECTORY: u8   = 0x10;
    const ATTRIBUTE_ARCHIVE: u8     = 0x20;
    const ATTRIBUTE_LFN: u8         = 0x0F;
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct DirectoryEntryLongFileName {
    order: u8,
    name_high: [Char16; 5],
    attributes: u8,
    long_entry_type: u8,
    checksum: u8,
    name_mid: [Char16; 6],
    zero: u16,
    name_low: [Char16; 2]
}