extern crate alloc;

use alloc::vec::Vec;
use core::borrow::Borrow;
use core::mem::size_of;

use crc::*;
use springboard::disk::DiskReader;
use uefi::{Char16, Guid, Status};

use crate::dprintln;
use super::mbr::MBR;

/// A structure describing the GUID Partition Table (GPT).
#[derive(Debug)]
pub struct GPT {
    /// Protective MBR.
    pub pmbr: MBR,
    /// Primary header.
    pub header: GPTHeader,
    /// Primary GPTEntry array.
    pub entries: Vec<GPTEntry>,
    /// Alternate GPTEntry array.
    pub alt_entries: Vec<GPTEntry>,
    /// Alternate header.
    pub alt_header: GPTHeader
}

impl GPT {
    /// Reads the GPT from a disk.
    pub fn read_gpt(reader: &DiskReader) -> Result<Self, Status> {
        // Read the PMBR
        let pmbr: MBR;
        match MBR::read_mbr(reader) {
            Ok(ok) => {
                pmbr = ok;
            }
            Err(err) => {
                return Err(err);
            }
        }
    
        // Read the headers
        let (header, alt_header): (GPTHeader, GPTHeader);
        match reader.read_block(1) {
            Ok(ok) => unsafe {
                header = *(ok[0..size_of::<GPTHeader>()].as_ptr() as *const GPTHeader);

                if !header.is_valid() {
                    dprintln!("Primary Header is invalid");
                    return Err(Status::ABORTED);
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
        match reader.read_block(reader.last_block){
            Ok(ok) => unsafe {
                alt_header = *(ok[0..size_of::<GPTHeader>()].as_ptr() as *const GPTHeader);

                if !alt_header.is_valid() {
                    dprintln!("Alternate Header is invalid");
                    return Err(Status::ABORTED);
                }
            },
            Err(err) => {
                return Err(err);
            }
        }

        // Read the entries
        let (entries, alt_entries): (Vec<GPTEntry>, Vec<GPTEntry>);
        match header.get_entries(reader) {
            Ok(ok) => {
                entries = ok;
            }
            Err(err) => {
                dprintln!("Failed to read entries from primary GPT header");
                return Err(err);
            }
        }
        match alt_header.get_entries(reader) {
            Ok(ok) => {
                alt_entries = ok;
            }
            Err(err) => {
                dprintln!("Failed to read entries from alternate GPT header");
                return Err(err);
            }
        }
    
        let ret = GPT {
            pmbr,
            header,
            entries,
            alt_entries,
            alt_header
        };

        // Check for validity
        if !ret.is_valid() {
            dprintln!("GPT is not valid");
            return Err(Status::ABORTED);
        }

        Ok(ret)
    }

    /// Checks if the GPT is valid.
    pub fn is_valid(&self) -> bool {
        // TODO: the MBR can also be non-protective and describe a uefi partition (partition type 0xEF), implement this?

        self.is_mbr_protective() &&         // Is the MBR protective? (validity checked in reading)
        self.header.is_valid() &&           // Is the primary header valid?
        self.alt_header.is_valid() &&       // Is the alternate header valid?
        self.entries.iter()                 // Are the primary and alternate entry arrays valid?
            .zip(self.alt_entries.iter())
            .filter(
                |(entry,alt_entry)|
                entry != alt_entry
            ).count() == 0
    }

    /// Checks if the PMBR is GPT protective compliant.
    fn is_mbr_protective(&self) -> bool {
        // The first entry should adhere to a specific format
        let pmbr_entry = self.pmbr.entries[0];
        
        if pmbr_entry.attributes != 0x00 {
            dprintln!("Invalid PMBR record attributes");
            return false;
        }
        if pmbr_entry.starting_chs != [0x00, 0x02, 0x00] {
            dprintln!("Invalid PMBR starting CHS");
            return false;
        }
        if pmbr_entry.partition_type != 0xEE {
            dprintln!("Invalid PMBR partition type");
            return false;
        }
        if pmbr_entry.starting_lba != 1 {
            dprintln!("Invalid PMBR starting LBA");
            return false;
        }
        
        // The remaining three entries should all be zero
        for x in 1..4 {
            let zero_entry = self.pmbr.entries[x];
            if
                zero_entry.attributes != 0         ||
                zero_entry.starting_chs != [0; 3]   ||
                zero_entry.partition_type != 0      ||
                zero_entry.ending_chs != [0; 3]     ||
                zero_entry.starting_lba != 0        ||
                zero_entry.sectors != 0
            {
                dprintln!("Auxiliary non-zero MBR entry detected");
                return false;
            }
        }
        
        true
    }
}

/// A structure describing a GPT Header.
/// [Specification](https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html#gpt-header).
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct GPTHeader {
    /// Identifies EFI-compatible partition table header. This value must contain the ASCII string “EFI PART”
    signature: [u8; 8],
    /// The revision number for this header.
    pub revision: u32,
    /// Size in bytes of the GPT header.
    pub header_size: u32,
    /// CRC32 checksum for the GPT Header structure.
    pub header_crc32: u32,
    #[doc(hidden)]
    reserved: u32,
    /// The LBA that contains this data structure.
    pub header_lba: u64,
    /// LBA address of the alternate GPT header.
    pub alt_header_lba: u64,
    /// The first usable logical block that may be used by a partition described by a GUID Partition Entry.
    pub first_usable_block: u64,
    /// The last usable logical block that may be used by a partition described by a GUID Partition Entry.
    pub last_usable_block: u64,
    /// GUID that can be used to uniquely identify the disk.
    pub disk_guid: Guid,
    /// GUID that can be used to uniquely identify the disk.
    pub entry_array_starting_lba: u64,
    /// The number of Partition Entries in the GUID Partition Entry array.
    pub entry_count: u32,
    /// The size, in bytes, of each the GUID Partition Entry structures in the GUID Partition Entry array.
    pub entry_size: u32,
    /// The CRC32 of the GUID Partition Entry array.
    pub entry_array_crc32: u32,
}

impl GPTHeader {
    /// The GPT signature, used to verify the data structure contains a GPT.
    const GPT_SIGNATURE: [u8; 8] = *b"EFI PART";

    /// Checks if a GPT Header is valid.
    fn is_valid(&self) -> bool {
        if self.signature != Self::GPT_SIGNATURE {
            dprintln!("Invalid GPT signature");
            return false;
        }

        if self.header_size < size_of::<GPTHeader>() as u32 {
            dprintln!("Invalid GPT header size");
            return false;
        }

        if (self.entry_size >> 7).count_ones() != 1 { // Is the size of a partition entry not a power of 128?
            dprintln!("GPT Header ascribes invalid partition entry size");
            return false;
        }

        unsafe {
            let mut payload = self.clone();
            payload.header_crc32 = 0;
            let payload_array = core::slice::from_raw_parts(payload.borrow() as *const GPTHeader as *const u8, self.header_size as usize);
            const HASHER: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

            if HASHER.checksum(payload_array) != self.header_crc32 {
                dprintln!("Bad Header CRC32 Checksum");
                return false;
            }
        }

        true
    }

    /// Obtains all of the entries pointed to by a GPT Header.
    fn get_entries(&self, reader: &DiskReader) -> Result<Vec<GPTEntry>, Status> {
        let mut ret = Vec::<GPTEntry>::new();

        // Read all the blocks that contain the GPT entries
        let partition_entry_array_bytes: Vec<u8>;
        match reader.read_blocks(
            self.entry_array_starting_lba,
            ((self.entry_size * self.entry_count) / reader.block_size & !reader.block_size) as usize
        ) {
            Ok(ok) => {
                partition_entry_array_bytes = ok;
            }
            Err(err) => {
                return Err(err);
            }
        }

        // Dereference entries from the above byte buffer, push to a return array
        for entry_idx in 0..self.entry_count {
            let entry = partition_entry_array_bytes.get((entry_idx * self.entry_size) as usize).unwrap() as *const u8 as *const GPTEntry;
            unsafe { ret.push((*entry).clone()); }
        }

        Ok(ret)
    }
}

/// A structure describing a GPT partition entry.
/// [Specification](#https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html#gpt-partition-entry-array).
#[derive(Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct GPTEntry {
    /// Unique ID that defines the purpose and type of this partition. A value of zero defines that this partition entry is not being used.
    pub type_guid: Guid,
    /// GUID that is unique for every partition entry.
    pub partition_guid: Guid,
    /// Starting LBA of the partition defined by this entry.
    pub starting_lba: u64,
    /// Ending LBA of the partition defined by this entry.
    pub ending_lba: u64,
    /// Attribute bits.
    pub attributes: u64,
    /// Null-terminated string containing the human-readable name of this partition.
    pub name: [Char16; 36]
}