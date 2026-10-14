#!/usr/bin/env bash
# Bootstrap skills-sync: install binary, clone library into XDG cache, sync Skills.
# Optional: SKILLS_SYNC_TUI=1 to open the TUI after sync.
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"

CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/skills-sync"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/skills-sync"
BIN_DIR="${SKILLS_SYNC_BIN_DIR:-$HOME/.local/bin}"

detect_repo_url() {
  if [[ -n "${SKILL_LIBRARY_REPO:-}" ]]; then
    printf '%s\n' "$SKILL_LIBRARY_REPO"
    return 0
  fi
  if [[ -d "$repo_root/.git" ]]; then
    local url
    url="$(git -C "$repo_root" remote get-url origin 2>/dev/null || true)"
    if [[ -n "$url" ]]; then
      printf '%s\n' "$url"
      return 0
    fi
    # No remote: use this checkout as the library source.
    printf '%s\n' "$repo_root"
    return 0
  fi
  echo "error: set SKILL_LIBRARY_REPO to the skill-library git URL (or run from a checkout)" >&2
  exit 1
}

github_release_base_from_url() {
  local url="$1"
  # Normalize git@github.com:owner/repo.git and https://github.com/owner/repo(.git)
  local owner_repo=""
  if [[ "$url" =~ github\.com[:/]([^/]+)/([^/.]+)(\.git)?$ ]]; then
    owner_repo="${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
  fi
  if [[ -n "$owner_repo" ]]; then
    printf 'https://github.com/%s/releases/latest/download\n' "$owner_repo"
  fi
}

REPO_URL="$(detect_repo_url)"
RELEASE_BASE="${SKILLS_SYNC_RELEASE_BASE:-}"
if [[ -z "$RELEASE_BASE" ]]; then
  RELEASE_BASE="$(github_release_base_from_url "$REPO_URL" || true)"
fi

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
    if curl -fsSL "$url" -o "$BIN_DIR/skills-sync"; then
      chmod +x "$BIN_DIR/skills-sync"
      return 0
    fi
    echo "warning: release download failed; falling back to local build if available" >&2
  fi
  if [[ -f "$repo_root/skills-sync/Cargo.toml" ]]; then
    echo "Building skills-sync from source..."
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

echo "Running skills-sync sync..."
skills-sync sync

if [[ "${SKILLS_SYNC_TUI:-0}" == "1" ]] || [[ "${1:-}" == "--tui" ]]; then
  echo "Launching skills-sync tui (q to quit)"
  exec skills-sync tui
fi

echo "Done. Next: skills-sync status | list | tui | adopt"
