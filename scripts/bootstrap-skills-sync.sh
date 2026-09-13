#!/usr/bin/env bash
# Bootstrap skills-sync: download binary (or build), clone library into XDG cache, launch TUI/init.
set -euo pipefail

REPO_URL="${SKILL_LIBRARY_REPO:-https://github.com/OWNER/skill-library.git}"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/skills-sync"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/skills-sync"
RELEASE_BASE="${SKILLS_SYNC_RELEASE_BASE:-}"
BIN_DIR="${SKILLS_SYNC_BIN_DIR:-$HOME/.local/bin}"

mkdir -p "$CACHE_DIR" "$CONFIG_DIR" "$BIN_DIR"

os="$(uname -s | tr '[:upper:]' '[:lower:]')"
arch="$(uname -m)"
case "$arch" in
  x86_64|amd64) arch="x86_64" ;;
  arm64|aarch64) arch="aarch64" ;;
esac

install_binary() {
  if command -v skills-sync >/dev/null 2>&1; then
    echo "skills-sync already on PATH: $(command -v skills-sync)"
    return 0
  fi
  if [[ -n "$RELEASE_BASE" ]]; then
    asset="skills-sync-${os}-${arch}"
    url="${RELEASE_BASE%/}/${asset}"
    echo "Downloading $url"
    curl -fsSL "$url" -o "$BIN_DIR/skills-sync"
    chmod +x "$BIN_DIR/skills-sync"
    return 0
  fi
  # Dev fallback: build from this repo if checked out
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  repo_root="$(cd "$script_dir/.." && pwd)"
  if [[ -f "$repo_root/skills-sync/Cargo.toml" ]]; then
    echo "Building skills-sync from source (no RELEASE_BASE set)..."
    (cd "$repo_root/skills-sync" && cargo build --release)
    install -m 755 "$repo_root/skills-sync/target/release/skills-sync" "$BIN_DIR/skills-sync"
    install -m 755 "$repo_root/skills-sync/target/release/skills-release" "$BIN_DIR/skills-release" || true
    return 0
  fi
  echo "error: skills-sync not found; set SKILLS_SYNC_RELEASE_BASE or run from a checkout" >&2
  exit 1
}

clone_library() {
  local dest="$CACHE_DIR/library"
  if [[ -d "$dest/.git" ]]; then
    echo "Updating library clone at $dest"
    git -C "$dest" fetch --tags --force
    git -C "$dest" pull --ff-only || true
  else
    echo "Cloning $REPO_URL -> $dest"
    git clone "$REPO_URL" "$dest"
    git -C "$dest" fetch --tags
  fi
}

init_personal_if_needed() {
  if [[ -f "$CONFIG_DIR/personal.toml" ]]; then
    return 0
  fi
  echo "Writing default personal.toml (all Skills, claude-code, user scope)"
  cat >"$CONFIG_DIR/personal.toml" <<'EOF'
skills = ["confluence", "jira", "story-review", "story-writing"]
harnesses = ["claude-code"]
placement_scope = "user"
EOF
}

install_binary
clone_library
init_personal_if_needed

export PATH="$BIN_DIR:$PATH"
echo "Launching skills-sync tui (q to quit). Run: skills-sync sync"
exec skills-sync tui
