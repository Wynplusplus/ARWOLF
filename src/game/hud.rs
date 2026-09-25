//! HUD state shared between the game logic and the status-bar renderer.
//!
//! SPDX-License-Identifier: MIT

#[derive(Clone)]
pub struct Hud {
    pub health: i32,
    pub ammo: i32,
    pub score: i32,
    pub lives: i32,
    /// bit 0 = gold key, bit 1 = silver key.
    pub keys: u8,
    pub weapon: usize,
    pub face_frame: usize,
    pub level: usize,
}
