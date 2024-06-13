use core::mem::size_of;

use uefi::Status;

use crate::dprintln;
use super::reader::DiskReader;

/// A structure describing the Master Boot Record (MBR).
#[derive(Clone, Copy, Debug,)]
#[repr(C, packed)]
pub struct MBR {
    /// x86 code used on a non-UEFI system to select an MBR partition and load the first logical block of that partition. Unused on UEFI systems.
    boot_code: [u8; 440],
    /// A unique disk signature which may be used by the OS to identify disks.
    pub disk_signature: u32,
    #[doc(hidden)]
    reserved: u16,
    /// Array of four MBR partitions.
    pub entries: [MBRPartitionEntry; 4],
    /// Identifies an MBR. This value must contain 0xAA55.
    signature: u16
}

impl MBR {
    /// The MBR signature, used to verify the data structure contains an MBR.
    const MBR_SIGNATURE: u16 = 0xAA55;

    /// Reads the MBR from a disk.
    pub fn read_mbr(reader: &DiskReader) -> Result<Self, Status> {
        let ret: MBR;
        match reader.read_block(0) {
            Ok(ok) => unsafe {
                ret = *(ok[0..size_of::<MBR>()].as_ptr() as *const MBR);
            },
            Err(err) => {
                return Err(err);
            }
        }
        
        // Check for validity
        if !ret.is_valid() {
            dprintln!("MBR signature is invalid");
            return Err(Status::ABORTED);
        }
    
        Ok(ret)
    }

    /// Checks if the MBR is valid.
    pub fn is_valid(&self) -> bool {
        if self.signature != Self::MBR_SIGNATURE {
            return false;
        }

        true
    }
}

/// A structure describing an MBR partition entry.
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct MBRPartitionEntry {
    /// Attribute bits.
    pub attributes: u8,
    /// CHS address of the first absolute sector in the partition.
    pub starting_chs: [u8; 3],
    /// The partition type.
    pub partition_type: u8,
    /// CHS address of the last absolute sector in the partition.
    pub ending_chs: [u8; 3],
    /// LBA address of the first absolute sector in the partition.
    pub starting_lba: u32,
    /// Number of sectors in the partition.
    pub sectors: u32
}