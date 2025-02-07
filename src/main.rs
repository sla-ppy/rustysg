use std::{fs, path::PathBuf};
use clap::Parser;

#[derive(Debug, Clone, clap::Subcommand)]
enum Command {
    Create { path: PathBuf },
    Generate,
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
        Command::Generate => {
            generate()?;
        },
    }
    Ok(())
}

fn create(path: PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(&path)?;
    fs::write(&format!("{}/_config.toml", path.to_string_lossy()), "# todo!")?;
    Ok(())
}

fn generate() -> anyhow::Result<()> {
    println!("Generating!");
    Ok(())
}