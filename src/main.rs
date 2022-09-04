mod tmux;
mod ui;
mod util;

extern crate skim;

fn main() {
    let inside_tmux = std::env::var("TMUX").is_ok();

    if let Some(folder) = get_item() {
        if let Some(basename) = util::get_basename(&folder) {
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


fn get_item() -> Option<String> {
    match util::get_zoxide_output() {
        Some(list) => ui::select(util::filter_folders(list)),
        None => None,
    }
}

