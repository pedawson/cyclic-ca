use crate::app::CyclicCAApp;
use crate::ca::{ColorScheme, Pattern};
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
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Randomize").clicked() {
                    app.ca.randomize();
                }

                if ui.button("Clear").clicked() {
                    app.ca.clear();
                    app.running = false;
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
        });
}
