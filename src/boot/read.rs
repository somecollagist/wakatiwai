extern crate alloc;

use alloc::string::ToString;

use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
use uefi::proto::device_path::DevicePath;
use uefi::proto::media::block::BlockIoProtocol;
use uefi::proto::media::disk::DiskIo;
use uefi::boot::{MemoryType, OpenProtocolAttributes, OpenProtocolParams, ScopedProtocol, SearchType};
use uefi::{Guid, Handle};

use crate::boot::{DISK_GUID_HANDLE_MAPPING, DiskReader, GPT};
use crate::fs::FileSystem;
use crate::{BootEntry, image_handle};

use super::BootFailure;

/// Stores the address of the handle which contains the device path protocol for the last booted entry's partition.
pub static PARTITION_HANDLE: spin::RwLock<Option<usize>> = spin::RwLock::new(None);

pub fn read_file(entry: &BootEntry, path: &str) -> Result<*mut [u8], BootFailure>{
    // Get a handle to the disk
    let disk_handle: Handle;
    match DISK_GUID_HANDLE_MAPPING.get(&entry.disk_guid) {
        Some(some) => {
            disk_handle = unsafe { Handle::from_ptr(*some as *mut core::ffi::c_void).unwrap() };
        }
        None => {
            return Err(BootFailure::NoSuchDisk);
        }
    }
    
    // Read the disk GPT
    let disk_gpt: GPT;
    match GPT::read_gpt(&DiskReader::new(
        &disk_handle,
        unsafe {
            uefi::boot::open_protocol(
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
            return Err(BootFailure::BadGPT(err));
        }
    }

    // Get the partition guid of the boot entry
    let partition_guid = match disk_gpt.entries.get(entry.partition as usize - 1) {
        Some(some) => {
            // Run a check to see if the existing entry is *used*
            let partition_guid_aligned = some.partition_guid;
            if partition_guid_aligned == Guid::ZERO {
                return Err(BootFailure::NoSuchPartition)
            }
            partition_guid_aligned
        }
        None => {
            // If the specified entry is beyond the number of *existing* entries (usually 128, will likely never fire)
            return Err(BootFailure::NoSuchPartition);
        }
    };

    for handle in uefi::boot::locate_handle_buffer(
        SearchType::ByProtocol(
            &BlockIoProtocol::GUID
        )
    ).unwrap().iter() {
        let dp_protocol: ScopedProtocol<DevicePath>;
        match unsafe {
            uefi::boot::open_protocol::<DevicePath>(
                OpenProtocolParams {
                    handle: *handle,
                    agent: image_handle!(),
                    controller: None
                },
                OpenProtocolAttributes::GetProtocol
            )
        } {
            Ok(ok) => {
                dp_protocol = ok;
            }
            Err(_) => {
                continue;
            }
        };
        let dpath = dp_protocol.to_string(DisplayOnly(true), AllowShortcuts(false)).unwrap().to_string();

        // If the device path doesn't point to the specified partition, skip
        if !dpath.contains(&format!("HD({},GPT,{}", entry.partition, partition_guid.to_string().to_uppercase())) {
            continue;
        }

        let mut partition_handle = PARTITION_HANDLE.write();
        *partition_handle = Some(handle.as_ptr() as usize);
        break;
    }

    let disk_protocol: ScopedProtocol<DiskIo>;
    match unsafe {
        uefi::boot::open_protocol::<DiskIo>(
            OpenProtocolParams {
                handle: Handle::from_ptr(PARTITION_HANDLE.read().unwrap() as *mut core::ffi::c_void).unwrap(),
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
            return Err(BootFailure::DiskIOProtocolFailure(err.status()))
        }
    };

    let reader = DiskReader::new(
        unsafe { &Handle::from_ptr(PARTITION_HANDLE.read().unwrap() as *mut core::ffi::c_void).unwrap() },
        disk_protocol,
        0
    );
    let fs = FileSystem::new_filesystem(entry.fs, reader).unwrap();
    let boot_read = fs.load_file(path).unwrap();
    let buffer = core::ptr::slice_from_raw_parts_mut(
        uefi::boot::allocate_pool(MemoryType::LOADER_DATA, boot_read.len()).unwrap().as_ptr(),
        boot_read.len()
    );

    unsafe {
        (*buffer).copy_from_slice(&boot_read);
    }

    Ok(buffer)
}