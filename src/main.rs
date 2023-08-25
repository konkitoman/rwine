mod build;
mod create_prefix;
mod init;
mod reactor;

use build::build;
use create_prefix::create_prefix;
use init::init;

use clap::Parser;

#[derive(clap::Subcommand)]
pub enum Commands {
    Init,
    Build,
    Run,
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
        Commands::Run => todo!(),
    }
}
