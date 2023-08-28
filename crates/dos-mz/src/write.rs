use std::io::Write;

use crate::DosMZ;

impl DosMZ {
    pub fn write<W: Write>(self, buffer: &mut W) -> std::io::Result<usize> {
        let data = self.as_bytes();
        let len = data.len();
        buffer.write_all(&data)?;
        Ok(len)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.extra_bytes as usize);
        buffer.extend([0x4d, 0x5Au8]);
        buffer.extend(self.extra_bytes.to_le_bytes());
        buffer.extend(self.pages.to_le_bytes());
        buffer.extend(self.realocation_items.to_le_bytes());
        buffer.extend(self.header_size.to_le_bytes());
        buffer.extend(self.minimum_allocation.to_le_bytes());
        buffer.extend(self.maximum_allocation.to_le_bytes());
        buffer.extend(self.initial_ss.to_le_bytes());
        buffer.extend(self.initial_sp.to_le_bytes());
        buffer.extend(0u16.to_le_bytes());
        buffer.extend(self.initial_ip.to_le_bytes());
        buffer.extend(self.initial_cs.to_le_bytes());
        buffer.extend(self.realocation_table.to_le_bytes());
        buffer.extend(self.overlay.to_le_bytes());
        buffer.extend([0u8; 8]);
        buffer.extend(self.oem_identifier.to_le_bytes());
        buffer.extend(self.oem_info.to_le_bytes());
        buffer.extend([0u8; 20]);
        buffer.extend(self.pe_header_start.to_le_bytes());
        buffer.extend(&self.stub);
        buffer
    }
}
