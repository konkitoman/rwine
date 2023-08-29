use super::IORead;
use std::io::Read;

use crate::{characteristic::SectionCharacteristics, SectionTable};

impl SectionTable {
    pub fn read(data: &mut impl Read) -> Result<Self, std::io::Error> {
        let mut buffer = [0u8; 8];
        data.read_exact(&mut buffer)?;

        let mut name = String::with_capacity(8);
        for byte in buffer {
            if byte == 0 {
                break;
            }
            name.push(byte as char)
        }

        let virtual_size = u32::from_read(data)?;
        let virtual_address = u32::from_read(data)?;
        let size_of_raw_data = u32::from_read(data)?;
        let pointer_to_raw_data = u32::from_read(data)?;
        let pointer_to_relocations = u32::from_read(data)?;
        let pointer_to_line_numbers = u32::from_read(data)?;
        let number_of_relocations = u16::from_read(data)?;
        let number_of_line_numbers = u16::from_read(data)?;
        let characteristics = SectionCharacteristics::from_bits_truncate(u32::from_read(data)?);

        Ok(Self {
            name,
            virtual_size,
            virtual_address,
            size_of_raw_data,
            pointer_to_raw_data,
            pointer_to_relocations,
            pointer_to_line_numbers,
            number_of_relocations,
            number_of_line_numbers,
            characteristics,
        })
    }
}
