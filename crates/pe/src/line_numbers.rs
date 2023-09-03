#[derive(Debug)]
pub struct LineNumbers {
    pub _type: LineNumbersType,

    /// Linenumber
    pub number: u16,
}

#[derive(Debug)]
#[repr(u32)]
pub enum LineNumbersType {
    SymbolTableIndex(u32),
    VirtualAdress(u32),
}
