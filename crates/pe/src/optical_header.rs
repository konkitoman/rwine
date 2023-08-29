use crate::{characteristic::DLLCharacteristics, subsystem::Subsystem};

#[derive(Debug)]
pub struct OpticalHeaderCommon {
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub adress_of_entry_point: u32,
    pub base_of_code: u32,
}

#[derive(Debug)]
pub struct OpticalHeaderWindowsPE32 {
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_os_version: u16,
    pub minor_os_version: u16,
    pub major_img_version: u16,
    pub minor_img_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: Subsystem,
    pub dll_characteristics: DLLCharacteristics,

    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,

    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
}

#[derive(Debug)]
pub struct OpticalHeaderWindowsPE32Plus {
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_os_version: u16,
    pub minor_os_version: u16,
    pub major_img_version: u16,
    pub minor_img_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: Subsystem,
    pub dll_characteristics: DLLCharacteristics,

    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,

    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
}

#[derive(Debug)]
pub struct DataDirectory {
    pub address: u32,
    pub size: u32,
}

#[derive(Debug)]
pub struct DataDirectoryes {
    pub export_table: DataDirectory,
    pub import_table: DataDirectory,
    pub resource_table: DataDirectory,
    pub exeption_table: DataDirectory,
    pub certificate_table: DataDirectory,
    pub base_realocation_table: DataDirectory,
    pub debug: DataDirectory,
    pub architectures: DataDirectory,
    pub global_ptr: DataDirectory,
    pub tsl_table: DataDirectory,
    pub load_config_table: DataDirectory,
    pub bound_import: DataDirectory,
    pub iat: DataDirectory,
    pub delay_import_descriptor: DataDirectory,
    pub clr_runtime: DataDirectory,
    pub reserved: DataDirectory,
}

#[derive(Debug)]
pub enum OpticalHeader {
    PE32 {
        common: OpticalHeaderCommon,
        base_of_data: u32,
        win: OpticalHeaderWindowsPE32,
        data_directoryes: DataDirectoryes,
    },
    PE32Plus {
        common: OpticalHeaderCommon,
        win: OpticalHeaderWindowsPE32Plus,
        data_directoryes: DataDirectoryes,
    },
}

#[derive(Debug)]
pub enum OpticalHeaderError {
    InvalidMagic,
    IsCorupted,
    IO(std::io::Error),
}

impl From<std::io::Error> for OpticalHeaderError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
