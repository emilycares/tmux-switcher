mod tmux;
mod ui;

extern crate skim;

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
    if let Some(config) = get_config_file_location() {
        let list = match get_config_file_output(config.to_owned()) {
            Some(file_data) => {
                if file_data.eq("") {
                    match get_zoxide_output() {
                        Some(data) => Some(data),
                        None => {
                            std::fs::write(config, file_data).unwrap_or_default();
                            None
                        }
                    }
                } else {
                    Some(file_data)
                }
            }
            None => get_zoxide_output(),
        };

        if let Some(list) = list {
            ui::select(list)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_zoxide_output() -> Option<String> {
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

fn get_config_file_output(location: String) -> Option<String> {
    let path = Path::new(&location);

    if path.exists() && path.is_file() {
        Some(std::fs::read_to_string(path).unwrap_or_default())
    } else {
        None
    }
}

fn get_config_file_location() -> Option<String> {
    match dirs::config_dir() {
        Some(path) => {
            let path = format!("{}/{}", path.display(), ".tmux-switcher");
            Some(path)
        }
        None => None,
    }
}
