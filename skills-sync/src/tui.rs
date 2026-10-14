use std::io::{self, Write};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};
use skills_sync::{PlacementKind, SkillsSync};

use crate::style;

pub fn run(app: SkillsSync) -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let result = run_loop(&mut terminal, app);
    ratatui::restore();
    result
}

fn title_style() -> Style {
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

fn run_loop(
    terminal: &mut DefaultTerminal,
    app: SkillsSync,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let mut message = Line::from(vec![Span::styled(
        "↑↓ move · s sync · a adopt · r refresh · q quit",
        Style::default().fg(Color::DarkGray),
    )]);
    let mut message_style = Style::default().fg(Color::DarkGray);

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
                    if w.from_override {
                        ListItem::new(Line::from(vec![
                            Span::styled(
                                format!("{} ", style::icon::OVERRIDE),
                                Style::default().fg(Color::Magenta),
                            ),
                            Span::styled(
                                format!("{}@{}", w.name, w.version),
                                Style::default().fg(Color::Magenta),
                            ),
                        ]))
                    } else {
                        ListItem::new(format!("{}@{}", w.name, w.version))
                    }
                })
                .collect();
            let list = List::new(items)
                .block(
                    Block::bordered()
                        .title(format!("{} Skills", style::icon::SKILL))
                        .title_style(title_style()),
                )
                .highlight_symbol("▶ ")
                .highlight_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_stateful_widget(list, main[0], &mut list_state);

            let selected = list_state.selected().and_then(|i| wanted.get(i));
            let detail_lines = if let Some(w) = selected {
                let st = status.skills.iter().find(|s| s.name == w.name);
                let mut lines: Vec<Line> = vec![
                    labeled("Skill", &w.name),
                    labeled("Version", &w.version),
                ];
                if w.from_override {
                    lines.push(Line::from(vec![
                        Span::styled("Source: ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            format!("{} Override", style::icon::OVERRIDE),
                            Style::default().fg(Color::Magenta),
                        ),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::styled("Source: ", Style::default().fg(Color::DarkGray)),
                        Span::raw(format!("{} Lock File", style::icon::LOCK)),
                    ]));
                }
                if let Some(s) = st {
                    lines.push(Line::from(vec![
                        Span::styled("Description: ", Style::default().fg(Color::DarkGray)),
                        Span::raw(s.description.clone()),
                    ]));
                    if s.deprecated {
                        let mut dep = vec![Span::styled(
                            format!("{} DEPRECATED", style::icon::WARN),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )];
                        if let Some(suc) = &s.successor {
                            dep.push(Span::styled(
                                format!(" → {suc}"),
                                Style::default().fg(Color::DarkGray),
                            ));
                        }
                        lines.push(Line::from(dep));
                    }
                }
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "Sync preview:",
                    Style::default().add_modifier(Modifier::BOLD),
                )));
                for h in &status.harnesses {
                    lines.push(Line::from(Span::styled(
                        format!("  → {h}/{}", w.name),
                        Style::default().fg(Color::Green),
                    )));
                }
                lines
            } else {
                vec![Line::from(Span::styled(
                    "No Skills selected in personal.toml",
                    Style::default().fg(Color::DarkGray),
                ))]
            };
            frame.render_widget(
                Paragraph::new(detail_lines)
                    .block(
                        Block::bordered()
                            .title(format!("{} Details", style::icon::LIST))
                            .title_style(title_style()),
                    )
                    .wrap(Wrap { trim: true }),
                main[1],
            );

            let ovr_line = if status.overrides.is_empty() {
                Line::from(Span::styled(
                    format!("{} Overrides: (none)", style::icon::OVERRIDE),
                    Style::default().fg(Color::DarkGray),
                ))
            } else {
                Line::from(Span::styled(
                    format!(
                        "{} Overrides: {}",
                        style::icon::OVERRIDE,
                        status
                            .overrides
                            .iter()
                            .map(|o| format!("{}@{}", o.name, o.version))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    Style::default().fg(Color::Magenta),
                ))
            };
            frame.render_widget(
                Paragraph::new(ovr_line).block(
                    Block::bordered()
                        .title(format!("{} Status", style::icon::STATUS))
                        .title_style(title_style()),
                ),
                layout[1],
            );
            frame.render_widget(Paragraph::new(message.clone()).style(message_style), layout[2]);
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
                            message = Line::from(format!(
                                "{} synced {} placement(s), {} warning(s)",
                                style::icon::OK,
                                r.placements.len(),
                                r.warnings.len()
                            ));
                            message_style = Style::default().fg(Color::Green);
                            for p in r.placements {
                                let k = match p.kind {
                                    PlacementKind::Symlink => "symlink",
                                    PlacementKind::Copy => "copy",
                                };
                                let _ = writeln!(io::stderr(), "{} ({k})", p.path.display());
                            }
                        }
                        Err(e) => {
                            message = Line::from(format!("{} sync error: {e}", style::icon::ERR));
                            message_style = Style::default().fg(Color::Red);
                        }
                    },
                    KeyCode::Char('a') => match app.adopt() {
                        Ok(r) => {
                            message = Line::from(format!(
                                "{} adopt updated={}, skipped overrides={}",
                                style::icon::OK,
                                r.updated.len(),
                                r.skipped_overrides.len()
                            ));
                            message_style = Style::default().fg(Color::Green);
                        }
                        Err(e) => {
                            message = Line::from(format!("{} adopt error: {e}", style::icon::ERR));
                            message_style = Style::default().fg(Color::Red);
                        }
                    },
                    KeyCode::Char('r') => {
                        message = Line::from(format!("{} refreshed", style::icon::CHECK));
                        message_style = Style::default().fg(Color::Cyan);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn labeled<'a>(label: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{label}: "), Style::default().fg(Color::DarkGray)),
        Span::raw(value),
    ])
}
