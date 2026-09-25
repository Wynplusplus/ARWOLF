# ARWOLF

ARWOLF is the **Android continuation of
[RWOLF](https://github.com/Wynplusplus/RWOLF)**: a clean-room reimplementation
of **Wolfenstein 3D** built with the [Bevy](https://bevyengine.org) engine. It
reads the original `WL6` data files that **you** supply and plays the original
maps, textures and digitised sounds through a from-scratch software raycaster.
The engine, menu and configuration are shared with RWOLF; ARWOLF adds the
Android build, on-screen touch controls and Android data discovery.

> **No game content is included.** ARWOLF ships no Wolfenstein 3D data and no
> id Software code. You must provide your own legally obtained copy of the
> registered (WL6) files. *Wolfenstein 3D* is a trademark of its respective
> owners; this project is unofficial and is not affiliated with or endorsed by
> them.

## AI disclaimer

This project was written **entirely by an AI coding agent** (DeepSeek V4.1
Flash, running in OpenCode), driven by a series of natural-language prompts
from the repository owner. The owner specified the features and reviewed and
tested the results; the AI wrote all source code, tests and documentation.

The prompting was a sequence of short, concrete English requests — for example
*"reimplement Wolfenstein 3D in Bevy so it loads the original WL6 data"*,
followed by targeted fixes and features such as *"it crashes opening a door"*,
*"the turn direction is reversed"*, *"build a level-select UI accessible with
Esc"*, *"add a config where the user supplies their own game folder"*,
*"create a Flatpak with install instructions"*, *"continue the fork for
Android"* (touch controls, APK packaging and data discovery) and *"push it to
git as a fork of RWOLF under the name ARWOLF"*. For each prompt the AI chose an
approach, implemented it, built it and reported back.

As with any AI-generated code, treat it as unreviewed: it may contain bugs.
There is no warranty.

## Game data

ARWOLF does not include any game data. Point it at your own WL6 files by copying
`wolf3d-bevy.toml.example` to `wolf3d-bevy.toml` and editing `data_dir`:

```toml
data_dir = "/path/to/WOLF3D"
```

Alternatively set the `WOLF3D_DATA_DIR` environment variable. If neither is
set, the engine looks in `./data`, `./WOLF3D` and `~/Downloads/WOLF3D`.

Expected files:

```
AUDIOHED.WL6  AUDIOT.WL6   GAMEMAPS.WL6  MAPHEAD.WL6
VGAHEAD.WL6   VGADICT.WL6  VGAGRAPH.WL6  VSWAP.WL6
```

## Build and run

```sh
cargo run --release
```

### Flatpak

A Flatpak manifest lives in [`flatpak/`](flatpak/). With `flatpak-builder`
installed:

```sh
flatpak/build.sh                                  # build + install
flatpak/setup-config.sh /path/to/WOLF3D           # point it at your data
flatpak run io.github.wynplusplus.wolf3dbevy
```

See [`flatpak/README.md`](flatpak/README.md) for details.

### Android

ARWOLF builds as a normal Bevy app with `#[bevy_main]` and is packaged with
[`cargo-apk`](https://crates.io/crates/cargo-apk) using the **NativeActivity**
backend (`android-native-activity`, part of AOSP — no Google Play Services or
other Google libraries are involved).

Prerequisites:

```sh
rustup target add aarch64-linux-android x86_64-linux-android
cargo install cargo-apk
# Android SDK + NDK, then:
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/<version>"
```

Build the APK:

```sh
cargo apk build --lib --release
```

`--lib` is required: the package also contains the desktop binary, which must
not be built for Android. The APK is written to `target/release/apk/`.

Install it, then push your own WL6 data into the app-specific external files
directory (writable over `adb` without any storage permission):

```sh
adb install -r target/release/apk/wolf3d-bevy.apk
adb push WOLF3D /sdcard/Android/data/io.github.wynplusplus.wolf3dbevy/files/WOLF3D
```

The app searches, in order: `WOLF3D_DATA_DIR`, the `data_dir` from
`wolf3d-bevy.toml`, the app-specific directory above, `/sdcard/WOLF3D`,
`/storage/emulated/0/WOLF3D` and `/sdcard/Download/WOLF3D`.

`min_sdk_version` is 26 because `bevy_audio` uses AAudio, and `strip = "strip"`
keeps the debug APK from ballooning to gigabytes.

## Controls

### Desktop

| Action | Key |
| --- | --- |
| Move / strafe | `W` `A` `S` `D` |
| Turn | mouse, or `←` `→` / `Q` `E` |
| Run | `Shift` |
| Fire | left mouse, or `Ctrl` |
| Use / open door | `Space` |
| Weapons | `1` `2` `3` `4` |
| Level-select menu | `Esc` |

### Touch (Android)

| Control | Action |
| --- | --- |
| Left half (drag) | Movement stick: up/down moves, left/right turns |
| Right half (drag) | Turn |
| `MENU` | Open/close the level-select overlay |
| `1` `2` `3` `4` | Select weapon |
| `RUN` | Run while held |
| `FIRE` | Fire |
| `USE` | Open doors / use switches |

The touch overlay can be exercised on the desktop by setting `WOLF3D_TOUCH=1`.

## License

MIT. See [`LICENSE`](LICENSE). No game assets are included or licensed here.
