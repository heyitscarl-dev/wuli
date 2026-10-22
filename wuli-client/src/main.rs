use glam::DVec2;
use macroquad::{
    camera::{Camera2D, set_camera}, color::{BLACK, ORANGE}, input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_down, mouse_position, mouse_wheel}, shapes::{draw_circle, draw_rectangle, draw_triangle}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width},
};
use wuli::body::Shape;

const DT: f64 = 1.0 / 240.0; // run physics at 240 Hz

type DisplayVec2 = macroquad::math::Vec2;

struct View {
    centre: DVec2,
    width: f64,
}

impl View {
    fn camera(&self) -> Camera2D {
        let w = self.width as f32;
        let h = w * screen_height() / screen_width();
        Camera2D { 
            target: DisplayVec2::ZERO,
            zoom: macroquad::math::vec2(2.0 / w, -2.0 / h),
            ..Default::default()
        }
    }

    fn to_camera(&self, p: DVec2) -> DisplayVec2 {
        let r = p - self.centre;
        macroquad::math::vec2(r.x as f32, r.y as f32)
    }
}

#[macroquad::main("wuli")]
async fn main() {
    let mut view = View { centre: DVec2 { x: 2.5, y: -2.5 }, width: 10.0 };
    let mut last_mouse: Option<(f32, f32)> = None;

    let initial = wuli_scenes::projectile_2();
    let mut world = initial.clone();
    let mut paused = false;

    let mut accumulator: f64 = 0.0;

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::R) {
            world = initial.clone();
            accumulator = 0.0;
        }

        if paused {
            if is_key_down(KeyCode::Period) {
                world.step(DT);
            }
        } else {
            accumulator += get_frame_time() as f64;
            while accumulator >= DT {
                world.step(DT);
                accumulator -= DT;
            }
        }

        let (_, wheel) = mouse_wheel();
        if wheel > 0.0 { view.width *= 0.9; }
        if wheel < 0.0 { view.width *= 1.1; }

        let mouse = mouse_position();
        if is_mouse_button_down(MouseButton::Right) {
            if let Some(last_mouse) = last_mouse {
                let m_per_px = view.width / screen_width() as f64;
                view.centre.x -= (mouse.0 - last_mouse.0) as f64 * m_per_px;
                view.centre.y += (mouse.1 - last_mouse.1) as f64 * m_per_px;
            }
            last_mouse = Some(mouse);
        } else {
            last_mouse = None;
        }

        clear_background(BLACK);
        set_camera(&view.camera());

        for body in &world.bodies {
            let c = view.to_camera(body.position);
            match body.shape {
                Shape::Circle { radius } => draw_circle(c.x, c.y, radius as f32, ORANGE),
                Shape::Rectangle { half_extents: h } => {
                    let [a, b, c, d] 
                        = Shape::rect_corners(body.position, body.angle, h);
                    draw_triangle(display_vec2(a), display_vec2(b), display_vec2(c), ORANGE);
                    draw_triangle(display_vec2(a), display_vec2(c), display_vec2(d), ORANGE);
                }
            }
        }

        next_frame().await
    }
}

fn display_vec2(v: DVec2) -> DisplayVec2 {
    DisplayVec2::new(v.x as f32, v.y as f32)
}