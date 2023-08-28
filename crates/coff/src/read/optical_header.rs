use super::IORead;
use std::io::Read;

use crate::{
    characteristic::DLLCharacteristics,
    optical_header::{
        OpticalHeader, OpticalHeaderCommon, OpticalHeaderError, OpticalHeaderWindowsPE32,
        OpticalHeaderWindowsPE32Plus,
    },
    subsystem, Subsystem,
};

impl OpticalHeaderCommon {
    pub fn read(data: &mut impl Read) -> Result<OpticalHeaderCommon, std::io::Error> {
        let major_linker_version = u8::from_read(data)?;
        let minor_linker_version = u8::from_read(data)?;
        let size_of_code = u32::from_read(data)?;
        let size_of_initialized_data = u32::from_read(data)?;
        let size_of_uninitialized_data = u32::from_read(data)?;
        let adress_of_entry_point = u32::from_read(data)?;
        let base_of_code = u32::from_read(data)?;

        Ok(Self {
            major_linker_version,
            minor_linker_version,
            size_of_code,
            size_of_initialized_data,
            size_of_uninitialized_data,
            adress_of_entry_point,
            base_of_code,
        })
    }
}

impl OpticalHeaderWindowsPE32 {
    pub fn read(data: &mut impl Read) -> Result<Self, std::io::Error> {
        let image_base = u32::from_read(data)?;

        let section_alignment = u32::from_read(data)?;
        let file_alignment = u32::from_read(data)?;
        let major_os_version = u16::from_read(data)?;
        let minor_os_version = u16::from_read(data)?;
        let major_img_version = u16::from_read(data)?;
        let minor_img_version = u16::from_read(data)?;
        let major_subsystem_version = u16::from_read(data)?;
        let minor_subsystem_version = u16::from_read(data)?;
        let win32_version = u32::from_read(data)?;
        let size_of_image = u32::from_read(data)?;
        let size_of_headers = u32::from_read(data)?;
        let checksum = u32::from_read(data)?;
        let subsystem = Subsystem::try_from(u16::from_read(data)?)
            .map_err(|e| std::io::Error::from_raw_os_error(22))?;
        let dll_characteristics = DLLCharacteristics::from_bits_truncate(u16::from_read(data)?);

        let size_of_stack_reserve = u32::from_read(data)?;
        let size_of_stack_commit = u32::from_read(data)?;
        let size_of_heap_reserve = u32::from_read(data)?;
        let size_of_heap_commit = u32::from_read(data)?;

        let loader_flags = u32::from_read(data)?;
        let number_of_rva_and_sizes = u32::from_read(data)?;

        Ok(Self {
            image_base,
            section_alignment,
            file_alignment,
            major_os_version,
            minor_os_version,
            major_img_version,
            minor_img_version,
            major_subsystem_version,
            minor_subsystem_version,
            win32_version,
            size_of_image,
            size_of_headers,
            checksum,
            subsystem,
            dll_characteristics,
            size_of_stack_reserve,
            size_of_stack_commit,
            size_of_heap_reserve,
            size_of_heap_commit,
            loader_flags,
            number_of_rva_and_sizes,
        })
    }
}

impl OpticalHeaderWindowsPE32Plus {
    pub fn read(data: &mut impl Read) -> Result<Self, std::io::Error> {
        let image_base = u64::from_read(data)?;

        let section_alignment = u32::from_read(data)?;
        let file_alignment = u32::from_read(data)?;
        let major_os_version = u16::from_read(data)?;
        let minor_os_version = u16::from_read(data)?;
        let major_img_version = u16::from_read(data)?;
        let minor_img_version = u16::from_read(data)?;
        let major_subsystem_version = u16::from_read(data)?;
        let minor_subsystem_version = u16::from_read(data)?;
        let win32_version = u32::from_read(data)?;
        let size_of_image = u32::from_read(data)?;
        let size_of_headers = u32::from_read(data)?;
        let checksum = u32::from_read(data)?;
        let subsystem = Subsystem::try_from(u16::from_read(data)?)
            .map_err(|_| std::io::Error::from_raw_os_error(22))?;
        let dll_characteristics = DLLCharacteristics::from_bits_truncate(u16::from_read(data)?);

        let size_of_stack_reserve = u64::from_read(data)?;
        let size_of_stack_commit = u64::from_read(data)?;
        let size_of_heap_reserve = u64::from_read(data)?;
        let size_of_heap_commit = u64::from_read(data)?;

        let loader_flags = u32::from_read(data)?;
        let number_of_rva_and_sizes = u32::from_read(data)?;

        Ok(Self {
            image_base,
            section_alignment,
            file_alignment,
            major_os_version,
            minor_os_version,
            major_img_version,
            minor_img_version,
            major_subsystem_version,
            minor_subsystem_version,
            win32_version,
            size_of_image,
            size_of_headers,
            checksum,
            subsystem,
            dll_characteristics,
            size_of_stack_reserve,
            size_of_stack_commit,
            size_of_heap_reserve,
            size_of_heap_commit,
            loader_flags,
            number_of_rva_and_sizes,
        })
    }
}

impl OpticalHeader {
    pub fn read(data: &mut impl Read) -> Result<OpticalHeader, OpticalHeaderError> {
        let magic = u16::from_read(data)?;

        match magic {
            0x10B => {
                let common = OpticalHeaderCommon::read(data)?;
                let base_of_data = u32::from_read(data)?;
                Ok(Self::PE32 {
                    common,
                    base_of_data,
                    win: OpticalHeaderWindowsPE32::read(data)?,
                })
            }
            0x20B => {
                let common = OpticalHeaderCommon::read(data)?;
                Ok(Self::PE32Plus {
                    common,
                    win: OpticalHeaderWindowsPE32Plus::read(data)?,
                })
            }
            _ => Err(OpticalHeaderError::InvalidMagic),
        }
    }
}
