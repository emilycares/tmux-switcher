use itertools::Itertools;
use std::{path::Path, process::Command};

/// Get name of last file/folder in path
pub fn get_basename(location: &str) -> Option<&str> {
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

/// Run: zoxide query -l
pub fn get_zoxide_output() -> Option<String> {
    match Command::new("zoxide").args(["query", "-l"]).output() {
        Ok(list) => {
            return Some(
                std::str::from_utf8(&list.stdout)
                    .unwrap_or_default()
                    .to_owned(),
            )
        }
        Err(_) => None,
    }
}

/// Remove all non git folders from the input
pub fn filter_folders(folders: String) -> String {
    folders
        .split('\n')
        .into_iter()
        .filter(|c| is_ignored(c))
        .filter(|c| is_git_dir(c))
        .sorted()
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Ignore specific dirs
fn is_ignored(location: &str) -> bool {
    if location.contains(".local/share/nvim/site/pack/packer/start") {
        return false;
    }

    true
}

/// Determine if directory has a git directory
fn is_git_dir(location: &str) -> bool {
    let git_folder = format!("{}/.git", location);
    let path = Path::new(&git_folder);

    match std::fs::metadata(path) {
        Ok(meta) => meta.is_dir(),
        Err(_) => false,
    }
}
