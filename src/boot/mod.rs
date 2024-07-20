mod efi;
mod elf;

extern crate alloc;

use alloc::string::ToString;
use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
use uefi::proto::device_path::DevicePath;
use uefi::proto::media::disk::DiskIo;
use uefi::table::boot::{MemoryType, OpenProtocolAttributes, OpenProtocolParams, ScopedProtocol};
use uefi::{Guid, Handle, Status};
use uefi_raw::protocol::block::BlockIoProtocol;

use crate::dev::gpt::GPT;
use crate::dev::reader::DiskReader;
use crate::dev::DISK_GUID_HANDLE_MAPPING;
use crate::fs::{FileSystem, FileSystemOperationError};
use crate::wtcore::Progtype::*;
use crate::{dprintln, image_handle, println, system_table, BootEntry};

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
    /// The specified partition cannot be accessed by the Simple File System protocol.
    InaccessibleSFS,
    /// The file system was unknown.
    UnknownFS,
    /// An error was encountered in reading the boot path.
    FSError(FileSystemOperationError),
    /// An error was encountered in opening a UEFI program.
    UEFIOpenError(Status),
    /// An error was encountered in reading a UEFI program.
    UEFIReadError(Status),
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
    
    // Read the disk GPT
    let disk_gpt: GPT;
    match GPT::read_gpt(&DiskReader::new(
        &disk_handle,
        unsafe {
            &st.boot_services().open_protocol(
                OpenProtocolParams {
                    handle: disk_handle,
                    agent: image_handle!(),
                    controller: None
                },
                OpenProtocolAttributes::GetProtocol
            ).unwrap()
        },
        0
    )) {
        Ok(ok) => {
            disk_gpt = ok;
        }
        Err(err) => {
            return Some(BootFailure::BadGPT(err));
        }
    }

    // Get the partition guid of the boot entry
    let partition_guid = match disk_gpt.entries.get(entry.partition as usize - 1) {
        Some(some) => {
            // Run a check to see if the existing entry is *used*
            let partition_guid_aligned = some.partition_guid;
            if partition_guid_aligned == Guid::ZERO {
                return Some(BootFailure::NoSuchPartition)
            }
            partition_guid_aligned
        }
        None => {
            // If the specified entry is beyond the number of *existing* entries (usually 128, will likely never fire)
            return Some(BootFailure::NoSuchPartition);
        }
    };

    let mut partition_handle: Option<Handle> = None;
    let mut dp_protocol: Option<ScopedProtocol<DevicePath>> = None;
    for handle in st.boot_services().locate_handle_buffer(
        uefi::table::boot::SearchType::ByProtocol(
            &BlockIoProtocol::GUID
        )
    ).unwrap().iter() {
        match unsafe {
            st.boot_services().open_protocol::<DevicePath>(
                OpenProtocolParams {
                    handle: *handle,
                    agent: image_handle!(),
                    controller: None
                },
                uefi::table::boot::OpenProtocolAttributes::GetProtocol
            )
        } {
            Ok(ok) => {
                dp_protocol = Some(ok);
            }
            Err(_) => {
                continue;
            }
        };
        let dpath = dp_protocol.as_ref().unwrap().to_string(st.boot_services(), DisplayOnly(true), AllowShortcuts(false)).unwrap().to_string();

        // If the device path doesn't point to the specified partition, skip
        if !dpath.contains(&format!("HD({},GPT,{}", entry.partition, partition_guid.to_string().to_uppercase())) {
            continue;
        }

        partition_handle = Some(*handle);
        break;
    }

    let disk_protocol: ScopedProtocol<DiskIo>;
    match unsafe {
        st.boot_services().open_protocol::<DiskIo>(
            OpenProtocolParams {
                handle: partition_handle.unwrap(),
                agent: image_handle!(),
                controller: None
            },
            OpenProtocolAttributes::GetProtocol
        )
    } {
        Ok(ok) => {
            disk_protocol = ok;
        }
        Err(err) => {
            return Some(BootFailure::DiskIOProtocolFailure(err.status()))
        }
    };

    let reader = DiskReader::new(&partition_handle.unwrap(), &disk_protocol, 0);
    let fs = FileSystem::new_filesystem(entry.fs, &reader).unwrap();
    let boot_read = fs.load_file(&entry.path).unwrap();
    let boot_buffer = core::ptr::slice_from_raw_parts_mut(
        st.boot_services().allocate_pool(MemoryType::LOADER_DATA, boot_read.len()).unwrap(),
        boot_read.len()
    );
    unsafe {
        (*boot_buffer).copy_from_slice(&boot_read);
    }

    match entry.progtype {
        UEFI => {
            efi::boot(boot_buffer, &(dp_protocol.unwrap()))
        },
        ELF => {
            elf::boot(entry)
        },
        _ => {
            panic!();
        }
    }
}
