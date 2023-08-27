mod build;
mod create_prefix;
mod init;
mod reactor;

use std::{
    char::CharTryFromError,
    fs::File,
    io::{BufReader, Read, Seek},
    path::PathBuf,
};

use build::build;
use create_prefix::create_prefix;
use init::init;

use clap::Parser;

#[derive(clap::Subcommand)]
pub enum Commands {
    Init,
    Build,
    Run { file: String },
    CreatePrefix,
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .try_init()
        .unwrap();
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => init(),
        Commands::Build => build(),
        Commands::CreatePrefix => create_prefix(),
        Commands::Run { file } => {
            let mut path = PathBuf::from(file);
            if !path.try_exists().expect("Cannot check if that path exist") {
                let new_path = PathBuf::from("./target/x86_64-pc-windows-msvc/debug").join(&path);
                if new_path.try_exists().unwrap() {
                    path = new_path;
                } else {
                    panic!("The path do not exists: {path:?}");
                }
            }

            let mut file = std::fs::File::options().read(true).open(path).unwrap();

            let mut buffer = BufReader::new(file);

            let dos_header = DosHeader::try_from(&mut buffer).unwrap();
            println!("DosHeader: {dos_header:?}");

            let coff_header = COFFHeader::try_from(&mut buffer).unwrap();
            println!("COFFHeader: {coff_header:?}");

            return;
            let mut magic = [0u8; 2];
            file.read_exact(&mut magic).unwrap();

            let mut major_linker_version = [0u8];
            file.read_exact(&mut major_linker_version).unwrap();

            let mut minor_linker_version = [0u8];
            file.read_exact(&mut minor_linker_version).unwrap();

            let mut size_of_code = [0u8; 4];
            file.read_exact(&mut size_of_code).unwrap();

            let mut size_of_initialized_data = [0u8; 4];
            file.read_exact(&mut size_of_initialized_data).unwrap();

            let mut size_of_uninitialized_data = [0u8; 4];
            file.read_exact(&mut size_of_uninitialized_data).unwrap();

            let mut adress_of_entry_point = [0u8; 4];
            file.read_exact(&mut adress_of_entry_point).unwrap();

            let magic = u16::from_le_bytes(magic);

            let major_linker_version = major_linker_version[0];
            let minor_linker_version = minor_linker_version[0];
            let size_of_code = u32::from_le_bytes(size_of_code);
            let size_of_initialized_data = u32::from_le_bytes(size_of_initialized_data);
            let size_of_uninitialized_data = u32::from_le_bytes(size_of_uninitialized_data);
            let adress_of_entry_point = u32::from_le_bytes(adress_of_entry_point);

            println!("Magic: {magic:0X}");
            println!("Linker version: {major_linker_version}.{minor_linker_version}");
            println!("Size Of Code: {size_of_code}");
            println!("Size of Init Data: {size_of_initialized_data}");
            println!("Size of Uninit Data: {size_of_uninitialized_data}");
            println!("Adress of entry point: {adress_of_entry_point}");
        }
    }
}

#[derive(Debug)]
pub struct DosHeader {
    pub extra_bytes: u16,
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

    pub stub: Vec<u8>,
}

#[derive(Debug)]
pub enum DosHeaderError {
    InvalidMagic,
    CheckSumFailed,
    IO(std::io::Error),
}

impl From<std::io::Error> for DosHeaderError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl TryFrom<&mut BufReader<File>> for DosHeader {
    type Error = DosHeaderError;

    fn try_from(data: &mut BufReader<File>) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; 2];

        data.read_exact(&mut buffer)?;
        let magic = u16::from_le_bytes(buffer);
        if magic != 0x5A4D {
            return Err(DosHeaderError::InvalidMagic);
        }

        data.read_exact(&mut buffer)?;
        let extra_bytes = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let pages = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let realocation_items = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let header_size = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let minimum_allocation = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let maximum_allocation = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let initial_ss = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let initial_sp = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let checksum = u16::from_le_bytes(buffer);
        if checksum != 0 {
            return Err(DosHeaderError::CheckSumFailed);
        }

        data.read_exact(&mut buffer)?;
        let initial_ip = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let initial_cs = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let realocation_table = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let overlay = u16::from_le_bytes(buffer);

        {
            let mut buffer = [0u8; 8];
            data.read_exact(&mut buffer)?;
            println!("Reserverd: {buffer:?}");
        }
        data.read_exact(&mut buffer)?;
        let oem_identifier = u16::from_le_bytes(buffer);

        data.read_exact(&mut buffer)?;
        let oem_info = u16::from_le_bytes(buffer);

        {
            let mut buffer = [0u8; 20];
            data.read_exact(&mut buffer)?;
            println!("Reserved: {buffer:?}");
        }

        let mut buffer = [0u8; 4];
        data.read_exact(&mut buffer)?;
        let pe_header_start = u32::from_le_bytes(buffer);

        let mut buffer = vec![0u8; extra_bytes as usize - 0x40];
        data.read_exact(&mut buffer)?;

        let stub = buffer;

        Ok(Self {
            extra_bytes,
            pages,
            realocation_items,
            header_size,
            minimum_allocation,
            maximum_allocation,
            initial_ss,
            initial_sp,
            initial_ip,
            initial_cs,
            realocation_table,
            overlay,
            oem_identifier,
            oem_info,
            pe_header_start,
            stub,
        })
    }
}

#[derive(Debug)]
#[repr(u16)]
pub enum Machine {
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

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct Characteristic: u16 {
        /// Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files.
        const ImageFileRelocsStripped = 0x0001;
        ///Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error.
        const ImageFileExecutableImage = 0x0002;
        ///COFF line numbers have been removed. This flag is deprecated and should be zero.
        const ImageFileLineNumsStripped = 0x0004;
        ///COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
        const ImageFileLocalSymsStripped = 0x0008;
        ///Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
        const ImageFileAggressiveWsTrim = 0x0010;
        ///Application can handle > 2-GB addresses.
        const ImageFileLargeAddressAware = 0x0020;
        ///This flag is reserved for future use.
        const RESERVED = 0x0040;
        ///Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero.
        const ImageFileBytesReversedLo = 0x0080;
        ///Machine is based on a 32-bit-word architecture.
        const ImageFile32bitMachine = 0x0100;
        ///Debugging information is removed from the image file.
        const ImageFileDebugStripped = 0x0200;
        ///If the image is on removable media, fully load it and copy it to the swap file.
        const ImageFileRemovableRunFromSwap = 0x0400;
        ///If the image is on network media, fully load it and copy it to the swap file.
        const ImageFileNetRunFromSwap = 0x0800;
        ///The image file is a system file, not a user program.
        const ImageFileSystem = 0x1000;
        ///The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
        const ImageFileDll = 0x2000;
        ///The file should be run only on a uniprocessor machine.
        const ImageFileUpSystemOnly = 0x4000;
        ///Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
        const ImageFileBytesReversedHi = 0x8000;
    }
}

#[derive(Debug)]
pub struct COFFHeader {
    pub machine: Machine,
    pub number_of_sectors: u16,
    pub time_data_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: Characteristic,
}

#[derive(Debug)]
pub enum COFFHeaderError {
    InvalidMagic,
    IO(std::io::Error),
}

impl From<std::io::Error> for COFFHeaderError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl TryFrom<&mut BufReader<File>> for COFFHeader {
    type Error = COFFHeaderError;

    fn try_from(data: &mut BufReader<File>) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; 4];
        data.read_exact(&mut buffer)?;
        let magic = u32::from_le_bytes(buffer);

        if magic != 0x00004550 {
            println!("COFF Magic: {magic:0X}");
            return Err(COFFHeaderError::InvalidMagic);
        }

        let mut buffer = [0u8; 2];
        data.read_exact(&mut buffer)?;

        let machine: Machine = u16::from_le_bytes(buffer).try_into().unwrap();
        data.read_exact(&mut buffer)?;
        let number_of_sectors = u16::from_le_bytes(buffer);

        let mut buffer = [0u8; 4];
        data.read_exact(&mut buffer)?;

        let time_data_stamp = u32::from_le_bytes(buffer);
        data.read_exact(&mut buffer)?;
        let pointer_to_symbol_table = u32::from_le_bytes(buffer);
        data.read_exact(&mut buffer)?;
        let number_of_symbols = u32::from_le_bytes(buffer);

        let mut buffer = [0u8; 2];
        data.read_exact(&mut buffer)?;

        let size_of_optional_header = u16::from_le_bytes(buffer);
        data.read_exact(&mut buffer)?;
        let characteristics = Characteristic::from_bits_retain(u16::from_le_bytes(buffer));
        Ok(Self {
            machine,
            number_of_sectors,
            time_data_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        })
    }
}
