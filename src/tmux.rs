use tmux_interface::TmuxCommand;

pub fn does_session_exist(name: &str) -> bool {
    let tmux = TmuxCommand::new();

    let sessions = tmux.list_sessions().output();

    if let Ok(sessions) = sessions {
        let sessions = sessions.to_string();
        return sessions
            .split('\n')
            .filter_map(|item| item.split_once(':'))
            .map(|item| item.0)
            .any(|x| x == name);
    }

    false
}

pub fn attach_client(name: &str) {
    let tmux = TmuxCommand::new();

    match tmux.attach_session().target_session(name).output() {
        Ok(_) => println!("attached"),
        Err(_) => println!("not attached"),
    }
}

pub fn switch_client(name: &str) {
    let tmux = TmuxCommand::new();

    match tmux.switch_client().target_session(name).output() {
        Ok(_) => println!("switched"),
        Err(_) => println!("not switched"),
    }
}

pub fn new_session(name: &str, directory: &str) {
    let tmux = TmuxCommand::new();

    match tmux
        .new_session()
        .detached()
        .start_directory(directory)
        .session_name(name)
        .output()
    {
        Ok(_) => println!("created"),
        Err(_) => println!("not created"),
    }
}
