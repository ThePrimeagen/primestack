#![feature(fs_try_exists)]

mod project;
mod input;
mod opts;

use anyhow::Result;

use clap::Parser;
use crate::{opts::Config, input::init, project::setup_project};

fn main() -> Result<()> {
    let mut config = Config::parse();

    _ = match config {
        Config::Init(i) => {
            setup_project(dbg!(init(i)?));
        }
    };

    return Ok(());
}
