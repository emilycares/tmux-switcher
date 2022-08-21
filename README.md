# tmux-switcher

A tool for creating tmux sessions and switching easily between them.

## Dependencies
- tmux
- zoxide

## Usage
1. Build the project

``` bash
cargo build --release
```

2. Add the following to your tmux config. Do not forget to change the file location and restart your tmux server

``` txt
bind T display-popup -E "/home/michael/Documents/rust/tmux-switcher/target/release/tmux-switcher"
```

3. Use zoxide, cd into your projects, and they will appear in the list
