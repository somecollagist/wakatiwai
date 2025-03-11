use crate::*;

pub const DRIVER_IO_SIZE: usize = PAGE_SIZE;
pub struct DriverIO([u8; DRIVER_IO_SIZE]);

impl DriverIO {
    pub fn to_byte_array(&self) -> &[u8] {
        &self.0
    }

    pub fn to_mut_byte_array(&mut self) -> &mut [u8] {
        &mut self.0
    }
}