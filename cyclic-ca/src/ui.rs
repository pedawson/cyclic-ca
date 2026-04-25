use crate::app::CyclicCAApp;
use crate::ca::{ColorScheme, Neighborhood, Pattern, Symmetry as CaSymmetry};
use eframe::egui;

pub fn render_about_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.about_open {
        return;
    }

    let mut open = app.about_open;
    egui::Window::new("About Cyclic Cellular Automata")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.heading("Cyclic Cellular Automata");
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new(concat!("Version ", env!("CARGO_PKG_VERSION")))
                        .strong(),
                );
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.label("A simulation of cyclic cellular automata where");
                ui.label("each cell type consumes the previous type in a");
                ui.label("repeating cycle, producing emergent wave patterns.");

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Built with:").strong());
                    ui.label("Rust \u{00b7} eframe \u{00b7} egui");
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Platform:").strong());
                    ui.label("macOS \u{00b7} Linux \u{00b7} Windows \u{00b7} WASM");
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.label(egui::RichText::new("Controls").strong());
                ui.add_space(4.0);
                egui::Grid::new("controls_grid")
                    .num_columns(2)
                    .spacing([16.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Scroll");
                        ui.label("Zoom in/out");
                        ui.end_row();
                        ui.label("Drag");
                        ui.label("Pan view");
                        ui.end_row();
                        ui.label("P");
                        ui.label("Pause / Resume");
                        ui.end_row();
                        ui.label("R");
                        ui.label("Restart");
                        ui.end_row();
                    });

                ui.add_space(8.0);
            });
        });

    app.about_open = open;
}

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
                app.ca
                    .resize(app.pending_width, app.pending_height, app.pending_types);
                app.ca.set_color_scheme(app.selected_color_scheme);
                app.step_counter = 0;
                app.texture_dirty = true;
                app.recording = false;
                app.recorded_frames.clear();
            }

            ui.add_space(4.0);
            ui.label("Quick Sizes:");
            ui.horizontal_wrapped(|ui| {
                let sizes: &[(&str, usize, usize)] =
                    &[("Square 200", 200, 200), ("Square 300", 300, 300)];
                for &(label, w, h) in sizes {
                    if ui.button(label).clicked() {
                        app.pending_width = w;
                        app.pending_height = h;
                        app.ca.resize(w, h, app.pending_types);
                        app.ca.set_color_scheme(app.selected_color_scheme);
                        app.step_counter = 0;
                        app.texture_dirty = true;
                        app.recording = false;
                        app.recorded_frames.clear();
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                let sizes: &[(&str, usize, usize)] = &[
                    ("Wide 320\u{00d7}200", 320, 200),
                    ("Wide 480\u{00d7}270", 480, 270),
                ];
                for &(label, w, h) in sizes {
                    if ui.button(label).clicked() {
                        app.pending_width = w;
                        app.pending_height = h;
                        app.ca.resize(w, h, app.pending_types);
                        app.ca.set_color_scheme(app.selected_color_scheme);
                        app.step_counter = 0;
                        app.texture_dirty = true;
                        app.recording = false;
                        app.recorded_frames.clear();
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                let sizes: &[(&str, usize, usize)] = &[
                    ("Portrait 200\u{00d7}320", 200, 320),
                    ("Portrait 270\u{00d7}480", 270, 480),
                ];
                for &(label, w, h) in sizes {
                    if ui.button(label).clicked() {
                        app.pending_width = w;
                        app.pending_height = h;
                        app.ca.resize(w, h, app.pending_types);
                        app.ca.set_color_scheme(app.selected_color_scheme);
                        app.step_counter = 0;
                        app.texture_dirty = true;
                        app.recording = false;
                        app.recorded_frames.clear();
                    }
                }
            });
        });
}

pub fn render_visual_panel(app: &mut CyclicCAApp, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("Visual")
        .default_open(app.visual_panel_open)
        .show(ui, |ui| {
            app.visual_panel_open = true;

            ui.label("Color Scheme:");
            for scheme in ColorScheme::ALL {
                if ui
                    .radio(app.selected_color_scheme == scheme, scheme.name())
                    .clicked()
                {
                    app.selected_color_scheme = scheme;
                    app.ca.set_color_scheme(scheme);
                    app.texture_dirty = true;
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
                if ui
                    .radio(app.selected_pattern == pattern, pattern.name())
                    .clicked()
                {
                    app.selected_pattern = pattern;
                    app.ca.apply_pattern(pattern);
                    app.step_counter = 0;
                    app.texture_dirty = true;
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
                if ui
                    .button(if app.running { "Stop" } else { "Start" })
                    .clicked()
                {
                    app.running = !app.running;
                }
                if ui.button("Step").clicked() {
                    app.ca.update();
                    app.ca.apply_symmetry(app.symmetry);
                    app.step_counter += 1;
                    app.texture_dirty = true;
                    if app.recording && app.recorded_frames.len() < app.max_record_frames {
                        app.recorded_frames.push(app.ca.to_rgb_bytes());
                    }
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Randomize").clicked() {
                    app.ca.randomize();
                    app.step_counter = 0;
                    app.texture_dirty = true;
                }
                if ui.button("Clear").clicked() {
                    app.ca.clear();
                    app.running = false;
                    app.step_counter = 0;
                    app.texture_dirty = true;
                }
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("\u{1f4f7} Export PNG").clicked() {
                    let now = ui.input(|i| i.time);
                    app.export_png(now);
                }
                if !app.recorded_frames.is_empty() {
                    if ui.button("Export GIF").clicked() {
                        let now = ui.input(|i| i.time);
                        app.export_gif(now);
                    }
                }
            });

            ui.add_space(4.0);

            // Recording
            ui.horizontal(|ui| {
                let rec_label = if app.recording {
                    "\u{23f9} Stop Rec"
                } else {
                    "\u{23fa} Record"
                };
                if ui.button(rec_label).clicked() {
                    if app.recording {
                        app.recording = false;
                    } else {
                        app.recorded_frames.clear();
                        app.recording = true;
                    }
                }
                if app.recording {
                    ui.label(
                        egui::RichText::new(format!(
                            "{}/{}",
                            app.recorded_frames.len(),
                            app.max_record_frames
                        ))
                        .small()
                        .weak(),
                    );
                } else if !app.recorded_frames.is_empty() {
                    ui.label(
                        egui::RichText::new(format!("{} frames", app.recorded_frames.len()))
                            .small()
                            .weak(),
                    );
                }
            });

            ui.add_space(4.0);

            // Paint mode
            ui.checkbox(&mut app.paint_mode, "Paint Mode");
            if app.paint_mode {
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    let max = app.ca.num_types.saturating_sub(1);
                    ui.add(egui::Slider::new(&mut app.paint_type, 0..=max));
                });
                let [r, g, b] = app.ca.get_color(app.paint_type);
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 8.0),
                    egui::Sense::hover(),
                );
                ui.painter()
                    .rect_filled(rect, 2.0, egui::Color32::from_rgb(r, g, b));
                ui.horizontal(|ui| {
                    ui.label("Brush:");
                    ui.add(egui::Slider::new(&mut app.brush_size, 1..=10));
                });
            }

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Speed:");
                ui.add(
                    egui::Slider::new(&mut app.speed, 0.25..=120.0)
                        .suffix(" fps")
                        .logarithmic(true),
                );
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

pub fn render_rules_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.rules_open {
        return;
    }

    let mut open = app.rules_open;
    egui::Window::new("Rules")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(280.0)
        .show(ctx, |ui| {
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
        });

    app.rules_open = open;
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
            // Appearance
            ui.strong("Appearance");
            ui.separator();
            ui.checkbox(&mut app.dark_mode, "Dark Mode");

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

            // Symmetry
            ui.strong("Symmetry");
            ui.separator();
            for sym in CaSymmetry::ALL {
                if ui.radio(app.symmetry == sym, sym.name()).clicked() {
                    app.symmetry = sym;
                    app.ca.apply_symmetry(sym);
                    app.texture_dirty = true;
                }
            }
            ui.label(
                egui::RichText::new("Applied after each simulation step")
                    .small()
                    .weak(),
            );

            ui.add_space(10.0);

            // Display
            ui.strong("Display");
            ui.separator();
            ui.checkbox(&mut app.show_step_counter, "Show step counter");

            ui.add_space(10.0);

            // View
            ui.strong("View");
            ui.separator();
            if ui.button("Reset Zoom & Pan").clicked() {
                app.reset_view();
            }
        });

    app.options_open = open;
}

pub fn render_presets_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.presets_open {
        return;
    }

    let mut open = app.presets_open;
    egui::Window::new("Presets")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.strong("Save Current Settings");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut app.preset_name_input);
            });
            ui.add_space(4.0);
            let can_save = !app.preset_name_input.trim().is_empty();
            if ui
                .add_enabled(can_save, egui::Button::new("Save Preset"))
                .clicked()
            {
                app.save_preset();
            }

            ui.add_space(12.0);

            ui.strong("Saved Presets");
            ui.separator();

            if app.presets.is_empty() {
                ui.label(egui::RichText::new("No presets saved yet.").weak().small());
            } else {
                let mut to_load: Option<usize> = None;
                let mut to_delete: Option<usize> = None;

                egui::ScrollArea::vertical()
                    .max_height(240.0)
                    .show(ui, |ui| {
                        for (i, preset) in app.presets.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&preset.name).strong());
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.small_button("\u{2715}").clicked() {
                                            to_delete = Some(i);
                                        }
                                        if ui.small_button("Load").clicked() {
                                            to_load = Some(i);
                                        }
                                    },
                                );
                            });
                            ui.label(
                                egui::RichText::new(format!(
                                    "{}x{} \u{00b7} {} types \u{00b7} {} \u{00b7} {:.2}fps",
                                    preset.width,
                                    preset.height,
                                    preset.num_types,
                                    preset.color_scheme.name(),
                                    preset.speed,
                                ))
                                .small()
                                .weak(),
                            );
                            ui.separator();
                        }
                    });

                if let Some(i) = to_load {
                    app.load_preset(i);
                }
                if let Some(i) = to_delete {
                    app.presets.remove(i);
                    app.save_presets_to_disk();
                }
            }
        });

    app.presets_open = open;
}

pub fn render_histogram_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.histogram_open {
        return;
    }

    let mut open = app.histogram_open;
    let counts = app.ca.population_counts();
    let max_count = counts.iter().copied().max().unwrap_or(1).max(1);
    let total = (app.ca.width * app.ca.height) as f32;
    let bar_bg = if app.dark_mode {
        egui::Color32::from_gray(60)
    } else {
        egui::Color32::from_gray(220)
    };

    egui::Window::new("Population")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(250.0)
        .show(ctx, |ui| {
            for (i, &count) in counts.iter().enumerate() {
                let [r, g, b] = app.ca.get_color(i);
                let color = egui::Color32::from_rgb(r, g, b);
                let frac = count as f32 / max_count as f32;
                let pct = 100.0 * count as f32 / total;

                ui.horizontal(|ui| {
                    let (swatch, _) = ui.allocate_exact_size(
                        egui::vec2(10.0, 10.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(swatch, 2.0, color);

                    let max_bar_w = 120.0;
                    let (bar_rect, _) = ui.allocate_exact_size(
                        egui::vec2(max_bar_w, 10.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(bar_rect, 2.0, bar_bg);
                    let filled = egui::Rect::from_min_size(
                        bar_rect.min,
                        egui::vec2(max_bar_w * frac, 10.0),
                    );
                    ui.painter().rect_filled(filled, 2.0, color);

                    ui.label(egui::RichText::new(format!("{:.1}%", pct)).small());
                });
            }
        });

    app.histogram_open = open;
}
