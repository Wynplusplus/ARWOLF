# Android

ARWOLF is a normal Bevy app with `#[bevy_main]`, packaged for Android with
[`cargo-apk`](https://crates.io/crates/cargo-apk) using the **NativeActivity**
backend (`android-native-activity`, part of AOSP). No Google Play Services or
other Google libraries are involved.

## Prerequisites

```sh
rustup target add aarch64-linux-android x86_64-linux-android
cargo install cargo-apk
# Android SDK + NDK, then:
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/<version>"
```

## Build

```sh
cargo apk build --lib --release
```

`--lib` is required: the package also contains the desktop binary, and
cargo-apk must not try to build it for Android. The APK is written to
`target/release/apk/wolf3d-bevy.apk`.

Build for a single architecture (e.g. for an emulator) with
`--target x86_64-linux-android`; `[package.metadata.android] build_targets`
lists `aarch64-linux-android` and `x86_64-linux-android` for the default
multi-architecture build.

## Release signing

cargo-apk refuses to sign a `--release` build unless a keystore is configured.
It looks in `[package.metadata.android.signing.release]` in `Cargo.toml`, or in
the `CARGO_APK_RELEASE_KEYSTORE` and `CARGO_APK_RELEASE_KEYSTORE_PASSWORD`
environment variables.

For a hobby release you can reuse the standard Android debug keystore (its
password is the public string `android`):

```sh
CARGO_APK_RELEASE_KEYSTORE="$HOME/.android/debug.keystore" \
CARGO_APK_RELEASE_KEYSTORE_PASSWORD="android" \
cargo apk build --lib --release
```

For a real release, generate your own keystore and keep it out of version
control:

```sh
keytool -genkeypair -v -keystore release.keystore \
  -alias arwolf -keyalg RSA -keysize 2048 -validity 10000
```

Then point `CARGO_APK_RELEASE_KEYSTORE` at it and set the password. The signed
APK is written to `target/release/apk/wolf3d-bevy.apk`.

## Install and supply data

No game data is bundled. Install the APK and push your own legally obtained WL6
files:

```sh
adb install -r target/release/apk/wolf3d-bevy.apk
android/push-data.sh /path/to/WOLF3D
```

The helper script copies the files into
`/sdcard/Android/data/io.github.wynplusplus.wolf3dbevy/files/WOLF3D`, which the
app can read without any storage permission. The app searches there first, then
`/sdcard/WOLF3D`, `/storage/emulated/0/WOLF3D` and `/sdcard/Download/WOLF3D`.

## Controls

The on-screen gamepad is drawn over the 3D view:

| Control | Action |
| --- | --- |
| Left half (drag) | Movement stick: up/down moves, left/right turns |
| Right half (drag) | Turn |
| `MENU` | Open/close the level-select overlay |
| `1` `2` `3` `4` | Select weapon |
| `RUN` | Run while held |
| `FIRE` | Fire |
| `USE` | Open doors / use switches |

The same overlay can be tried on the desktop with `WOLF3D_TOUCH=1`.

## Notes

* `min_sdk_version` is 26 because `bevy_audio` uses AAudio (API 26+).
* `strip = "strip"` in the manifest keeps the debug APK from growing to
  gigabytes.
* The window is `WindowMode::BorderlessFullscreen` on Android.
* Data discovery lives in `src/data/mod.rs` (`find_data_dir`); the touch
  controls are in `src/touch.rs`.
