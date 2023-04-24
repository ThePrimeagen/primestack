/** this is really shitty code, but i haven't made a tui yet, so i don't care **/
use anyhow::Result;
use std::{str::FromStr, path::PathBuf};

use clap::{Parser, ValueEnum};

use crate::opts::Init;

pub const BACKENDS: [&str; 2] = ["vercel", "actix"];

#[derive(ValueEnum, Debug, Parser, Clone)]
pub enum InitBackends {
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
        };
    }
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

pub fn init(init: &mut Init) -> Result<()> {
    _ = select_backend(init);
    _ = select_name(init);
    _ = select_path(init);

    return Ok(());
}
