#![no_main]
#![no_std]

wakatiwai_udive::fs_prelude!();

fn main(args: &FSDriverArgs) -> Result<Vec<u8>, Status> {
    Err(Status::NOT_FOUND)
}