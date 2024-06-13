mod disk;
pub mod data;

extern crate alloc;

use alloc::vec::Vec;
use core::mem::size_of;
use core::slice::from_raw_parts;

use crate::fs::*;
use crate::wprintln;

pub struct FAT12_16 {
    bpb: disk::BPB,
    ebpb: disk::EBPB,
    fstype: FS,
    root_dir: Vec<data::DirectoryEntry>
}

impl FAT12_16 {
    pub fn new(reader: &DiskReader, fstype: FS) -> Self {
        let boot_record_buffer = reader.read_sector(0).unwrap();

        let mut ret = unsafe {
            Self {
                bpb: *(boot_record_buffer[0..size_of::<disk::BPB>()].as_ptr() as *const disk::BPB),
                ebpb: *(boot_record_buffer[size_of::<disk::BPB>()..size_of::<disk::BPB>()+size_of::<disk::EBPB>()].as_ptr() as *const disk::EBPB),
                fstype,
                root_dir: Vec::new()
            }
        };
        drop(boot_record_buffer);
        
        let root_dir_buffer = reader.read_sectors(ret.first_data_sector() - ret.root_dir_sectors(), ret.root_dir_sectors() as usize).unwrap();
        ret.root_dir = ret.read_dir_raw(&root_dir_buffer);

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

    fn fat_size(&self) -> u16 {
        self.bpb.sectors_per_fat
    }

    fn root_dir_sectors(&self) -> u64 {
        (((self.bpb.root_dir_entry_count * 32) + (self.bpb.bytes_per_sector - 1)) / self.bpb.bytes_per_sector) as u64
    }

    fn first_fat_sector(&self) -> u64 {
        self.bpb.reserved_sector_count as u64
    }

    fn first_data_sector(&self) -> u64 {
        self.first_fat_sector() + (self.bpb.table_count as u16 * self.fat_size()) as u64 + self.root_dir_sectors()
    }

    fn data_sectors(&self) -> u64 {
        self.total_sectors() - self.first_data_sector()
    }

    fn total_clusters(&self) -> u64 {
        self.data_sectors() / self.bpb.sectors_per_cluster as u64
    }

    fn get_next_cluster(&self, reader: &DiskReader, cluster: u16) -> Option<u16> {
        if cluster == 0 || cluster == 1 {
            wprintln!("Origin cluster is reserved");
            return None;
        }

        if self.fstype == FS::FAT12 {
            // FAT12
            let fat_offset = cluster + (cluster / 2);
            let fat_sector = self.bpb.reserved_sector_count + (fat_offset / reader.sector_size as u16);
            let ent_offset = fat_offset % reader.sector_size as u16;
            
            let mut table_value = unsafe {
                *(reader.read_sectors(fat_sector as u64, 2).unwrap().as_ptr().add(ent_offset as usize) as *const u16)
            };
            table_value = if cluster & 0x01 == 1 { table_value >> 4 } else { table_value & 0xFFF};

            match table_value {
                0xFF7 => {
                    // Bad cluster
                    wprintln!("Bad cluster");
                    return None;
                },
                0xFF8..=u16::MAX => {
                    // End of cluster chain
                    return None;
                },
                0x000 => {
                    // Free cluster
                    wprintln!("Free cluster");
                    return None;
                },
                0x001 => {
                    // Reserved cluster
                    wprintln!("Reserved cluster");
                    return None;
                },
                _ => {
                    return Some(table_value);
                }
            }
        }
        else {
            // FAT16
            let fat_offset = cluster * 2;
            let fat_sector = self.bpb.reserved_sector_count + (fat_offset / reader.sector_size as u16);
            let ent_offset = fat_offset % reader.sector_size as u16;
            
            let table_value = unsafe {
                *(reader.read_sector(fat_sector as u64).unwrap().as_ptr().add(ent_offset as usize) as *const u16)
            };

            match table_value {
                0xFFF7 => {
                    // Bad cluster
                    wprintln!("Bad cluster");
                    return None;
                },
                0xFFF8..=u16::MAX => {
                    // End of cluster chain
                    return None;
                },
                0x000 => {
                    // Free cluster
                    wprintln!("Free cluster");
                    return None;
                },
                0x001 => {
                    // Reserved cluster
                    wprintln!("Reserved cluster");
                    return None;
                },
                _ => {
                    return Some(table_value);
                }
            }
        }  
    }

    fn read_cluster_chain(&self, reader: &DiskReader, starting_cluster: u16) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut cluster = starting_cluster;
        loop {
            buffer.append(
                &mut reader.read_sector(
                    ((cluster as u64 - 2) * self.bpb.sectors_per_cluster as u64) + self.first_data_sector()
                ).unwrap()
            );

            match self.get_next_cluster(reader, cluster) {
                Some(some) => {
                    cluster = some;
                }
                None => {
                    break;
                }
            }
        }

        buffer
    }

    fn read_dir_raw(&self, data: &[u8]) -> Vec<data::DirectoryEntry> {
        let mut ret = Vec::new();
        let mut entry_long_file_name_buffer = Vec::new();
        for entry_idx in 0..data.len()/32 {
            match *data.get(entry_idx*32).unwrap() {
                0x00 => {
                    // No more entries in directory
                    break;
                },
                0xE5 => {
                    // Entry is unused
                    continue;
                }
                _ => {}
            }

            if *data.get((entry_idx*32)+11).unwrap() == 0x0F {
                // Entry is a long name
                unsafe {
                    entry_long_file_name_buffer.insert(
                        0, // Long entry names go in reverse order, so insert to beginning
                        *(from_raw_parts(
                            data.as_ptr().add(entry_idx*32),
                            size_of::<data::DirectoryEntryLongFileName>()
                        ).as_ptr() as *const data::DirectoryEntryLongFileName)
                    );
                }
            }
            else {
                // Entry is metadata
                unsafe {
                    ret.push(
                        data::DirectoryEntry {
                            long_name: entry_long_file_name_buffer.clone(),
                            metadata: *(from_raw_parts(
                                data.as_ptr().add(entry_idx*32),
                                size_of::<data::DirectoryEntryMetadata>()
                            ).as_ptr() as *const data::DirectoryEntryMetadata)
                        }
                    )
                }

                entry_long_file_name_buffer.clear();
            }
        }

        ret
    }
}

impl FileSystemAPI for FAT12_16 {
    fn load_file(&self, path: &str, reader: &DiskReader) -> Result<Vec<u8>, FileSystemOperationError> {
        let mut dir_levels: Vec<&str> = path.split("/").collect();
        dir_levels.remove(0); // remove leading root dir qualifier
        let file_name = dir_levels.remove(dir_levels.len()-1);
        
        let mut dir_entries = self.root_dir.clone();
        for dir_level in dir_levels {
            match dir_entries.iter().find(|t| &t.name() == dir_level) {
                Some(some) => {
                    dir_entries = self.read_dir_raw(
                        &self.read_cluster_chain(reader, some.metadata.first_cluster)
                    );
                }
                None => {
                    return Err(FileSystemOperationError::FileNotFound);
                }
            }
        }

        match dir_entries.iter().find(|t| &t.name() == file_name) {
            Some(some) => {
                if !some.is_file() {
                    return Err(FileSystemOperationError::ReadDirectoryAsFile);
                }

                return Ok(self.read_cluster_chain(reader, some.metadata.first_cluster));
            }
            None => {
                return Err(FileSystemOperationError::FileNotFound);
            }
        }
    }
}