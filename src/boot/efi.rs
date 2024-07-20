use uefi::proto::device_path::DevicePath;
use uefi::{Handle, Status};

use crate::{dprintln, eprintln, image_handle, system_table};

use super::BootFailure;

pub fn boot(boot_buffer: *mut [u8], partition_device_path: &DevicePath) -> Option<BootFailure> {
    let st = system_table!();
    let ldimg: Handle;
    match st.boot_services().load_image(
        image_handle!(),
        uefi::table::boot::LoadImageSource::FromBuffer {
            buffer: unsafe { boot_buffer.as_ref().unwrap() },
            file_path: Some(partition_device_path) 
        }
    ) {
        Ok(ok) => {
            ldimg = ok;
        }
        Err(err) => {
            return Some(BootFailure::UEFILoadError(err.status()))
        }
    }

    match st.boot_services().start_image(ldimg) {
        Ok(_) => {}
        Err(err) => {
            if err.status() == Status::INVALID_PARAMETER || err.status() == Status::SECURITY_VIOLATION {
                eprintln!("Possible boot failure: {}", err.status());
            }
            dprintln!("Boot option returned status: {}", err.status());
        }
    }

    None
}