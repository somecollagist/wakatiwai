extern crate alloc;

use alloc::string::ToString;
use core::slice;

use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
use uefi::proto::device_path::DevicePath;
use uefi::proto::media::file::{File, FileInfo};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{MemoryType, OpenProtocolParams};
use uefi::{CStr16, Guid, Handle, Status};
use uefi_raw::protocol::file_system::{FileAttribute, SimpleFileSystemProtocol};

use crate::dev::gpt::GPT;
use crate::dev::reader::DiskReader;
use crate::dev::DISK_GUID_HANDLE_MAPPING;
use crate::{dprintln, eprintln, image_handle, system_table, BootEntry};

use super::BootFailure;

pub fn boot(entry: &BootEntry) -> Option<BootFailure> {
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
                uefi::table::boot::OpenProtocolAttributes::GetProtocol
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
    dprintln!("{} of {}", entry.partition as usize - 1, disk_gpt.entries.len());
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

    // Iterate through all SFS protocol handles
    // TODO: expand this for all partitions somehow
    for handle in st.boot_services().locate_handle_buffer(
        uefi::table::boot::SearchType::ByProtocol(
            &SimpleFileSystemProtocol::GUID
        )
    ).unwrap().iter() {
        // Get the device path associated to each SFS protocol (the handle supports both)
        let dp_protocol = st.boot_services().open_protocol_exclusive::<DevicePath>(*handle).unwrap();
        let dpath = dp_protocol.to_string(st.boot_services(), DisplayOnly(true), AllowShortcuts(false)).unwrap().to_string();

        // If the device path doesn't point to the specified partition, skip
        if !dpath.contains(&format!("HD({},GPT,{}", entry.partition, partition_guid.to_string().to_uppercase())) {
            continue;
        }

        // Open the target UEFI file
        let mut sfs_protocol = st.boot_services().open_protocol_exclusive::<SimpleFileSystem>(*handle).unwrap();
        let mut volume = sfs_protocol.open_volume().unwrap();
        let mut boot_file_handle = match volume.open(
            CStr16::from_str_with_buf(&entry.path[1..].replace("/", "\\"), vec![0 as u16; entry.path.len()].as_mut()).unwrap(),
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::READ_ONLY | FileAttribute::HIDDEN | FileAttribute::SYSTEM
        ) {
            Ok(ok) => {
                ok
            }
            Err(err) => {
                return Some(BootFailure::UEFIOpenError(err.status()));
            }
        };
        drop(sfs_protocol);
        
        // Read the UEFI file into a buffer
        let boot_size = boot_file_handle.get_boxed_info::<FileInfo>().unwrap().file_size() as usize + 1;
        let boot_buffer = unsafe {
            slice::from_raw_parts_mut(
                st.boot_services().allocate_pool(
                    MemoryType::LOADER_DATA,
                    boot_size
                ).unwrap(),
                boot_size
            )
        };
        match boot_file_handle.into_regular_file().unwrap().read(boot_buffer) {
            Ok(_) => {},
            Err(err) => {
                return Some(BootFailure::UEFIReadError(err.status()));
            }
        }

        // Load the UEFI file as an image
        let ldimg = match st.boot_services().load_image(
            image_handle!(),
            uefi::table::boot::LoadImageSource::FromBuffer {
                buffer: &boot_buffer,
                file_path: Some(&dp_protocol)
            }
        ) {
            Ok(ok) => {
                ok
            }
            Err(err) => {
                return Some(BootFailure::UEFILoadError(err.status()));
            }
        };
        drop(dp_protocol);

        // Start the image
        // N.B. This is probably a no return
        match st.boot_services().start_image(ldimg) {
            Ok(_) => {}
            Err(err) => {
                if err.status() == Status::INVALID_PARAMETER || err.status() == Status::SECURITY_VIOLATION {
                    eprintln!("Possible boot failure: {}", err.status());
                }
                dprintln!("Boot option returned status: {}", err.status());
                return None
            }
        }
    }

    Some(BootFailure::InaccessibleSFS)
}