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
use fork::fork;
use init::init;

use clap::Parser;
use rwine_pe::{CoffFileHeader, DosMZ, OpticalHeader, SectionTable};

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

            let dos_header = DosMZ::read(&mut buffer).unwrap();
            println!("DosHeader: {dos_header:#?}");

            let coff_header = CoffFileHeader::read(&mut buffer).unwrap();
            println!("COFFHeader: {coff_header:#?}");

            let optical_header = OpticalHeader::read(&mut buffer).unwrap();
            println!("Optical header: {optical_header:#?}");

            let mut sections = Vec::new();
            for _ in 0..coff_header.number_of_sectors {
                let section_table = SectionTable::read(&mut buffer).unwrap();
                println!("Section Table: {section_table:#?}");
                sections.push(section_table);
            }

            let mut sections_data = std::collections::HashMap::new();

            for section in sections {
                buffer.seek(std::io::SeekFrom::Start(section.pointer_to_raw_data as u64));
                let mut buff = vec![0; section.size_of_raw_data as usize];
                buffer.read_exact(&mut buff);
                sections_data.insert(section.name, buff);
            }

            for (name, data) in sections_data {
                println!("Name: {name}");
                let data = String::from_utf8_lossy(&data).to_string();
                println!("{data}");
            }

            let cursor = buffer.seek(std::io::SeekFrom::Current(0)).unwrap();
            println!("Cursor: {cursor:0X}");

            // match fork().unwrap() {
            //     fork::Fork::Parent(child) => {
            //         for var in std::env::args() {
            //             prctl::set_name("rwine PARENT");
            //         }
            //     }
            //     fork::Fork::Child => {
            //         println!("C: {}", std::process::id());
            //         prctl::set_name("rwine CHILD");
            //         for var in std::env::args() {}
            //     }
            // }
        }
    }
}
