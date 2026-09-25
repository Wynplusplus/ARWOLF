//! A clean-room reimplementation of Wolfenstein 3D built on the Bevy engine.
//!
//! The crate is split into:
//! * [`config`] — user configuration, including where the game data lives,
//! * [`data`] — decoders for the original WL6 data files,
//! * [`render`] — a software raycaster that reproduces the original's look,
//! * [`game`] — the game rules, actors and player logic,
//! * [`touch`] — on-screen controls for touch devices (Android),
//! * [`app`] — the Bevy app that ties everything together.
//!
//! SPDX-License-Identifier: MIT

use bevy::app::AppExit;
use bevy::prelude::*;

pub mod android;
pub mod app;
pub mod config;
pub mod data;
pub mod game;
pub mod render;
pub mod touch;

/// Application entry point.
///
/// On Android `#[bevy_main]` also generates the `android_main` symbol required
/// by `NativeActivity`. The desktop binary in `src/main.rs` calls this and
/// propagates the exit code.
#[bevy_main]
pub fn main() -> AppExit {
    app::run()
}
