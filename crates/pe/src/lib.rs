//! Source [microsoft](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#coff-file-header-object-and-image)

mod characteristic;
mod coff_file_header;
mod coff_realocations;
mod line_numbers;
mod machine;
mod mz;
mod optical_header;
mod read;
mod section_table;
mod subsystem;
mod type_indicators;
mod write;

pub use characteristic::CoffCharacteristic;
pub use machine::Machine;
pub use subsystem::Subsystem;

pub use coff_file_header::*;
pub use line_numbers::*;
pub use mz::*;
pub use optical_header::*;
pub use section_table::*;
