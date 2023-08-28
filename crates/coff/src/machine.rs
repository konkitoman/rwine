#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(u16)]
pub enum Machine {
    #[default]
    Unknown = 0x000,
    Alpha = 0x184,
    Alpha64 = 0x284,
    Am33 = 0x1d3,
    Amd64 = 0x8664,
    Arm = 0x1c0,
    Arm64 = 0xaa64,
    Armnt = 0x1c4,
    Ebc = 0xebc,
    I386 = 0x14c,
    Ia64 = 0x200,
    Loongarch32 = 0x6232,
    Loongarch64 = 0x6264,
    M32r = 0x9041,
    Mips16 = 0x266,
    Mipsfpu = 0x366,
    Mipsfpu16 = 0x466,
    Powerpc = 0x1f0,
    Powerpcfp = 0x1f1,
    R4000 = 0x166,
    Riscv32 = 0x5032,
    Riscv64 = 0x5064,
    Riscv128 = 0x5128,
    Sh3 = 0x1a2,
    Sh3dsp = 0x1a3,
    Sh4 = 0x1a6,
    Sh5 = 0x1a8,
    Thumb = 0x1c2,
    Wcemipsv2 = 0x169,
}

impl TryFrom<u16> for Machine {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0x000 => Self::Unknown,
            0x184 => Self::Alpha,
            0x284 => Self::Alpha64,
            0x1d3 => Self::Am33,
            0x8664 => Self::Amd64,
            0x1c0 => Self::Arm,
            0xaa64 => Self::Arm64,
            0x1c4 => Self::Armnt,
            0xebc => Self::Ebc,
            0x14c => Self::I386,
            0x200 => Self::Ia64,
            0x6232 => Self::Loongarch32,
            0x6264 => Self::Loongarch64,
            0x9041 => Self::M32r,
            0x266 => Self::Mips16,
            0x366 => Self::Mipsfpu,
            0x466 => Self::Mipsfpu16,
            0x1f0 => Self::Powerpc,
            0x1f1 => Self::Powerpcfp,
            0x166 => Self::R4000,
            0x5032 => Self::Riscv32,
            0x5064 => Self::Riscv64,
            0x5128 => Self::Riscv128,
            0x1a2 => Self::Sh3,
            0x1a3 => Self::Sh3dsp,
            0x1a6 => Self::Sh4,
            0x1a8 => Self::Sh5,
            0x1c2 => Self::Thumb,
            0x169 => Self::Wcemipsv2,
            _ => return Err(()),
        })
    }
}
