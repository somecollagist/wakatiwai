/// BIOS Parameter block, common to all FAT filesystems.
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct BPB {
    /// Legacy jumper code.
    jump: [u8; 3],
    /// OEM Identifier.
    oem: [u8; 8],
    /// The number of bytes per sector.
    pub bytes_per_sector: u16,
    /// The number of sectors per cluster.
    pub sectors_per_cluster: u8,
    /// The number of reserved sectors including boot record sectors.
    pub reserved_sector_count: u16,
    /// Number of File Allocation Tables (FATs) on the partition.
    pub table_count: u8,
    /// Number of root directory entries.
    pub root_dir_entry_count: u16,
    /// The total number of sectors in the partition. If 0, more than `u16::MAX` sectors are tracked, consult `BPB.large_sector_count`.
    pub small_sector_count: u16,
    /// Describes the type of media this file system exists on.
    media_descriptor_type: u8,
    #[doc(hidden)]
    reserved: u16,
    /// Number of sectors per track.
    sectors_per_track: u16,
    /// Number of sides on the storage media.
    sides_on_media: u16,
    /// The LBA of the start of the partition.
    partition_lba_start: u32,
    /// The total number of sectors in the partition if greater than `u16::MAX`.
    pub large_sector_count: u32,
}

/// Extended boot record for FAT32.
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct EBPB {
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

impl EBPB {
    /// Shortened boot signature for the EBR.
    pub const SHORTENED_BOOT_SIGNATURE: u8 = 0x28;
    /// Extended boot signature for the EBR - permits access to certain properties.
    pub const EXTENDED_BOOT_SIGNATURE: u8 = 0x29;

    pub fn volume_label(&self) -> Option<[u8; 11]> {
        if self.signature != Self::EXTENDED_BOOT_SIGNATURE {
            return None;
        }
        Some(self.volume_label)
    }

    pub fn system_identifier(&self) -> Option<[u8; 8]> {
        if self.signature != Self::EXTENDED_BOOT_SIGNATURE {
            return None;
        }
        Some(self.fs_type_label)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct FSInfo {
    /// Lead signature.
    lead_signature: u32,
    #[doc(hidden)]
    reserved0: [u8; 480],
    /// Middle signature.
    mid_signature: u32,
    /// The last known free cluster count on the volume. This should be checked; accuracy cannot be guaranteed.
    last_known_free_cluster_count: u32,
    /// The cluster number at which the file system should start looking for available clusters.
    available_cluster_hint: u32,
    #[doc(hidden)]
    reserved1: [u8; 12],
    /// Trailing signature.
    trail_signature: u32
}

impl FSInfo {
    pub const LEAD_FSINFO_SIGNATURE: u32 = 0x41615252;
    pub const MID_FSINFO_SIGNATURE: u32 = 0x61417272;
    pub const TRAIL_FSINFO_SIGNATURE: u32 = 0xAA550000;
}