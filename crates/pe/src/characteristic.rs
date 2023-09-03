bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq)]
    pub struct CoffCharacteristic: u16 {
        /// Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files.
        const ImageFileRelocsStripped = 0x0001;

        ///Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error.
        const ImageFileExecutableImage = 0x0002;

        ///COFF line numbers have been removed. This flag is deprecated and should be zero.
        const ImageFileLineNumsStripped = 0x0004;

        ///COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
        const ImageFileLocalSymsStripped = 0x0008;

        ///Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
        const ImageFileAggressiveWsTrim = 0x0010;

        ///Application can handle > 2-GB addresses.
        const ImageFileLargeAddressAware = 0x0020;

        ///This flag is reserved for future use.
        const RESERVED = 0x0040;

        ///Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero.
        const ImageFileBytesReversedLo = 0x0080;

        ///Machine is based on a 32-bit-word architecture.
        const ImageFile32bitMachine = 0x0100;

        ///Debugging information is removed from the image file.
        const ImageFileDebugStripped = 0x0200;

        ///If the image is on removable media, fully load it and copy it to the swap file.
        const ImageFileRemovableRunFromSwap = 0x0400;

        ///If the image is on network media, fully load it and copy it to the swap file.
        const ImageFileNetRunFromSwap = 0x0800;

        ///The image file is a system file, not a user program.
        const ImageFileSystem = 0x1000;

        ///The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
        const ImageFileDll = 0x2000;

        ///The file should be run only on a uniprocessor machine.
        const ImageFileUpSystemOnly = 0x4000;

        ///Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
        const ImageFileBytesReversedHi = 0x8000;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq)]
    pub struct DLLCharacteristics: u16{

        ///Image can handle a high entropy 64-bit virtual address space.
        const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA= 0x0020;

        ///DLL can be relocated at load time.
        const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE= 0x0040;

        ///Code Integrity checks are enforced.
        const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY= 0x0080;

        ///Image is NX compatible.
        const IMAGE_DLLCHARACTERISTICS_NX_COMPAT= 0x0100;

        ///Isolation aware, but do not isolate the image.
        const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION= 0x0200;

        ///Does not use structured exception (SE) handling. No SE handler may be called in this image.
        const IMAGE_DLLCHARACTERISTICS_NO_SEH= 0x0400;

        ///Do not bind the image.
        const IMAGE_DLLCHARACTERISTICS_NO_BIND= 0x0800;

        ///Image must execute in an AppContainer.
        const IMAGE_DLLCHARACTERISTICS_APPCONTAINER= 0x1000;

        ///A WDM driver.
        const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER= 0x2000;

        ///Image supports Control Flow Guard.
        const IMAGE_DLLCHARACTERISTICS_GUARD_CF= 0x4000;

        ///Terminal Server aware.    }
        const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE= 0x8000;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq)]
        pub struct SectionCharacteristics: u32{

        ///The section should not be padded to the next boundary. This flag is obsolete and is replaced by IMAGE_SCN_ALIGN_1BYTES. This is valid only for object files.
        const IMAGE_SCN_TYPE_NO_PAD = 0x00000008;

        ///The section contains executable code.
        const IMAGE_SCN_CNT_CODE = 0x00000020;

        ///The section contains initialized data.
        const IMAGE_SCN_CNT_INITIALIZED_DATA = 0x00000040;

        ///The section contains uninitialized data.
        const IMAGE_SCN_CNT_UNINITIALIZED_DATA = 0x00000080;

        ///Reserved for future use.
        const IMAGE_SCN_LNK_OTHER = 0x00000100;

        ///The section contains comments or other information. The .drectve section has this type. This is valid for object files only.
        const IMAGE_SCN_LNK_INFO = 0x00000200;

        ///The section will not become part of the image. This is valid only for object files.
        const IMAGE_SCN_LNK_REMOVE = 0x00000800;

        ///The section contains COMDAT data. For more information, see COMDAT Sections (Object Only). This is valid only for object files.
        const IMAGE_SCN_LNK_COMDAT = 0x00001000;

        ///The section contains data referenced through the global pointer (GP).
        const IMAGE_SCN_GPREL = 0x00008000;

        ///Reserved for future use.
        const IMAGE_SCN_MEM_PURGEABLE = 0x00020000;

        ///Reserved for future use.
        const IMAGE_SCN_MEM_16BIT = 0x00020000;

        ///Reserved for future use.
        const IMAGE_SCN_MEM_LOCKED = 0x00040000;

        ///Reserved for future use.
        const IMAGE_SCN_MEM_PRELOAD = 0x00080000;

        ///Align data on a 1-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_1BYTES = 0x00100000;

        ///Align data on a 2-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_2BYTES = 0x00200000;

        ///Align data on a 4-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_4BYTES = 0x00300000;

        ///Align data on an 8-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_8BYTES = 0x00400000;

        ///Align data on a 16-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_16BYTES = 0x00500000;

        ///Align data on a 32-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_32BYTES = 0x00600000;

        ///Align data on a 64-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_64BYTES = 0x00700000;

        ///Align data on a 128-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_128BYTES = 0x00800000;

        ///Align data on a 256-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_256BYTES = 0x00900000;

        ///Align data on a 512-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_512BYTES = 0x00A00000;

        ///Align data on a 1024-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_1024BYTES = 0x00B00000;

        ///Align data on a 2048-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_2048BYTES = 0x00C00000;

        ///Align data on a 4096-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_4096BYTES = 0x00D00000;

        ///Align data on an 8192-byte boundary. Valid only for object files.
        const IMAGE_SCN_ALIGN_8192BYTES = 0x00E00000;

        ///The section contains extended relocations.
        const IMAGE_SCN_LNK_NRELOC_OVFL = 0x01000000;

        ///The section can be discarded as needed.
        const IMAGE_SCN_MEM_DISCARDABLE = 0x02000000;

        ///The section cannot be cached.
        const IMAGE_SCN_MEM_NOT_CACHED = 0x04000000;

        ///The section is not pageable.
        const IMAGE_SCN_MEM_NOT_PAGED = 0x08000000;

        ///The section can be shared in memory.
        const IMAGE_SCN_MEM_SHARED = 0x10000000;

        ///The section can be executed as code.
        const IMAGE_SCN_MEM_EXECUTE = 0x20000000;

        ///The section can be read.
        const IMAGE_SCN_MEM_READ = 0x40000000;

        ///The section can be written to.
        const IMAGE_SCN_MEM_WRITE = 0x80000000;

    }
}
