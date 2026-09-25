//! Desktop entry point.
//!
//! The real entry point lives in the library (`#[bevy_main]`) so the same code
//! can be packaged for Android.

use bevy::app::AppExit;

fn main() {
    // Propagate a nonzero exit code when startup fails (e.g. missing game
    // data), so launchers such as Flatpak can report the failure.
    if let AppExit::Error(code) = wolf3d_bevy::main() {
        std::process::exit(code.get() as i32);
    }
}
