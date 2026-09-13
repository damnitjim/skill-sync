use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use skills_sync::{Harness, PlacementScope, SkillsSync, SyncPaths};

mod tui;

#[derive(Parser, Debug)]
#[command(name = "skills-sync", about = "Sync Agent Skills into Claude Code, Codex, Cursor, and Pi")]
struct Cli {
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
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let app = SkillsSync::with_xdg()?;

    match cli.command {
        Commands::Resolve => {
            for w in app.resolve_wanted()? {
                let marker = if w.from_override { " (override)" } else { "" };
                println!("{}@{}{marker}", w.name, w.version);
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
                    "skipped Overrides: {}",
                    report.skipped_overrides.join(", ")
                );
            }
            if !report.updated.is_empty() {
                println!("adopted: {}", report.updated.join(", "));
            }
            print_sync(&report.sync);
        }
        Commands::Status => {
            let status = app.status()?;
            println!(
                "placement_scope: {}  harnesses: {}",
                status.placement_scope,
                status.harnesses.join(", ")
            );
            if !status.overrides.is_empty() {
                println!("Overrides:");
                for o in &status.overrides {
                    println!("  {}@{}", o.name, o.version);
                }
            }
            for s in &status.skills {
                let dep = if s.deprecated {
                    format!(
                        " DEPRECATED{}",
                        s.successor
                            .as_ref()
                            .map(|x| format!(" → {x}"))
                            .unwrap_or_default()
                    )
                } else {
                    String::new()
                };
                let ovr = if s.from_override { " [override]" } else { "" };
                println!(
                    "{}: effective {} (lock {}){ovr}{dep}",
                    s.name, s.effective_version, s.lock_version
                );
            }
        }
        Commands::Tui => tui::run(app)?,
        Commands::Init {
            skills,
            harnesses,
            scope,
        } => {
            init_personal(app.paths(), &skills, &harnesses, &scope)?;
            println!("wrote {}", app.paths().config_dir.join("personal.toml").display());
        }
        Commands::Override { skill, version } => {
            app.set_override(&skill, &version)?;
            println!("override {skill}@{version}");
        }
        Commands::ClearOverride { skill } => {
            app.clear_override(&skill)?;
            println!("cleared override for {skill}");
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
            println!("catalog written to {}", out.display());
        }
    }
    Ok(())
}

fn print_sync(report: &skills_sync::SyncReport) {
    for p in &report.placements {
        let kind = match p.kind {
            skills_sync::PlacementKind::Symlink => "symlink",
            skills_sync::PlacementKind::Copy => "copy",
        };
        println!("{} -> {} ({kind}, {})", p.skill, p.path.display(), p.harness);
    }
    for w in &report.warnings {
        eprintln!("warning: {w}");
    }
}

fn init_personal(
    paths: &SyncPaths,
    skills: &[String],
    harnesses: &[String],
    scope: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // validate
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
