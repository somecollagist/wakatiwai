use uefi::proto::media::file::{Directory, File, FileAttribute, FileHandle, FileInfo, FileMode};
use uefi::{CString16, Error, Status};

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::ToString;

use crate::*;

fn get_driver_dir() -> Result<FileHandle, Error> {
    let mut efifs = uefi::boot::get_image_file_system(uefi::boot::image_handle())?;
    let mut efivol = efifs.open_volume()?;
    efivol.open(
        DRIVER_DIRECTORY,
        FileMode::Read,
        FileAttribute::DIRECTORY
    )
}

fn get_boot_driver_dir() -> Result<FileHandle, Error> {
    get_driver_dir().unwrap().into_directory().unwrap().open(
        BOOT_DRIVER_DIRECTORY,
        FileMode::Read,
        FileAttribute::DIRECTORY
    )
}

fn get_fs_driver_dir() -> Result<FileHandle, Error> {
    get_driver_dir()?.into_directory().unwrap().open(
        FSYS_DRIVER_DIRECTORY,
        FileMode::Read,
        FileAttribute::DIRECTORY
    )
}

fn is_valid_driver(handle: &mut FileHandle) -> bool {
    let driver_info: Box<FileInfo> = handle.get_boxed_info().unwrap();
    
    // Ensure that the driver is a file
    if !driver_info.is_regular_file() {
        return false;
    }
    // Ensure that the driver has the efi extension
    if !driver_info.file_name().to_string().ends_with(".efi") {
        return false;
    }

    true
}

fn get_driver_files_from_dir(directory: &mut Directory) -> Result<Vec<Driver>, Status> {
    let mut drivers: Vec<Driver> = Vec::new();

    loop {
        match directory.read_entry_boxed() {
            Ok(ok) => {
                if ok.is_none() {
                    // No more files
                    break;
                }

                // Open a handle on the directory member
                let mut file_handle = directory.open(
                    ok.unwrap().file_name(),
                    FileMode::Read,
                    FileAttribute::READ_ONLY
                );
                
                // If unable to read, error
                if file_handle.is_err() {
                    return Err(file_handle.err().unwrap().status());
                }

                // Skip if invalid
                if !is_valid_driver(file_handle.as_mut().unwrap()) {
                    continue;
                }

                let mut driver_name = CString16::new();
                driver_name.push_str((file_handle.as_mut().unwrap().get_boxed_info().unwrap() as Box<FileInfo>).file_name());

                drivers.push(
                    Driver {
                        name: driver_name,
                        driver_type: None,
                        exec_handle: None
                    }
                );
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(drivers)
}

pub fn get_boot_drivers() -> Result<Vec<BootDriver>, Status> {
    let mut ret: Vec<BootDriver> = Vec::new();
    match get_boot_driver_dir() {
        Ok(ok) => {
            for driver in get_driver_files_from_dir(&mut ok.into_directory().unwrap())?.iter() {
                ret.push(
                    BootDriver(
                        Driver {
                            name: driver.name.clone(),
                            driver_type: Some(DriverType::BOOT),
                            exec_handle: driver.exec_handle
                        }
                    )
                )
            }

            return Ok(ret);
        }
        Err(err) => {
            return Err(err.status());
        }
    }
}

pub fn get_fs_drivers() -> Result<Vec<FSDriver>, Status> {
    let mut ret: Vec<FSDriver> = Vec::new();
    match get_fs_driver_dir() {
        Ok(ok) => {
            for driver in get_driver_files_from_dir(&mut ok.into_directory().unwrap())?.iter() {
                ret.push(
                    FSDriver(
                        Driver {
                            name: driver.name.clone(),
                            driver_type: Some(DriverType::FS),
                            exec_handle: driver.exec_handle
                        }
                    )
                )
            }

            return Ok(ret)
        }
        Err(err) => {
            return Err(err.status());
        }
    }
}