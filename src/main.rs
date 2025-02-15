use serde::{Deserialize, Serialize};

mod finder;
mod tmux;
mod ui;
mod util;


use clap::Parser;

/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Add this folder to the choosable locaions
    #[clap(long)]
    pub add_this: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let storage_location = &quickcfg::get_location("tmux-switcher")
        .await
        .expect("Unable to get storage dir");
    let mut storage: Storage = quickcfg::load(storage_location).await;

    if args.add_this {
        if let Ok(current_dir) = std::env::current_dir() {
            storage
                .projects
                .push(current_dir.to_string_lossy().to_string());
            let _ = quickcfg::save(storage, storage_location).await;
        }

        return;
    }

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
    util::remove_running_symbol(ui::select(util::filter_folders(list), use_tmux).map(|f| f.value))
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
            use_tmux: true,
            projects,
        }
    }
}
