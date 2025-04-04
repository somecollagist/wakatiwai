#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct BPB {
    /// Legacy jumper code.
    jump: [u8; 3],
    /// OEM Identifier.
    pub oem: [u8; 8],
    /// The number of bytes per sector.
    pub bytes_per_sector: u16,
    /// The number of sectors per cluster.
    pub sectors_per_cluster: u8,
    /// The number of reserved sectors including boot record sectors.
    pub reserved_sector_count: u16,
    /// Number of File Allocation Tables (FATs) on the partition.
    pub fat_count: u8,
    /// Number of root directory entries.
    pub root_dir_entry_count: u16,
    /// The total number of sectors in the partition. If 0, more than `u16::MAX` sectors are tracked, consult `BPB.large_sector_count`.
    pub small_sector_count: u16,
    /// Describes the type of media this file system exists on.
    pub media_descriptor_type: u8,
    /// Number of sectors per FAT
    pub small_sectors_per_fat: u16,
    /// Number of sectors per track.
    pub sectors_per_track: u16,
    /// Number of sides on the storage media.
    pub sides_on_media: u16,
    /// The LBA of the start of the partition.
    pub partition_lba_start: u32,
    /// The total number of sectors in the partition if greater than `u16::MAX`.
    pub large_sector_count: u32,
}