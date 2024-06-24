#![allow(dead_code)]

pub mod fat12_16;
pub mod fat32;

#[derive(Clone, Copy, Debug, Default)]
pub struct Time(u16);
#[derive(Clone, Copy, Debug, Default)]
pub struct Date(u16);

impl Time {
    pub fn hour(&self) -> u8 {
        ((self.0 >> 10) & 0x3F) as u8
    }

    pub fn minute(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    pub fn second(&self) -> u8 {
        ((self.0 >> 0) & 0x0F) as u8 * 2
    }
}

impl Date {
    pub fn year(&self) -> u16 {
        ((self.0 >> 8) & 0xFF) as u16 + 1980
    }

    pub fn month(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    pub fn day(&self) -> u8 {
        ((self.0 >> 0) & 0x0F) as u8
    }
}