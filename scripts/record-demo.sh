#!/usr/bin/env bash
# Prepare an isolated HOME/XDG fixture and record docs/demo/skills-sync.gif with VHS.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEMO_DIR="$REPO_ROOT/docs/demo"
FIXTURE="$DEMO_DIR/.fixture"
TAPE="$DEMO_DIR/skills-sync.tape"

if ! command -v vhs >/dev/null 2>&1; then
  echo "error: vhs not found; install with: brew install vhs" >&2
  exit 1
fi

echo "Building skills-sync (release)..."
(cd "$REPO_ROOT/skills-sync" && cargo build --release --bin skills-sync)

rm -rf "$FIXTURE"
mkdir -p \
  "$FIXTURE/home/work" \
  "$FIXTURE/config/skills-sync" \
  "$FIXTURE/cache/skills-sync"

# Library clone with tags so Lock File pins resolve.
echo "Cloning library into fixture cache..."
git clone --local "$REPO_ROOT" "$FIXTURE/cache/skills-sync/library"
git -C "$FIXTURE/cache/skills-sync/library" fetch --tags --force

cat >"$FIXTURE/config/skills-sync/personal.toml" <<'EOF'
skills = ["confluence", "jira", "story-review", "story-writing"]
harnesses = ["claude-code", "cursor"]
placement_scope = "user"
EOF

export HOME="$FIXTURE/home"
export XDG_CONFIG_HOME="$FIXTURE/config"
export XDG_CACHE_HOME="$FIXTURE/cache"
export PATH="$REPO_ROOT/skills-sync/target/release:$PATH"

# Smoke-check Sync works in the fixture before recording.
echo "Warming fixture with skills-sync sync..."
(cd "$HOME/work" && skills-sync sync >/dev/null)

echo "Recording $TAPE ..."
cd "$REPO_ROOT"
vhs "$TAPE"

echo "Wrote $DEMO_DIR/skills-sync.gif"
