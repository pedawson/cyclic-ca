use crate::app::CyclicCAApp;
use crate::ca::{ColorScheme, Neighborhood, Pattern, Symmetry as CaSymmetry, MATRIX_MAX};
use eframe::egui;

// ── Aspect ratio presets ───────────────────────────────────────────────────────
const ASPECT_PRESETS: &[(&str, usize, usize)] = &[
    ("Square 200",       200, 200),
    ("Square 300",       300, 300),
    ("Wide 320×200",     320, 200),
    ("Wide 480×270",     480, 270),
    ("Portrait 200×320", 200, 320),
    ("Portrait 270×480", 270, 480),
];

// ── Palette presets ───────────────────────────────────────────────────────────
const PALETTE_PRESETS: &[(&str, [[u8; 3]; 6])] = &[
    ("Sunset", [
        [255,  60,  20],
        [255, 140,   0],
        [255, 215,   0],
        [200,  80, 120],
        [130,  30, 100],
        [ 70,  10,  80],
    ]),
    ("Arctic", [
        [240, 250, 255],
        [180, 220, 240],
        [ 90, 175, 225],
        [ 40, 130, 200],
        [ 15,  75, 160],
        [  5,  25,  80],
    ]),
    ("Neon", [
        [255,   0, 120],
        [  0, 230, 255],
        [120, 255,   0],
        [255, 225,   0],
        [255,  80,   0],
        [180,   0, 255],
    ]),
    ("Pastel", [
        [255, 180, 193],
        [200, 170, 235],
        [165, 230, 210],
        [255, 218, 180],
        [175, 215, 235],
        [235, 195, 235],
    ]),
    ("Lava", [
        [ 10,   0,   0],
        [100,   5,   0],
        [210,  30,   0],
        [255, 120,   0],
        [255, 220,  40],
        [255, 255, 200],
    ]),
];

// ─────────────────────────────────────────────────────────────────────────────

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

            // Clamp types to ≤ 6 while Custom palette is active
            let max_types: usize = if app.selected_color_scheme == ColorScheme::Custom { 6 } else { 24 };
            app.pending_types = app.pending_types.min(max_types);

            ui.horizontal(|ui| {
                ui.label("Types:");
                ui.add(egui::Slider::new(&mut app.pending_types, 3..=max_types));
            });

            ui.add_space(4.0);

            if ui.button("Apply").clicked() {
                app.apply_size_change();
            }

            ui.add_space(6.0);
            ui.label(egui::RichText::new("Quick Sizes:").small());
            ui.add_space(2.0);

            let mut chunks = ASPECT_PRESETS.chunks(2);
            while let Some(pair) = chunks.next() {
                ui.horizontal(|ui| {
                    for &(label, w, h) in pair {
                        if ui.small_button(label).clicked() {
                            app.pending_width = w;
                            app.pending_height = h;
                            app.apply_size_change();
                        }
                    }
                });
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
                let clicked = ui.radio(app.selected_color_scheme == scheme, scheme.name()).clicked();
                if clicked {
                    if scheme == ColorScheme::Custom {
                        app.activate_custom_scheme();
                    } else {
                        app.selected_color_scheme = scheme;
                        app.ca.set_color_scheme(scheme);
                    }
                }
            }

            if app.selected_color_scheme == ColorScheme::Custom {
                ui.add_space(4.0);
                // Mini swatch row as a quick preview
                ui.horizontal(|ui| {
                    for i in 0..6 {
                        let [r, g, b] = app.custom_palette[i];
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(18.0, 18.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(rect, 3.0, egui::Color32::from_rgb(r, g, b));
                    }
                });
                ui.add_space(2.0);
                if ui.button("✏ Edit Palette…").clicked() {
                    app.palette_open = true;
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

            // Playback controls
            ui.horizontal(|ui| {
                if ui.button(if app.running { "⏹ Stop" } else { "▶ Start" }).clicked() {
                    app.running = !app.running;
                }
                if ui.button("⏭ Step").clicked() {
                    app.ca.update();
                    app.step_counter += 1;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("🔀 Randomize").clicked() {
                    app.ca.randomize();
                    app.step_counter = 0;
                }
                if ui.button("🗑 Clear").clicked() {
                    app.ca.clear();
                    app.running = false;
                    app.step_counter = 0;
                }
            });

            ui.add_space(4.0);
            if ui.button("📷 Export PNG").clicked() {
                let now = ui.input(|i| i.time);
                app.export_png(now);
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
            ui.label(format!("Grid: {}×{}", app.ca.width, app.ca.height));
            ui.label(format!("Types: {}", app.ca.num_types));
            ui.label(format!("Status: {}", if app.running { "Running" } else { "Stopped" }));
            if app.show_step_counter {
                ui.label(format!("Step: {}", app.step_counter));
            }

            ui.add_space(8.0);

            // Save / Load
            ui.separator();
            ui.strong("Save / Load State");
            ui.horizontal(|ui| {
                if ui.button("💾 Save").clicked() {
                    let now = ui.input(|i| i.time);
                    app.save_state(now);
                }
                if ui.button("📂 Load").clicked() {
                    let now = ui.input(|i| i.time);
                    app.load_state(now);
                }
            });
            ui.label(
                egui::RichText::new("Desktop/CyclicCA_state.ccas").small().weak(),
            );

            ui.add_space(8.0);

            // Auto-stop
            ui.separator();
            ui.strong("Auto-Stop");
            ui.checkbox(&mut app.auto_stop_enabled, "Enabled");
            if app.auto_stop_enabled {
                ui.horizontal(|ui| {
                    ui.label("Stop at step:");
                    ui.add(
                        egui::DragValue::new(&mut app.auto_stop_steps)
                            .speed(10.0)
                            .range(1..=1_000_000),
                    );
                });
                if app.step_counter > 0 {
                    let rem = app.auto_stop_steps.saturating_sub(app.step_counter);
                    ui.label(egui::RichText::new(format!("{} steps remaining", rem)).small().weak());
                }
            }

            ui.add_space(8.0);

            // GIF recording
            ui.separator();
            ui.strong("GIF Export");
            if !app.recording {
                ui.horizontal(|ui| {
                    ui.label("Frames:");
                    ui.add(egui::DragValue::new(&mut app.record_frame_target).speed(1.0).range(4..=500));
                });
                ui.horizontal(|ui| {
                    ui.label("Capture every:");
                    ui.add(egui::DragValue::new(&mut app.record_capture_every).speed(1.0).range(1..=60));
                    ui.label("steps");
                });
                ui.label(
                    egui::RichText::new(format!(
                        "≈{:.0}s of simulation",
                        app.record_frame_target as f32 * app.record_capture_every as f32 / app.speed
                    ))
                    .small().weak(),
                );
                ui.add_space(4.0);
                if ui.button("⏺ Start Recording").clicked() {
                    app.start_recording();
                    if !app.running { app.running = true; }
                }
            } else {
                ui.label(
                    egui::RichText::new(format!(
                        "⏺ {}/{} frames",
                        app.record_frames.len(),
                        app.record_frame_target,
                    ))
                    .color(egui::Color32::from_rgb(220, 80, 80)),
                );
                if ui.button("⏹ Stop & Save GIF").clicked() {
                    let now = ui.input(|i| i.time);
                    app.finish_recording(now);
                }
            }
        });
}

// ── Options window ────────────────────────────────────────────────────────────

pub fn render_options_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.options_open { return; }

    let mut open = app.options_open;
    egui::Window::new("Options")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(280.0)
        .show(ctx, |ui| {
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
                    .small().weak(),
            );

            ui.add_space(10.0);
            ui.strong("Performance");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Steps/frame:");
                ui.add(egui::Slider::new(&mut app.steps_per_frame, 1..=20));
            });
            ui.label(
                egui::RichText::new("CA steps computed per display frame")
                    .small().weak(),
            );

            ui.add_space(10.0);
            ui.strong("Symmetry");
            ui.separator();
            for sym in CaSymmetry::ALL {
                if ui.radio(app.symmetry == sym, sym.name()).clicked() {
                    app.symmetry = sym;
                    app.ca.apply_symmetry(sym);
                }
            }
            ui.label(
                egui::RichText::new("Applied after each simulation step")
                    .small().weak(),
            );

            ui.add_space(10.0);
            ui.strong("Display");
            ui.separator();
            ui.checkbox(&mut app.show_step_counter, "Show step counter");

            ui.add_space(10.0);
            ui.strong("View");
            ui.separator();
            if ui.button("Reset Zoom & Pan").clicked() {
                app.reset_view();
            }
        });

    app.options_open = open;
}

// ── Presets window ────────────────────────────────────────────────────────────

pub fn render_presets_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.presets_open { return; }

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
            if ui.add_enabled(can_save, egui::Button::new("Save Preset")).clicked() {
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

                egui::ScrollArea::vertical().max_height(240.0).show(ui, |ui| {
                    for (i, preset) in app.presets.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&preset.name).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("✕").clicked() { to_delete = Some(i); }
                                if ui.small_button("Load").clicked() { to_load = Some(i); }
                            });
                        });
                        ui.label(
                            egui::RichText::new(format!(
                                "{}×{} · {} types · {} · {:.2}fps",
                                preset.width, preset.height, preset.num_types,
                                preset.color_scheme.name(), preset.speed,
                            ))
                            .small().weak(),
                        );
                        ui.separator();
                    }
                });

                if let Some(i) = to_load { app.load_preset(i); }
                if let Some(i) = to_delete { app.presets.remove(i); }
            }
        });

    app.presets_open = open;
}

// ── Custom Palette window ─────────────────────────────────────────────────────

pub fn render_palette_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.palette_open { return; }

    let mut open = app.palette_open;
    egui::Window::new("Custom Palette")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(330.0)
        .show(ctx, |ui| {
            // ── Row of 6 colour swatches ───────────────────────────────────
            ui.horizontal(|ui| {
                ui.add_space(4.0);
                for i in 0..6usize {
                    let [r, g, b] = app.custom_palette[i];
                    let swatch_color = egui::Color32::from_rgb(r, g, b);
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(42.0, 42.0),
                        egui::Sense::click(),
                    );
                    let painter = ui.painter();
                    painter.rect_filled(rect, 6.0, swatch_color);

                    // Slot number (1-based)
                    let label_color = if r as u32 + g as u32 + b as u32 > 380 {
                        egui::Color32::from_black_alpha(160)
                    } else {
                        egui::Color32::from_white_alpha(200)
                    };
                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", i + 1),
                        egui::FontId::proportional(13.0),
                        label_color,
                    );

                    // Selection ring
                    if i == app.palette_selected {
                        painter.rect_stroke(rect, 6.0, egui::Stroke::new(2.5, egui::Color32::WHITE));
                        painter.rect_stroke(rect.expand(2.5), 7.0, egui::Stroke::new(1.0, egui::Color32::from_gray(30)));
                    } else if response.hovered() {
                        painter.rect_stroke(rect, 6.0, egui::Stroke::new(1.5, egui::Color32::from_gray(200)));
                    }

                    if response.clicked() {
                        app.select_palette_slot(i);
                    }

                    if i < 5 { ui.add_space(4.0); }
                }
            });

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(format!("Editing slot {}", app.palette_selected + 1))
                    .small().weak(),
            );
            ui.add_space(4.0);

            // ── RGB sliders ───────────────────────────────────────────────
            let slot = app.palette_selected;
            let mut r = app.custom_palette[slot][0];
            let mut g = app.custom_palette[slot][1];
            let mut b = app.custom_palette[slot][2];

            let r_ch = ui.horizontal(|ui| {
                ui.label("R");
                ui.add(egui::Slider::new(&mut r, 0u8..=255u8)).changed()
            }).inner;
            let g_ch = ui.horizontal(|ui| {
                ui.label("G");
                ui.add(egui::Slider::new(&mut g, 0u8..=255u8)).changed()
            }).inner;
            let b_ch = ui.horizontal(|ui| {
                ui.label("B");
                ui.add(egui::Slider::new(&mut b, 0u8..=255u8)).changed()
            }).inner;

            if r_ch || g_ch || b_ch {
                app.update_palette_from_rgb([r, g, b]);
            }

            // ── Hex input ─────────────────────────────────────────────────
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Hex #");
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut app.palette_hex_input)
                        .desired_width(72.0)
                        .font(egui::TextStyle::Monospace),
                );
                // Show validity indicator
                let valid = crate::app::CyclicCAApp::parse_hex(&app.palette_hex_input).is_some();
                if !valid && !app.palette_hex_input.is_empty() {
                    ui.label(egui::RichText::new("✗").color(egui::Color32::from_rgb(220, 60, 60)));
                }
                if ui.button("Apply").clicked()
                    || (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    app.apply_hex_input();
                }
            });

            ui.add_space(12.0);

            // ── Preset palettes ───────────────────────────────────────────
            ui.separator();
            ui.strong("Presets");
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                for &(name, palette) in PALETTE_PRESETS {
                    if ui.button(name).clicked() {
                        app.custom_palette = palette;
                        app.select_palette_slot(app.palette_selected);
                        if app.selected_color_scheme == ColorScheme::Custom {
                            app.apply_custom_palette();
                        }
                    }
                }
                ui.add_space(4.0);
                if ui.button("↺ Reset to defaults").clicked() {
                    app.reset_palette();
                }
            });

            ui.add_space(8.0);

            // ── Gradient fill ─────────────────────────────────────────────
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("⟷ Gradient  1 → 6").clicked() {
                    app.interpolate_palette();
                }
                ui.label(
                    egui::RichText::new("fills slots 2–5")
                        .small().weak(),
                );
            });

            ui.add_space(6.0);

            // ── Activate button ───────────────────────────────────────────
            let is_active = app.selected_color_scheme == ColorScheme::Custom;
            if is_active {
                ui.label(egui::RichText::new("✔ Active — CA is using this palette").small()
                    .color(egui::Color32::from_rgb(80, 200, 120)));
            } else {
                if ui.button("Use this palette").clicked() {
                    app.activate_custom_scheme();
                }
            }
        });

    app.palette_open = open;
}

// ── Rule Editor window ────────────────────────────────────────────────────────

pub fn render_rule_editor_window(app: &mut CyclicCAApp, ctx: &egui::Context) {
    if !app.rule_editor_open { return; }

    let n = app.ca.num_types.min(MATRIX_MAX);

    let mut open = app.rule_editor_open;
    egui::Window::new("Rule Editor")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(360.0)
        .show(ctx, |ui| {

            // ── Enable toggle ─────────────────────────────────────────────
            ui.horizontal(|ui| {
                let label = if app.ca.use_rule_matrix {
                    egui::RichText::new("● Matrix rules active")
                        .color(egui::Color32::from_rgb(80, 220, 120))
                        .strong()
                } else {
                    egui::RichText::new("○ Standard cyclic rule active").weak()
                };
                if ui.checkbox(&mut app.ca.use_rule_matrix, "").changed() {
                    // When enabling, clamp to MATRIX_MAX types
                    if app.ca.use_rule_matrix && app.ca.num_types > MATRIX_MAX {
                        let (w, h) = (app.ca.width, app.ca.height);
                        app.ca.resize(w, h, MATRIX_MAX);
                        app.pending_types = MATRIX_MAX;
                        if app.selected_color_scheme == ColorScheme::Custom {
                            app.apply_custom_palette();
                        }
                    }
                }
                ui.label(label);
            });
            ui.label(
                egui::RichText::new(
                    "Each ✔ means: the ROW type can eat the COLUMN type.\n\
                     When multiple predators qualify, the one with more neighbours wins."
                )
                .small().weak(),
            );

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);

            // ── Matrix grid ───────────────────────────────────────────────
            // Header row: "eats →" label + one coloured box per type (columns = victims)
            let swatch = egui::vec2(28.0, 22.0);
            let rounding = 3.0;

            ui.horizontal(|ui| {
                // Corner label
                ui.add_space(70.0); // row-header width
                ui.label(egui::RichText::new("victim →").small().weak());
            });

            ui.horizontal(|ui| {
                ui.add_space(70.0);
                for col in 0..n {
                    let [r, g, b] = app.ca.type_color(col);
                    let (rect, _) = ui.allocate_exact_size(swatch, egui::Sense::hover());
                    ui.painter().rect_filled(rect, rounding, egui::Color32::from_rgb(r, g, b));
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", col + 1),
                        egui::FontId::proportional(11.0),
                        egui::Color32::from_white_alpha(220),
                    );
                }
            });

            ui.add_space(4.0);

            // One row per eater type
            for row in 0..n {
                ui.horizontal(|ui| {
                    // Row header: coloured swatch + "Type N eats:"
                    let [r, g, b] = app.ca.type_color(row);
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(18.0, 18.0), egui::Sense::hover()
                    );
                    ui.painter().rect_filled(rect, rounding, egui::Color32::from_rgb(r, g, b));
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", row + 1),
                        egui::FontId::proportional(10.0),
                        egui::Color32::from_white_alpha(220),
                    );
                    ui.label(egui::RichText::new(" eats:").small());

                    // Checkbox per victim column
                    for col in 0..n {
                        // Shade the standard-cycle diagonal differently
                        let is_default = col == (row + 1) % n;
                        let cell_size = swatch;

                        // Allocate space for a centred checkbox
                        let (cell_rect, _) = ui.allocate_exact_size(cell_size, egui::Sense::hover());
                        let cb_rect = egui::Rect::from_center_size(
                            cell_rect.center(),
                            egui::vec2(16.0, 16.0),
                        );

                        // Background tint for the standard-cycle diagonal
                        if is_default {
                            ui.painter().rect_filled(
                                cell_rect.expand(1.0),
                                2.0,
                                egui::Color32::from_white_alpha(12),
                            );
                        }

                        // Can't self-eat
                        if row == col {
                            ui.painter().text(
                                cell_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "—",
                                egui::FontId::proportional(12.0),
                                egui::Color32::from_gray(60),
                            );
                        } else {
                            // Draw checkbox manually so we can position it inside the cell
                            let cb_response = ui.allocate_rect(cb_rect, egui::Sense::click());
                            let checked = app.ca.rule_matrix[row][col];

                            let bg = if checked {
                                egui::Color32::from_rgb(60, 180, 100)
                            } else {
                                egui::Color32::from_gray(45)
                            };
                            let border = if cb_response.hovered() {
                                egui::Color32::from_gray(180)
                            } else {
                                egui::Color32::from_gray(90)
                            };

                            ui.painter().rect_filled(cb_rect, 3.0, bg);
                            ui.painter().rect_stroke(cb_rect, 3.0, egui::Stroke::new(1.0, border));

                            if checked {
                                ui.painter().text(
                                    cb_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    "✔",
                                    egui::FontId::proportional(11.0),
                                    egui::Color32::WHITE,
                                );
                            }

                            if cb_response.clicked() {
                                app.ca.rule_matrix[row][col] = !checked;
                            }
                        }
                    }
                });
            }

            ui.add_space(10.0);
            ui.separator();

            // ── Controls ──────────────────────────────────────────────────
            ui.horizontal(|ui| {
                if ui.button("↺ Reset to standard cycle").clicked() {
                    app.ca.reset_rule_matrix();
                }
                if ui.button("✕ Clear all").clicked() {
                    app.ca.rule_matrix = [[false; MATRIX_MAX]; MATRIX_MAX];
                }
            });
            ui.label(
                egui::RichText::new(
                    "Tip: clearing all rules freezes the grid — try unusual combos!"
                )
                .small().weak(),
            );
        });

    app.rule_editor_open = open;
}
