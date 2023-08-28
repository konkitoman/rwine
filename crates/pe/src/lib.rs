//! Source [microsoft](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#coff-file-header-object-and-image)

mod characteristic;
mod file_header;
mod machine;
mod mz;
mod optical_header;
mod read;
mod subsystem;
mod write;

pub use characteristic::Characteristic;
pub use machine::Machine;
pub use subsystem::Subsystem;

pub use file_header::*;
pub use mz::*;
pub use optical_header::*;
