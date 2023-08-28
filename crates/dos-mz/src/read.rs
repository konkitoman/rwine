use std::io::Read;

use crate::{DosMZ, DosMZError};

impl DosMZ {
    pub fn read<R: Read>(data: &mut R) -> Result<Self, DosMZError> {
        let mut buffer = [0u8; 2];

        data.read_exact(&mut buffer)?;
        let magic = u16::from_le_bytes(buffer);
        if magic != 0x5A4D {
            return Err(DosMZError::InvalidMagic);
        }

        data.read_exact(&mut buffer)?;
        let extra_bytes = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let pages = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let realocation_items = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let header_size = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let minimum_allocation = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let maximum_allocation = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let initial_ss = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let initial_sp = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let checksum = u16::from_le_bytes(buffer);
        if checksum != 0 {
            return Err(DosMZError::CheckSumFailed);
        }

        data.read_exact(&mut buffer)?;
        let initial_ip = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let initial_cs = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let realocation_table = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let overlay = u16::from_le_bytes(buffer);

        {
            let mut buffer = [0u8; 8];
            data.read_exact(&mut buffer)?;
        }
        data.read_exact(&mut buffer)?;
        let oem_identifier = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let oem_info = u16::from_le_bytes(buffer);

        {
            let mut buffer = [0u8; 20];
            data.read_exact(&mut buffer)?;
        }

        let mut buffer = [0u8; 4];
        data.read_exact(&mut buffer)?;
        let pe_header_start = u32::from_le_bytes(buffer);

        let mut buffer = vec![0u8; extra_bytes as usize - 0x40];
        data.read_exact(&mut buffer)?;

        let stub = buffer;

        Ok(Self {
            extra_bytes,
            pages,
            realocation_items,
            header_size,
            minimum_allocation,
            maximum_allocation,
            initial_ss,
            initial_sp,
            initial_ip,
            initial_cs,
            realocation_table,
            overlay,
            oem_identifier,
            oem_info,
            pe_header_start,
            stub,
        })
    }
}
