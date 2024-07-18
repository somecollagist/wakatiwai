mod efi;
mod elf;

extern crate alloc;

use uefi::Status;

use crate::fs::FileSystemOperationError;
use crate::wtcore::Progtype::*;
use crate::{dprintln, println, BootEntry};

/// Possible failures that may occur when trying to boot a given entry.
#[derive(Debug)]
#[allow(dead_code)]
pub enum BootFailure {
    /// The specified disk could not be found.
    NoSuchDisk,
    /// A DiskIO protocol to the disk could not be opened.
    DiskIOProtocolFailure(Status),
    /// The disk's GPT could not be read successfully.
    BadGPT(Status),
    /// The specified partition does not exist on disk.
    NoSuchPartition,
    /// The specified partition cannot be accessed by the Simple File System protocol.
    InaccessibleSFS,
    /// The file system was unknown.
    UnknownFS,
    /// An error was encountered in reading the boot path.
    FSError(FileSystemOperationError),
    /// An error was encountered in opening a UEFI program.
    UEFIOpenError(Status),
    /// An error was encountered in reading a UEFI program.
    UEFIReadError(Status),
    /// An error was encountered in loading a UEFI program.
    UEFILoadError(Status),
    /// An error was encountered in booting a UEFI program.
    UEFIBootError(Status)
}


/// Attempts to boot to a given entry.
/// Returns an `Some(BootFailure)` if booting failed, and None upon the exiting of a boot program.
pub fn attempt_boot(entry: &BootEntry) -> Option<BootFailure> {
    println!("Booting \"{}\"...", entry.name);
    dprintln!("{}", entry);

    match entry.progtype {
        UEFI => {
            efi::boot(entry)
        },
        ELF => {
            elf::boot(entry)
        },
        _ => {
            panic!();
        }
    }
}
