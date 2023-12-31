use std::io::{Read, Result};
mod coff_file_header;
mod coff_relocations;
mod line_numbers;
mod mz;
mod optical_header;
mod section_table;
mod type_indicators;

pub trait IORead: Sized {
    fn from_read(stream: &mut impl Read) -> Result<Self>;
}

impl IORead for u8 {
    fn from_read(stream: &mut impl Read) -> Result<Self> {
        let mut buffer = [0u8; 1];
        stream.read_exact(&mut buffer)?;
        Ok(u8::from_le_bytes(buffer))
    }
}

impl IORead for u16 {
    fn from_read(stream: &mut impl Read) -> Result<Self> {
        let mut buffer = [0u8; 2];
        stream.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }
}

impl IORead for u32 {
    fn from_read(stream: &mut impl Read) -> Result<Self> {
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl IORead for u64 {
    fn from_read(stream: &mut impl Read) -> Result<Self> {
        let mut buffer = [0u8; 8];
        stream.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }
}
