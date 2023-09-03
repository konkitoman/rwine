use std::io::Read;

use crate::{LineNumbers, LineNumbersType};

use super::IORead;

impl LineNumbers {
    pub fn read(data: &mut impl Read) -> Result<LineNumbers, std::io::Error> {
        let _type = u32::from_read(data)?;
        let number = u16::from_read(data)?;

        Ok(if number == 0 {
            Self {
                _type: LineNumbersType::SymbolTableIndex(_type),
                number,
            }
        } else {
            Self {
                _type: LineNumbersType::VirtualAdress(_type),
                number,
            }
        })
    }
}
