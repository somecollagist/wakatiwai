use uefi::{boot::MemoryType, mem::memory_map::MemoryMap, Status};

use crate::*;

pub fn get_args() -> Result<&'static io::DriverIO, ()>{
    unsafe {
        if DRIVER_ARGS.is_none() {
            return Err(());
        }

        Ok(DRIVER_ARGS.unwrap().as_ref().unwrap())
    }
}

pub fn get_mut_returns() -> Result<&'static mut io::DriverIO, ()>{
    unsafe {
        if DRIVER_RETS.is_none() {
            return Err(());
        }

        Ok(DRIVER_RETS.unwrap().as_mut().unwrap())
    }
}

pub fn find_driver_call_buffer() -> Status {
    unsafe {
        // Iterate over the memory map to detect the buffers
        for mement in uefi::boot::memory_map(MemoryType::LOADER_DATA).unwrap().entries() {
            if mement.ty == DRIVER_ARGS_MEMTYPE {
                DRIVER_ARGS = Some(mement.phys_start as *mut io::DriverIO);
            }
            else if mement.ty == DRIVER_RETS_MEMTYPE {
                DRIVER_RETS = Some(mement.phys_start as *mut io::DriverIO);
            }

            // If both buffers are found, succeed
            if DRIVER_ARGS.is_some() && DRIVER_RETS.is_some() {
                return Status::SUCCESS
            }
        }
    }

    // One or both buffers were not found
    Status::NOT_FOUND
}

#[macro_export]
macro_rules! prelude {
    ($version:literal, $callback:expr) => {
        unsafe extern "efiapi" fn driver_supported(
            _binding:           *const uefi_raw::protocol::driver::DriverBindingProtocol,
            _controller:        uefi_raw::Handle,
            _remaining:         *const uefi_raw::protocol::device_path::DevicePathProtocol
        ) -> uefi::Status {
            uefi::Status::SUCCESS
        }
        
        unsafe extern "efiapi" fn driver_start(
            _binding:           *const uefi_raw::protocol::driver::DriverBindingProtocol,
            _controller:        uefi_raw::Handle,
            _remaining:         *const uefi_raw::protocol::device_path::DevicePathProtocol
        ) -> uefi::Status {
            uefi::Status::SUCCESS
        }
        
        unsafe extern "efiapi" fn driver_stop(
            _binding:           *const uefi_raw::protocol::driver::DriverBindingProtocol,
            _controller:        uefi_raw::Handle,
            _child_count:       usize,
            _child_handle_buf:  *const uefi_raw::Handle
        ) -> uefi::Status {
            uefi::Status::SUCCESS
        }
        
        static mut DRIVER_BINDING: uefi_raw::protocol::driver::DriverBindingProtocol =
        uefi_raw::protocol::driver::DriverBindingProtocol {
            supported:              driver_supported,
            start:                  driver_start,
            stop:                   driver_stop,
            version:                $version,
            image_handle:           core::ptr::null_mut(),
            driver_binding_handle:  core::ptr::null_mut()
        };
        
        #[uefi::entry]
        fn main() -> uefi::Status {
            uefi::helpers::init().unwrap();
        
            // Locate callback registry
            let locate_callback_registry_status = springboard::driver::find_driver_call_buffer();
            if !locate_callback_registry_status.is_success() {
                return locate_callback_registry_status;
            }
        
            // Create and attach event to invoke driver
            unsafe {
                let event;
                match uefi::boot::create_event(
                    uefi_raw::table::boot::EventType::NOTIFY_SIGNAL,
                    uefi_raw::table::boot::Tpl::CALLBACK,
                    Some($callback),
                    Some(
                        core::ptr::NonNull::new(
                            springboard::DRIVER_ARGS.unwrap() as *mut core::ffi::c_void
                        ).unwrap()
                    )
                ) {
                    Ok(ok) => {
                        event = ok;
                    }
                    Err(err) => {
                        return err.status();
                    }
                };
        
                *springboard::DRIVER_CALL.unwrap() = event;
            }
        
            // Install protocol interface
            unsafe {
                match uefi::boot::install_protocol_interface(
                    Some(uefi::boot::image_handle()),
                    &uefi_raw::protocol::driver::DriverBindingProtocol::GUID,
                    core::ptr::addr_of!(DRIVER_BINDING).cast()
                ) {
                    Ok(_) => {
                        return uefi::Status::SUCCESS
                    }
                    Err(err) => {
                        return err.status();
                    }
                }
            }
        }
    };
}