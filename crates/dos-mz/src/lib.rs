//! Source https://wiki.osdev.org/MZ

mod error;
mod read;
mod write;

pub use error::DosMZError;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DosMZ {
    extra_bytes: u16,
    pub pages: u16,
    pub realocation_items: u16,
    pub header_size: u16,
    pub minimum_allocation: u16,
    pub maximum_allocation: u16,
    pub initial_ss: u16,
    pub initial_sp: u16,
    pub initial_ip: u16,
    pub initial_cs: u16,
    pub realocation_table: u16,
    pub overlay: u16,

    pub oem_identifier: u16,
    pub oem_info: u16,
    pub pe_header_start: u32,

    stub: Vec<u8>,
}

impl DosMZ {
    pub fn stub(&self) -> &[u8] {
        &self.stub
    }

    /// The max stub len is 65471
    pub fn set_stub(&mut self, stub: Vec<u8>) {
        self.stub = stub;
        self.extra_bytes += (0x40 + self.stub.len()) as u16;
    }
}

#[cfg(test)]
mod tests {
    use crate::DosMZ;

    #[test]
    fn init_write_read() {
        let mut dos_mz = DosMZ::default();
        dos_mz.pages = 1;
        dos_mz.realocation_table = 64;
        dos_mz.set_stub(vec![
            0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01, 0x4C, 0xCD, 0x21,
            0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x20, 0x63,
            0x61, 0x6E, 0x6E, 0x6F, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20, 0x69,
            0x6E, 0x20, 0x44, 0x4F, 0x53, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x24, 0x00, 00,
        ]);

        let mut buffer = Vec::with_capacity(120);
        dos_mz.clone().write(&mut buffer).unwrap();
        let mut buffer = std::io::Cursor::new(buffer);
        let new_dos_mz = DosMZ::read(&mut buffer).unwrap();
        assert_eq!(dos_mz, new_dos_mz)
    }
}
