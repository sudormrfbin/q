use anyhow::{Context, Result};
use skim::prelude::unbounded;
use skim::prelude::SkimOptionsBuilder;
use skim::Skim;
use skim::SkimItem;
use skim::SkimItemReceiver;
use skim::SkimItemSender;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
struct FileItem {
    path: PathBuf,
}

impl SkimItem for FileItem {
    fn text(&self) -> std::borrow::Cow<str> {
        self.path.to_string_lossy()
    }
}

pub fn select(files: &[PathBuf]) -> Result<Option<PathBuf>> {
    let items: Vec<FileItem> = files
        .iter()
        .map(|path| FileItem { path: path.clone() })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("40%"))
        // When setting height, this is apparently required to properly clear
        // the skim ui after selection.
        // https://github.com/lotabout/skim/issues/494#issuecomment-1776565846
        .no_clear_start(true)
        .multi(false)
        .build()
        .context("Failed to build skim options")?;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in items {
        let _ = tx.send(Arc::new(item));
    }
    drop(tx);

    let selected = Skim::run_with(&options, Some(rx))
        .filter(|out| !out.is_abort)
        .and_then(|mut out| out.selected_items.pop())
        .map(|item| {
            item.as_any()
                .downcast_ref::<FileItem>()
                .expect("Failed to downcast skim item")
                .path
                .clone()
        });

    Ok(selected)
}
