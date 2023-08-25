mod build;
mod init;
mod reactor;

use build::build;
use init::init;

use clap::Parser;

#[derive(clap::Subcommand)]
pub enum Commands {
    Init,
    Build,
    Run,
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
        Commands::Run => todo!(),
    }
}
