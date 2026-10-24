use egui::Context;
use glam::DVec2;
use macroquad::{
    camera::set_camera,
    color::{Color, RED, WHITE},
    input::{MouseButton, is_mouse_button_down, mouse_position, mouse_wheel},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
    window::{clear_background, next_frame, screen_width},
};
use wuli::{
    HandleNotFound, World,
    body::{BodyHandle, Shape},
};

use crate::view::View;

pub mod view;

#[macroquad::main("wuli")]
async fn main() {
    let scene = wuli_scenes::colliding_circles_a();
    let mut world = scene.clone();
    let mut view = View::new(0.0, 0.0, 10.0);

    let mut accumulator = 0.0;
    let dt = 1.0 / 240.0;

    let mut speed = 1.0;
    let mut paused = false;
    let mut reset = false;
    let mut reset_camera = false;

    let mut last_mouse = None;

    loop {
        if reset {
            world = scene.clone();
            reset = false;
        }
        if reset_camera {
            view.centre = DVec2::ZERO;
            reset_camera = false;
        }

        tick(
            &mut world,
            &mut accumulator,
            dt,
            if !paused { speed } else { 0.0 },
        );
        handle_controls(&mut view, &mut last_mouse);

        clear_background(Color::from_rgba(10, 10, 10, 255));
        set_camera(&view.camera());
        draw_velocities(&world);
        draw_bodies(&world);
        draw_ui(&mut speed, &mut paused, &mut reset, &mut reset_camera);

        next_frame().await
    }
}

fn tick(world: &mut World, accumulator: &mut f64, dt: f64, speed: f64) {
    *accumulator += get_frame_time() as f64 * speed;
    while *accumulator > dt {
        world.step(dt);
        *accumulator -= dt;
    }
}

fn handle_controls(view: &mut View, last_mouse: &mut Option<(f32, f32)>) {
    // Zoom
    let (_, wheel) = mouse_wheel();
    if wheel > 0.0 {
        view.width *= 0.9;
    }
    if wheel < 0.0 {
        view.width *= 1.1;
    }

    // Pan
    let mouse = mouse_position();
    if is_mouse_button_down(MouseButton::Right) {
        if let Some(last_mouse) = last_mouse {
            let m_per_px = view.width / screen_width() as f64;
            view.centre.x -= (mouse.0 - last_mouse.0) as f64 * m_per_px;
            view.centre.y += (mouse.1 - last_mouse.1) as f64 * m_per_px;
        }
        *last_mouse = Some(mouse);
    } else {
        *last_mouse = None;
    }
}

fn draw_bodies(world: &World) {
    world.bodies.iter().for_each(|(handle, _)| {
        let _ = draw_body(world, handle);
    });
}

fn draw_velocities(world: &World) {
    world.bodies.iter().for_each(|(handle, _)| {
        let _ = draw_velocity(world, handle);
    });
}

fn draw_body(world: &World, body: BodyHandle) -> Result<(), HandleNotFound> {
    let body = world.body(body)?;
    let centre = body.position;

    match body.shape {
        Shape::Circle { radius } => {
            draw_circle(centre.x as f32, centre.y as f32, radius as f32, WHITE)
        }
        _ => {}
    }

    Ok(())
}

fn draw_velocity(world: &World, body: BodyHandle) -> Result<(), HandleNotFound> {
    let body = world.body(body)?;
    let centre = body.position;
    let dest = centre + body.velocity * 0.1;
    draw_line(
        centre.x as f32,
        centre.y as f32,
        dest.x as f32,
        dest.y as f32,
        0.025,
        RED,
    );

    Ok(())
}

fn draw_ui(speed: &mut f64, paused: &mut bool, reset: &mut bool, reset_camera: &mut bool) {
    egui_macroquad::ui(|ctx| {
        draw_controls_ui(ctx, paused, reset, speed, reset_camera);
    });
    egui_macroquad::draw();
}

fn draw_controls_ui(
    ctx: &Context,
    paused: &mut bool,
    reset: &mut bool,
    speed: &mut f64,
    reset_camera: &mut bool,
) {
    if ctx.input(|i| i.key_pressed(egui::Key::Space)) && !ctx.wants_keyboard_input() {
        *paused = !*paused;
    }
    if ctx.input(|i| i.key_pressed(egui::Key::R)) && !ctx.wants_keyboard_input() {
        *reset = true;
    }

    egui::Window::new("controls")
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .default_pos([12.0, 12.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status_icon = if *paused { "▶" } else { "⏸" };
                if ui
                    .button(status_icon)
                    .on_hover_text("Pause and Play")
                    .clicked()
                {
                    *paused = !*paused;
                }
                if ui.button("↺").on_hover_text("Reset Scene").clicked() {
                    *reset = true;
                }
                if ui.button("⌖").on_hover_text("Reset Camera").clicked() {
                    *reset_camera = true;
                }

                ui.add(egui::Slider::new(speed, 0.0..=4.0).suffix("x"));
            });
        });
}
