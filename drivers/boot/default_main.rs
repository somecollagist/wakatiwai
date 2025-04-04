#![no_main]
#![no_std]

springboard::boot_prelude!();

fn main(args: &BootDriverArgs) -> Option<Status> {
    Some(Status::ABORTED)
}