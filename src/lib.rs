#![no_std]

/// The signature that identifies the start of the [`BootloaderRequest`].
pub const SIGNATURE: [u64; 3] = [
    0b10011101_00010111_00010101_01011111_11110011_01100010_10011111_00001001,
    0b11010100_11010111_00101111_01000011_00111111_00100100_01010101_10111101,
    0b01000111_00000110_00110010_11010110_00110010_10010101_10110110_11110010,
];

/// The version of the API that this currently describes.
pub const API_VERSION: u64 = 0;

/// The segment type that specifies the location of the the bootloader request.
pub const BOOTLOADER_REQUEST_ELF_SEGMENT: u32 = 0x69B2BA6E;

/// Information that the kernel shares with the bootloader to allow the
/// bootloader to properly load the kernel.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BootloaderRequest {
    /// The signature to indicate that this request is valid.
    pub signature: [u64; 3],
    /// The version of the API that this kernel expects to communicate using.
    pub api_version: u64,
}

/// Information that the kernel requires to properly boot, to be passed
/// in an architecture specific register upon kernel entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BootloaderResponse {
    /// A utf-8 string containing the name of the loading bootloader.
    pub bootloader_name: *const u8,
    /// The length, in bytes, of [`BootloaderResponse::bootloader_name`].
    pub bootloader_name_length: usize,
    /// A utf-8 string containing the version of the loading bootloader.
    pub bootloader_version: *const u8,
    /// The length, in bytes, of [`BootloaderResponse::bootloader_version`].
    pub bootloader_version_length: usize,

    /// The virtual address of the base of the kernel.
    pub kernel_virtual_address: *const core::ffi::c_void,
    /// Offset of the higher half direct mapped memory.
    ///
    /// This region of memory is mapped as readable, writable, and executable.
    pub direct_map: usize,

    /// An array of [`MemoryMapEntry`]s.
    ///
    /// The entries are guaranteed to be sorted by base address, lowest to highest.
    /// All regions are guaranteed to be 4096 byte aligned for both base and size.
    /// All regions are guaranteed to not overlap with any other entry.
    pub memory_map_entries: *mut MemoryMapEntry,
    /// The number of [`MemoryMapEntry`]s to which [`BootloaderResponse::memory_map_entries`]
    /// points.
    pub memory_map_entry_count: usize,

    /// The address of the 32-bit SMBIOS entry point.
    ///
    /// NULL if not found.
    pub sm_bios_entry_32: *const core::ffi::c_void,
    /// The address of the 64-bit SMBIOS entry point.
    ///
    /// NULL if not found.
    pub sm_bios_entry_64: *const core::ffi::c_void,

    /// A pointer to the ACPI RSDP table.
    ///
    /// NULL if not found.
    pub rsdp_table_ptr: *const core::ffi::c_void,
    /// A pointer to the UEFI System Table.
    ///
    /// NULL if not found.
    pub uefi_system_table_ptr: *const core::ffi::c_void,

    /// A pointer to the UEFI memory map.
    pub uefi_memory_map: *const core::ffi::c_void,
    /// The size, in bytes, of the UEFI memory map.
    pub uefi_memory_map_size: usize,
    /// The size, in bytes, of the provided UEFI memory map descriptors.
    pub uefi_memory_map_descriptor_size: usize,
    /// The version of the provided UEFI memory map descriptors.
    pub uefi_memory_map_descriptor_version: u64,

    /// An array of [`ModuleEntry`]s.
    pub module_entries: *mut ModuleEntry,
    /// The number of [`ModuleEntry`]s to which [`BootloaderResponse::module_entries`] points.
    pub module_entry_count: usize,
}

/// A descriptor of a memory region.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryMapEntry {
    /// The kind of the region of memory this [`MemoryMapEntry`] describes.
    pub kind: MemoryMapEntryKind,
    /// The base of the region of memory this [`MemoryMapEntry`] describes.
    pub base: u64,
    /// The size, in bytes, of the region of memory this [`MemoryMapEntry`] describes.
    pub size: u64,
}

/// The kind of a memory region.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryMapEntryKind(u64);

impl MemoryMapEntryKind {
    /// The memory region is available for general use.
    pub const USABLE: Self = Self(0);
    /// THe memory region should not be touched by the OS.
    pub const RESERVED: Self = Self(1);
    /// The memory region should be preserved by the OS
    /// until ACPI is enabled.
    pub const ACPI_RECLAIMABLE: Self = Self(2);
    /// The memory region should be preserved by the OS in the working
    /// and ACPI S1-S3 states.
    pub const ACPI_NONVOLATILE_STORAGE: Self = Self(3);
    /// The memory region contains errors and should not be used.
    pub const UNUSABLE: Self = Self(4);
    /// The memory region must be accepted before use.
    pub const UNACCEPTED: Self = Self(5);
    /// The memory region contains structures provided by the loading bootloader.
    ///
    /// Once everything from the bootloader has been copied, this memory region can
    /// be used.
    pub const BOOTLOADER: Self = Self(6);
    /// The memory region contains kernel code or data.
    pub const KERNEL: Self = Self(7);
    /// The memory region contains a module.
    ///
    /// A memory region of this type contains only a single module, and serves no purpose other
    /// than storing the data of that module.
    pub const MODULE: Self = Self(8);
}

/// A descriptor of a module loaded at boot time.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleEntry {
    /// The name of the loaded module.
    pub name: *const u8,
    /// The length, in bytes, of [`ModuleEntry::name`].
    pub name_length: usize,

    /// The address of the loaded module.
    ///
    /// This is always 4096 byte aligned.
    pub address: *const u8,
    /// The size, in bytes, of the loaded module.
    pub size: usize,
}
