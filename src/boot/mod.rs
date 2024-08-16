mod efi;
mod elf;
mod linux;

mod read;

extern crate alloc;

use uefi::Status;

use crate::dev::gpt::GPT;
use crate::dev::reader::DiskReader;
use crate::dev::DISK_GUID_HANDLE_MAPPING;
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
    UEFIBootError(Status),
    /// There is insufficient memory to load the kernel.
    InsufficientMemory(Status),
    /// An unsupported (old) version of Linux was requested to be booted.
    OldLinuxBootProtocol,
    /// The Linux boot option does not have relocatable protected-mode code.
    LinuxNotRelocatable
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
        LINUX => {
            linux::boot(entry)
        },
        _ => {
            panic!();
        }
    }
}
