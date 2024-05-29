use crate::*;
use crate::blockdev::device::*;

#[derive(Debug)]
pub enum BootFailure {
    Unknown
}

pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure> {
    println!("Booting \"{}\"...", entry.name);

    get_bootloader_device_handle();

    return Some(BootFailure::Unknown);
}
