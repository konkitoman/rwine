use super::IORead;
use std::io::Read;

use crate::{
    type_indicators::{TypeIndicators, TypeIndicatorsAMD64, TypeIndicatorsI386},
    Machine,
};

impl TypeIndicators {
    pub fn read(data: &mut impl Read, machine: &Machine) -> Result<Self, std::io::Error> {
        let flags = u16::from_read(data)?;

        Ok(match machine {
            Machine::Amd64 => Self::AMD64(TypeIndicatorsAMD64::from_bits_truncate(flags)),
            Machine::I386 => Self::I386(TypeIndicatorsI386::from_bits_truncate(flags)),
            _ => {
                panic!("Not implemented for machine: {machine:?}")
            }
        })
    }
}
