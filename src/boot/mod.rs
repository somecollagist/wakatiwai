mod partition;

use crate::wtcore::config::BootEntry;
use crate::{dprintln, println};

use alloc::vec::Vec;
use springboard::boot::BootDriverArgs;
use springboard::disk::DiskReader;
use springboard::fs::FSDriverArgs;
use springboard::{wakatiwai::*, BootDriver, FSDriver};
use uefi::proto::device_path::build::media::FilePath;
use uefi::proto::device_path::build::DevicePathBuilder;
use uefi::proto::device_path::DevicePath;
use uefi::proto::media::disk::DiskIo;
use uefi::{CStr16, Char16, Status};

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

pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure>{
    println!("Booting \"{}\"...", entry.name);
    dprintln!("{}", entry);

    let filebuf = read_boot_file(entry);
    if filebuf.is_err() {
        return Some(filebuf.err().unwrap());
    }

    boot(entry, filebuf.unwrap())
}

fn read_boot_file(entry: &BootEntry) -> Result<Vec<u8>, BootFailure> {
    let mut fs_driver: FSDriver;
    match get_fs_driver(&entry.fstype) {
        Ok(None) => {
            return Err(BootFailure::NoFSDriver);
        },
        Ok(Some(some)) => {
            fs_driver = some;
        }
        Err(err) => {
            return Err(BootFailure::DriverSearchFailed(err));
        }
    }

    dprintln!("Acquired FS Driver");

    let partition_handle = partition::get_partition_handle(entry);
    if partition_handle.is_err() {
        return Err(partition_handle.err().unwrap());
    }

    dprintln!("Acquired partition handle");

    let fs_driver_load_status = fs_driver.load();
    if fs_driver_load_status.is_error() {
        return Err(BootFailure::DriverLoadFailed(fs_driver_load_status));
    }

    let buffer = fs_driver.invoke(&mut FSDriverArgs {
        path:       &entry.path,
        diskreader: DiskReader::new(
            partition_handle.as_ref().unwrap(),
            uefi::boot::open_protocol_exclusive::<DiskIo>(*partition_handle.as_ref().unwrap()).unwrap(),
            0
        )
    });
    if buffer.is_err() {
        return Err(BootFailure::DriverInvokeFailed(buffer.err().unwrap()));
    }

    Ok(buffer.unwrap().to_vec())
}

fn boot(entry: &BootEntry, filebuf: Vec<u8>) -> Option<BootFailure> {
    let mut boot_driver: BootDriver;
    match get_boot_driver(&entry.ostype) {
        Ok(None) => {
            return Some(BootFailure::NoBootDriver);
        }
        Ok(Some(some)) => {
            boot_driver = some;
        }
        Err(err) => {
            return Some(BootFailure::DriverSearchFailed(err));
        }
    }

    dprintln!("Acquired Boot Driver");

    let boot_driver_load_status = boot_driver.load();
    if boot_driver_load_status.is_error() {
        return Some(BootFailure::DriverLoadFailed(boot_driver_load_status));
    }

    let partition_handle = partition::get_partition_handle(entry);
    if partition_handle.is_err() {
        return Some(partition_handle.err().unwrap());
    }

    let mut imgpath_cstr16;
    unsafe {
        imgpath_cstr16 = entry.path
            .chars()
            .map(|x| if x == '/' { '\\' } else { x })
            .map(|x| Char16::from_u16_unchecked(x as u16))
            .collect::<Vec<Char16>>();
        imgpath_cstr16.push(Char16::from_u16_unchecked('\0' as u16));
    };

    let mut imgpath_backing_vec = Vec::new();
    let mut imgpath = DevicePathBuilder::with_vec(&mut imgpath_backing_vec);
    for node in uefi::boot::open_protocol_exclusive::<DevicePath>(partition_handle.unwrap()).unwrap().node_iter() {
        imgpath = imgpath.push(&node).unwrap();
    }
    imgpath = imgpath.push(&FilePath {
        path_name: CStr16::from_char16_until_nul(imgpath_cstr16.as_slice()).unwrap()
    }).unwrap();

    boot_driver.invoke(&mut BootDriverArgs {
        img: filebuf,
        imgpath: imgpath.finalize().unwrap(),
        cmdline: &entry.args
    });
    
    None
}