mod fat;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::dev::reader::DiskReader;
use crate::wtcore::FS;
use crate::eprintln;

#[derive(Debug)]
pub enum FileSystemOperationError {
    ReadDirectoryAsFile,
    NonAbsolutePath,
    FileNotFound
}

trait FileSystemAPI {
    fn load_file(&self, path: &str, reader: &DiskReader) -> Result<Vec<u8>, FileSystemOperationError>;
}

pub struct FileSystem{
    backing: Box<dyn FileSystemAPI>,
    reader: DiskReader
}

impl FileSystem {
    pub fn new_filesystem(fs: FS, reader: DiskReader) -> Option<FileSystem> {
        let backing: Box<dyn FileSystemAPI> = match fs {
            FS::FAT12 | FS::FAT16 => Box::new(fat::fat12_16::FAT12_16::new(&reader, fs)) as Box<dyn FileSystemAPI>,
            FS::FAT32 => Box::new(fat::fat32::FAT32::new(&reader)) as Box<dyn FileSystemAPI>,
            _ => {
                eprintln!("Cannot create backing in-memory filesystem for type \"{:?}\"", fs);
                return None;
            }
        };

        Some(FileSystem {
            backing,
            reader
        })
    }

    pub fn load_file(&self, path: &str) -> Result<Vec<u8>, FileSystemOperationError> {
        if !path.starts_with("/") {
            return Err(FileSystemOperationError::NonAbsolutePath);
        }

        self.backing.load_file(path, &self.reader)
    }
}


