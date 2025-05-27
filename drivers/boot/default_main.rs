#![no_main]
#![no_std]

wakatiwai_udive::boot_prelude!();

fn main(args: &BootDriverArgs) -> Option<Status> {
    Some(Status::ABORTED)
}