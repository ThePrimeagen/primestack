/*
use anyhow::Result;
use std::{path::PathBuf, env::current_dir};
use walkdir::WalkDir;

fn walk(dir: PathBuf) -> Result<Vec<PathBuf>> {

    let mut dir = current_dir()?;
    dir.push("src/pages");
    return Ok(WalkDir::new(&dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|x| x.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect());
}

// { ... }
// const obj = {};
//
// for (const [k,v] of Object.entries(obj)) {
//    obj[k] = v;
// }

fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let dir = PathBuf::from(dir);
    let dir = dir.join("src").join("pages");

    println!("cargo:warning={:?}", dir);
    if let Ok(paths) = walk(dir) {
        println!("cargo:warning=paths={:?}", paths);
    }
}
*/
