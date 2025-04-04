use core::ffi::c_void;

use crate::{disk::DiskReader, *};

pub struct FSDriverArgs<'a> {
    pub path: &'a str,
    pub diskreader: DiskReader
}

impl FSDriver {
    pub fn name(&self) -> String {
        self.0.name()
    }

    pub fn load(&mut self) -> Status {
        self.0.load()
    }

    pub fn unload(&mut self) -> Status {
        self.0.unload()
    }

    pub fn invoke(&mut self, args: &mut FSDriverArgs) -> Result<&[u8], Result<Status, Status>> {
        let mut dio = DriverIO {
            inptr:  args as *mut FSDriverArgs as *mut c_void,
            outptr: core::ptr::null_mut()
        };
        
        let invoke_status = self.0.invoke(&mut dio, FSYS_DRIVER_IO_MEMTYPE);

        if invoke_status.is_ok_and(|t| t.is_success()) {
            return Ok(
                unsafe {
                    alloc::slice::from_raw_parts(
                        dio.outptr.add(size_of::<usize>()) as *mut u8,
                        *(dio.outptr as *mut usize)
                    )
                }
            )
        }
        
        return Err(invoke_status);
    }
}

#[macro_export]
macro_rules! fs_prelude {
    () => {
        extern crate alloc;
        use alloc::vec::Vec;

        use uefi::Status;

        use springboard::fs::FSDriverArgs;

        #[uefi::entry]
        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn  _entry() -> Status {
            uefi::helpers::init().unwrap();

            // Locate driver io struct
            let find_io_mem_status = springboard::driver::find_io_memory(springboard::FSYS_DRIVER_IO_MEMTYPE);
            if find_io_mem_status.is_error() {
                return find_io_mem_status;
            }
            let dio = springboard::io::DriverIO::allocated_driver_io().unwrap();

            let main_status = main(unsafe {
                core::mem::transmute::<*mut core::ffi::c_void, *mut FSDriverArgs>(dio.inptr).as_ref().unwrap()
            });

            if main_status.is_ok() {
                let filevec = main_status.unwrap();
                // Allocate space for the size and content of filevec, return the allocated pointer
                let vecptr = uefi::boot::allocate_pages(
                    uefi::boot::AllocateType::AnyPages,
                    uefi::boot::MemoryType::LOADER_DATA,
                    filevec.len() + size_of::<usize>() / uefi::boot::PAGE_SIZE
                ).unwrap();
                *(vecptr.as_ptr() as *mut usize) = filevec.len();
                core::ptr::copy(filevec.as_ptr(), vecptr.as_ptr().add(size_of::<usize>()), filevec.len());
                dio.outptr = vecptr.as_ptr() as *mut core::ffi::c_void;

                return Status::SUCCESS;
            }

            main_status.err().unwrap()
        }
    };
}