use std::path::PathBuf;

use clap::Parser;

use crate::input::InitBackends;

#[derive(Debug, Parser, Clone)]
pub struct Init {
    // backend is an enum
    #[clap(short, long)]
    pub backend: Option<InitBackends>,

    #[clap(short, long)]
    pub path: Option<PathBuf>,

    #[clap(short, long)]
    pub name: Option<String>,
}

#[derive(Debug, Parser)]
pub enum Config {
    Init(Init),
}
impl Config {
    pub(crate) fn parse() -> Config {
        todo!()
    }
}


