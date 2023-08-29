use std::io::Read;

use super::IORead;
use crate::{coff_realocations::CoffRealocations, type_indicators::TypeIndicators, Machine};

impl CoffRealocations {
    pub fn read(data: &mut impl Read, machine: &Machine) -> Result<Self, std::io::Error> {
        let virtual_adress = u32::from_read(data)?;
        let symbol_table_index = u32::from_read(data)?;
        let type_indicators = TypeIndicators::read(data, machine)?;
        Ok(Self {
            virtual_adress,
            symbol_table_index,
            type_indicators,
        })
    }
}
