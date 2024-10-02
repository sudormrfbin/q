use std::env;
use std::path::Path;

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

fn main() -> Result<()> {
    let args = Args::parse();
    let config = sources::load()?;
    if let Some(nickname) = args.nickname {
        if let Some(file) = config.nicknames.get(&nickname) {
            open_in_editor(Path::new(file))?;
        } else {
            eprintln!("Nickname not found",);
        }
    }

    Ok(())
}
