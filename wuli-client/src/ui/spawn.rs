use egui::{Button, Context, Grid, RichText, SidePanel, Ui};

const COLLAPSED_WIDTH: f32 = 30.0;

pub(crate) struct State {
    pub collapsed: bool,
}

/// Draw the `spawn` ui component, responsible for providing
/// a menu of possible bodies and constraints to
pub(crate) fn ui_draw_spawn_component(state: &mut State, context: &Context) {
    let mut panel = SidePanel::right("ui-component:spawn");

    if state.collapsed {
        panel = panel.exact_width(COLLAPSED_WIDTH);
    }

    panel.show(context, |ui| {
        if state.collapsed {
            draw_collapsed(state, ui);
            return;
        }

        draw_uncollapsed(state, ui);
    });
}

fn draw_justified_toggle(state: &mut State, ui: &mut Ui, height: f32, label: impl Into<String>) {
    let pad = (height - ui.spacing().interact_size.y) / 2.0; // ui.spacing().interact_size is the size of a standard button.
    ui.vertical(|ui| {
        ui.add_space(pad.max(0.0));
        if ui.add(Button::new(label.into()).frame(false)).clicked() {
            state.collapsed = !state.collapsed;
        }
    });
}

/// Draw the `spawn` ui component in its collapsed (i.e. closed) state.
fn draw_collapsed(state: &mut State, ui: &mut Ui) {
    draw_justified_toggle(state, ui, ui.available_height(), "◀");
}

/// Draw the `spawn` ui component, in its uncollapsed (i.e. opened) state.
fn draw_uncollapsed(state: &mut State, ui: &mut Ui) {
    let height = ui.available_height();
    ui.set_min_height(height);
    ui.add_space(ui.spacing().interact_size.y); // match "offset" created by toggle button; interact_size.y matches the button height

    ui.horizontal(|ui| {
        draw_justified_toggle(state, ui, height, "▶");
        ui.vertical(|ui| {
            draw_bodies(state, ui);
            ui.separator();
            draw_constraints(state, ui);
        });
    });
}

fn draw_bodies(state: &mut State, ui: &mut Ui) {
    ui.label(RichText::new("Bodies").strong().underline());
    Grid::new("ui-component:spawn:bodies").show(ui, |ui| {
        ui.label("Body");
        ui.label("Body");
        ui.end_row();
        ui.label("Body");
        ui.label("Body");
        ui.end_row();
        ui.label("Body");
        ui.label("Body");
    });
}

fn draw_constraints(state: &mut State, ui: &mut Ui) {
    ui.label(RichText::new("Constraints").strong().underline());
    Grid::new("ui-component:spawn:constraints").show(ui, |ui| {
        ui.label("Constraint");
        ui.label("Constraint");
        ui.end_row();
        ui.label("Constraint");
        ui.label("Constraint");
        ui.end_row();
        ui.label("Constraint");
        ui.label("Constraint");
    });
}
