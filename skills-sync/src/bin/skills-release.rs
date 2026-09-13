use std::process::ExitCode;

use clap::Parser;
use skills_sync::release::{detect_version_bumps, proposed_tags, validate_tag_frontmatter};

#[derive(Parser, Debug)]
#[command(name = "skills-release", about = "Propose/create per-Skill git tags on version bumps")]
struct Cli {
    /// Repository root (defaults to cwd)
    #[arg(long, default_value = ".")]
    repo: std::path::PathBuf,
    /// Parent revision (e.g. before merge)
    #[arg(long)]
    parent: String,
    /// Head revision (e.g. merge commit)
    #[arg(long, default_value = "HEAD")]
    head: String,
    /// Create tags with `git tag` instead of only printing them
    #[arg(long)]
    create: bool,
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
    let bumps = detect_version_bumps(&cli.repo, &cli.parent, &cli.head)?;
    let tags = proposed_tags(&bumps);
    if tags.is_empty() {
        println!("no version bumps detected");
        return Ok(());
    }
    for (name, ver) in &bumps {
        validate_tag_frontmatter(&cli.repo, name, ver)?;
        let tag = format!("{name}@{ver}");
        println!("{tag}");
        if cli.create {
            let status = std::process::Command::new("git")
                .args(["tag", &tag])
                .current_dir(&cli.repo)
                .status()?;
            if !status.success() {
                return Err(format!("failed to create tag {tag}").into());
            }
        }
    }
    Ok(())
}
