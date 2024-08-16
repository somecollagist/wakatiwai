mod structures;

use core::slice::from_raw_parts_mut;

use uefi::table::boot::{AllocateType, MemoryType, PAGE_SIZE};

use crate::boot::{BootEntry, BootFailure};
use crate::{dprintln, eprintln, println, system_table, wprintln};

const MIN_SUPPORTED_VERSION: u16 = 0x020D;
const DEFAULT_LOAD_LOCATION: u64 = 0x1780000;

pub fn boot(entry: &BootEntry) -> Option<BootFailure> {
    let st = system_table!();
    let kernel_image: *mut [u8];
    let kernel_load_addr: u64;
    let mut initrd: Option<*mut [u8]> = None;
    let mut initrd_load_addr: Option<u64> = None;

    // Read kernel
    println!("Reading kernel...");
    match super::read::read_file(entry, &entry.path) {
        Ok(ok) => {
            kernel_image = ok;
        }
        Err(err) => {
            return Some(err);
        }
    }

    // Access the header and check if we support booting this
    let kernel_header = unsafe {
        (kernel_image as *mut u8).add(0x1F1) as *mut structures::LinuxSetupHeader
    };
    unsafe {
        if (*kernel_header).version < MIN_SUPPORTED_VERSION {
            eprintln!(
                "Linux boot protocol version {}.{} is too old to be supported, minimum version is {}.{}.",
                ((*kernel_header).version & 0xFF00) >> 8,
                ((*kernel_header).version & 0x00FF),
                (MIN_SUPPORTED_VERSION & 0xFF00) >> 8,
                (MIN_SUPPORTED_VERSION & 0x00FF),
            );
            return Some(BootFailure::OldLinuxBootProtocol);
        }

        if (*kernel_header).relocatable_kernel == 0 {
            eprintln!("Linux kernel must be protected-mode relocatable.");
            return Some(BootFailure::LinuxNotRelocatable);
        }
    }

    // Allocate kernel space
    match st.boot_services().allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        (kernel_image.len() / PAGE_SIZE) + 1
    ) {
        Ok(ok) => {
            kernel_load_addr = ok;
        }
        Err(err) => {
            return Some(BootFailure::InsufficientMemory(err.status()));
        }
    }
    dprintln!("Kernel sucessfully read and allocated");

    // Read initrd if specified
    if !entry.initrd.is_empty() {
        println!("Reading initrd...");
        match super::read::read_file(entry, &entry.initrd) {
            Ok(ok) => {
                initrd = Some(ok);
            }
            Err(err) => {
                return Some(err);
            }
        }

        // Allocate initrd space
        match st.boot_services().allocate_pages(
            AllocateType::MaxAddress(unsafe { (*kernel_header).initrd_addr_max as u64 }),
            MemoryType::LOADER_DATA,
            (initrd.unwrap().len() / PAGE_SIZE) + 1
        ) {
            Ok(ok) => {
                initrd_load_addr = Some(ok);
            }
            Err(err) => {
                return Some(BootFailure::InsufficientMemory(err.status()));
            }
        }
    
        dprintln!("Initrd successfully read and allocated");
    }
    else {
        wprintln!("No initrd specified.");
    }

    // Write necessary information to the header for booting
    unsafe {
        let heap_end = if (*kernel_header).loadflags & structures::LinuxSetupHeader::LOADFLAGS_LOADED_HIGH != 0 { 0xE000 } else { 0x9800 };
        (*kernel_header).setup(
            0xFFFF,                                                                         // vid_mode
            0 | structures::LinuxSetupHeader::LOADFLAGS_CAN_USE_HEAP,                       // loadflags
            if initrd_load_addr.is_none() { 0 } else { initrd_load_addr.unwrap() as u32 },  // ramdisk_image
            if initrd.is_none() { 0 } else { initrd.unwrap().len() as u32 },                // ramdisk_size
            heap_end - 0x200,                                                               // heap_end_ptr
            ((*kernel_header).pref_address as u32) + heap_end as u32,                       // cmdline_ptr
            None,                                                                           // kernel_alignment
            0                                                                               // setup_data
        );
    }

    // Load the kernel and initrd (if it exists) to the allocated spaces
    unsafe {
        let kernel_load_slice = from_raw_parts_mut(kernel_load_addr as *mut u8, kernel_image.len());
        kernel_load_slice.copy_from_slice(kernel_image.as_ref().unwrap());
        dprintln!("Loaded kernel to {:#x}", kernel_load_addr);

        if initrd.is_some() {
            let initrd_load_slice = from_raw_parts_mut(initrd_load_addr.unwrap() as *mut u8, initrd.unwrap().len());
            initrd_load_slice.copy_from_slice(initrd.unwrap().as_ref().unwrap());
            dprintln!("Loaded initrd to {:#x}", initrd_load_addr.unwrap());
        }
    }

    let mut boot_params: structures::LinuxBootParams = structures::LinuxBootParams::new();
    boot_params.hdr = unsafe { *kernel_header };

    unsafe {
        core::arch::asm!(
            "jmp {entry_64}",
            entry_64 = in(reg) kernel_load_addr + 0x200
        );
    }

    None
}