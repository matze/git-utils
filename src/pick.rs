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
use std::env;
use std::io;
use std::process::Command;
use termion::event::Key;
use termion::raw::IntoRawMode;

#[derive(Debug)]
struct Commit {
    hash: String,
    message: String,
}

impl Commit {
    fn from(line: &str) -> Self {
        let split = line.split_once(' ').unwrap();

        Self {
            hash: split.0.to_string(),
            message: split.1.to_string(),
        }
    }
}

struct Item {
    commit: Commit,
    selected: bool,
}

impl Item {
    fn new(commit: Commit) -> Self {
        Self {
            commit,
            selected: false,
        }
    }

    fn to_list_item(&self) -> ListItem {
        let select_char = if self.selected { "[•] " } else { "[ ] " };

        ListItem::new(Line::from(vec![
            Span::raw(select_char),
            Span::raw(&self.commit.hash),
            Span::raw(": "),
            Span::raw(&self.commit.message),
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
    fn new(commits: Vec<Commit>) -> App {
        App {
            list: list::List::new(commits.into_iter().map(Item::new).collect()),
        }
    }

    fn selected(self) -> Vec<Commit> {
        self.list
            .items
            .into_iter()
            .filter(|item| item.selected)
            .map(|item| item.commit)
            .collect()
    }
}

fn is_branch(name: &str) -> Result<bool> {
    Ok(Command::new("git")
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{}", name),
        ])
        .output()?
        .status
        .success())
}

fn commits(branch: &str) -> Result<Vec<Commit>> {
    let output = Command::new("git")
        .args(["log", branch, "^HEAD", "--no-merges", "--oneline"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to list commits"));
    }

    let output = std::str::from_utf8(&output.stdout)?.trim();
    Ok(output.split('\n').map(Commit::from).collect())
}

fn pick(commits: Vec<Commit>) -> Result<()> {
    for commit in commits.into_iter().rev() {
        let output = Command::new("git")
            .args(["cherry-pick", &commit.hash])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to cherry-pick {} ({})",
                commit.hash,
                commit.message
            ));
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow!("No branch to cherry pick from given"));
    }

    let branch = &args[1];

    if !is_branch(branch)? {
        return Err(anyhow!("{} is not a valid branch", branch));
    }

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let mut app = App::new(commits(branch)?);

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
                Paragraph::new("Choose commits using <h>, <j>, <space> and select with <q>"),
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

    pick(app.selected())?;

    Ok(())
}
