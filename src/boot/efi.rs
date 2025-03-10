use uefi::proto::device_path::DevicePath;
use uefi::boot::{LoadImageSource, OpenProtocolAttributes, OpenProtocolParams};
use uefi::{Handle, Status};

use crate::{dprintln, eprintln, image_handle, BootEntry};

use super::BootFailure;

pub fn boot(entry: &BootEntry) -> Option<BootFailure> {
    let ldimg: Handle;
    
    // Load the UEFI program into a new image
    match uefi::boot::load_image(
        image_handle!(),
        unsafe {
            LoadImageSource::FromBuffer {
                // Read in the given file
                buffer: super::read::read_file(entry, &entry.path).unwrap().as_ref().unwrap(),
                // Requires a path to the partition it exists on
                file_path: Some(
                    &uefi::boot::open_protocol::<DevicePath>(
                        OpenProtocolParams {
                            // PARTITION_HANDLE contains the address of the handle which contains the device path of the required partition (!)
                            handle: Handle::from_ptr(super::read::PARTITION_HANDLE.read().unwrap() as *mut core::ffi::c_void).unwrap(),
                            agent: image_handle!(),
                            controller: None
                        },
                        OpenProtocolAttributes::GetProtocol
                    ).unwrap()
                ) 
            }
        }
    ) {
        Ok(ok) => {
            ldimg = ok;
        }
        Err(err) => {
            return Some(BootFailure::UEFILoadError(err.status()))
        }
    }

    // Start the loaded image
    match uefi::boot::start_image(ldimg) {
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