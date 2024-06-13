use alloc::vec::Vec;
use uefi::proto::media::block::BlockIO;
use uefi::proto::media::disk::DiskIo;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams, ScopedProtocol};
use uefi::{Handle, Status};

use crate::{image_handle, system_table};

pub struct DiskReader<'disk> {
    protocol: &'disk ScopedProtocol<'disk, DiskIo>,
    pub abs_offset: u64,
    pub media_id: u32,
    pub sector_size: u32,
    pub block_size: u32,
    pub last_block: u64
}

impl DiskReader<'_> {
    pub fn new<'disk>(handle: &Handle, protocol: &'disk ScopedProtocol<'disk, DiskIo>, abs_offset: u64) -> DiskReader<'disk> {
        let st = system_table!();
        
        let media_id: u32;
        let sector_size: u32;
        let block_size: u32;
        let last_block: u64;

        unsafe {
            let block_io_protocol = st.boot_services().open_protocol::<BlockIO>(
                OpenProtocolParams {
                    handle: *handle,
                    agent: image_handle!(),
                    controller: None
                },
                OpenProtocolAttributes::GetProtocol
            ).unwrap();
            
            media_id = block_io_protocol.media().media_id();
            block_size = block_io_protocol.media().block_size();
            if block_io_protocol.media().logical_blocks_per_physical_block() == 0 {
                sector_size = block_size;
            } else { 
                sector_size = block_size / block_io_protocol.media().logical_blocks_per_physical_block();
            }
            last_block = block_io_protocol.media().last_block();
        }

        DiskReader {
            protocol,
            abs_offset,
            media_id,
            sector_size,
            block_size,
            last_block
        }
    }

    pub fn read_bytes<'disk>(&self, offset: u64, count: usize) -> Result<Vec<u8>, Status> {
        let mut buffer = vec![0 as u8; count];
        let status = self.protocol.read_disk(
            self.media_id,
            self.abs_offset + offset,
            &mut buffer
        );
        
        if status.is_err() {
            return Err(status.err().unwrap().status());
        }
        Ok(buffer)
    }

    pub fn read_sector<'disk>(&self, sector: u64) -> Result<Vec<u8>, Status> {
        self.read_bytes(sector * self.sector_size as u64, self.sector_size as usize)
    }

    pub fn read_sectors<'disk>(&self, sector: u64, count: usize) -> Result<Vec<u8>, Status> {
        self.read_bytes(sector * self.sector_size as u64, count * self.sector_size as usize)
    }

    pub fn read_block<'disk>(&self, lba: u64) -> Result<Vec<u8>, Status> {
        self.read_bytes(lba * self.block_size as u64, self.block_size as usize)
    }

    pub fn read_blocks<'disk>(&self, lba: u64, count: usize) -> Result<Vec<u8>, Status> {
        self.read_bytes(lba * self.block_size as u64, count * self.block_size as usize)
    }
}