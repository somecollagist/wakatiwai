extern crate alloc;

use alloc::vec::Vec;

use uefi::proto::media::disk::DiskIo;
use uefi::table::boot::{LoadImageSource, ScopedProtocol};
use uefi::{Handle, Status};

use crate::dev::gpt::GPT;
use crate::dev::reader::DiskReader;
use crate::dev::DISK_GUID_HANDLE_MAPPING;
use crate::fs::{FileSystem, FileSystemOperationError};
use crate::wtcore::Progtype::*;
use crate::{dprintln, image_handle, print, println, system_table, BootEntry};

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
    UnknownFS,
    /// An error was encountered in reading the boot path.
    FSError(FileSystemOperationError),
    /// An error was encountered in loading a UEFI program.
    UEFILoadError(Status),
    /// An error was encountered in booting a UEFI program.
    UEFIBootError(Status)
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
            disk_handle = unsafe { Handle::from_ptr(*some as *mut core::ffi::c_void).unwrap() };
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
            match FileSystem::new_filesystem(entry.fs.clone(), &reader) {
                Some(some) => {
                    filesystem = some;
                    dprintln!("Acquired filesystem of boot option");
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

    let boot_payload: Vec<u8>;
    match filesystem.load_file(&entry.path) {
        Ok(ok) => {
            boot_payload = ok;
            dprintln!("Successfully read boot program");
        }
        Err(err) => {
            return Some(BootFailure::FSError(err));
        }
    }

    match entry.progtype {
        UEFI => {
            let uefi_image_handle: Handle;
            match st.boot_services().load_image(
                image_handle!(),
                LoadImageSource::FromBuffer {
                    buffer: &boot_payload,
                    file_path: None
                }
            ) {
                Ok(ok) => {
                    uefi_image_handle = ok;
                }
                Err(err) => {
                    return Some(BootFailure::UEFILoadError(err.status()))
                }
            }
            dprintln!("UEFI program handle loaded, passing control...");

            // Flushes current colour (resets to default)
            print!("");

            match st.boot_services().start_image(uefi_image_handle) {
                Ok(_) => {}
                Err(err) => {
                    return Some(BootFailure::UEFIBootError(err.status()));
                }
            }
        },
        ELF => {
            todo!();
        },
        _ => {
            panic!("Unknown progtype {:?}", entry.progtype);
        }
    }

    None
}
