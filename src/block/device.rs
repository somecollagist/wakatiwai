extern crate alloc;

use alloc::vec::Vec;
use core::ops::Deref;

use uefi::prelude::*;
use uefi::proto::media::block::*;
use uefi::table::boot::ScopedProtocol;
use uefi::table::boot::SearchType::ByProtocol;
use uefi::Error;

use crate::*;

pub fn get_block_device_handles(
    system_table: &SystemTable<Boot>,
) -> Result<Vec<Result<ScopedProtocol<BlockIO>, Error>>, Status> {
    let mut block_device_handles: Vec<Result<ScopedProtocol<BlockIO>, Error>> = Vec::new();
    let device_buffer = match system_table
        .boot_services()
        .locate_handle_buffer(ByProtocol(&BlockIoProtocol::GUID))
    {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("Unable to obtain block device handle buffer");
            return Err(err.status());
        }
    };

    for block_device_handle in device_buffer.deref() {
        block_device_handles.push(
            system_table
                .boot_services()
                .open_protocol_exclusive::<BlockIO>(*block_device_handle),
        );
    }

    Ok(block_device_handles)
}
