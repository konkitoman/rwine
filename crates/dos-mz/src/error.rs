#[derive(Debug)]
pub enum DosMZError {
    InvalidMagic,
    CheckSumFailed,
    IO(std::io::Error),
}

impl From<std::io::Error> for DosMZError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
