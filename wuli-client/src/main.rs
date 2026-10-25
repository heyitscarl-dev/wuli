use egui::{Align, Color32, Context, CornerRadius, Layout, Shadow, Stroke, Visuals};
use glam::DVec2;
use macroquad::{
    camera::set_camera, color::Color, input::{MouseButton, is_mouse_button_down, mouse_position, mouse_wheel}, math::{Vec2, vec2}, shapes::{DrawRectangleParams, draw_line, draw_poly, draw_poly_lines, draw_rectangle_ex, draw_rectangle_lines_ex, draw_triangle}, time::get_frame_time, window::{clear_background, next_frame, screen_width},
};
use wuli::{
    HandleNotFound, World,
    body::{BodyHandle, Shape},
};

use crate::view::View;

pub mod view;

// TikZ look: black ink and gray!20 fills on white paper, strokes a fixed number of pixels wide
const PAPER: Color = Color::new(1.0, 1.0, 1.0, 1.0);
// gray!20: 20% of TikZ gray (50% black) mixed with white
const FILL: Color = Color::new(0.9, 0.9, 0.9, 1.0);
const INK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
const LINE_PX: f32 = 2.5;

#[macroquad::main("wuli")]
async fn main() {
    let options = wuli_scenes::available();
    let mut current = 0;

    let mut world = (options[current].1)();
    let mut view = View::new(0.0, 0.0, 10.0);

    let mut accumulator = 0.0;
    let dt = 1.0 / 240.0;

    let mut speed = 1.0;
    let mut paused = false;
    let mut reset = false;
    let mut reset_camera = false;
    let mut show_debug = false;

    let mut velocities = false;

    let mut last_mouse = None;

    loop {
        if reset {
            world = (options[current].1)();
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

        clear_background(PAPER);
        set_camera(&view.camera());
        // World units per screen pixel, so strokes keep their width when zooming
        let px = (view.width / screen_width() as f64) as f32;
        draw_bodies(&world, px);
        if velocities {
            draw_velocities(&world, px);
        }

        egui_macroquad::ui(|ctx| {
            ctx.set_visuals(tikz_visuals());
            draw_controls_ui(
                ctx,
                &options,
                &mut current,
                &mut paused,
                &mut reset,
                &mut speed,
                &mut reset_camera,
                &mut show_debug,
            );
            if show_debug {
                draw_debug_ui(ctx, &mut velocities);
            }
        });
        egui_macroquad::draw();

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

fn draw_bodies(world: &World, px: f32) {
    world.bodies.iter().for_each(|(handle, _)| {
        let _ = draw_body(world, handle, px);
    });
}

fn draw_velocities(world: &World, px: f32) {
    world.bodies.iter().for_each(|(handle, _)| {
        let _ = draw_velocity(world, handle, px);
    });
}

fn draw_body(world: &World, body: BodyHandle, px: f32) -> Result<(), HandleNotFound> {
    let body = world.body(body)?;
    let (x, y) = (body.position.x as f32, body.position.y as f32);

    match body.shape {
        Shape::Circle { radius } => {
            // \draw[fill=gray!20] (x, y) circle (r);
            draw_poly(x, y, 64, radius as f32, 0.0, FILL);
            draw_poly_lines(x, y, 64, radius as f32, 0.0, LINE_PX * px, INK);
        },
        Shape::Rectangle { half_extents } => {
            let extents = half_extents * 2.0;
            draw_rectangle_ex(x, y, extents.x as f32, extents.y as f32, DrawRectangleParams { 
                rotation: body.angle as f32, color: FILL, offset: vec2(0.5, 0.5)
            });
            draw_rectangle_lines_ex(x, y, extents.x as f32, extents.y as f32, LINE_PX * px, DrawRectangleParams { 
                rotation: body.angle as f32, color: INK, offset: vec2(0.5, 0.5)
            });
        }
    }

    Ok(())
}

fn draw_velocity(world: &World, body: BodyHandle, px: f32) -> Result<(), HandleNotFound> {
    let body = world.body(body)?;
    let tip = body.position + body.velocity * 0.1;
    draw_arrow(
        vec2(body.position.x as f32, body.position.y as f32),
        vec2(tip.x as f32, tip.y as f32),
        px,
    );

    Ok(())
}

/// A `->`-style arrow with a TikZ "stealth" tip
fn draw_arrow(start: Vec2, tip: Vec2, px: f32) {
    let (length, width) = (8.0 * px, 3.5 * px);
    let Some(dir) = (tip - start).try_normalize() else {
        return;
    };
    if start.distance(tip) < length {
        return;
    }

    let normal = dir.perp() * width;
    let base = tip - dir * length;
    let notch = tip - dir * length * 0.7;

    draw_line(start.x, start.y, notch.x, notch.y, LINE_PX * px, INK);
    draw_triangle(tip, base + normal, notch, INK);
    draw_triangle(tip, notch, base - normal, INK);
}

/// Light egui theme with square corners, thin black outlines and no shadows
fn tikz_visuals() -> Visuals {
    let mut visuals = Visuals::light();
    let ink = Stroke::new(1.0_f32, Color32::BLACK);

    visuals.window_fill = Color32::WHITE;
    visuals.panel_fill = Color32::WHITE;
    visuals.window_stroke = ink;
    visuals.window_corner_radius = CornerRadius::ZERO;
    visuals.menu_corner_radius = CornerRadius::ZERO;
    visuals.window_shadow = Shadow::NONE;
    visuals.popup_shadow = Shadow::NONE;

    let w = &mut visuals.widgets;
    for state in [
        &mut w.noninteractive,
        &mut w.inactive,
        &mut w.hovered,
        &mut w.active,
        &mut w.open,
    ] {
        state.corner_radius = CornerRadius::ZERO;
    }
    w.inactive.bg_stroke = ink;
    w.hovered.bg_stroke = ink;

    visuals
}

fn draw_controls_ui(
    ctx: &Context,
    options: &[(String, fn() -> World)],
    current: &mut usize,
    paused: &mut bool,
    reset: &mut bool,
    speed: &mut f64,
    reset_camera: &mut bool,
    show_debug: &mut bool,
) {
    if ctx.input(|i| i.key_pressed(egui::Key::Space)) && !ctx.wants_keyboard_input() {
        *paused = !*paused;
    }
    if ctx.input(|i| i.key_pressed(egui::Key::R)) && !ctx.wants_keyboard_input() {
        *reset = true;
    }
    if ctx.input(|i| i.key_pressed(egui::Key::D)) && !ctx.wants_keyboard_input() {
        *show_debug = !*show_debug;
    }

    egui::TopBottomPanel::top("controls").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let before = *current;
            egui::ComboBox::from_id_salt("scene")
                .selected_text(options[*current].0.as_str())
                .show_ui(ui, |ui| {
                    for (i, (name, _)) in options.iter().enumerate() {
                        ui.selectable_value(current, i, name.as_str());
                    }
                });
            if *current != before {
                *reset = true;
            }

            let status_text = if *paused { "▶" } else { "⏸" };
            if ui.button(status_text).clicked() {
                *paused = !*paused;
            }
            if ui
                .button("Reset Scene")
                .on_hover_text("Reset Scene")
                .clicked()
            {
                *reset = true;
            }
            if ui
                .button("Reset Camera")
                .on_hover_text("Reset Camera")
                .clicked()
            {
                *reset_camera = true;
            }
            ui.add(egui::Slider::new(speed, 0.0..=4.0).suffix("x"));
            ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                ui.toggle_value(show_debug, "Show Debug");
            });
        });
    });
}

fn draw_debug_ui(ctx: &Context, velocities: &mut bool) {
    egui::Window::new("Debug")
        .default_open(false)
        .default_pos([12.0, 50.0])
        .resizable(false)
        .show(ctx, |ui| {
            ui.checkbox(velocities, "Show Velocities");
        });
}
