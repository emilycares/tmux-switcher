use skim::prelude::*;
use std::io::Cursor;

/// Open ui for folder picker
pub fn select(list: String, prefix: bool) -> Option<String> {
    let mut options = SkimOptionsBuilder::default();
    //.height(Some("50%"))
    //.multi(false)
    if prefix {
        options.query(Some("@"));
    }
    let options = options.build().unwrap();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(list));

    // `run_with` would read and show items from the stream
    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if let Some(item) = selected_item.first() {
        let text = item.text();

        return Some(text.into_owned());
    }

    None
}
