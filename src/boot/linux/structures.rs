#![allow(dead_code)]
#![allow(unused_variables)]

use core::mem::size_of;

const EDD_MBR_SIG_MAX: usize            = 16;
const E820_MAX_ENTRIES_ZEROPAGE: usize  = 128;
const EDDMAXNR: usize                   = 6;

/// The header used by the Linux kernel in booting.
/// [Specification](https://www.kernel.org/doc/html/v5.4/x86/boot.html#the-real-mode-kernel-header).
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct LinuxSetupHeader {
    /// The size of the setup code in 512-byte sectors. If this field is 0, the real value is 4.
    pub setup_sects: u8,
    /// If this field is non-zero, the root defaults to read-only. This should be controlled by using the `ro` or `rw` options on the command line instead.
    #[deprecated]
    pub root_flags: u16,
    /// The size of the protected-mode code in units of 16-byte paragraphs. This cannot be trusted beyond protocol version 2.04 if the LOAD_HIGH flag is set.
    pub syssize: u32,
    /// This field is oboslete.
    #[deprecated]
    pub ram_size: u16,
    /// The video mode to be used by the kernel.
    pub vid_mode: u16,
    /// The default root device device number. This field should be controlled by using the `root=` option on the command line instead.
    #[deprecated]
    pub root_dev: u16,
    /// Contains the MBR bootable magic number, `0xAA55`.
    pub boot_flag: u16,
    /// Contains an x86 jump that can be used to determine the size of the header.
    pub jump: u16,
    /// Contains the magic number `HdrS`.
    pub signature: [u8; 4],
    /// Contains the bootloader version number.
    pub version: u16,
    /// A 16-bit real mode far subroutine that enters protected mode.
    pub realmode_swtch: u32,
    /// The load_low segment (`0x1000`).
    #[deprecated]
    pub start_sys_seg: u16,
    /// If non-zero, this is a pointer to a null-terminated human-readable kernel version number string, less `0x200` (i.e. the string is located `0x200` _after_ this pointer.)
    pub kernel_version: u16,
    /// Used to identify the bootloader.
    pub type_of_loader: u8,
    /// This field is a bitmask.
    pub loadflags: u8,
    /// When using kernel 2.00 or 2.01, this specifes how much data is moved in addition to the real-mode kernel if the load address is not `0x90000`.
    #[deprecated]
    pub setup_move_size: u16,
    /// The address to jump to in protected mode. This defaults the load address of the kernel, and can be used by the bootloader to determine the proper load address.
    pub code32_start: u32,
    /// The 32-bit linear address of the initial ramdisk or ramfs. Leave as zero if none is used.
    pub ramdisk_image: u32,
    /// The size of the initial ramdisk or ramfs. Leave as zero if none is used.
    pub ramdisk_size: u32,
    /// This field is obsolete.
    #[deprecated]
    pub bootsect_kludge: u32,
    /// This field marks the offset of the end of the setup stack/heap, relative to the beginning of real-mode code, minus `0x200`.
    pub heap_end_ptr: u16,
    /// This is an extension of the loader version number.
    pub ext_loader_ver: u8,
    /// This is an extension of the loader version type.
    pub ext_loader_type: u8,
    /// The 32-bit linear address of the kernel command line, located anywhere between the end of the setup heap and `0xA0000`.
    pub cmdline_ptr: u32,
    /// The maximum address (i.e. the address of the highest safe byte) that may be occupied by the initial ramdisk/ramfs contents.
    pub initrd_addr_max: u32,
    /// Alignment unit required by the kernel (if `relocatable_kernel` is `true`). If a relocatable kernel is loaded at an alignment incompatible with this field, it will be realigned at initialisation.
    pub kernel_alignment: u32,
    /// If this field is non-zero, the protected-mode part of the kernel can be loaded to any address that satisfies `kernel_alignment`.
    pub relocatable_kernel: u8,
    /// If this field is non-zero, it indicates as a power of two the minimum aligned required by the kernel to boot.
    pub min_alignment: u8,
    /// This field is a bitmask.
    pub xloadflags: u16,
    /// The maximum size of the command line without the terminating zero.
    pub cmdline_size: u32,
    /// In a paravirtualised environment, this will be non-zero to indicate the nature of the system.
    pub hardware_subarch: u32,
    /// If `hardware_subarch` is non-zero, this will be a pointer to data that is specific to the given hardware subarchitecture.
    pub hardware_subarch_data: u64,
    /// If this field is non-zero, it contains the offset from the beginning of the protected-mode code to the payload. **Beware that this payload may be compressed.**
    pub payload_offset: u32,
    /// The length of the payload.
    pub payload_length: u32,
    /// The 64-bit physical pointer to a null-terminated single linked list of `LinuxSetupData` structs. 
    pub setup_data: u64,
    /// If this field is non-zero, it indicates the preferred load address for the kernel.
    pub pref_address: u64,
    /// This field indicates the amount of linear contiguous mmeory from the kernel start address needed before the kernel can examine the memory map.
    pub init_size: u32,
    /// This field indicates the offset from the beginning of the kernel image to the EFI handover protocol entry point.
    pub handover_offset: u32
}


impl LinuxSetupHeader {
    /// If set, protected-mode code is loaded at `0x10000`, otherwise at `0x1000`.
    /// This field is read only.
    pub const LOADFLAGS_LOADED_HIGH: u8             = (1 << 0);
    /// If set, KASLR (Kernel Address Space Layout Randomisation) is enabled.
    /// This field is used internally by the compressed kernel to communicate KASLR status to the kernel proper.
    pub const LOADFLAGS_KASLRFLAG: u8               = (1 << 1);
    /// If set, suppress early messages.
    /// This field must be specified.
    pub const LOADFLAGS_QUIET_FLAG: u8              = (1 << 5);
    /// If set, do not reload segment registers in the 32-bit entry point.
    /// This field must be specified.
    pub const LOADFLAGS_KEEP_SEGMENTS: u8           = (1 << 6);
    /// If set, the value at `heap_end_ptr` is considered valid. Otherwise, some setup code functionality will be disabled.
    /// This field must be specified.
    pub const LOADFLAGS_CAN_USE_HEAP: u8            = (1 << 7);
    /// Mask for writable fields.
    pub const LOADFLAGS_WRITABLE_MASK: u8           = Self::LOADFLAGS_QUIET_FLAG | Self::LOADFLAGS_KEEP_SEGMENTS | Self::LOADFLAGS_CAN_USE_HEAP;

    /// If set, this kernel has the legacy 64-bit entry point at 0x200.
    /// This field is read only.
    pub const XLOADFLAGS_KERNEL64_LEGACY_ENTRY: u8  = (1 << 0);
    /// If set, the kernel/boot parameters/command line/ramdisk/ramfs can be loaded above 4G.
    /// This field is read only.
    pub const XLOADFLAGS_CAN_LOAD_ABOVE4_G: u8      = (1 << 1);
    /// If set, the kernel supports the 32-bit EFI handoff entry point given at `handover_offset`.
    /// This field is read only.
    pub const XLOADFLAGS_EFIHANDOVER32: u8          = (1 << 2);
    /// If set, the kernel supports the 64-bit EFI handoff entry point given at `handover_offset+0x200`.
    /// This field is read only.
    pub const XLOADFLAGS_EFIHANDOVER64: u8          = (1 << 3);
    /// If set, the kernel supports kexec EFI boot with EFI runtime support.
    /// This field is read only.
    pub const XLOADFLAGS_EFIKEXEC: u8               = (1 << 4);

    pub fn setup(
        &mut self,
        vid_mode: u16,
        loadflags: u8,
        ramdisk_image: u32,
        ramdisk_size: u32,
        heap_end_ptr: u16,
        cmdline_ptr: u32,
        kernel_alignment: Option<u32>,
        setup_data: u64
    ) {
        if self.setup_sects == 0 {
            self.setup_sects = 4; // for backwards compatibility (probably unnecessary)
        }

        self.vid_mode = vid_mode;
        self.type_of_loader = 0xFF; // Wakatiwai has no assigned bootloader ID, so use this
        self.loadflags = (self.loadflags & !Self::LOADFLAGS_WRITABLE_MASK) | (loadflags & Self::LOADFLAGS_WRITABLE_MASK);
        self.ramdisk_image = ramdisk_image;
        self.ramdisk_size = ramdisk_size;
        self.heap_end_ptr = heap_end_ptr;
        self.cmdline_ptr = cmdline_ptr;
        self.kernel_alignment = kernel_alignment.unwrap_or_else(|| self.kernel_alignment);
        self.setup_data = setup_data;
    }
}

/// A struct used to define a more extensible boot parameters passing mechanism.
#[repr(C, packed)]
pub struct LinuxSetupData {
    /// A 64-bit physical pointer to the next node of the linked list, or `0` if at the end.
    pub next: u64,
    /// Used to identify the contents of the data.
    pub data_type: u32,
    /// The length of the data field.
    pub length: u32,
    /// The payload of the setup data.
    pub data: [u8]
}

#[derive(Default)]
#[repr(C, packed)]
pub struct ScreenInfo {
    pub orig_x: u8,
    pub orig_y: u8,
    pub ext_mem_k: u16,
    pub orig_video_page: u16,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u8,
    pub reserved0: u8,
    pub orig_video_ega_bx: u16,
    pub reserved1: u16,
    pub orig_video_lines: u8,
    pub orig_video_is_vga: u8,
    pub orig_video_points: u16,
    pub lfb_width: u16,
    pub lfb_height: u16,
    pub lfb_depth: u16,
    pub lfb_base: u16,
    pub lfb_size: u16,
    pub cl_magic: u16,
    pub cl_offset: u16,
    pub lfb_linelength: u16,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
    pub vesapm_seg: u16,
    pub vesapm_off: u16,
    pub pages: u16,
    pub vesa_attrs: u16,
    pub capabilities: u32,
    pub ext_lfb_base: u32,
    pub reserved2: [u8; 2]
}

#[derive(Default)]
#[repr(C, packed)]
pub struct APMBIOSInfo {
    pub version: u16,
    pub cseg: u16,
    pub offset: u32,
    pub cseg_16: u16,
    pub dseg: u16,
    pub flags: u16,
    pub cseg_len: u16,
    pub cseg_16_len: u16,
    pub dseg_len: u16
}

#[derive(Default)]
#[repr(C, packed)]
pub struct ISTInfo {
    pub signature: u32,
    pub command: u32,
    pub event: u32,
    pub perf_level: u32
}

#[derive(Default)]
#[repr(C, packed)]
pub struct SysDescTable {
    pub length: u16,
    pub table: [u8; 14]
}

#[derive(Default)]
#[repr(C, packed)]
pub struct OLPCOFWHeader {
    pub ofw_magic: u32,
    pub ofw_version: u32,
    pub cif_handler: u32,
    pub irq_desc_table: u32,
}

#[repr(C, packed)]
pub struct EDIDInfo {
    pub dummy: [u8; 128]
}

#[derive(Default)]
#[repr(C, packed)]
pub struct EFIInfo {
    pub efi_loader_signature: u32,
    pub efi_systab: u32,
    pub efi_memdesc_size: u32,
    pub efi_memdesc_version: u32,
    pub efi_memmap: u32,
    pub efi_memmap_size: u32,
    pub efi_systab_hi: u32,
    pub efi_memmap_hi: u32
}

#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
pub struct BootE820Entry {
    pub addr: u64,
    pub size: u64,
    pub region_type: u32
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct EDDInfo {
    pub device: u8,
    pub version: u8,
    pub interface_support: u16,
    pub legacy_max_cylinder: u16,
    pub legacy_max_head: u8,
    pub legacy_sectors_per_track: u8,
    pub params: EDDDeviceParams
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct EDDDeviceParams {
    pub length: u16,
    pub info_flags: u16,
    pub num_default_cylinders: u32,
    pub num_default_heads: u32,
    pub sectors_per_track: u32,
    pub number_of_sectors: u64,
    pub bytes_per_sector: u16,
    pub dpte_ptr: u32,
    pub key: u16,
    pub device_path_info_length: u8,
    pub reserved0: u8,
    pub reserved1: u16,
    pub host_bus_type: [u8; 4],
    pub interface_type: [u8; 8],
    pub interface_path: InterfacePath,
    pub device_path: DevicePath,
    pub reserved2: u8,
    pub checksum: u8
}
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub union InterfacePath {
    pub isa: ISA,
    pub pci: PCI,
    pub ibnd: IBND,
    pub xprs: XPRS,
    pub htpt: HTPT,
    pub unknown: UnknownInterface
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ISA {
    pub base_addr: u16,
    pub reserved0: u16,
    pub reserved1: u32
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PCI {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
    pub channel: u8,
    pub reserved: u32
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IBND {
    pub reserved: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct XPRS {
    pub reserved: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct HTPT {
    pub reserved: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UnknownInterface {
    pub reserved: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub union DevicePath {
    pub ata: ATA,
    pub atapi: ATAPI,
    pub scsi: SCSI,
    pub usb: USB,
    pub i1394: I1394,
    pub fibre: Fibre,
    pub i2o: I2O,
    pub raid: RAID,
    pub sata: SATA,
    pub unknown: UnknownDevice
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ATA {
    pub device: u8,
    pub reserved0: u8,
    pub reserved1: u16,
    pub reserved2: u32,
    pub reserved3: u64,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct ATAPI {
    pub device: u8,
    pub lun: u8,
    pub reserved0: u8,
    pub reserved1: u8,
    pub reserved2: u32,
    pub reserved3: u64,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SCSI {
    pub id: u16,
    pub lun: u64,
    pub reserved0: u16,
    pub reserved1: u32
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct USB {
    pub serial_number: u64,
    pub reserved: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct I1394 {
    pub eui: u64,
    pub reserved: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Fibre {
    pub wwid: u64,
    pub lun: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct I2O {
    pub identity_tag: u64,
    pub reserved: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct RAID {
    pub array_number: u32,
    pub reserved0: u32,
    pub reserved1: u64
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SATA {
    pub device: u8,
    pub reserved0: u8,
    pub reserved1: u16,
    pub reserved2: u32,
    pub reserved3: u64,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct UnknownDevice {
    pub reserved0: u64,
    pub reserved1: u64
}

#[repr(C, packed)]
pub struct LinuxBootParams {
    pub screen_info: ScreenInfo,
    pub apm_bios_info: APMBIOSInfo,
    pub pad2: [u8; 4],
    pub tboot_addr: u64,
    pub ist_info: ISTInfo,
    pub acpi_rsdp_addr: u64,
    pub pad3: [u8; 3],
    pub hd0_info: [u8; 16],
    pub hd1_info: [u8; 16],
    pub sys_desc_table: SysDescTable,
    pub olpc_ofw_header: OLPCOFWHeader,
    pub ext_ramdisk_image: u32,
    pub ext_ramdisk_size: u32,
    pub ext_cmd_line_ptr: u32,
    pub pad4: [u8; 112],
    pub cc_blob_addr: u32,
    pub edid_info: EDIDInfo,
    pub efi_info: EFIInfo,
    pub alt_mem_k: u32,
    pub scratch: u32,
    pub e820_entries: u8,
    pub eddbuf_entries: u8,
    pub edd_mbr_sig_buf_entries: u8,
    pub kbd_status: u8,
    pub secure_boot: u8,
    pub pad5: [u8; 2],
    pub sentinel: u8,
    pub pad6: [u8; 1],
    pub hdr: LinuxSetupHeader,
    pub pad7: [u8; 0x290-0x1F1-size_of::<LinuxSetupHeader>()],
    pub edd_mbr_sig_buffer: [u32; EDD_MBR_SIG_MAX],
    pub e820_table: [BootE820Entry; E820_MAX_ENTRIES_ZEROPAGE],
    pub pad8: [u8; 48],
    pub eddbuf: [EDDInfo; EDDMAXNR],
    pub pad9: [u8; 276]
}

impl LinuxBootParams {
    pub fn new() -> Self {
        Self {
            screen_info: ScreenInfo::default(),
            apm_bios_info: APMBIOSInfo::default(),
            pad2: [0; 4],
            tboot_addr: 0,
            ist_info: ISTInfo::default(),
            acpi_rsdp_addr: 0,
            pad3: [0; 3],
            hd0_info: [0; 16],
            hd1_info: [0; 16],
            sys_desc_table: SysDescTable::default(),
            olpc_ofw_header: OLPCOFWHeader::default(),
            ext_ramdisk_image: 0,
            ext_ramdisk_size: 0,
            ext_cmd_line_ptr: 0,
            pad4: [0; 112],
            cc_blob_addr: 0,
            edid_info: EDIDInfo {
                dummy: [0; 128]
            },
            efi_info: EFIInfo::default(),
            alt_mem_k: 0,
            scratch: 0,
            e820_entries: 0,
            eddbuf_entries: 0,
            edd_mbr_sig_buf_entries: 0,
            kbd_status: 0,
            secure_boot: 0,
            pad5: [0; 2],
            sentinel: 0,
            pad6: [0; 1],
            hdr: LinuxSetupHeader::default(),
            pad7: [0; 0x290-0x1F1-size_of::<LinuxSetupHeader>()],
            edd_mbr_sig_buffer: [0; EDD_MBR_SIG_MAX],
            e820_table: [BootE820Entry::default(); E820_MAX_ENTRIES_ZEROPAGE],
            pad8: [0; 48],
            eddbuf: [EDDInfo {
                device: 0,
                version: 0,
                interface_support: 0,
                legacy_max_cylinder: 0,
                legacy_max_head: 0,
                legacy_sectors_per_track: 0,
                params: EDDDeviceParams {
                    length: 0,
                    info_flags: 0,
                    num_default_cylinders: 0,
                    num_default_heads: 0,
                    sectors_per_track: 0,
                    number_of_sectors: 0,
                    bytes_per_sector: 0,
                    dpte_ptr: 0,
                    key: 0,
                    device_path_info_length: 0,
                    reserved0: 0,
                    reserved1: 0,
                    host_bus_type: [0; 4],
                    interface_type: [0; 8],
                    interface_path: InterfacePath {
                        unknown: UnknownInterface {
                            reserved: 0
                        }
                    },
                    device_path: DevicePath {
                        unknown: UnknownDevice {
                            reserved0: 0,
                            reserved1: 0
                        }
                    },
                    reserved2: 0,
                    checksum: 0
                }
            }; EDDMAXNR],
            pad9: [0; 276]
        }
    }
}