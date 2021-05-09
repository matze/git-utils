mod event;
mod git;
mod list;

use anyhow::Result;
use event::{Event, Events};
use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Clear, List, ListItem};
use tui::Terminal;

struct Item {
    name: String,
    selected: bool,
}

impl Item {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            selected: false,
        }
    }

    fn to_list_item(&self) -> ListItem {
        let select_char = if self.selected { "☠️  " } else { "   " };
        ListItem::new(Spans::from(vec![
            Span::raw(select_char),
            Span::raw(&self.name),
        ]))
    }

    fn toggle(&mut self) {
        self.selected = !self.selected;
    }
}

struct App {
    list: list::List<Item>,
}

impl App {
    fn new(branches: &[&str]) -> App {
        let items = branches
            .iter()
            .filter_map(|b| {
                if b != &"master" {
                    Some(Item::new(b))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        App {
            list: list::List::new(items),
        }
    }
}

fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let branches = git::branches()?;
    let events = Events::new();
    let mut app = App::new(&branches.iter().map(|s| s as &str).collect::<Vec<_>>());

    println!("{}", termion::clear::All);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let items = app
                .list
                .items
                .iter()
                .map(|item| item.to_list_item())
                .collect::<Vec<_>>();

            let list =
                List::new(items).highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_widget(Clear, f.size());
            f.render_stateful_widget(list, chunks[0], &mut app.list.state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('j') => {
                    app.list.next();
                }
                Key::Char('k') => {
                    app.list.previous();
                }
                Key::Char(' ') => {
                    if let Some(item) = app.list.selected() {
                        item.toggle();
                    }
                }
                _ => {}
            }
        }
    }

    println!("{}", termion::clear::All);

    let selected = app
        .list
        .items
        .iter()
        .filter(|item| item.selected)
        .collect::<Vec<_>>();

    if !selected.is_empty() {
        git::delete_branches(
            &selected
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>(),
        )?;
    }

    Ok(())
}
