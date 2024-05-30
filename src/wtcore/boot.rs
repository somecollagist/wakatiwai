use crate::*;
use crate::wtcore::config::BootEntry;

/// Possible failures that may occur when trying to boot a given entry.
#[derive(Debug)]
pub enum BootFailure {
    /// Unknown failure.
    Unknown,
}

/// Attempts to boot to a given entry.
/// Returns an `Some(BootFailure)` if booting failed, and None upon the exiting of a boot program.
pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure> {
    println!("Booting \"{}\"...", entry.name);

    return Some(BootFailure::Unknown);
}
