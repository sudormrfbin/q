use ignore::WalkBuilder;
use sources::Sources;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

mod sources;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    nickname: Option<String>,
}

fn open_in_editor(path: &Path) -> Result<()> {
    let editor = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .or_else(|_| env::var("EDIT"))
        .with_context(|| {
            "cannot find editor to open files, set the $EDIROR environment variable"
        })?;

    std::process::Command::new(editor).arg(path).status()?;
    Ok(())
}

fn get_all_files(sources: &Sources) -> Vec<PathBuf> {
    sources
        .directories
        .iter()
        .map(|dir| WalkBuilder::new(dir).build())
        .flatten()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|e| e.is_file()))
        .map(|file| file.into_path())
        .collect()

    // for dir in &sources.directories {
    //     let walker = WalkBuilder::new(dir).build();
    //     let files = walker
    //         .filter_map(|e| e.ok())
    //         .filter(|e| e.file_type().is_some_and(|f| f.is_file()))
    //         .map(|f| f.into_path());
    //     all_files.extend(files);
    // }

    // Ok(all_files)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let sources = sources::load()?;
    if let Some(nickname) = args.nickname {
        if let Some(file) = sources.nicknames.get(&nickname) {
            open_in_editor(Path::new(file))?;
        } else {
            eprintln!("Nickname not found",);
        }
    } else {
        get_all_files(&sources)
            .iter()
            .for_each(|f| println!("{f:?}"));
    }

    Ok(())
}
