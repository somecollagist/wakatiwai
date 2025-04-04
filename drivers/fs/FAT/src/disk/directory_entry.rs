use uefi::Char16;

#[allow(unused)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Time(u16);
#[allow(unused)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Date(u16);

#[allow(unused)]
impl Time {
    pub fn hour(&self) -> u8 {
        ((self.0 >> 10) & 0x3F) as u8
    }

    pub fn minute(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    pub fn second(&self) -> u8 {
        ((self.0 >> 0) & 0x0F) as u8 * 2
    }
}

#[allow(unused)]
impl Date {
    pub fn year(&self) -> u16 {
        ((self.0 >> 8) & 0xFF) as u16 + 1980
    }

    pub fn month(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    pub fn day(&self) -> u8 {
        ((self.0 >> 0) & 0x0F) as u8
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct DirectoryEntryMetadata {
    pub name: [u8; 8],
    pub extension: [u8; 3],
    pub attributes: u8,
    reserved: u8,
    creation_time_cs: u8,
    creation_time: Time,
    creation_date: Date,
    accessed_date: Date,
    first_cluster_high: u16,
    modified_time: Time,
    modified_date: Date,
    first_cluster_low: u16,
    pub file_size: u32
}

#[allow(unused)]
impl DirectoryEntryMetadata {
    pub const ATTRIBUTE_READ_ONLY: u8   = 0x01;
    pub const ATTRIBUTE_HIDDEN: u8      = 0x02;
    pub const ATTRIBUTE_SYSTEM: u8      = 0x04;
    pub const ATTRIBUTE_VOLUME_ID: u8   = 0x08;
    pub const ATTRIBUTE_DIRECTORY: u8   = 0x10;
    pub const ATTRIBUTE_ARCHIVE: u8     = 0x20;
    pub const ATTRIBUTE_LFN: u8         = 0x0F;

    pub fn first_cluster(&self) -> u32 {
        (self.first_cluster_high as u32) << 16 | (self.first_cluster_low as u32)
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct DirectoryEntryLongFileName {
    order: u8,
    pub name_high: [Char16; 5],
    attributes: u8,
    long_entry_type: u8,
    checksum: u8,
    pub name_mid: [Char16; 6],
    zero: u16,
    pub name_low: [Char16; 2]
}