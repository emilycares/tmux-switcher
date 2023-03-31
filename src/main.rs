use serde::{Deserialize, Serialize};

mod tmux;
mod ui;
mod util;

extern crate skim;

#[tokio::main]
async fn main() {
    let storage_location = &quickcfg::get_location("tmux-switcher")
        .await
        .expect("Unable to get storage dir");
    let storage: Storage = quickcfg::load(storage_location).await;

    if let Some(folder) = get_item(storage.projects, storage.use_tmux) {
        if storage.use_tmux {
            let inside_tmux = std::env::var("TMUX").is_ok();
            if let Some(basename) = util::get_basename(&folder) {
                let session_exists = tmux::does_session_exist(basename);

                if !session_exists {
                    tmux::new_session(basename, &folder);
                }

                if inside_tmux {
                    tmux::switch_client(basename);
                } else {
                    tmux::attach_client(basename);
                }
            }
        } else {
            println!("cd {folder}");
        }
    }
}

fn get_item(list: Vec<String>, use_tmux: bool) -> Option<String> {
    util::remove_running_symbol(ui::select(util::filter_folders(list), use_tmux))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub use_tmux: bool,
    pub projects: Vec<String>,
}

impl Default for Storage {
    fn default() -> Self {
        let mut projects = vec![];

        if let Ok(current_folder) = std::env::current_dir() {
            projects.push(current_folder.to_str().unwrap_or("/").to_string())
        }

        Self {
            use_tmux: false,
            projects,
        }
    }
}
