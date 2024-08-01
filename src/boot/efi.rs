use uefi::{proto::device_path::DevicePath, table::boot::{OpenProtocolAttributes, OpenProtocolParams}, Handle, Status};

use crate::{dprintln, eprintln, image_handle, system_table, BootEntry};

use super::BootFailure;

pub fn boot(entry: &BootEntry) -> Option<BootFailure> {
    let st = system_table!();
    let ldimg: Handle;
    match st.boot_services().load_image(
        image_handle!(),
        unsafe {
            uefi::table::boot::LoadImageSource::FromBuffer {
                buffer: super::read::read_file(entry, &entry.path).unwrap().as_ref().unwrap(),
                file_path: Some(
                    &st.boot_services().open_protocol::<DevicePath>(
                        OpenProtocolParams {
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