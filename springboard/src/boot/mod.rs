use core::ffi::c_void;

use alloc::string::String;
use uefi::proto::device_path::DevicePath;

use crate::*;

#[derive(Debug)]
pub struct BootDriverArgs<'a> {
    pub img: Vec<u8>,
    pub imgpath: &'a DevicePath,
    pub cmdline: &'a str,
}

impl BootDriver {
    pub fn name(&self) -> String {
        self.0.name()
    }

    pub fn load(&mut self) -> Status {
        self.0.load()
    }

    pub fn unload(&mut self) -> Status {
        self.0.unload()
    }

    pub fn invoke(&mut self, args: &mut BootDriverArgs) -> Option<Result<Status, Status>> {
        let mut dio = DriverIO {
            inptr:  args as *mut BootDriverArgs as *mut c_void,
            outptr: core::ptr::null_mut()
        };

        let invoke_status = self.0.invoke(&mut dio, BOOT_DRIVER_IO_MEMTYPE);

        if invoke_status.is_ok_and(|t| t.is_success()) {
            return None;
        }
        
        return Some(invoke_status);
    }
}

#[macro_export]
macro_rules! boot_prelude {
    () => {
        use uefi::Status;

        use springboard::boot::BootDriverArgs;

        #[uefi::entry]
        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn  _entry() -> Status {
            uefi::helpers::init().unwrap();

            // Locate driver io struct
            let find_io_mem_status = springboard::driver::find_io_memory(springboard::BOOT_DRIVER_IO_MEMTYPE);
            if find_io_mem_status.is_error() {
                return find_io_mem_status;
            }
            let dio = springboard::io::DriverIO::allocated_driver_io().unwrap();

            let main_status = main(unsafe {
                core::mem::transmute::<*mut core::ffi::c_void, *mut BootDriverArgs>(dio.inptr).as_ref().unwrap()
            });

            if main_status.is_none() {
                return Status::SUCCESS;
            }

            main_status.unwrap()
        }
    };
}