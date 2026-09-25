#!/usr/bin/env bash
# Point the installed wolf3d-bevy Flatpak at a Wolfenstein 3D (WL6) folder and
# grant the sandbox read-only access to it.
#
# Usage:
#   flatpak/setup-config.sh /path/to/WOLF3D
#   flatpak/setup-config.sh            # prompts for the path
#
# Re-run it any time to change the folder.
set -euo pipefail

APP_ID="io.github.wynplusplus.wolf3dbevy"

DATA_DIR="${1:-}"
if [ -z "$DATA_DIR" ]; then
  read -rp "Path to your WOLF3D folder (containing VSWAP.WL6): " DATA_DIR
fi

if [ ! -f "$DATA_DIR/VSWAP.WL6" ]; then
  echo "error: $DATA_DIR does not contain VSWAP.WL6" >&2
  exit 1
fi

# Make sure the app is installed before adding an override.
if ! flatpak info "$APP_ID" >/dev/null 2>&1; then
  echo "error: $APP_ID is not installed. Build it first:" >&2
  echo "  flatpak/build.sh" >&2
  exit 1
fi

# Resolve to an absolute, canonical path for the sandbox override.
DATA_DIR="$(cd "$DATA_DIR" && pwd)"

# The sandboxed config lives under ~/.var/app/<app-id>/config, which is what
# the game's XDG config lookup reads.
CONFIG_FILE="$HOME/.var/app/$APP_ID/config/wolf3d-bevy/config.toml"
mkdir -p "$(dirname "$CONFIG_FILE")"
cat > "$CONFIG_FILE" <<EOF
# Written by flatpak/setup-config.sh
data_dir = "$DATA_DIR"
EOF
echo "wrote $CONFIG_FILE"

echo "granting read-only access to $DATA_DIR ..."
flatpak override --user --filesystem="$DATA_DIR:ro" "$APP_ID"

echo
echo "Done. Start the game with:"
echo "  flatpak run $APP_ID"
