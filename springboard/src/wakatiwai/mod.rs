pub mod locate;

use uefi::boot::AllocateType;
use uefi::Status;

use crate::*;

pub fn get_mut_args() -> Result<&'static mut io::DriverIO, ()>{
    unsafe {
        if DRIVER_ARGS.is_none() {
            return Err(());
        }

        Ok(DRIVER_ARGS.unwrap().as_mut().unwrap())
    }
}

pub fn get_returns() -> Result<&'static io::DriverIO, ()>{
    unsafe {
        if DRIVER_RETS.is_none() {
            return Err(());
        }

        Ok(DRIVER_RETS.unwrap().as_ref().unwrap())
    }
}

pub fn allocate_driver_call_buffer() -> Status {
    unsafe {
        // Do not allow reallocation of buffers
        if DRIVER_ARGS.is_some() || DRIVER_RETS.is_some() {
            return Status::ABORTED;
        }

        /* Pages are allocated here instead of pool memory. While pool
         * memory would be preferable here (since it has a smaller
         * footprint), a page is allocated to ensure that the pointers are
         * well-aligned to the pages. I tried pool memory, it wasn't happy :(
         */

        // Allocate enough pages to hold the argument buffer
        match uefi::boot::allocate_pages(
            AllocateType::AnyPages,
            DRIVER_ARGS_MEMTYPE,
            io::DRIVER_IO_SIZE / PAGE_SIZE
        ) {
            Ok(ok) => {
                (&mut *(ok.as_ptr() as *mut [u8; io::DRIVER_IO_SIZE])).fill(0);
                DRIVER_ARGS = Some(ok.as_ptr() as *mut io::DriverIO);
            }
            Err(err) => {
                return err.status();
            }
        }

        // Allocate enough pages to hold the argument buffer
        match uefi::boot::allocate_pages(
            AllocateType::AnyPages,
            DRIVER_RETS_MEMTYPE,
            io::DRIVER_IO_SIZE / PAGE_SIZE
        ) {
            Ok(ok) => {
                (&mut *(ok.as_ptr() as *mut [u8; io::DRIVER_IO_SIZE])).fill(0);
                DRIVER_RETS = Some(ok.as_ptr() as *mut io::DriverIO);
            }
            Err(err) => {
                return err.status();
            }
        }
    }

    Status::SUCCESS
}