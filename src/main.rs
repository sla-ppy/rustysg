use std::{fs, path::PathBuf};
use chrono::{NaiveDate, NaiveTime};
use clap::Parser;
use template::{context::{Context, Metadata}, engine::Engine, handlebars_engine::HandlebarsEngine};

mod template;

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

fn generate(_path: PathBuf) -> anyhow::Result<()> {
    let template: String = fs::read_to_string("template.html")?;
    println!("Generating!");
    let context = Context::new("This is my content!".to_string(), Metadata {
        title: "My cool post".to_string(),
        description: "My first awesome post that I wrote".to_string(),
        author: "Me Myself and I".to_string(),
        date: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
        time: NaiveTime::from_hms_opt(10, 11, 12).unwrap(),
    });
    let mut engine = HandlebarsEngine::new();
    let result = engine.render(&template, context);
    println!("{}", result);
    Ok(())
}
