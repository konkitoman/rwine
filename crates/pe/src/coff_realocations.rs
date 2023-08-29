use crate::type_indicators::TypeIndicators;

#[derive(Debug)]
pub struct CoffRealocations {
    pub virtual_adress: u32,
    pub symbol_table_index: u32,
    pub type_indicators: TypeIndicators,
}
