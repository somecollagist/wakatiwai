use crate::*;

#[derive(Debug)]
pub enum BootFailure {
    Unknown,
    MissingGPT,
    MissingPartition,
}

pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure> {
    println!("Booting \"{}\"...", entry.name);

    return Some(BootFailure::Unknown);
}
