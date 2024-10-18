mod data;
mod disk;

extern crate alloc;

use alloc::vec::Vec;
use core::mem::size_of;
use core::slice::from_raw_parts;

use crate::dev::reader::DiskReader;
use crate::fs::{FileSystemAPI, FileSystemOperationError};
use crate::wprintln;

pub struct FAT32 {
    bpb: disk::BPB,
    ebpb: disk::EBPB,
    fsinfo: disk::FSInfo,
    root_dir: Vec<data::DirectoryEntry>
}

impl FAT32 {
    pub fn new(reader: &DiskReader) -> Self {
        let boot_record_buffer = reader.read_sector(0).unwrap();
        let bpb = unsafe { *(boot_record_buffer[0..size_of::<disk::BPB>()].as_ptr() as *const disk::BPB) };
        let ebpb = unsafe { *(boot_record_buffer[size_of::<disk::BPB>()..size_of::<disk::BPB>()+size_of::<disk::EBPB>()].as_ptr() as *const disk::EBPB) };
        drop(boot_record_buffer);

        let fsinfo_buffer = reader.read_sector(ebpb.fsinfo_sector as u64).unwrap();
        let fsinfo = unsafe { *(fsinfo_buffer[0..size_of::<disk::FSInfo>()].as_ptr() as *const disk::FSInfo) };
        drop(fsinfo_buffer);

        let mut ret = Self {
            bpb,
            ebpb,
            fsinfo,
            root_dir: Vec::new()
        };

        let root_dir_buffer = ret.read_cluster_chain(reader, ebpb.root_dir_cluster);
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

    fn fat_size(&self) -> u32 {
        self.ebpb.sectors_per_fat
    }

    fn root_dir_sectors(&self) -> u64 {
        (((self.bpb.root_dir_entry_count * 32) + (self.bpb.bytes_per_sector - 1)) / self.bpb.bytes_per_sector) as u64
    }

    fn first_fat_sector(&self) -> u64 {
        self.bpb.reserved_sector_count as u64
    }

    fn first_data_sector(&self) -> u64 {
        self.first_fat_sector() + (self.bpb.table_count as u32 * self.fat_size()) as u64 + self.root_dir_sectors()
    }

    fn data_sectors(&self) -> u64 {
        self.total_sectors() - self.first_data_sector()
    }

    fn total_clusters(&self) -> u64 {
        self.data_sectors() / self.bpb.sectors_per_cluster as u64
    }

    fn get_next_cluster(&self, reader: &DiskReader, cluster: u32) -> Option<u32> {
        if cluster == 0 || cluster == 1 {
            wprintln!("Origin cluster is reserved");
            return None;
        }

        let table_value = u32::from_le_bytes(
            reader.read_bytes(
                (self.first_fat_sector() * reader.sector_size as u64) + (cluster as u64 * 4),
                size_of::<u32>()
            ).unwrap().try_into().unwrap()
        ) & 0x0FFFFFFF;

        match table_value {
            0x0FFFFFF7 => {
                // Bad cluster
                wprintln!("Bad cluster");
                return None;
            },
            0x0FFFFFF8..=u32::MAX => {
                // End of cluster chain
                return None;
            },
            0x00000000 => {
                // Free cluster
                wprintln!("Free cluster");
                return None;
            },
            0x00000001 => {
                // Reserved cluster
                wprintln!("Reserved cluster");
                return None;
            },
            _ => {
                return Some(table_value);
            }
        }
    }

    fn read_cluster_chain(&self, reader: &DiskReader, starting_cluster: u32) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut cluster = starting_cluster;
        loop {
            buffer.append(
                &mut reader.read_sectors(
                    ((cluster as u64 - 2) * self.bpb.sectors_per_cluster as u64) + self.first_data_sector(),
                    self.bpb.sectors_per_cluster as usize
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

impl FileSystemAPI for FAT32 {
    fn load_file(&self, path: &str, reader: &DiskReader) -> Result<Vec<u8>, FileSystemOperationError> {
        let mut dir_levels: Vec<&str> = path.split("/").collect();
        dir_levels.remove(0); // remove leading root dir qualifier
        let file_name = dir_levels.remove(dir_levels.len()-1);
        
        let mut dir_entries = self.root_dir.clone();
        for dir_level in dir_levels {
            match dir_entries.iter().find(|t| t.name() == dir_level.to_uppercase() ) {
                Some(some) => {
                    if !some.is_directory() {
                        return Err(FileSystemOperationError::FileNotFound);
                    }

                    dir_entries = self.read_dir_raw(
                        &self.read_cluster_chain(reader, some.metadata.first_cluster())
                    );
                }
                None => {
                    return Err(FileSystemOperationError::FileNotFound);
                }
            }
        }

        match dir_entries.iter().find(|t| t.name() == file_name.to_uppercase() ) {
            Some(some) => {
                if !some.is_file() {
                    return Err(FileSystemOperationError::ReadDirectoryAsFile);
                }

                let mut ret = self.read_cluster_chain(reader, some.metadata.first_cluster());
                ret.truncate(some.metadata.file_size as usize);
                return Ok(ret.to_vec());
            }
            None => {
                return Err(FileSystemOperationError::FileNotFound);
            }
        }
    }
}