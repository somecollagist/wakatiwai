#![no_std]
#![allow(
    static_mut_refs,
    unsafe_op_in_unsafe_fn
)]

extern crate alloc;

pub mod boot;
pub mod fs;
pub mod wakatiwai;
pub mod driver;
pub mod disk;
pub mod io;

use crate::io::DriverIO;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ptr::NonNull;

use uefi::boot::{open_protocol_exclusive, AllocateType, LoadImageSource, MemoryType};
use uefi::proto::device_path::build::media::FilePath;
use uefi::proto::device_path::build::DevicePathBuilder;
use uefi::proto::device_path::{DeviceSubType, LoadedImageDevicePath};
use uefi::{cstr16, CStr16, CString16, Handle, Status};

static mut DRIVER_IO: Option<*mut io::DriverIO> = None;
pub const BOOT_DRIVER_IO_MEMTYPE: MemoryType    = MemoryType::custom(0xCA11_B007);
pub const FSYS_DRIVER_IO_MEMTYPE: MemoryType    = MemoryType::custom(0xCA11_F575);

const DRIVER_DIRECTORY: &CStr16                 = cstr16!("\\EFI\\wakatiwai\\drivers");
const BOOT_DRIVER_DIRECTORY: &CStr16            = cstr16!("boot");
const FSYS_DRIVER_DIRECTORY: &CStr16            = cstr16!("fs");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DriverType {
    BOOT,
    FS
}

#[derive(Clone, Debug)]
struct Driver {
    name: CString16,
    driver_type: Option<DriverType>,
    exec_handle: Option<Handle>
}

#[derive(Clone, Debug)]
pub struct BootDriver(Driver);
#[derive(Clone, Debug)]
pub struct FSDriver(Driver);

impl Driver {
    pub fn name(&self) -> String {
        self.name.to_string().replace(".efi", "")
    }

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
            return Status::INVALID_PARAMETER;
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

    fn invoke(&mut self, invoke_io: &mut DriverIO, memtype: MemoryType) -> Result<Status, Status> {
        // Cannot invoke an unloaded image
        if self.exec_handle.is_none() {
            return Err(Status::NOT_READY);
        }

        // Allocate IO memory
        unsafe {
            let alloc_status = self.allocate_io_memory(memtype);
            if alloc_status.is_error() {
                return Err(alloc_status);
            }
        }

        // Bind to input
        unsafe {
            DRIVER_IO.unwrap().as_mut().unwrap().inptr = invoke_io.inptr;
        }

        // Start the image
        let driver_status = match uefi::boot::start_image(self.exec_handle.unwrap()) {
            Ok(_) => Status::SUCCESS,
            Err(err) => err.status()
        };

        // Bind to output
        unsafe {
            invoke_io.outptr = DRIVER_IO.unwrap().as_mut().unwrap().outptr;
        }

        // Free IO memory
        unsafe {
            let free_status = self.free_io_memory();
            if free_status.is_error() {
                return Err(free_status);
            }
        }

        return Ok(driver_status);
    }

    unsafe fn allocate_io_memory(&self, memtype: MemoryType) -> Status {
        // Disallow duplicate reallocations
        if DRIVER_IO.is_some() {
            return Status::ABORTED;
        }

        match uefi::boot::allocate_pages(
            AllocateType::AnyPages,
            memtype,
            DriverIO::page_count()
        ) {
            Ok(ok) => {
                // Zero the memory region
                (&mut *(ok.as_ptr() as *mut DriverIO)).zero();
                DRIVER_IO = Some(ok.as_ptr() as *mut DriverIO);
            }
            Err(err) => {
                return err.status();
            }
        }

        Status::SUCCESS
    }

    unsafe fn free_io_memory(&self) -> Status {
        // Cannot free memory that does not exist
        if DRIVER_IO.is_none() {
            return Status::ABORTED;
        }

        match uefi::boot::free_pages(
            NonNull::new(DRIVER_IO.unwrap() as *mut u8).unwrap(),
            DriverIO::page_count()
        ) {
            Ok(_) => {
                DRIVER_IO = None;
                Status::SUCCESS
            }
            Err(err) => {
                err.status()
            }
        }
    }
}