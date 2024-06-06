use core::ffi::c_void;

use gpt::GPTEntry;
use uefi::proto::media::block::BlockIO;
use uefi::table::boot::ScopedProtocol;
use uefi::{Handle, Status};

use crate::{blockdev::*, system_table};
use crate::{dprintln, println};
use crate::wtcore::config::BootEntry;

/// Possible failures that may occur when trying to boot a given entry.
#[derive(Debug)]
#[allow(dead_code)]
pub enum BootFailure {
    /// The specified disk could not be found.
    NoSuchDisk,
    /// A BlockIO protocol to the disk could not be opened.
    DiskBlockIOProtocolFailure(Status),
    /// The disk's GPT could not be read successfully.
    BadGPT(Status),
    /// The specified partition does not exist on disk.
    NoSuchPartition
}

/// Attempts to boot to a given entry.
/// Returns an `Some(BootFailure)` if booting failed, and None upon the exiting of a boot program.
pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure> {
    println!("Booting \"{}\"...", entry.name);
    dprintln!("{}", entry);
    let st = system_table!();

    let disk_handle: Handle;
    match DISK_GUID_HANDLE_MAPPING.get(&entry.disk_guid) {
        Some(some) => {
            disk_handle = unsafe { Handle::from_ptr(*some as *mut c_void).unwrap() };
        }
        None => {
            return Some(BootFailure::NoSuchDisk);
        }
    }

    let disk_block_io_protocol: ScopedProtocol<BlockIO>;
    match st.boot_services().open_protocol_exclusive::<BlockIO>(disk_handle) {
        Ok(ok) => {
            disk_block_io_protocol = ok;
        }
        Err(err) => {
            return Some(BootFailure::DiskBlockIOProtocolFailure(err.status()));
        }
    }

    let disk_gpt: gpt::GPT;
    match gpt::GPT::read_gpt(&disk_block_io_protocol) {
        Ok(ok) => {
            disk_gpt = ok;
        }
        Err(err) => {
            return Some(BootFailure::BadGPT(err));
        }
    }

    let partition: GPTEntry;
    match disk_gpt.entries.get((entry.partition-1) as usize) {
        Some(some) => {
            partition = some.clone();
        }
        None => {
            return Some(BootFailure::NoSuchPartition);
        }
    }

    None
}
