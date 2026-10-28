use egui::{Align, Align2, Color32, Context, CornerRadius, Layout, Shadow, Stroke, Ui, Visuals};
use glam::{DVec2, dvec2};
use macroquad::{
    camera::set_camera,
    color::{Color, LIME},
    input::{
        KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_down,
        is_mouse_button_pressed, mouse_position, mouse_wheel,
    },
    math::{Vec2, vec2},
    shapes::{
        DrawRectangleParams, draw_circle, draw_line, draw_poly, draw_poly_lines, draw_rectangle_ex,
        draw_rectangle_lines_ex, draw_triangle,
    },
    time::get_frame_time,
    window::{clear_background, next_frame, screen_width},
};
use wuli::{
    HandleNotFound, World,
    body::{BodyHandle, Shape},
    constraints::{
        Constraint, ConstraintHandle, ConstraintKind, distance::DistanceConstraint,
        pin::PinConstraint,
    },
};

use crate::{select::Selection, ui::spawn::ui_draw_spawn_component, view::View};

pub mod select;
pub mod ui;
pub mod view;

// TikZ look: black ink and gray!20 fills on white paper, strokes a fixed number of pixels wide
const PAPER: Color = Color::new(1.0, 1.0, 1.0, 1.0);
// gray!20: 20% of TikZ gray (50% black) mixed with white
const FILL: Color = Color::new(0.9, 0.9, 0.9, 1.0);
const INK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
const LINE_PX: f32 = 2.5;

const ACCENT: Color = Color::new(0.4, 0.4, 0.8, 1.0);

struct State {
    pub spawn: ui::spawn::State,
}

#[macroquad::main("wuli")]
async fn main() {
    let options = wuli_scenes::available();
    let mut current = 0;

    let mut state = State {
        spawn: ui::spawn::State { collapsed: true },
    };

    let mut world = (options[current].1)();
    let mut view = View::new(0.0, 0.0, 10.0);

    let mut accumulator = 0.0;
    let dt = 1.0 / 240.0;

    let mut speed = 1.0;
    let mut paused = true;
    let mut reset = false;
    let mut reset_camera = false;
    let mut show_debug = false;

    let mut velocities = false;
    let mut selected = Selection::None;
    let mut dragging = None;

    let mut last_mouse = None;

    loop {
        if reset {
            world = (options[current].1)();
            selected = Selection::None;
            dragging = None;
            reset = false;
        }
        if reset_camera {
            view.centre = DVec2::ZERO;
            view.width = 10.0;
            reset_camera = false;
        }

        tick(
            &mut world,
            &mut accumulator,
            dt,
            if !paused { speed } else { 0.0 },
        );

        clear_background(PAPER);
        set_camera(&view.camera());
        // World units per screen pixel, so strokes keep their width when zooming
        let px = (view.width / screen_width() as f64) as f32;
        draw_bodies(&world, px, &selected);
        if velocities {
            draw_velocities(&world, px);
        }
        draw_constraints(&world, px, &selected);

        egui_macroquad::ui(|ctx| {
            ctx.set_visuals(tikz_visuals());

            handle_controls(
                ctx,
                &mut world,
                &mut view,
                &mut last_mouse,
                &mut selected,
                &mut dragging,
            );
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
                draw_debug_ui(ctx, &world, &mut velocities);
            }
            if !selected.is_none() {
                draw_inspector_ui(ctx, &mut world, &selected);
            }

            ui_draw_spawn_component(&mut state.spawn, ctx);
        });
        egui_macroquad::draw();

        next_frame().await
    }
}

fn get_body_under_mouse(world: &World, view: &View) -> Option<BodyHandle> {
    let mouse = mouse_position();
    let mouse = view
        .camera()
        .screen_to_world(macroquad::math::vec2(mouse.0, mouse.1));
    let point = dvec2(mouse.x as f64, mouse.y as f64);
    world.body_at(point)
}

fn tick(world: &mut World, accumulator: &mut f64, dt: f64, speed: f64) {
    *accumulator += get_frame_time() as f64 * speed;
    while *accumulator > dt {
        world.step(dt);
        *accumulator -= dt;
    }
}

fn handle_controls(
    ctx: &Context,
    world: &mut World,
    view: &mut View,
    last_mouse: &mut Option<(f32, f32)>,
    selected: &mut Selection,
    dragging: &mut Option<(ConstraintHandle, PinConstraint)>,
) {
    if is_key_pressed(KeyCode::Delete) && !ctx.wants_keyboard_input() {
        selected.delete(world);
        *dragging = None;
    }

    if is_mouse_button_pressed(MouseButton::Left) && !ctx.wants_pointer_input() {
        let new_selection = Selection::from_mouse(world, view);
        if new_selection.is_none() {
            *dragging = None;
            *selected = new_selection;
        } else if new_selection == *selected
            && let Selection::Body(handle) = new_selection
        {
            let mouse = view.camera().screen_to_world(macroquad::math::vec2(
                mouse_position().0,
                mouse_position().1,
            ));
            let body = world.body(handle).unwrap();
            let anchor =
                (dvec2(mouse.x as f64, mouse.y as f64) - body.position).rotate_angle(-body.angle);

            let pin = PinConstraint {
                body: handle,
                local_anchor: anchor,
                target: dvec2(mouse.x as f64, mouse.y as f64),
                compliance: 1e-4 * body.inv_mass,
                damping: 200.0,
                angular_damping: 10.0,
                lambda: 0.0,
            };
            let pin_handle = world.add_constraint(Constraint {
                kind: ConstraintKind::Pin {
                    constraint: pin.clone(),
                },
                soft: true,
            });
            *dragging = Some((pin_handle, pin));
        } else {
            if is_key_down(KeyCode::LeftShift) {
                *selected = selected.clone().appended(new_selection);
            } else {
                *selected = new_selection;
            }
        }
    }

    if is_mouse_button_down(MouseButton::Left) && !ctx.wants_pointer_input() {
        if let Some((handle, _)) = dragging {
            let mouse = view.camera().screen_to_world(macroquad::math::vec2(
                mouse_position().0,
                mouse_position().1,
            ));
            let mut body = None;
            if let Ok(Constraint {
                kind:
                    ConstraintKind::Pin {
                        constraint:
                            PinConstraint {
                                target,
                                body: body_handle,
                                ..
                            },
                    },
                ..
            }) = world.constraint_mut(*handle)
            {
                *target = dvec2(mouse.x as f64, mouse.y as f64);
                body = Some(*body_handle);
            }
        }
    } else {
        if let Some((handle, _)) = dragging {
            world.constraints.remove(*handle);
        }
        *dragging = None;
    }

    // Zoom
    if !ctx.wants_pointer_input() {
        let (_, wheel) = mouse_wheel();
        if wheel > 0.0 {
            view.width *= 0.9;
        }
        if wheel < 0.0 {
            view.width *= 1.1;
        }
    }

    // Pan
    let mouse = mouse_position();
    if is_mouse_button_down(MouseButton::Right) && !ctx.wants_pointer_input() {
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

fn draw_bodies(world: &World, px: f32, selected: &Selection) {
    world.bodies.iter().for_each(|(handle, _)| {
        let _ = draw_body(world, handle, px, selected);
    });
}

fn draw_constraints(world: &World, px: f32, selected: &Selection) {
    world.constraints.iter().for_each(|(h, _)| {
        let _ = draw_constraint(world, px, h, selected);
    });
}

fn draw_constraint(
    world: &World,
    px: f32,
    handle: ConstraintHandle,
    selected: &Selection,
) -> Result<(), HandleNotFound> {
    let constraint = world.constraint(handle)?;
    let color = if selected.contains_constraint(handle) {
        ACCENT
    } else {
        INK
    };

    match constraint {
        Constraint {
            kind:
                ConstraintKind::Distance {
                    constraint: DistanceConstraint { a, b, .. },
                },
            ..
        } => {
            let a = world.body(*a)?;
            let b = world.body(*b)?;
            draw_line(
                a.position.x as f32,
                a.position.y as f32,
                b.position.x as f32,
                b.position.y as f32,
                LINE_PX * px,
                color,
            );
        }
        Constraint {
            kind: ConstraintKind::Contact { collision },
            ..
        } => {
            draw_circle(
                collision.contact.x as f32,
                collision.contact.y as f32,
                0.05,
                LIME,
            );
        }
        Constraint {
            kind: ConstraintKind::Pin { constraint },
            ..
        } => {
            let body = world.body(constraint.body)?;
            let anchor = constraint.local_anchor.rotate_angle(body.angle);
            draw_line(
                body.position.x as f32 + anchor.x as f32,
                body.position.y as f32 + anchor.y as f32,
                constraint.target.x as f32,
                constraint.target.y as f32,
                LINE_PX * px,
                color,
            );
        }
    }

    Ok(())
}

fn draw_velocities(world: &World, px: f32) {
    world.bodies.iter().for_each(|(handle, _)| {
        let _ = draw_velocity(world, handle, px);
    });
}

fn draw_body(
    world: &World,
    body: BodyHandle,
    px: f32,
    selected: &Selection,
) -> Result<(), HandleNotFound> {
    let outline = if selected.contains_body(body) {
        ACCENT
    } else {
        INK
    };
    let body = world.body(body)?;
    let (x, y) = (body.position.x as f32, body.position.y as f32);

    match body.shape {
        Shape::Circle { radius } => {
            // \draw[fill=gray!20] (x, y) circle (r);
            draw_poly(x, y, 64, radius as f32, 0.0, FILL);
            draw_poly_lines(x, y, 64, radius as f32, 0.0, LINE_PX * px, outline);
        }
        Shape::Rectangle { half_extents } => {
            let extents = half_extents * 2.0;
            draw_rectangle_ex(
                x,
                y,
                extents.x as f32,
                extents.y as f32,
                DrawRectangleParams {
                    rotation: body.angle as f32,
                    color: FILL,
                    offset: vec2(0.5, 0.5),
                },
            );
            draw_rectangle_lines_ex(
                x,
                y,
                extents.x as f32,
                extents.y as f32,
                LINE_PX * px,
                DrawRectangleParams {
                    rotation: body.angle as f32,
                    color: outline,
                    offset: vec2(0.5, 0.5),
                },
            );
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
            ui.add(
                egui::Slider::new(speed, 0.0..=4.0)
                    .suffix("x")
                    .step_by(0.01),
            );
            ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                ui.toggle_value(show_debug, "Show Debug");
            });
        });
    });
}

fn draw_debug_ui(ctx: &Context, world: &World, velocities: &mut bool) {
    egui::Window::new("Debug")
        .default_open(false)
        .default_pos([12.0, 50.0])
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Information").underline());
            egui::Grid::new("debug-infos").striped(true).show(ui, |ui| {
                ui.label("Energy: ");
                ui.label(format!("{:.2} J", world.energy()));
                ui.end_row();

                ui.label("Bodies: ");
                ui.label(world.bodies.len().to_string());
                ui.end_row();
            });

            ui.checkbox(velocities, "Show Velocities");
        });
}

fn draw_inspector_ui(ctx: &Context, world: &mut World, selected: &Selection) {
    egui::Window::new("Inspector")
        .anchor(Align2::RIGHT_TOP, [-12.0, 40.0])
        .default_open(true)
        .resizable(true)
        .show(ctx, |ui| match selected {
            Selection::Body(handle) => draw_body_inspector_ui(ui, world, *handle),
            Selection::Constraint(_handle) => draw_constraint_inspector_ui(),
            Selection::Many(_selections) => draw_many_inspector_ui(),
            _ => {}
        });
}

fn draw_body_inspector_ui(ui: &mut Ui, world: &mut World, selected: BodyHandle) {
    let body = world
        .body_mut(selected)
        .expect("expected selected body to exist");

    ui.label(egui::RichText::new("Physical Quantities").underline());
    egui::Grid::new("quantities").striped(true).show(ui, |ui| {
        // Shape
        ui.label("Shape: ");
        ui.label(format!("{}", body.shape));
        ui.end_row();

        // Mass
        ui.label("Mass: ");
        ui.label(if body.inv_mass == 0.0 {
            "static".to_string()
        } else {
            format!("{:.2}", 1.0 / body.inv_mass).to_string()
        });
        ui.end_row();

        ui.label("x: ");
        ui_vector_field(ui, &mut body.position);
        ui.end_row();

        ui.label("v: ");
        ui_vector_field(ui, &mut body.velocity);
        ui.end_row();

        ui.label("θ: ");
        ui.add(
            egui::DragValue::new(&mut body.angle)
                .custom_parser(|v| v.parse::<f64>().ok().map(|v| v.to_radians()))
                .custom_formatter(|v, _| format!("{:.2}", v.to_degrees())),
        );
        ui.end_row();

        ui.label("ω: ");
        ui.add(
            egui::DragValue::new(&mut body.angular_velocity)
                .custom_parser(|v| v.parse::<f64>().ok().map(|v| v.to_radians()))
                .custom_formatter(|v, _| format!("{:.2}", v.to_degrees())),
        );
        ui.end_row();
    });

    ui.label(egui::RichText::new("Constraints").underline());
    world
        .constraints
        .iter()
        .filter(|(_, c)| c.kind.bodies().contains(&selected))
        .for_each(|(_, c)| {
            ui.label(format!("{}", c.to_string())); // TODO: constraint naming!
        });
}

fn draw_constraint_inspector_ui() {}
fn draw_many_inspector_ui() {}

fn ui_vector_field(ui: &mut egui::Ui, vector: &mut DVec2) {
    let (x, y) = (&mut vector.x, &mut vector.y);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0; // tighter, like a written vector
        ui.label("(");
        ui.add(egui::DragValue::new(x));
        ui.label("|");
        ui.add(egui::DragValue::new(y));
        ui.label(")");
    });
}
