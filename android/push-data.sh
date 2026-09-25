#!/usr/bin/env sh
# Push your own Wolfenstein 3D (WL6) data to an attached Android device.
#
# Usage: android/push-data.sh /path/to/WOLF3D
#
# The data is copied into the app-specific external files directory, which the
# app can read without any storage permission.
set -eu

PKG=io.github.wynplusplus.wolf3dbevy
DATA_DIR=${1:-}

if [ -z "$DATA_DIR" ]; then
    echo "usage: $0 /path/to/WOLF3D" >&2
    exit 2
fi
if [ ! -f "$DATA_DIR/VSWAP.WL6" ]; then
    echo "error: $DATA_DIR does not contain VSWAP.WL6" >&2
    exit 1
fi

DEST="/sdcard/Android/data/$PKG/files"
adb shell mkdir -p "$DEST"
adb push "$DATA_DIR" "$DEST/"
echo "Pushed $(basename "$DATA_DIR") to $DEST/"
