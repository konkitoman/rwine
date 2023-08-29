use std::io::Write;

use crate::CoffFileHeader;

impl CoffFileHeader {
    pub fn write<W: Write>(self, buffer: &mut W) -> std::io::Result<usize> {
        let data = self.as_bytes();
        let len = data.len();
        buffer.write_all(&data)?;
        Ok(len)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(24);
        buffer.extend([0x50, 0x45, 0x00, 0x00]);
        buffer.extend((self.machine as u16).to_le_bytes());
        buffer.extend(self.number_of_sectors.to_le_bytes());
        buffer.extend(self.time_data_stamp.to_le_bytes());
        buffer.extend(self.pointer_to_symbol_table.to_le_bytes());
        buffer.extend(self.number_of_symbols.to_le_bytes());
        buffer.extend(self.size_of_optional_header.to_le_bytes());
        buffer.extend(self.characteristics.bits().to_le_bytes());
        buffer
    }
}
