use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use anyhow::{Ok, Result};
use crossterm::event::KeyEventKind;
use crossterm::{
    cursor::{self, Hide, Show},
    event::Event,
    event::{read, KeyCode},
    style::Stylize,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};

pub trait FinderItem {
    fn search_include(&self, search: &str) -> bool;
    fn initial_seleted(&self) -> bool;
}

#[derive(Clone)]
pub struct StageAble<T: Clone> {
    pub staged: bool,
    pub data: T,
}

impl<T: ToString + Clone> ToString for StageAble<T> {
    fn to_string(&self) -> String {
        self.data.to_string()
    }
}

impl<T: Clone> StageAble<T> {
    pub fn new(data: T) -> StageAble<T> {
        StageAble {
            staged: false,
            data,
        }
    }
}

pub fn ui<T>(input: Vec<T>, search: String) -> Result<Option<T>>
where
    T: ToString + FinderItem + Clone + PartialEq,
{
    let mut input: Vec<StageAble<T>> = input
        .into_iter()
        .map(|i| StageAble::new(i))
        .map(|c| {
            let mut init_staged = c;
            init_staged.staged = init_staged.data.initial_seleted();
            init_staged
        })
        .collect();
    let (mut theight, mut twith) = terminal::size()?;
    setup_ui()?;
    let mut stdout = stdout();

    let mut search = search;
    let bind_items = input.clone();
    let mut filtered_items = filter_items(&bind_items, &search);
    let mut selected: usize = 0;
    let mut saved = false;

    render_base(
        theight,
        twith,
        &search,
        selected,
        &filtered_items,
        &mut stdout,
    )?;

    'ui: loop {
        // while poll(Duration::from_millis(100))? {
        match read()? {
            Event::Key(m) => {
                if m.kind == KeyEventKind::Press {
                    match m.code {
                        KeyCode::Esc => break 'ui,
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            if let Some(s) = filtered_items.get(selected) {
                                input = input
                                    .clone()
                                    .into_iter()
                                    .map(|mut i| {
                                        if i.data == s.data {
                                            i.staged = !i.staged;
                                        }
                                        i
                                    })
                                    .collect();
                            }
                            saved = true;
                            break 'ui;
                        }
                        KeyCode::Char(c) => {
                            search.push(c);
                            filtered_items = filter_items(&input, &search);
                            let len = filtered_items.len();
                            if len > 1 {
                                let last = len - 1;
                                if selected > last {
                                    selected = last;
                                }
                            }
                            if len == 0 {
                                selected = 0;
                            }
                        }
                        KeyCode::Backspace => {
                            search.pop();
                            filtered_items = filter_items(&input, &search);
                        }
                        KeyCode::Up => {
                            if selected != usize::MIN {
                                selected = selected.saturating_sub(1);
                            }
                        }
                        KeyCode::Down => {
                            if selected != usize::MAX && selected < filtered_items.len() - 1 {
                                selected = selected.saturating_add(1);
                            }
                        }
                        _ => (),
                    }
                }
            }
            Event::Resize(w, h) => {
                twith = w;
                theight = h;
            }
            _ => (),
            // }
        }

        render_base(
            theight,
            twith,
            &search,
            selected,
            &filtered_items,
            &mut stdout,
        )?;

        thread::sleep(Duration::from_millis(10));
    }
    teardown_ui()?;
    if saved {
        Ok(input.into_iter().find(|a| a.staged).map(|s| s.data))
    } else {
        Ok(None)
    }
}

fn filter_items<'a, T>(items: &'a [StageAble<T>], search: &'a str) -> Vec<&'a StageAble<T>>
where
    T: FinderItem + Clone,
{
    let search = search.to_lowercase();
    items
        .iter()
        .filter(|c| c.data.search_include(&search))
        .collect()
}

fn render_base<T>(
    theight: u16,
    twith: u16,
    search: &String,
    selected: usize,
    filtered_items: &Vec<&StageAble<T>>,
    stdout: &mut std::io::Stdout,
) -> Result<(), anyhow::Error>
where
    T: ToString + Clone,
{
    let lines = render_canvas(&theight, &twith, search, &selected, filtered_items);
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::MoveTo(0, 0))?;
    for line in lines.iter() {
        stdout.queue(cursor::MoveToNextLine(1))?;
        stdout.write_all(line.as_bytes())?;
    }
    stdout.queue(cursor::MoveTo(0, theight))?;
    stdout.write_all("Usage: <Esc>: Close, <Enter>: Edit and close".as_bytes())?;
    stdout.flush()?;
    Ok(())
}

fn render_canvas<T>(
    _theight: &u16,
    _twith: &u16,
    search: &str,
    selected: &usize,
    items: &Vec<&StageAble<T>>,
) -> Vec<String>
where
    T: ToString + Clone,
{
    let mut out = vec![format!("Search: {search}")];
    out.extend(render_items(items, selected));
    out
}

fn render_items<T>(items: &Vec<&StageAble<T>>, selected: &usize) -> Vec<String>
where
    T: ToString + Clone,
{
    items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let mut line = item.to_string();
            if &i == selected {
                line = line.green().to_string();
            }
            line
        })
        .collect()
}

fn setup_ui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    Ok(())
}
fn teardown_ui() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
