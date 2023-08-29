use crate::{CoffCharacteristic, Machine};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoffFileHeader {
    pub machine: Machine,
    pub number_of_sectors: u16,
    pub time_data_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: CoffCharacteristic,
}

#[derive(Debug)]
pub enum FileHeaderError {
    InvalidMagic,
    InvalidMachine,
    IO(std::io::Error),
}

impl From<std::io::Error> for FileHeaderError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CoffFileHeader, Machine};

    #[test]
    pub fn init_write_read() {
        let mut coff = CoffFileHeader::default();
        coff.machine = Machine::Amd64;
        coff.number_of_sectors = 1;
        coff.time_data_stamp = 0x2186;
        coff.number_of_symbols = 1;
        coff.pointer_to_symbol_table = 0x64;

        let mut buffer = Vec::with_capacity(24);
        coff.clone().write(&mut buffer).unwrap();
        let mut buffer = std::io::Cursor::new(buffer);
        let new_coff = CoffFileHeader::read(&mut buffer).unwrap();
        assert_eq!(coff, new_coff)
    }
}
