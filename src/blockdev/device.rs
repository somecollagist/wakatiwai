extern crate alloc;

use alloc::vec::Vec;

use uefi::Status;
use uefi::proto::media::block::BlockIO;
use uefi::table::boot::ScopedProtocol;

use crate::dprintln;

/// Reads the specified block from a BlockIO protocol.
pub fn read_block(device_block_io: &ScopedProtocol<BlockIO>, block: u64) -> Result<Vec<u8>, Status> {
    read_blocks(device_block_io, block, 1)
}

/// Reads the specified blocks from a BlockIO protocol.
pub fn read_blocks(device_block_io: &ScopedProtocol<BlockIO>, first_block: u64, block_count: u64) -> Result<Vec<u8>, Status> {
    let mut buffer = vec![0 as u8; (block_count * device_block_io.media().block_size() as u64) as usize];

    let read_status = device_block_io.read_blocks(
        device_block_io.media().media_id(),
        first_block as u64,
        &mut buffer
    );
    
    if read_status.is_err() {
        dprintln!("Unable to read blocks {}-{}: {:?}", first_block, first_block+block_count-1, read_status);
        return Err(read_status.unwrap_err().status());
    }

    Ok(buffer)
}