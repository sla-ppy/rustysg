use std::{fs, path::PathBuf};
use clap::Parser;

#[derive(Debug, Clone, clap::Subcommand)]
enum Command {
    Create { path: PathBuf },
    Generate { path: PathBuf },
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
        Command::Generate { path } => {
            generate(path)?;
        },
    }
    Ok(())
}

fn create(path: PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(&path)?;
    fs::write(&format!("{}/_config.toml", path.to_string_lossy()), "# todo!")?;
    fs::create_dir(&format!("{}/template", path.to_string_lossy()))?;
    fs::write(&format!("{}/template/base_template.html", path.to_string_lossy()), "")?;
    fs::write(&format!("{}/content.html", path.to_string_lossy()),"")?;
    Ok(())
}

fn generate(path: PathBuf) -> anyhow::Result<()> {
    let path: String = path.to_string_lossy().to_string();
    let mut template: String = fs::read_to_string(path + "/template/base_template.html")?;
    println!("Generating!");
    let input = "Hello World!";
    template = template.replace("{{ content.html }}", input);
    println!("{}", template);
    Ok(())
}
