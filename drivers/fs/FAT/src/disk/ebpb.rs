use core::fmt::Debug;

/// Shortened boot signature for the EBR.
#[allow(unused)]
const SHORTENED_BOOT_SIGNATURE: u8 = 0x28;
/// Extended boot signature for the EBR - permits access to certain properties.
#[allow(unused)]
const EXTENDED_BOOT_SIGNATURE: u8 = 0x29;

// pub trait EBPB: Debug + Sized {
//     fn volume_label(&self) -> Option<[u8; 11]>;
//     fn system_identifier(&self) -> Option<[u8; 8]>;

//     fn sectors_per_fat(&self) -> Option<u32>;
//     fn root_dir_cluster(&self) -> Option<u32>;
//     fn fsinfo_cluster(&self) -> Option<u16>;
// }

#[derive(Debug)]
pub enum EBPB {
    #[allow(unused)]
    _1216(EBPB_1216),
    _32(EBPB_32)
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct EBPB_1216 {
    /// The drive number - almost certainly useless since this doesn't consider removable media.
    drive_number: u8,
    /// Flags in windows NT, otherwise reserved.
    reserved: u8,
    /// FAT signature.
    signature: u8,
    /// VolumeID serial number, used for tracking volumes between computers.
    serial_number: u32,
    /// The label of this volume.
    volume_label: [u8; 11],
    /// A representation of this file system, for display purposes only.
    fs_type_label: [u8; 8],
    /// Boot code, can be safely ignored.
    boot_code: [u8; 448],
    /// Bootable partition signature 0xAA55.
    boot_signature: u16
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct EBPB_32 {
    /// Number of sectors per FAT.
    pub sectors_per_fat: u32,
    /// Flags.
    flags: u16,
    /// Major FAT32 version.
    version_number_major: u8,
    /// Minor FAT32 version.
    version_number_minor: u8,
    /// Cluster of the root directory.
    pub root_dir_cluster: u32,
    /// Sector containing the FSInfo structure.
    pub fsinfo_sector: u16,
    /// Sector containing the backup boot.
    backup_boot_sector: u16,
    #[doc(hidden)]
    reserved0: [u8; 12],
    /// The drive number - almost certainly useless since this doesn't consider removable media.
    drive_number: u8,
    /// Flags in windows NT, otherwise reserved.
    reserved1: u8,
    /// FAT signature.
    signature: u8,
    /// VolumeID serial number, used for tracking volumes between computers.
    serial_number: u32,
    /// The label of this volume.
    volume_label: [u8; 11],
    /// A representation of this file system, for display purposes only.
    fs_type_label: [u8; 8],
    /// Boot code, can be safely ignored.
    boot_code: [u8; 420],
    /// Bootable partition signature 0xAA55.
    boot_signature: u16
}