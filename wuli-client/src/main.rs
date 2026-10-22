use glam::DVec2;
use macroquad::{
    color::{BLACK, DARKGRAY, ORANGE},
    input::{KeyCode, is_key_pressed},
    shapes::draw_circle,
    time::get_frame_time,
    window::{clear_background, next_frame},
};

use std::collections::VecDeque;

const TRAIL_LEN: usize = 300;
const DT: f64 = 1.0 / 240.0; // run physics at 240 Hz

#[macroquad::main("wuli")]
async fn main() {
    let initial = wuli_scenes::projectile(5.0);
    let mut world = initial.clone();
    let mut paused = false;

    let mut accumulator: f64 = 0.0;
    let mut trails: Vec<VecDeque<DVec2>> = Vec::new();

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::R) {
            world = initial.clone();
            accumulator = 0.0;
            trails.clear();
        }

        if paused {
            if is_key_pressed(KeyCode::Period) {
                world.step(DT);
            }
        } else {
            accumulator += get_frame_time() as f64;
            while accumulator >= DT {
                world.step(DT);
                accumulator -= DT;
            }

            trails.resize_with(world.bodies().len(), VecDeque::new);
            for (trail, body) in trails.iter_mut().zip(world.bodies()) {
                trail.push_back(body.position);
                if trail.len() > TRAIL_LEN {
                    trail.pop_front();
                }
            }
        }

        clear_background(BLACK);

        for trail in &trails {
            for p in trail {
                let x = 100.0 + p.x as f32 * 50.0;
                let y = 100.0 - p.y as f32 * 50.0;
                draw_circle(x, y, 1.5, DARKGRAY);
            }
        }

        for body in &world.bodies {
            let x = 100.0 + body.position.x as f32 * 50.0;
            let y = 100.0 - body.position.y as f32 * 50.0;
            draw_circle(x, y, 5.0, ORANGE);
        }

        next_frame().await
    }
}
