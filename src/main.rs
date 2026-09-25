//! Desktop entry point.

use bevy::app::AppExit;

fn main() {
    // Propagate a nonzero exit code when startup fails (e.g. missing game
    // data), so launchers such as Flatpak can report the failure.
    if let AppExit::Error(code) = wolf3d_bevy::app::run() {
        std::process::exit(code.get() as i32);
    }
}
