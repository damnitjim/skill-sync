use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use skills_sync::{Harness, PlacementScope, SkillsSync, SyncPaths};

mod style;
mod tui;

#[derive(Parser, Debug)]
#[command(name = "skills-sync", about = "Sync Agent Skills into Claude Code, Codex, Cursor, and Pi")]
struct Cli {
    /// Disable ANSI colors (also honors NO_COLOR)
    #[arg(long, global = true, env = "SKILLS_SYNC_NO_COLOR")]
    no_color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Resolve and show wanted Skill Versions (Lock File + Overrides)
    Resolve,
    /// Place selected Skills into Harness directories
    Sync,
    /// Adopt Lock File Versions (skips Overrides) and Sync
    Adopt,
    /// Show status including Overrides and Deprecation
    Status,
    /// List installed Skills and Harness paths
    List,
    /// Unlink a Skill from Harnesses, bins, and personal config
    Remove {
        skill: String,
    },
    /// Validate Lock File pins against git tags (or HEAD frontmatter)
    ValidateLock {
        /// Library root containing skills.lock (defaults to XDG cache clone)
        #[arg(long)]
        library: Option<PathBuf>,
    },
    /// Validate Skill frontmatter, Migration Docs, deps, and lock completeness
    ValidateSkills {
        /// Library root containing skills/ and skills.lock (defaults to XDG cache clone)
        #[arg(long)]
        library: Option<PathBuf>,
    },
    /// Interactive TUI
    Tui,
    /// First-run personal setup
    Init {
        #[arg(long, value_delimiter = ',')]
        skills: Vec<String>,
        #[arg(long, value_delimiter = ',', default_value = "claude-code")]
        harnesses: Vec<String>,
        #[arg(long, default_value = "user")]
        scope: String,
    },
    /// Set a personal Version Override
    Override {
        skill: String,
        version: String,
    },
    /// Clear a personal Version Override
    ClearOverride {
        skill: String,
    },
    /// Generate static Catalog markdown
    Catalog {
        #[arg(long)]
        out: PathBuf,
        /// Library root containing skills/ (defaults to XDG cache clone)
        #[arg(long)]
        library: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            for line in e.to_string().lines() {
                eprintln!(
                    "{} {}",
                    style::error(style::icon::ERR),
                    style::error(format!("error: {line}"))
                );
            }
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    style::init(cli.no_color);
    let app = SkillsSync::with_xdg()?;

    match cli.command {
        Commands::Resolve => {
            for w in app.resolve_wanted()? {
                if w.from_override {
                    println!(
                        "{}@{} {}",
                        w.name,
                        w.version,
                        style::magenta(format!("{} override", style::icon::OVERRIDE))
                    );
                } else {
                    println!("{}@{}", w.name, w.version);
                }
            }
        }
        Commands::Sync => {
            let report = app.sync()?;
            print_sync(&report);
        }
        Commands::Adopt => {
            let report = app.adopt()?;
            if !report.skipped_overrides.is_empty() {
                println!(
                    "{} {}",
                    style::warn(style::icon::OVERRIDE),
                    style::warn(format!(
                        "skipped Overrides: {}",
                        report.skipped_overrides.join(", ")
                    ))
                );
            }
            if !report.updated.is_empty() {
                println!(
                    "{} {}",
                    style::success(style::icon::OK),
                    style::success(format!("adopted: {}", report.updated.join(", ")))
                );
            }
            for m in &report.migrations {
                println!(
                    "{} {}",
                    style::blue(style::icon::MIGRATE),
                    style::blue(format!(
                        "migration {} {} → {}: {}",
                        m.skill, m.from_version, m.to_version, m.summary
                    ))
                );
            }
            print_sync(&report.sync);
        }
        Commands::List => {
            for skill in app.list()? {
                println!(
                    "{} {}",
                    style::icon::SKILL,
                    style::bold(format!("{}@{}", skill.name, skill.version))
                );
                for p in &skill.placements {
                    println!(
                        "  {} {} {}",
                        style::accent(&p.harness),
                        style::dim("->"),
                        style::dim(p.path.display().to_string())
                    );
                }
            }
        }
        Commands::Remove { skill } => {
            app.remove(&skill)?;
            println!(
                "{} {}",
                style::success(style::icon::CHECK),
                style::success(format!("removed {skill}"))
            );
        }
        Commands::ValidateLock { library } => {
            let lib = library.unwrap_or_else(|| app.paths().library_clone.clone());
            skills_sync::release::validate_lock_tags(&lib)?;
            println!(
                "{} {}",
                style::success(style::icon::LOCK),
                style::success(format!("Lock File pins OK ({})", lib.display()))
            );
        }
        Commands::ValidateSkills { library } => {
            let lib = library.unwrap_or_else(|| app.paths().library_clone.clone());
            skills_sync::validate_skills(&lib)?;
            println!(
                "{} {}",
                style::success(style::icon::CHECK),
                style::success(format!("Skills OK ({})", lib.display()))
            );
        }
        Commands::Status => {
            print_status(&app.status()?);
        }
        Commands::Tui => tui::run(app)?,
        Commands::Init {
            skills,
            harnesses,
            scope,
        } => {
            init_personal(app.paths(), &skills, &harnesses, &scope)?;
            println!(
                "{} {}",
                style::success(style::icon::CHECK),
                style::success(format!(
                    "wrote {}",
                    app.paths().config_dir.join("personal.toml").display()
                ))
            );
        }
        Commands::Override { skill, version } => {
            app.set_override(&skill, &version)?;
            println!(
                "{} {}",
                style::magenta(style::icon::OVERRIDE),
                style::magenta(format!("override {skill}@{version}"))
            );
        }
        Commands::ClearOverride { skill } => {
            app.clear_override(&skill)?;
            println!(
                "{} {}",
                style::success(style::icon::CHECK),
                style::success(format!("cleared override for {skill}"))
            );
        }
        Commands::Catalog { out, library } => {
            if let Some(lib) = library {
                let mut paths = app.paths().clone();
                paths.library_clone = lib;
                let app = SkillsSync::new(paths);
                app.generate_catalog(&out)?;
            } else {
                app.generate_catalog(&out)?;
            }
            println!(
                "{} {}",
                style::success(style::icon::CHECK),
                style::success(format!("catalog written to {}", out.display()))
            );
        }
    }
    Ok(())
}

fn print_status(status: &skills_sync::StatusReport) {
    println!(
        "{} {}  {} {}",
        style::accent("placement_scope:"),
        status.placement_scope,
        style::accent("harnesses:"),
        status.harnesses.join(", ")
    );
    if !status.overrides.is_empty() {
        println!("{}", style::magenta(format!("{} Overrides:", style::icon::OVERRIDE)));
        for o in &status.overrides {
            println!(
                "  {}",
                style::magenta(format!("{}@{}", o.name, o.version))
            );
        }
    }
    for s in &status.skills {
        let mut line = format!(
            "{}: effective {} (lock {})",
            style::bold(&s.name),
            s.effective_version,
            s.lock_version
        );
        if s.from_override {
            line.push(' ');
            line.push_str(&style::magenta(format!("{} override", style::icon::OVERRIDE)));
        }
        if s.deprecated {
            line.push(' ');
            line.push_str(&style::warn(format!(
                "{} DEPRECATED{}",
                style::icon::WARN,
                s.successor
                    .as_ref()
                    .map(|x| style::dim(format!(" → {x}")))
                    .unwrap_or_default()
            )));
        }
        println!("{line}");
    }
}

fn print_sync(report: &skills_sync::SyncReport) {
    for p in &report.placements {
        let (icon, kind) = match p.kind {
            skills_sync::PlacementKind::Symlink => (style::icon::LINK, "symlink"),
            skills_sync::PlacementKind::Copy => (style::icon::COPY, "copy"),
        };
        let kind_s = match p.kind {
            skills_sync::PlacementKind::Symlink => style::success(kind),
            skills_sync::PlacementKind::Copy => style::warn(kind),
        };
        println!(
            "{} {} {} {} ({}, {})",
            icon,
            style::bold(&p.skill),
            style::dim("->"),
            style::dim(p.path.display().to_string()),
            kind_s,
            style::accent(&p.harness)
        );
    }
    for w in &report.warnings {
        eprintln!(
            "{} {}",
            style::warn(style::icon::WARN),
            style::warn(format!("warning: {w}"))
        );
    }
}

fn init_personal(
    paths: &SyncPaths,
    skills: &[String],
    harnesses: &[String],
    scope: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let _scope = match scope {
        "user" => PlacementScope::User,
        "project" => PlacementScope::Project,
        other => return Err(format!("invalid scope: {other}").into()),
    };
    for h in harnesses {
        Harness::parse(h)?;
    }
    std::fs::create_dir_all(&paths.config_dir)?;
    let skills_list = skills
        .iter()
        .map(|s| format!("\"{s}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let harness_list = harnesses
        .iter()
        .map(|h| format!("\"{h}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let body = format!(
        "skills = [{skills_list}]\nharnesses = [{harness_list}]\nplacement_scope = \"{scope}\"\n"
    );
    std::fs::write(paths.config_dir.join("personal.toml"), body)?;
    Ok(())
}
