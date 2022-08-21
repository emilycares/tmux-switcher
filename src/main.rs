mod tmux;

extern crate skim;

use skim::prelude::*;
use std::io::Cursor;
use std::path::Path;
use std::{env::var, process::Command};

fn main() {
    let inside_tmux = var("TMUX").is_ok();

    if let Some(folder) = get_item() {
        if let Some(basename) = get_basename(&folder) {
            let session_exists = tmux::does_session_exist(&basename);

            if !session_exists {
                tmux::new_session(&basename, &folder);
            }

            if inside_tmux {
                tmux::switch_client(&basename);
            } else {
                tmux::attach_client(&basename);
            }
        }
    }
}

fn get_basename(location: &str) -> Option<&str> {
    let path = Path::new(location);
    let basename = path.components().last();

    if let Some(component) = basename {
        return match component {
            std::path::Component::Prefix(_) => None,
            std::path::Component::RootDir => None,
            std::path::Component::CurDir => None,
            std::path::Component::ParentDir => None,
            std::path::Component::Normal(basename) => {
                if let Some(basename) = basename.to_str() {
                    return Some(basename);
                }

                None
            }
        };
    }

    None
}

fn get_item() -> Option<String> {
    match Command::new("zoxide").args(["query", "-l"]).output() {
        Ok(list) => {
            let list = std::str::from_utf8(&list.stdout).unwrap_or_default();
            return select(list);
        }
        Err(_) => None,
    }
}

fn select(list: &str) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        //.height(Some("50%"))
        //.multi(false)
        .build()
        .unwrap();

    let input = list.to_string();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // `run_with` would read and show items from the stream
    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    if let Some(item) = selected_item.first() {
        let text = item.text();

        return Some(text.into_owned());
    }

    None
}
