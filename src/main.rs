use std::{fs, path::PathBuf};
use clap::Parser;

#[derive(Debug, Clone, clap::Subcommand)]
enum Command {
    Create { path: PathBuf },
}

#[derive(Debug, clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Create { path } => {
            create(path)?;
        },
    }
    Ok(())
}

fn create(path: PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(&path)?;
    fs::write(&format!("{}/_config.toml", path.to_string_lossy()), "# todo!")?;
    Ok(())
}
