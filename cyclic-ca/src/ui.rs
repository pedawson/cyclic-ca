use crate::app::CyclicCAApp;
use crate::ca::{ColorScheme, Neighborhood, Pattern};
use eframe::egui;

pub fn render_grid_panel(app: &mut CyclicCAApp, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("Grid")
        .default_open(app.grid_panel_open)
        .show(ui, |ui| {
            app.grid_panel_open = true;

            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(egui::Slider::new(&mut app.pending_width, 50..=500));
            });

            ui.horizontal(|ui| {
                ui.label("Height:");
                ui.add(egui::Slider::new(&mut app.pending_height, 50..=500));
            });

            ui.horizontal(|ui| {
                ui.label("Types:");
                ui.add(egui::Slider::new(&mut app.pending_types, 3..=24));
            });

            ui.add_space(8.0);

            if ui.button("Apply").clicked() {
                app.ca.resize(app.pending_width, app.pending_height, app.pending_types);
                app.ca.set_color_scheme(app.selected_color_scheme);
                app.step_counter = 0;
            }
        });
}

pub fn render_visual_panel(app: &mut CyclicCAApp, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("Visual")
        .default_open(app.visual_panel_open)
        .show(ui, |ui| {
            app.visual_panel_open = true;

            ui.label("Color Scheme:");
            for scheme in ColorScheme::ALL {
                if ui.radio(app.selected_color_scheme == scheme, scheme.name()).clicked() {
                    app.selected_color_scheme = scheme;
                    app.ca.set_color_scheme(scheme);
                }
            }
        });
}

pub fn render_patterns_panel(app: &mut CyclicCAApp, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("Patterns")
        .default_open(app.patterns_panel_open)
        .show(ui, |ui| {
            app.patterns_panel_open = true;

            ui.label("Pattern:");
            for pattern in Pattern::ALL {
                if ui.radio(app.selected_pattern == pattern, pattern.name()).clicked() {
                    app.selected_pattern = pattern;
                    app.ca.apply_pattern(pattern);
                    app.step_counter = 0;
                }
            }
        });
}

pub fn render_simulation_panel(app: &mut CyclicCAApp, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("Simulation")
        .default_open(app.simulation_panel_open)
        .show(ui, |ui| {
            app.simulation_panel_open = true;

            ui.horizontal(|ui| {
                if ui.button(if app.running { "Stop" } else { "Start" }).clicked() {
                    app.running = !app.running;
                }

                if ui.button("Step").clicked() {
                    app.ca.update();
                    app.step_counter += 1;
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Randomize").clicked() {
                    app.ca.randomize();
                    app.step_counter = 0;
                }

                if ui.button("Clear").clicked() {
                    app.ca.clear();
                    app.running = false;
                    app.step_counter = 0;
                }
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut app.speed, 1.0..=120.0).suffix(" fps"));
            });

            ui.add_space(4.0);
            ui.label(format!("Grid: {}x{}", app.ca.width, app.ca.height));
            ui.label(format!("Types: {}", app.ca.num_types));
            ui.label(format!(
                "Status: {}",
                if app.running { "Running" } else { "Stopped" }
            ));
            if app.show_step_counter {
                ui.label(format!("Step: {}", app.step_counter));
            }
        });
}

pub fn render_options_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.options_open {
        return;
    }

    let mut open = app.options_open;
    egui::Window::new("Options")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(280.0)
        .show(ctx, |ui| {
            // Simulation Rules
            ui.strong("Simulation Rules");
            ui.separator();

            ui.label("Neighborhood:");
            for nb in Neighborhood::ALL {
                if ui.radio(app.ca.neighborhood == nb, nb.name()).clicked() {
                    app.ca.neighborhood = nb;
                }
            }

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Threshold:");
                ui.add(egui::Slider::new(&mut app.ca.threshold, 1..=8));
            });
            ui.label(
                egui::RichText::new("Min prey neighbors needed to consume a cell")
                    .small()
                    .weak(),
            );

            ui.add_space(10.0);

            // Performance
            ui.strong("Performance");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Steps/frame:");
                ui.add(egui::Slider::new(&mut app.steps_per_frame, 1..=20));
            });
            ui.label(
                egui::RichText::new("CA steps computed per display frame")
                    .small()
                    .weak(),
            );

            ui.add_space(10.0);

            // Display
            ui.strong("Display");
            ui.separator();
            ui.checkbox(&mut app.show_step_counter, "Show step counter");
        });

    app.options_open = open;
}
