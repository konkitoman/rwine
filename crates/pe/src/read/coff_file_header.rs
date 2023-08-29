use super::IORead;
use std::io::Read;

use crate::{CoffCharacteristic, CoffFileHeader, FileHeaderError};

impl CoffFileHeader {
    pub fn read(data: &mut impl Read) -> Result<Self, FileHeaderError> {
        let magic = u32::from_read(data)?;

        if magic != 0x00004550 {
            return Err(FileHeaderError::InvalidMagic);
        }

        let Ok(machine) = u16::from_read(data)?.try_into() else {
            return Err(FileHeaderError::InvalidMachine);
        };

        let number_of_sectors = u16::from_read(data)?;
        let time_data_stamp = u32::from_read(data)?;
        let pointer_to_symbol_table = u32::from_read(data)?;
        let number_of_symbols = u32::from_read(data)?;

        let size_of_optional_header = u16::from_read(data)?;
        let characteristics = CoffCharacteristic::from_bits_truncate(u16::from_read(data)?);

        Ok(Self {
            machine,
            number_of_sectors,
            time_data_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        })
    }
}
