#![no_main]
#![no_std]

springboard::boot_prelude!();

fn main(args: &BootDriverArgs) -> Option<Status> {
    match uefi::boot::load_image(
        uefi::boot::image_handle(),
        uefi::boot::LoadImageSource::FromBuffer {
            buffer: &args.img,
            file_path: Some(args.imgpath)
        }
    ) {
        Ok(ok) => {
            match uefi::boot::start_image(ok) {
                Ok(_) => {
                    None
                }
                Err(err) => {
                    Some(err.status())
                }
            }
        }
        Err(err) => {
            Some(err.status())
        }
    }
}