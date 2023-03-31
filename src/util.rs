use itertools::Itertools;
use std::path::Path;

use crate::tmux;

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

/// Remove all non git folders from the input
pub fn filter_folders(folders: Vec<String>) -> String {
    folders
        .into_iter()
        .filter(|c| is_ignored(c))
        .filter(|c| is_git_dir(c))
        .sorted()
        .map(|m| m.replace(".", "_"))
        .map(running)
        .collect::<Vec<String>>()
        .join("\n")
}

fn running(c: String) -> String {
    let Some(basename) = get_basename(&c) else {
        return c.to_string();
    };

    match tmux::does_session_exist(basename) {
        true => {
            format!("@{c}")
        }
        false => c.to_string(),
    }
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

pub fn remove_running_symbol(item: Option<String>) -> Option<String> {
    if let Some(item) = item {
        if item.starts_with('@') {
            let item = item.get(1..).unwrap_or_default().to_string();

            return Some(item);
        } else {
            return Some(item);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::util::remove_running_symbol;

    #[test]
    fn remove_running_symbol_test() {
        let path_with_running_symbol = Some("@/tmp/dotfiles".to_string());

        let out = remove_running_symbol(path_with_running_symbol);

        assert_eq!(out, Some("/tmp/dotfiles".to_string()));
    }
}
