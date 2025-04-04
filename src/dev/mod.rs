pub mod mbr;
pub mod gpt;

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use springboard::disk::DiskReader;
use spin::Lazy;
use uefi::boot::{OpenProtocolAttributes, OpenProtocolParams, ScopedProtocol, SearchType};
use uefi::proto::media::disk::DiskIo;
use uefi::{Guid, Handle};
use uefi_raw::protocol::*;

use crate::{dprintln, eprintln, image_handle};

/// A map of GPT disk GUIDs to the corresponding disk handle addresses.
pub static DISK_GUID_HANDLE_MAPPING: Lazy<BTreeMap<Guid, u64>> = Lazy::new(|| {
    let mut disk_guid_handle_mapping: BTreeMap<Guid, u64> = BTreeMap::new();
    
    // Iterate all devide handles
    let device_handles = get_block_io_device_handles();
    for device_handle in device_handles.iter() {
        dprintln!("Opening device handle {:#010x}", device_handle.as_ptr() as u64);
        
        // Attempt to open a DiskIo protocol on this device, continue if unable to do so
        let protocol: ScopedProtocol<DiskIo>;
        unsafe {
            // Cannot open as exclusive otherwise crashes
            match uefi::boot::open_protocol::<DiskIo>(
                OpenProtocolParams {
                    handle: *device_handle,
                    agent: image_handle!(),
                    controller: None
                },
                OpenProtocolAttributes::GetProtocol
            ) {
                Ok(ok) => {
                    protocol = ok;
                }
                Err(err) => {
                    dprintln!("Unable to open DiskIO protocol on device handle {:#010x}: {:?}", device_handle.as_ptr() as u64, err.status());
                    continue;
                }
            }
        }

        let reader = DiskReader::new(device_handle, protocol, 0);

        // Attempt to read a GPT, push its GUID and the current handle if it exists, otherwise continue
        match gpt::GPT::read_gpt(&reader) {
            Ok(ok) => {
                disk_guid_handle_mapping.insert(
                    ok.header.disk_guid,
                    device_handle.as_ptr() as u64
                );
            }
            Err(err) => {
                dprintln!("Unable to read current disk GPT on media {}: {:?}", reader.media_id, err);
            }
        }
    }

    disk_guid_handle_mapping
});

/// The GPT GUID of the bootloader.
pub static BOOTLOADER_DISK_GUID: Lazy<Guid> = Lazy::new(|| {
    // First handle is always the current media (I think?)
    let device_handles = get_block_io_device_handles();
    let bootloader_device_handle = device_handles.get(0).unwrap();
    
    // Iterate over all disk GUIDS to find a matching handle
    for mapping in DISK_GUID_HANDLE_MAPPING.iter() {
        if *mapping.1 == bootloader_device_handle.as_ptr() as u64 {
            return *mapping.0;
        }
    }

    // If the disk GPT could not be found, default and alert.
    eprintln!("Cannot ascertain bootloader disk GUID");
    Guid::ZERO
});

/// Returns an array of all BlockIO Handles.
pub fn get_block_io_device_handles() -> Vec<Handle> {
    let mut ret = Vec::new();
    
    for handle in uefi::boot::locate_handle_buffer(
        SearchType::ByProtocol(
            &block::BlockIoProtocol::GUID
        )
    ).unwrap().iter() {
        ret.push(*handle);
    }

    return ret;
}