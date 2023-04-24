mod project;
mod input;
mod opts;

use anyhow::Result;

use clap::Parser;
use crate::{opts::Config, input::init};

fn main() -> Result<()> {
    let mut config = Config::parse();

    _ = match &mut config {
        Config::Init(i) => init(i),
    };

    println!("config: {:?}", config);
    return Ok(());
}
