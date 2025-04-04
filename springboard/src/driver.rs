use uefi::boot::MemoryType;
use uefi::mem::memory_map::MemoryMap;
use uefi::Status;

use crate::io::DriverIO;
use crate::*;

pub unsafe fn find_io_memory(memtype: MemoryType) -> Status {
    // Iterate over the memory map to detect the buffers
    for mement in uefi::boot::memory_map(MemoryType::LOADER_DATA).unwrap().entries() {
        if mement.ty == memtype {
            DRIVER_IO = Some(mement.phys_start as *mut DriverIO);
            return Status::SUCCESS;
        }
    }

    // The buffer wasn't found
    Status::NOT_FOUND
}