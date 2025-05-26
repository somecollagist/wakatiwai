mod partition;

use crate::wtcore::config::BootEntry;
use crate::{dprintln, image_handle, println};

use alloc::vec::Vec;
use uefi::Status;

use wakatiwai_udive::boot::BootDriverArgs;
use wakatiwai_udive::disk::DiskReader;
use wakatiwai_udive::fs::FSDriverArgs;
use wakatiwai_udive::{wakatiwai::*, BootDriver, FSDriver};

/// Possible failures that may occur when trying to boot a given entry.
#[derive(Debug)]
#[allow(dead_code)]
pub enum BootFailure {
    NoDisk,
    NoPartition,
    PartitionNotFound,
    BadGPT(Status),
    DriverSearchFailed(Status),
    DriverLoadFailed(Status),
    DriverUnloadFailed(Status),
    DriverInvokeFailed(Result<Status, Status>),
    NoBootDriver,
    NoFSDriver,
}

pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure> {
    println!("Booting \"{}\"...", entry.name);
    dprintln!("{}", entry);

    let partition_handle = partition::get_partition_handle(entry).unwrap();
    dprintln!("Acquired partition handle");

    // Acquire FS driver
    let mut fs_driver: FSDriver;
    match get_fs_driver(&entry.fstype) {
        Ok(None) => {
            return Some(BootFailure::NoFSDriver);
        }
        Ok(Some(some)) => {
            dprintln!("Acquired {} file system driver", entry.fstype);
            fs_driver = some;
            let fs_load_status = fs_driver.load();
            if fs_load_status.is_error() {
                return Some(BootFailure::DriverLoadFailed(fs_load_status));
            }
        }
        Err(err) => {
            return Some(BootFailure::DriverSearchFailed(err))
        }
    }

    // // FS shenanigans
    let buffer: Vec<u8>;
    match fs_driver.invoke(&mut FSDriverArgs {
        path:       &entry.path,
        diskreader: DiskReader::new(
            &partition_handle,
            unsafe {
                uefi::boot::open_protocol(
                    uefi::boot::OpenProtocolParams {
                        handle: partition_handle,
                        agent: image_handle!(),
                        controller: None,
                    },
                    uefi::boot::OpenProtocolAttributes::GetProtocol
                ).unwrap()
            },
            0
        )
    }) {
        Ok(ok) => {
            dprintln!("Successfully read {}", entry.path);
            buffer = ok.to_vec();
        }
        Err(err) => {
            return Some(BootFailure::DriverInvokeFailed(err))
        }
    }

    if entry.ostype == "UEFI" {
        dprintln!("Using internal UEFI loader...");
        return match uefi::boot::load_image(
            uefi::boot::image_handle(),
            uefi::boot::LoadImageSource::FromBuffer {
                buffer: &buffer,
                file_path: None
            }
        ) {
                Ok(ok) => {
                    match uefi::boot::start_image(ok) {
                        Ok(_) => {
                            None
                        }
                        Err(err) => {
                            Some(BootFailure::DriverInvokeFailed(Ok(err.status())))
                        }
                    }
                }
                Err(err) => {
                    Some(BootFailure::DriverInvokeFailed(Ok(err.status())))
                }
            }
    }

    // Boot option needs specialised OS driver
    // Acquire boot driver
    let mut boot_driver: BootDriver;
    match get_boot_driver(&entry.ostype) {
        Ok(None) => {
            return Some(BootFailure::NoBootDriver);
        }
        Ok(Some(some)) => {
            dprintln!("Acquired {} boot driver", entry.ostype);
            boot_driver = some;
            let boot_load_status = boot_driver.load();
            if boot_load_status.is_error() {
                return Some(BootFailure::DriverLoadFailed(boot_load_status));
            }
        }
        Err(err) => {
            return Some(BootFailure::DriverSearchFailed(err))
        }
    }

    // Boot shenanigans
    let mut boot_args = BootDriverArgs {
        img: buffer,
        cmdline: &entry.args
    };
    dprintln!("Invoking {} driver for {}", entry.ostype, entry.name);
    dprintln!("{}", boot_args);
    match boot_driver.invoke(&mut boot_args) {
        None => {
            return None;
        }
        Some(some) => {
            return Some(BootFailure::DriverInvokeFailed(some));
        }
    }
}