use alloc::string::ToString;
use springboard::disk::DiskReader;
use uefi::boot::{open_protocol, OpenProtocolAttributes, OpenProtocolParams, ScopedProtocol};
use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
use uefi::proto::device_path::DevicePath;
use uefi::proto::media::disk::DiskIo;
use uefi::{Guid, Handle};
use uefi_raw::protocol::block::BlockIoProtocol;

use crate::dev::gpt::GPT;
use crate::dev::DISK_GUID_HANDLE_MAPPING;
use crate::image_handle;
use crate::wtcore::config::BootEntry;

use super::BootFailure;

pub fn get_partition_handle(entry: &BootEntry) -> Result<Handle, BootFailure> {
    // Acquire handle to disk from GUID
    let disk_handle: Handle;
    match DISK_GUID_HANDLE_MAPPING.get(&entry.disk_guid) {
        Some(some) => {
            disk_handle = unsafe { Handle::from_ptr(*some as *mut core::ffi::c_void).unwrap() };
        }
        None => {
            return Err(BootFailure::NoDisk);
        }
    }

    // Read disk GPT
    let disk_gpt: GPT;
    match GPT::read_gpt(&DiskReader::new(
        &disk_handle,
        unsafe {
            open_protocol::<DiskIo>(
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
            return Err(BootFailure::BadGPT(err))
        }
    }

    // Get partition guid of boot entry
    let partition_guid: Guid;
    match disk_gpt.entries.get(entry.partition as usize - 1) {
        Some(some) => {
            // Ensure that the entry is used
            partition_guid = some.partition_guid;
            if partition_guid == Guid::ZERO {
                return Err(BootFailure::NoPartition)
            }
        }
        None => {
            // Fires if the partition is beyond 128
            return Err(BootFailure::NoPartition)
        }
    }

    // Search for the partition handle
    for handle in uefi::boot::locate_handle_buffer(
        uefi::boot::SearchType::ByProtocol(&BlockIoProtocol::GUID)
    ).unwrap().iter() {
        let dp_protocol: ScopedProtocol<DevicePath>;
        unsafe {
            match open_protocol::<DevicePath>(
                OpenProtocolParams {
                    handle: *handle,
                    agent: image_handle!(),
                    controller: None
                },
                OpenProtocolAttributes::GetProtocol
            ) {
                Ok(ok) => {
                    dp_protocol = ok;
                }
                Err(_) => {
                    // It's possible that not every BlockIO device can open a DevicePath
                    continue;
                }
            };

            // Check if the device path points to the disk and partition
            if (
                dp_protocol.to_string(DisplayOnly(true), AllowShortcuts(false)).unwrap().to_string()
            ).contains(&format!("HD({},GPT,{}", entry.partition, partition_guid.to_string().to_uppercase())) {
                return Ok(*handle);
            }
        }
    } 

    Err(BootFailure::PartitionNotFound)
}