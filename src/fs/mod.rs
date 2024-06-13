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
    ReadFileAsDirectory,
    NonAbsolutePath,
    FileNotFound
}

trait FileSystemAPI {
    fn load_file(&self, path: &str, reader: &DiskReader) -> Result<Vec<u8>, FileSystemOperationError>;
}

pub struct FileSystem<'fs>{
    backing: Box<dyn FileSystemAPI>,
    reader: &'fs DiskReader<'fs>
}

impl FileSystem<'_> {
    pub fn new_filesystem<'fs>(fs: FS, reader: &'fs DiskReader) -> Option<FileSystem<'fs>> {
        let backing: Box<dyn FileSystemAPI> = match fs {
            FS::FAT12 | FS::FAT16 => Box::new(fat::fat12_16::FAT12_16::new(reader, fs)),
            _ => {
                eprintln!("Cannot create backing in-memory filesystem for type \"{:?}\"", fs);
                return None;
            }
        } as Box<dyn FileSystemAPI>;

        Some(FileSystem {
            backing,
            reader
        })
    }

    pub fn load_file<'fs>(&self, path: &str) -> Result<Vec<u8>, FileSystemOperationError> {
        if !path.starts_with("/") {
            return Err(FileSystemOperationError::NonAbsolutePath);
        }

        self.backing.load_file(path, self.reader)
    }
}


