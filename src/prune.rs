mod event;
mod list;

use anyhow::{anyhow, Result};
use event::{Event, Events};
use ratatui::backend::TermionBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, List, ListItem, Paragraph};
use ratatui::Terminal;
use std::io;
use std::process::Command;
use termion::event::Key;
use termion::raw::IntoRawMode;

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
        let select_char = if self.selected { "[•] " } else { "[ ] " };

        ListItem::new(Line::from(vec![
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

    fn selected(&self) -> Vec<&str> {
        self.list
            .items
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.name.as_str())
            .collect()
    }
}

fn branches() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(&["for-each-ref", "--format=%(refname:short)", "refs/heads/"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to list branches"));
    }

    let output = std::str::from_utf8(&output.stdout)?.trim();
    Ok(output.split('\n').map(String::from).collect())
}

fn delete_branches(branches: &[&str]) -> Result<()> {
    if branches.is_empty() {
        return Ok(());
    }

    let mut args = vec!["branch", "-D"];
    args.extend_from_slice(branches);

    let output = Command::new("git").args(&args).output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to delete branches"));
    }

    Ok(())
}

fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let branches = branches()?;
    let events = Events::new();
    let mut app = App::new(&branches.iter().map(|s| s as &str).collect::<Vec<_>>());

    println!("{}", termion::clear::All);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let items = app
                .list
                .items
                .iter()
                .map(|item| item.to_list_item())
                .collect::<Vec<_>>();

            let list = List::new(items).highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightMagenta),
            );

            f.render_widget(Clear, f.size());
            f.render_widget(
                Paragraph::new("Choose branches using <h>, <j>, <space> and select with <q>"),
                chunks[0],
            );
            f.render_stateful_widget(list, chunks[2], &mut app.list.state);
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
                        app.list.next();
                    }
                }
                _ => {}
            },
        }
    }

    delete_branches(&app.selected())?;

    Ok(())
}
