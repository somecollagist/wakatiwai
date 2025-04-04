#![no_main]
#![no_std]

springboard::fs_prelude!();

fn main(args: &FSDriverArgs) -> Result<Vec<u8>, Status> {
    Err(Status::NOT_FOUND)
}