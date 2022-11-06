use std::{fs, env};

mod tmux;
mod ui;
mod util;

extern crate skim;

fn main() {
    let inside_tmux = std::env::var("TMUX").is_ok();

    if let Some(folder) = get_item() {
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
    }
}

fn get_item() -> Option<String> {
    if let Some(list) = get_project_list() {

        
        return util::remove_running_symbol(ui::select(util::filter_folders(list)));
    }
    None
}

fn get_project_list() -> Option<String> {
    if cfg!(windows) {
        let Ok(location) = env::var("USERPROFILE") else {
            println!("Unbale to read USERPROFILE");

            return None;
        };
        let location = format!("{location}\\tmux-switcher.txt");
        let Ok(content) = fs::read_to_string(&location) else {
            println!("Unble to read file: {location}");

            return None;
        };

        Some(content)
    } else {
        util::get_zoxide_output()
    }
}
