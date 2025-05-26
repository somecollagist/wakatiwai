mod bpb;
mod ebpb;
mod fsinfo;

pub mod directory_entry;

use alloc::vec::Vec;
use directory_entry::{DirectoryEntryLongFileName, DirectoryEntryMetadata};
use wakatiwai_udive::disk::DiskReader;

use crate::{data::DirectoryEntry, FATType};

#[derive(Debug)]
pub struct FAT {
    pub fat_type: Option<FATType>,
    bpb: bpb::BPB,
    ebpb: ebpb::EBPB
}

impl FAT {
    pub fn new(diskreader: &DiskReader) -> Self {
        // Read the first sector of the disk, containing the BPB and EBPB
        let boot_record_buffer = diskreader.read_sector(0).unwrap();
        let bpb = unsafe {
            *(boot_record_buffer[
                0..size_of::<bpb::BPB>()
            ].as_ptr() as *const bpb::BPB)
        };
        let ebpb_1216_buffer = unsafe {
            *(boot_record_buffer[
                size_of::<bpb::BPB>()..size_of::<bpb::BPB>()+size_of::<ebpb::EBPB_1216>()
            ].as_ptr() as *const ebpb::EBPB_1216)
        };
        let ebpb_32_buffer = unsafe {                                                   // There's a good chance that this is wrong.
            *(boot_record_buffer[                                                       // However, we will put this in for now anyway
                size_of::<bpb::BPB>()..size_of::<bpb::BPB>()+size_of::<ebpb::EBPB_32>() // so we can read EBPB fields (use direct byte access)
            ].as_ptr() as *const ebpb::EBPB_32)
        };

        let mut ret = Self {
            fat_type: None,
            bpb,
            ebpb: ebpb::EBPB::_32(ebpb_32_buffer)
        };
        
        match ret.total_clusters() {
            0..4085 => {
                ret.fat_type    = Some(FATType::FAT12);
                ret.ebpb        = ebpb::EBPB::_1216(ebpb_1216_buffer);
            }
            4085..65524 => {
                ret.fat_type    = Some(FATType::FAT16);
                ret.ebpb        = ebpb::EBPB::_1216(ebpb_1216_buffer);

            }
            _ => {
                ret.fat_type    = Some(FATType::FAT32);
            }
        };

        ret
    }

    fn total_sectors(&self) -> u64 {
        if self.bpb.small_sector_count == 0 {
            self.bpb.large_sector_count as u64
        }
        else {
            self.bpb.small_sector_count as u64
        }
    }

    fn sectors_per_fat(&self) -> u64 {
        if self.bpb.small_sectors_per_fat != 0 {
            self.bpb.small_sectors_per_fat as u64
        }
        else {
            if let ebpb::EBPB::_32(_32) = self.ebpb {
                _32.sectors_per_fat as u64
            }
            else {
                unreachable!();
            }
        }

    }

    fn root_dir_sectors(&self) -> u64 {
        (((self.bpb.root_dir_entry_count * 32) + (self.bpb.bytes_per_sector - 1)) / self.bpb.bytes_per_sector) as u64
    }

    fn first_fat_sector(&self) -> u64 {
        self.bpb.reserved_sector_count as u64
    }

    fn first_data_sector(&self) -> u64 {
        self.first_fat_sector() + (self.bpb.fat_count as u64 * self.sectors_per_fat()) + self.root_dir_sectors()
    }

    fn data_sectors(&self) -> u64 {
        self.total_sectors() - self.first_data_sector()
    }

    fn total_clusters(&self) -> u64 {
        self.data_sectors() / self.bpb.sectors_per_cluster as u64
    }

    fn get_next_cluster(&self, diskreader: &DiskReader, current_cluster: u32) -> Option<u32> {
        if current_cluster <= 1 {
            // Origin cluster is reserved
            return None;
        }

        let table_value = match self.fat_type.unwrap() {
            FATType::FAT12 => {
                u16::from_le_bytes(
                    diskreader.read_bytes(
                        (self.first_fat_sector() * diskreader.sector_size as u64) + (current_cluster + (current_cluster / 2)) as u64,
                        size_of::<u16>()
                    ).unwrap().try_into().unwrap()
                ) as u32
            }
            FATType::FAT16 => {
                u16::from_le_bytes(
                    diskreader.read_bytes(
                        (self.first_fat_sector() * diskreader.sector_size as u64) + (current_cluster * 2) as u64,
                        size_of::<u16>()
                    ).unwrap().try_into().unwrap()
                ) as u32
            }
            FATType::FAT32 => {
                u32::from_le_bytes(
                    diskreader.read_bytes(
                        (self.first_fat_sector() * diskreader.sector_size as u64) + (current_cluster * 4) as u64,
                        size_of::<u32>()
                    ).unwrap().try_into().unwrap()
                ) & 0x0FFFFFFF
            }
        };

        match (self.fat_type.unwrap(), table_value) {
            // Bad clusters
            (FATType::FAT12, 0xFF7)                     |
            (FATType::FAT16, 0xFFF7)                    |
            (FATType::FAT32, 0x0FFFFFF7)                |
            // End of cluster chain
            (FATType::FAT12, 0xFF8..=0xFFFF)            |
            (FATType::FAT16, 0xFFF8..=0xFFFF)           |
            (FATType::FAT32, 0x0FFFFFF8..=0xFFFFFFFF)   |
            // Free clusters
            (_, 0x0)                                    |
            // Reserved clusters
            (_, 0x1)
            => {
                return None
            }
            // Next used cluster
            _ => {
                Some(table_value)
            }
        }
    }

    pub fn read_cluster_chain(&self, diskreader: &DiskReader, start_cluster: u32) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut current_cluster = start_cluster;
        loop {
            buffer.append(
                &mut diskreader.read_sectors(
                    ((current_cluster as u64 - 2) * self.bpb.sectors_per_cluster as u64) + self.first_data_sector(),
                    self.bpb.sectors_per_cluster as usize
                ).unwrap()
            );

            match self.get_next_cluster(diskreader, current_cluster) {
                Some(some) => {
                    current_cluster = some;
                }
                None => {
                    break;
                }
            }
        }

        buffer
    }

    pub fn get_root_directory(&self, diskreader: &DiskReader) -> Vec<DirectoryEntry> {
        let root_directory_buffer: Vec<u8> = match self.ebpb {
            ebpb::EBPB::_1216(_) => {
                diskreader.read_sectors(
                    self.first_data_sector() - self.root_dir_sectors(),
                    self.root_dir_sectors() as usize
                ).unwrap()
            }
            ebpb::EBPB::_32(_32) => {
                self.read_cluster_chain(
                    diskreader,
                    _32.root_dir_cluster
                )
            }
        };
        self.read_raw_directory(&root_directory_buffer)
    }

    pub fn read_raw_directory(&self, data: &[u8]) -> Vec<DirectoryEntry> {
        const ENTRY_DATA_STRUCT_SIZE: usize = 32;
        const ENTRY_DATA_STRUCT_ATTR_OFFSET: usize = 11;

        let mut ret = Vec::new();
        let mut entry_long_file_name_buffer = Vec::new();
        for entry_idx in 0..data.len()/ENTRY_DATA_STRUCT_SIZE {
            let entry_offset = entry_idx*ENTRY_DATA_STRUCT_SIZE;
            
            match *data.get(entry_offset).unwrap() {
                0x00 => {
                    break;      // No more entries in the directory
                }
                0xE5 => {
                    continue;   // Entry is unused
                }
                _ => {}
            }

            let attrs = *data.get(entry_offset+ENTRY_DATA_STRUCT_ATTR_OFFSET).unwrap();
            if attrs == 0x0F {
                // Entry is a long name
                unsafe {
                    entry_long_file_name_buffer.insert(
                        0,  // These go in reverse order because Bill Gates hates me :(
                        *(data.as_ptr().add(entry_offset) as *const DirectoryEntryLongFileName)
                    )
                }
            }
            else {
                // Entry is metadata
                unsafe {
                    ret.push(
                        DirectoryEntry {
                            long_name: entry_long_file_name_buffer.clone(),
                            metadata: *(data.as_ptr().add(entry_offset) as *const DirectoryEntryMetadata)
                        }
                    )
                }

                entry_long_file_name_buffer.clear();
            }
        }

        ret
    }
}