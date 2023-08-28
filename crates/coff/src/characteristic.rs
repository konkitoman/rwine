bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq)]
    pub struct Characteristic: u16 {
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
