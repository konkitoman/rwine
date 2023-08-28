use super::IORead;
use std::io::Read;

use crate::{DosMZ, DosMZError};

impl DosMZ {
    pub fn read<R: Read>(data: &mut R) -> Result<Self, DosMZError> {
        let magic = u16::from_read(data)?;
        if magic != 0x5A4D {
            return Err(DosMZError::InvalidMagic);
        }

        let extra_bytes = u16::from_read(data)?;
        let pages = u16::from_read(data)?;

        let realocation_items = u16::from_read(data)?;
        let header_size = u16::from_read(data)?;
        let minimum_allocation = u16::from_read(data)?;
        let maximum_allocation = u16::from_read(data)?;
        let initial_ss = u16::from_read(data)?;
        let initial_sp = u16::from_read(data)?;

        let checksum = u16::from_read(data)?;
        if checksum != 0 {
            return Err(DosMZError::CheckSumFailed);
        }

        let initial_ip = u16::from_read(data)?;
        let initial_cs = u16::from_read(data)?;
        let realocation_table = u16::from_read(data)?;
        let overlay = u16::from_read(data)?;

        {
            let mut buffer = [0u8; 8];
            data.read_exact(&mut buffer)?;
        }
        let oem_identifier = u16::from_read(data)?;
        let oem_info = u16::from_read(data)?;

        {
            let mut buffer = [0u8; 20];
            data.read_exact(&mut buffer)?;
        }

        let pe_header_start = u32::from_read(data)?;

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
