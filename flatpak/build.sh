#!/usr/bin/env bash
# Build and install wolf3d-bevy as a per-user Flatpak.
#
# Usage: flatpak/build.sh
#
# Requires flatpak-builder and the Flathub remote (see flatpak/README.md).
set -euo pipefail

APP_ID="io.github.wynplusplus.wolf3dbevy"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$ROOT/build-dir"

if ! command -v flatpak-builder >/dev/null; then
  echo "error: flatpak-builder is not installed (see flatpak/README.md)" >&2
  exit 1
fi

flatpak-builder \
  --user \
  --force-clean \
  --install \
  --install-deps-from=flathub \
  "$BUILD_DIR" \
  "$ROOT/$APP_ID.yaml"

echo
echo "Installed. Run it with:"
echo "  flatpak run $APP_ID"
echo
echo "Configure your game data folder with:"
echo "  flatpak/setup-config.sh /path/to/WOLF3D"
