use std::io::{self, Write};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}};
use skills_sync::{PlacementKind, SkillsSync};

pub fn run(app: SkillsSync) -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let result = run_loop(&mut terminal, app);
    ratatui::restore();
    result
}

fn run_loop(
    terminal: &mut DefaultTerminal,
    app: SkillsSync,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let mut message = String::from("↑↓ move · s sync · a adopt · r refresh · q quit");

    loop {
        let status = app.status().unwrap_or_default();
        let wanted = app.resolve_wanted().unwrap_or_default();

        terminal.draw(|frame: &mut Frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(5),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            let main = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
                .split(layout[0]);

            let items: Vec<ListItem> = wanted
                .iter()
                .map(|w| {
                    let mark = if w.from_override { " [ovr]" } else { "" };
                    ListItem::new(format!("{}@{}{mark}", w.name, w.version))
                })
                .collect();
            let list = List::new(items)
                .block(Block::bordered().title("Skills"))
                .highlight_symbol(">> ");
            frame.render_stateful_widget(list, main[0], &mut list_state);

            let selected = list_state.selected().and_then(|i| wanted.get(i));
            let detail = if let Some(w) = selected {
                let st = status.skills.iter().find(|s| s.name == w.name);
                let mut text = format!("Skill: {}\nVersion: {}\n", w.name, w.version);
                if w.from_override {
                    text.push_str("Source: Override\n");
                } else {
                    text.push_str("Source: Lock File\n");
                }
                if let Some(s) = st {
                    text.push_str(&format!("Description: {}\n", s.description));
                    if s.deprecated {
                        text.push_str("DEPRECATED");
                        if let Some(suc) = &s.successor {
                            text.push_str(&format!(" → {suc}"));
                        }
                        text.push('\n');
                    }
                }
                text.push_str("\nSync preview:\n");
                for h in &status.harnesses {
                    text.push_str(&format!("  → {h}/{}\n", w.name));
                }
                text
            } else {
                "No Skills selected in personal.toml".into()
            };
            frame.render_widget(
                Paragraph::new(detail)
                    .block(Block::bordered().title("Details"))
                    .wrap(Wrap { trim: true }),
                main[1],
            );

            let mut ovr = String::from("Overrides: ");
            if status.overrides.is_empty() {
                ovr.push_str("(none)");
            } else {
                ovr.push_str(
                    &status
                        .overrides
                        .iter()
                        .map(|o| format!("{}@{}", o.name, o.version))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
            }
            frame.render_widget(
                Paragraph::new(ovr).block(Block::bordered().title("Status")),
                layout[1],
            );
            frame.render_widget(Paragraph::new(message.as_str()), layout[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        let i = list_state.selected().unwrap_or(0);
                        let max = wanted.len().saturating_sub(1);
                        list_state.select(Some((i + 1).min(max)));
                    }
                    KeyCode::Up => {
                        let i = list_state.selected().unwrap_or(0);
                        list_state.select(Some(i.saturating_sub(1)));
                    }
                    KeyCode::Char('s') => match app.sync() {
                        Ok(r) => {
                            message = format!(
                                "synced {} placement(s), {} warning(s)",
                                r.placements.len(),
                                r.warnings.len()
                            );
                            for p in r.placements {
                                let k = match p.kind {
                                    PlacementKind::Symlink => "symlink",
                                    PlacementKind::Copy => "copy",
                                };
                                let _ = writeln!(io::stderr(), "{} ({k})", p.path.display());
                            }
                        }
                        Err(e) => message = format!("sync error: {e}"),
                    },
                    KeyCode::Char('a') => match app.adopt() {
                        Ok(r) => {
                            message = format!(
                                "adopt updated={}, skipped overrides={}",
                                r.updated.len(),
                                r.skipped_overrides.len()
                            );
                        }
                        Err(e) => message = format!("adopt error: {e}"),
                    },
                    KeyCode::Char('r') => {
                        message = "refreshed".into();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
