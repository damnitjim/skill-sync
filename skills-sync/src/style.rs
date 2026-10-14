//! CLI/TUI presentation helpers (icons + optional ANSI color).

use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::style::Stylize;

static COLOR_ENABLED: AtomicBool = AtomicBool::new(true);

pub mod icon {
    pub const OK: &str = "✅";
    pub const WARN: &str = "⚠️";
    pub const ERR: &str = "❌";
    pub const LINK: &str = "🔗";
    pub const COPY: &str = "📄";
    pub const OVERRIDE: &str = "⚡";
    pub const LOCK: &str = "🔒";
    pub const MIGRATE: &str = "📖";
    pub const SKILL: &str = "📦";
    pub const LIST: &str = "📋";
    pub const STATUS: &str = "📊";
    pub const CHECK: &str = "✓";
}

/// Pure decision helper (testable without mutating process env).
pub fn color_enabled_from(no_color_flag: bool, no_color_env_set: bool, is_tty: bool) -> bool {
    !no_color_flag && !no_color_env_set && is_tty
}

pub fn init(no_color_flag: bool) {
    let no_color_env = std::env::var_os("NO_COLOR").is_some();
    let enabled = color_enabled_from(no_color_flag, no_color_env, io::stdout().is_terminal());
    COLOR_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn color_enabled() -> bool {
    COLOR_ENABLED.load(Ordering::Relaxed)
}

pub fn success(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.green().to_string())
}

pub fn warn(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.yellow().to_string())
}

pub fn error(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.red().to_string())
}

pub fn accent(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.cyan().bold().to_string())
}

pub fn dim(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.dark_grey().to_string())
}

pub fn magenta(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.magenta().to_string())
}

pub fn bold(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.bold().to_string())
}

pub fn blue(s: impl AsRef<str>) -> String {
    paint(s.as_ref(), |t| t.blue().to_string())
}

fn paint(s: &str, f: impl FnOnce(&str) -> String) -> String {
    if color_enabled() {
        f(s)
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::color_enabled_from;

    #[test]
    fn color_on_when_tty_and_no_suppressors() {
        assert!(color_enabled_from(false, false, true));
    }

    #[test]
    fn color_off_for_flag_env_or_non_tty() {
        assert!(!color_enabled_from(true, false, true));
        assert!(!color_enabled_from(false, true, true));
        assert!(!color_enabled_from(false, false, false));
    }
}
