extern crate alloc;

use alloc::{borrow::ToOwned, format, string::{String, ToString}, vec::Vec};
use uefi::{CStr16, CString16, Char16};

use super::disk::directory_entry::*;

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