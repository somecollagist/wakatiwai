use core::ffi::c_void;

use alloc::vec::Vec;
use uefi::proto::media::disk::DiskIo;
use uefi::table::boot::ScopedProtocol;
use uefi::{Handle, Status};

use crate::dev::reader::DiskReader;
use crate::dev::DISK_GUID_HANDLE_MAPPING;
use crate::dev::gpt::GPT;
use crate::fs::{self, FileSystem};
use crate::wtcore::config::BootEntry;
use crate::{dprintln, println, system_table};

/// Possible failures that may occur when trying to boot a given entry.
#[derive(Debug)]
#[allow(dead_code)]
pub enum BootFailure {
    /// The specified disk could not be found.
    NoSuchDisk,
    /// A DiskIO protocol to the disk could not be opened.
    DiskIOProtocolFailure(Status),
    /// The disk's GPT could not be read successfully.
    BadGPT(Status),
    /// The specified partition does not exist on disk.
    NoSuchPartition,
    /// The file system was unknown.
    UnknownFS
}

/// Attempts to boot to a given entry.
/// Returns an `Some(BootFailure)` if booting failed, and None upon the exiting of a boot program.
pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure> {
    println!("Booting \"{}\"...", entry.name);
    dprintln!("{}", entry);
    let st = system_table!();

    // Get a handle to the disk
    let disk_handle: Handle;
    match DISK_GUID_HANDLE_MAPPING.get(&entry.disk_guid) {
        Some(some) => {
            disk_handle = unsafe { Handle::from_ptr(*some as *mut c_void).unwrap() };
        }
        None => {
            return Some(BootFailure::NoSuchDisk);
        }
    }

    let protocol: ScopedProtocol<DiskIo>;
    match st.boot_services().open_protocol_exclusive::<DiskIo>(disk_handle) {
        Ok(ok) => {
            protocol = ok;
        }
        Err(err) => {
            return Some(BootFailure::DiskIOProtocolFailure(err.status()));
        }
    }

    let mut reader = DiskReader::new(&disk_handle, &protocol, 0);

    let disk_gpt: GPT;
    match GPT::read_gpt(&reader) {
        Ok(ok) => {
            disk_gpt = ok;
        }
        Err(err) => {
            return Some(BootFailure::BadGPT(err));
        }
    }

    let filesystem: FileSystem;
    match disk_gpt.entries.get((entry.partition-1) as usize) {
        Some(some) => {
            reader.abs_offset = some.starting_lba * reader.block_size as u64;
            match fs::FileSystem::new_filesystem(entry.fs.clone(), &reader) {
                Some(some) => {
                    filesystem = some;
                }
                None => {
                    return Some(BootFailure::UnknownFS);
                }
            }
        }
        None => {
            return Some(BootFailure::NoSuchPartition);
        }
    }

    dprintln!("{:?}", filesystem.load_file("/tests/core/image_fdisk.sh").unwrap().iter().map(|t| *t as char).collect::<Vec<char>>());

    None
}
