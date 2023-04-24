use std::path::PathBuf;

use clap::Parser;

use crate::input::nnitBackends;

#[derive(Debug, Parser, Clone)]
pub struct Init {
    // backend is an enum
    #[clap(short, long)]
    pub backend: Option<InitBackends>,

    #[clap(short, long)]
    pub path: Option<PathBuf>,

    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(short, long)]
    pub turso: Option<bool>,
}

#[derive(Debug, Parser)]
pub enum Config {
    Init(Init),
}


