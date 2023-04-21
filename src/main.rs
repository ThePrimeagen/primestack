use std::{str::FromStr, path::PathBuf};

use anyhow::Result;
use clap::{Parser, ValueEnum};

const BACKENDS: [&str; 2] = ["vercel", "actix"];

#[derive(ValueEnum, Debug, Parser, Clone)]
enum InitBackends {
    Vercel,
    Actix,
}

impl FromStr for InitBackends {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "vercel" => Ok(InitBackends::Vercel),
            "actix" => Ok(InitBackends::Actix),
            _ => Err(anyhow::anyhow!("Invalid backend")),
        }
    }
}

#[derive(Debug, Parser, Clone)]
struct Init {
    // backend is an enum
    #[clap(short, long)]
    backend: Option<InitBackends>,

    #[clap(short, long)]
    path: Option<PathBuf>,

    #[clap(short, long)]
    name: Option<String>,
}

#[derive(Debug, Parser)]
enum Config {
    Init(Init),
}

fn select_backend(init: &mut Init) -> Result<()> {
    if init.backend.is_some() {
        return Ok(());
    }

    loop {
        println!("Select a backend: {}", BACKENDS.join(", "));

        // read from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if BACKENDS.contains(&input) {
            init.backend = Some(input.parse()?);
            break;
        }
        println!("Invalid backend");
    }

    return Ok(());
}

fn select_name(init: &mut Init) -> Result<()> {
    if init.name.is_some() {
        return Ok(());
    }

    println!("What is the project name?");

    // read from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    init.name = Some(input.trim().to_string());

    return Ok(());
}

fn select_path(init: &mut Init) -> Result<()> {
    if init.path.is_some() {
        return Ok(());
    }
    loop {
        println!("What directory to use? (default: ./)");

        // read from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let mut input = input.trim();
        if input.is_empty() {
            input = ".";
        }
        if let Ok(path) = PathBuf::from_str(input.trim()) {
            init.path = Some(path);
            break;
        }
        println!("Invalid path");
    }

    return Ok(());
}

fn init(init: &mut Init) -> Result<()> {
    _ = select_backend(init);
    _ = select_name(init);
    _ = select_path(init);

    return Ok(());
}

fn main() -> Result<()> {
    let mut config = Config::parse();

    _ = match &mut config {
        Config::Init(i) => init(i),
    };

    println!("config: {:?}", config);
    return Ok(());
}
