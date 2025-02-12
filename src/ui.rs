use std::fmt::Display;

use crate::finder::{self, FinderItem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folder {
    pub value: String,
}

impl Folder {
    pub fn new(value: String) -> Self {
        Folder { value }
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FinderItem for Folder {
    fn search_include(&self, search: &str) -> bool {
        self.value.to_lowercase().to_lowercase().contains(search)
    }

    fn initial_seleted(&self) -> bool {
        false
    }
}

/// Open ui for folder picker
pub fn select(list: Vec<Folder>, prefix: bool) -> Option<Folder> {
    let search = match prefix {
        true => "@".to_string(),
        false => String::new(),
    };
    finder::ui(list, search).unwrap()
}
