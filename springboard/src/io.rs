use core::{ffi::c_void, ptr::null_mut};

use uefi::boot::PAGE_SIZE;

use crate::DRIVER_IO;

pub struct DriverIO {
    pub inptr:  *mut c_void,
    pub outptr: *mut c_void
}

impl DriverIO {
    pub fn allocated_driver_io() -> Option<&'static mut DriverIO> {
        unsafe {
            if DRIVER_IO.is_none() {
                return None;
            }

            return Some(
                DRIVER_IO.unwrap().as_mut().unwrap()
            )
        }
    }

    pub const fn page_count() -> usize {
        (size_of::<DriverIO>() + PAGE_SIZE - 1) / PAGE_SIZE
    }

    pub fn zero(&mut self) {
        self.inptr  = null_mut();
        self.outptr = null_mut();
    }
}