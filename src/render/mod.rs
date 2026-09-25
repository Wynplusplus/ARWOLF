//! The software renderer.
//!
//! SPDX-License-Identifier: MIT

pub mod framebuffer;
pub mod hud;
pub mod raycast;

#[cfg(test)]
mod tests {
    use super::framebuffer::{Framebuffer, VIEW_H, VIEW_W};
    use super::hud::draw_status_bar;
    use super::raycast::{
        Camera, collect_sprites, render_sprites, render_walls, render_weapon,
    };
    use crate::data::palette;
    use crate::data::GameData;
    use crate::game::actor::Difficulty;
    use crate::game::world::World;

    /// Render the first frame of E1M1 and dump it as a PPM for visual
    /// inspection. The test is a no-op when no data directory is available.
    #[test]
    fn render_e1m1_frame() {
        let Some(dir) = crate::data::find_data_dir() else {
            return;
        };
        let data = GameData::load(&dir).unwrap();
        let world = World::new(&data, 0, 0, Difficulty::Normal).unwrap();
        let mut fb = Framebuffer::new(VIEW_W, VIEW_H);
        let cam = Camera {
            x: world.player.x,
            y: world.player.y,
            angle: world.player.angle,
        };
        let mut zbuf = [f32::INFINITY; VIEW_W];
        render_walls(&mut fb, &data.vswap, &world.level, cam, &mut zbuf);
        let sprites = collect_sprites(&world.level, &world.actors, world.player.angle);
        render_sprites(&mut fb, &data.vswap, cam, &zbuf, &sprites);
        render_weapon(&mut fb, &data.vswap, world.player.weapon_sprite());
        draw_status_bar(&mut fb, &data.vga, &world.hud);

        let mut ppm = format!("P6\n{} {}\n255\n", VIEW_W, VIEW_H).into_bytes();
        for &idx in &fb.pixels {
            ppm.extend_from_slice(&palette::to_rgb(idx));
        }
        let out = std::env::temp_dir().join("wolf3d_frame.ppm");
        std::fs::write(&out, ppm).unwrap();
        eprintln!("wrote {}", out.display());
    }

    /// Render the starting area from four headings for visual inspection.
    #[test]
    fn render_e1m1_headings() {
        let Some(dir) = crate::data::find_data_dir() else {
            return;
        };
        let data = GameData::load(&dir).unwrap();
        let base = World::new(&data, 0, 0, Difficulty::Normal).unwrap();
        for (n, angle) in [0.0f32, 1.5708, 3.1416, 4.7124].into_iter().enumerate() {
            let mut fb = Framebuffer::new(VIEW_W, VIEW_H);
            let cam = Camera {
                x: base.player.x,
                y: base.player.y,
                angle,
            };
            let mut zbuf = [f32::INFINITY; VIEW_W];
            render_walls(&mut fb, &data.vswap, &base.level, cam, &mut zbuf);
            let sprites = collect_sprites(&base.level, &base.actors, angle);
            render_sprites(&mut fb, &data.vswap, cam, &zbuf, &sprites);
            draw_status_bar(&mut fb, &data.vga, &base.hud);
            let mut ppm = format!("P6\n{} {}\n255\n", VIEW_W, VIEW_H).into_bytes();
            for &idx in &fb.pixels {
                ppm.extend_from_slice(&palette::to_rgb(idx));
            }
            let out = std::env::temp_dir().join(format!("wolf3d_heading{n}.ppm"));
            std::fs::write(&out, ppm).unwrap();
        }
        eprintln!("wrote headings");
    }

    /// Look at the first enemy in E1M1 to verify sprite rendering.
    #[test]
    fn render_e1m1_enemy() {
        let Some(dir) = crate::data::find_data_dir() else {
            return;
        };
        let data = GameData::load(&dir).unwrap();
        let base = World::new(&data, 0, 0, Difficulty::Normal).unwrap();
        eprintln!(
            "E1M1 statics={} actors={}",
            base.level.statics.len(),
            base.actors.len()
        );
        let Some(a) = base.actors.first() else {
            return;
        };
        // Stand two tiles to the west of the enemy, facing east.
        let mut fb = Framebuffer::new(VIEW_W, VIEW_H);
        let cam = Camera {
            x: a.x - 2.0,
            y: a.y,
            angle: 0.0,
        };
        let mut zbuf = [f32::INFINITY; VIEW_W];
        render_walls(&mut fb, &data.vswap, &base.level, cam, &mut zbuf);
        let sprites = collect_sprites(&base.level, &base.actors, cam.angle);
        render_sprites(&mut fb, &data.vswap, cam, &zbuf, &sprites);
        draw_status_bar(&mut fb, &data.vga, &base.hud);
        let mut ppm = format!("P6\n{} {}\n255\n", VIEW_W, VIEW_H).into_bytes();
        for &idx in &fb.pixels {
            ppm.extend_from_slice(&palette::to_rgb(idx));
        }
        std::fs::write(std::env::temp_dir().join("wolf3d_enemy.ppm"), ppm).unwrap();
    }

    /// Render the `Escape` level-select overlay for visual inspection.
    #[test]
    fn render_level_select_menu() {
        let Some(dir) = crate::data::find_data_dir() else {
            return;
        };
        let data = GameData::load(&dir).unwrap();
        let mut fb = Framebuffer::new(VIEW_W, VIEW_H);
        crate::render::hud::draw_level_select(&mut fb, &data.vga, 2, 4);
        let mut ppm = format!("P6\n{} {}\n255\n", VIEW_W, VIEW_H).into_bytes();
        for &idx in &fb.pixels {
            ppm.extend_from_slice(&palette::to_rgb(idx));
        }
        std::fs::write(std::env::temp_dir().join("wolf3d_level_select.ppm"), ppm).unwrap();
    }

    #[test]
    fn render_text_overlay() {
        let Some(dir) = crate::data::find_data_dir() else {
            return;
        };
        let data = GameData::load(&dir).unwrap();
        let mut fb = Framebuffer::new(VIEW_W, VIEW_H);
        fb.clear(0x00);
        crate::render::hud::draw_center_text(&mut fb, &data.vga, "LEVEL COMPLETE", 70, 4);
        crate::render::hud::draw_center_text(&mut fb, &data.vga, "GET PSYCHED!", 90, 2);
        let mut ppm = format!("P6\n{} {}\n255\n", VIEW_W, VIEW_H).into_bytes();
        for &idx in &fb.pixels {
            ppm.extend_from_slice(&palette::to_rgb(idx));
        }
        std::fs::write(std::env::temp_dir().join("wolf3d_text.ppm"), ppm).unwrap();
    }
}
