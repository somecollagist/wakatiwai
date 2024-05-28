extern crate alloc;

use alloc::alloc::*;

use uefi::helpers::system_table;
use uefi::table::boot::MemoryType;

use crate::eprintln;

/// The global allocator type.
#[derive(Default)]
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match system_table().boot_services().allocate_pool(MemoryType::LOADER_DATA, layout.size()) {
            Ok(ok) => ok,
            Err(_) => { 
                // INVALID_PARAMETER is thrown if the wrong type is requested or if the buffer is null - neither will happen.
                alloc_error(layout);
             }
        }
     }
     unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        match system_table().boot_services().free_pool(ptr) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Invalid buffer on memory free");
            }
        }
     }
}

/// If there is an out of memory error, just panic.
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Out of memory");
}

/// The static global allocator.
#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;