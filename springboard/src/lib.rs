#![no_std]
#![allow(static_mut_refs)]

extern crate alloc;

pub mod boot_driver;
pub mod fs_driver;
pub mod driver;
pub mod io;
pub mod wakatiwai;

use alloc::vec::Vec;
use uefi::boot::{open_protocol_exclusive, LoadImageSource, MemoryType, PAGE_SIZE};
use uefi::proto::device_path::build::media::FilePath;
use uefi::proto::device_path::build::DevicePathBuilder;
use uefi::proto::device_path::{DeviceSubType, LoadedImageDevicePath};
use uefi::{cstr16, CStr16, CString16, Handle, Status};

const DRIVER_ARGS_MEMTYPE: MemoryType           = MemoryType::custom(0xCA11_A565);
const DRIVER_RETS_MEMTYPE: MemoryType           = MemoryType::custom(0xCA11_5E75);

static mut DRIVER_ARGS: Option<*mut io::DriverIO>   = None;
static mut DRIVER_RETS: Option<*mut io::DriverIO>   = None;

const DRIVER_DIRECTORY: &CStr16                 = cstr16!("\\EFI\\wakatiwai\\drivers");
const BOOT_DRIVER_DIRECTORY: &CStr16            = cstr16!("boot");
const FSYS_DRIVER_DIRECTORY: &CStr16            = cstr16!("fs");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DriverType {
    BOOT,
    FS
}

#[derive(Debug)]
struct Driver {
    name: CString16,
    driver_type: Option<DriverType>,
    exec_handle: Option<Handle>
}

#[derive(Debug)]
pub struct BootDriver(Driver);
#[derive(Debug)]
pub struct FSDriver(Driver);

impl Driver {
    pub fn path(&self) -> Option<CString16> {
        match self.driver_type {
            Some(some) => {
                let mut ret = CString16::new();
                ret.push_str(DRIVER_DIRECTORY);
                ret.push_str(cstr16!("\\"));
                ret.push_str(
                    if some == DriverType::BOOT { BOOT_DRIVER_DIRECTORY }
                    else { FSYS_DRIVER_DIRECTORY }
                );
                ret.push_str(cstr16!("\\"));
                ret.push_str(&self.name);

                return Some(ret);
            }
            None => {
                return None;
            }
        }
    }

    pub fn load(&mut self) -> Status {
        // Disallow reloading drivers
        if self.exec_handle.is_some() {
            return Status::INVALID_PARAMETER;
        }

        // Build the fully-qualified device path to load
        let mut driver_devpath_build_vec = Vec::new();
        let mut driver_devpath_builder = DevicePathBuilder::with_vec(&mut driver_devpath_build_vec);
    
        // Push the partition device path (which wakatiwai resides on)
        let ldimg = open_protocol_exclusive::<LoadedImageDevicePath>(uefi::boot::image_handle()).unwrap();
        for partition_devpath_node in ldimg.node_iter() {
            // Ignore the file path part, as that points to wakatiwai
            if partition_devpath_node.sub_type() == DeviceSubType(4) {
                break;
            }
            
            driver_devpath_builder = driver_devpath_builder.push(&partition_devpath_node).unwrap();
        }
        // Push the actual path of the file
        driver_devpath_builder = driver_devpath_builder.push(&FilePath { path_name: &self.path().unwrap() }).unwrap();
        
        // Load the driver
        match uefi::boot::load_image(
            uefi::boot::image_handle(),
            LoadImageSource::FromDevicePath {
                device_path: driver_devpath_builder.finalize().unwrap(),
                boot_policy: uefi::proto::BootPolicy::ExactMatch
            }
        ) {
            Ok(ok) => {
                self.exec_handle = Some(ok);
            }
            Err(err) => {
                return err.status();
            }
        }
    
        Status::SUCCESS
    }

    pub fn unload(&mut self) -> Status {
        // Cannot unload an unloaded driver
        if self.exec_handle.is_none() {
            return Status::INVALID_PARAMETER
        }

        match uefi::boot::unload_image(self.exec_handle.unwrap()) {
            Ok(_) => {
                return Status::SUCCESS;
            }
            Err(err) => {
                return err.status();
            }
        }
    }

    pub fn invoke(&mut self) -> Status {
        // Cannot invoke an unloaded image
        if self.exec_handle.is_none() {
            return Status::NOT_READY;
        }

        // Start the image
        match uefi::boot::start_image(self.exec_handle.unwrap()) {
            Ok(_) => Status::SUCCESS,
            Err(err) => err.status()
        }
    }
}