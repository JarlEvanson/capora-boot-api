#![no_std]

/// Information that the kernel shares with the bootloader to allow the
/// bootloader to properly load the kernel.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BootloaderRequest {
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

    /// The physical address of the base of the kernel.
    pub kernel_physical_address: u64,
    /// The virtual address of the base of the kernel.
    pub kernel_virtual_address: *const core::ffi::c_void,

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

    /// The memory region contains the UEFI memory map.
    pub const UEFI_MEMORY_MAP: Self = Self(0x8000_0000_0000_0000);
    /// The memory region contains the memory map to be used for allocation.
    pub const MEMORY_MAP: Self = Self(0x8000_0000_0000_0001);
    /// The memory region is used for the list of module descriptors.
    pub const MODULE_DESCRIPTORS: Self = Self(0x8000_0000_0000_0002);

    /// The memory region occupied by the kernel.
    pub const KERNEL: Self = Self(0x8000_0000_0001_0000);
    /// The inclusive start of the range of memory kinds corresponding to memory used for
    /// a specific module.
    pub const MODULE_MEMORY_START: Self = Self(0x8000_0000_0001_0001);
    /// The inclusive end of the range of memory kinds corresponding to memory used for
    /// a specific module.
    pub const MODULE_MEMORY_END: Self = Self(0xFFFF_FFFF_FFFF_FFFF);
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
