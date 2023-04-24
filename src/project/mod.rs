use std::fs;

use anyhow::{bail, Context, Result};

use crate::opts::Init;

fn init_dir(init: &Init) -> Result<()> {
    let path = init.path.expect("this should of been setup by now");
    if let Ok(exists) = fs::try_exists(path) {
        if exists && path.is_file() {
            bail!("path provided {:?} exists and is a file", path);
        } else if !exists {
            fs::create_dir_all(path)
                .context(format!("failed to create project directory: {:?}", path))?;
        }
    }

    return Ok(());
}

fn init_rust(init: &Init) -> Result<()> {

    return Ok(());
}

fn init_turso(init: &Init) -> Result<()> {

    return Ok(());
}

fn init_vercel(init: &Init) -> Result<()> {

    return Ok(());
}

fn init_leptos(init: &Init) -> Result<()> {

    return Ok(());
}

fn init_example(init: &Init) -> Result<()> {

    return Ok(());
}

pub fn setup_project(init: Init) -> Result<()> {

    init_dir(&init)?;
    init_rust(&init)?;
    init_turso(&init)?;
    init_vercel(&init)?;
    init_leptos(&init)?;
    init_example(&init)?;

    return Ok(());
}
