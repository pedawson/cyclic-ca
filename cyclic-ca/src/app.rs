use crate::ca::{ColorScheme, CyclicCellularAutomata, Neighborhood, Pattern, Symmetry};
use crate::ui;
use eframe::egui;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Preset {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub num_types: usize,
    pub color_scheme: ColorScheme,
    pub neighborhood: Neighborhood,
    pub threshold: usize,
    pub speed: f32,
    pub steps_per_frame: usize,
    pub symmetry: Symmetry,
}

pub struct CyclicCAApp {
    pub ca: CyclicCellularAutomata,
    pub running: bool,
    pub texture: Option<egui::TextureHandle>,

    pub pending_width: usize,
    pub pending_height: usize,
    pub pending_types: usize,

    pub selected_pattern: Pattern,
    pub selected_color_scheme: ColorScheme,

    pub speed: f32,
    pub last_update: f64,

    pub grid_panel_open: bool,
    pub visual_panel_open: bool,
    pub patterns_panel_open: bool,
    pub simulation_panel_open: bool,

    pub about_open: bool,
    pub rules_open: bool,
    pub options_open: bool,
    pub steps_per_frame: usize,
    pub step_counter: u64,
    pub show_step_counter: bool,

    pub symmetry: Symmetry,

    pub presets_open: bool,
    pub presets: Vec<Preset>,
    pub preset_name_input: String,

    pub export_message: Option<(String, f64)>,

    pub zoom: f32,
    pub pan: egui::Vec2,

    pub texture_dirty: bool,

    pub dark_mode: bool,

    pub paint_mode: bool,
    pub paint_type: usize,
    pub brush_size: usize,

    pub histogram_open: bool,

    pub recording: bool,
    pub recorded_frames: Vec<Vec<u8>>,
    pub max_record_frames: usize,
}

impl Default for CyclicCAApp {
    fn default() -> Self {
        let width = 200;
        let height = 200;
        let num_types = 12;

        Self {
            ca: CyclicCellularAutomata::new(width, height, num_types),
            running: false,
            texture: None,
            pending_width: width,
            pending_height: height,
            pending_types: num_types,
            selected_pattern: Pattern::Random,
            selected_color_scheme: ColorScheme::Rainbow,
            speed: 30.0,
            last_update: 0.0,
            grid_panel_open: true,
            visual_panel_open: true,
            patterns_panel_open: true,
            simulation_panel_open: true,
            about_open: false,
            rules_open: false,
            options_open: false,
            steps_per_frame: 1,
            step_counter: 0,
            show_step_counter: true,
            symmetry: Symmetry::None,
            presets_open: false,
            presets: Self::load_presets_from_disk(),
            preset_name_input: String::new(),
            export_message: None,
            zoom: 1.0,
            pan: egui::Vec2::ZERO,
            texture_dirty: true,
            dark_mode: false,
            paint_mode: false,
            paint_type: 0,
            brush_size: 1,
            histogram_open: false,
            recording: false,
            recorded_frames: Vec::new(),
            max_record_frames: 500,
        }
    }
}

impl CyclicCAApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::theme::load_fonts(&cc.egui_ctx);
        crate::theme::apply_visuals(&cc.egui_ctx, false);
        Self::default()
    }

    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan = egui::Vec2::ZERO;
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        let image = self.ca.to_color_image();
        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.texture =
                Some(ctx.load_texture("ca_grid", image, egui::TextureOptions::NEAREST));
        }
    }

    pub fn export_png(&mut self, now: f64) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let desktop = std::path::Path::new(&home).join("Desktop");
        let filename = format!(
            "CyclicCA_{}x{}_t{}.png",
            self.ca.width, self.ca.height, self.step_counter
        );
        let path = desktop.join(&filename);
        let bytes = self.ca.to_rgb_bytes();
        let result = image::save_buffer(
            &path,
            &bytes,
            self.ca.width as u32,
            self.ca.height as u32,
            image::ColorType::Rgb8,
        );
        self.export_message = Some(match result {
            Ok(_) => (format!("Saved: {}", filename), now + 4.0),
            Err(e) => (format!("Export failed: {}", e), now + 4.0),
        });
    }

    pub fn export_gif(&mut self, now: f64) {
        if self.recorded_frames.is_empty() {
            self.export_message = Some(("No frames recorded".to_string(), now + 4.0));
            return;
        }

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let desktop = std::path::Path::new(&home).join("Desktop");
        let filename = format!(
            "CyclicCA_{}x{}_{}frames.gif",
            self.ca.width, self.ca.height, self.recorded_frames.len()
        );
        let path = desktop.join(&filename);

        match self.write_gif(&path) {
            Ok(_) => {
                let msg =
                    format!("Saved: {} ({} frames)", filename, self.recorded_frames.len());
                self.export_message = Some((msg, now + 4.0));
                self.recorded_frames.clear();
            }
            Err(e) => {
                self.export_message = Some((format!("GIF failed: {}", e), now + 4.0));
            }
        }
    }

    fn write_gif(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        let mut encoder = image::codecs::gif::GifEncoder::new(file);
        encoder.set_repeat(image::codecs::gif::Repeat::Infinite)?;

        let w = self.ca.width as u32;
        let h = self.ca.height as u32;

        for rgb_data in &self.recorded_frames {
            let rgba: Vec<u8> = rgb_data
                .chunks(3)
                .flat_map(|c| [c[0], c[1], c[2], 255])
                .collect();
            let img =
                image::RgbaImage::from_raw(w, h, rgba).ok_or("Failed to create image")?;
            let delay = image::Delay::from_numer_denom_ms(100, 1);
            let frame = image::Frame::from_parts(img, 0, 0, delay);
            encoder.encode_frame(frame)?;
        }

        Ok(())
    }

    pub fn save_preset(&mut self) {
        let name = self.preset_name_input.trim().to_string();
        if name.is_empty() {
            return;
        }
        self.presets.retain(|p| p.name != name);
        self.presets.push(Preset {
            name,
            width: self.ca.width,
            height: self.ca.height,
            num_types: self.ca.num_types,
            color_scheme: self.ca.color_scheme,
            neighborhood: self.ca.neighborhood,
            threshold: self.ca.threshold,
            speed: self.speed,
            steps_per_frame: self.steps_per_frame,
            symmetry: self.symmetry,
        });
        self.preset_name_input.clear();
        self.save_presets_to_disk();
    }

    pub fn load_preset(&mut self, idx: usize) {
        if let Some(p) = self.presets.get(idx).cloned() {
            self.ca.resize(p.width, p.height, p.num_types);
            self.ca.set_color_scheme(p.color_scheme);
            self.ca.neighborhood = p.neighborhood;
            self.ca.threshold = p.threshold;
            self.speed = p.speed;
            self.steps_per_frame = p.steps_per_frame;
            self.symmetry = p.symmetry;
            self.selected_color_scheme = p.color_scheme;
            self.pending_width = p.width;
            self.pending_height = p.height;
            self.pending_types = p.num_types;
            self.step_counter = 0;
            self.texture_dirty = true;
            self.reset_view();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_presets_to_disk(&self) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let dir = std::path::Path::new(&home)
            .join(".config")
            .join("cyclic-ca");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("presets.json");
        if let Ok(json) = serde_json::to_string_pretty(&self.presets) {
            let _ = std::fs::write(path, json);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_presets_to_disk(&self) {}

    #[cfg(not(target_arch = "wasm32"))]
    fn load_presets_from_disk() -> Vec<Preset> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&home)
            .join(".config")
            .join("cyclic-ca")
            .join("presets.json");
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    #[cfg(target_arch = "wasm32")]
    fn load_presets_from_disk() -> Vec<Preset> {
        Vec::new()
    }
}

impl eframe::App for CyclicCAApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);

        crate::theme::apply_visuals(ctx, self.dark_mode);

        // Simulation tick
        if self.running {
            let interval = 1.0 / self.speed as f64;
            if now - self.last_update >= interval {
                for _ in 0..self.steps_per_frame {
                    self.ca.update();
                    self.ca.apply_symmetry(self.symmetry);
                    self.step_counter += 1;
                }
                self.last_update = now;
                self.texture_dirty = true;

                if self.recording && self.recorded_frames.len() < self.max_record_frames {
                    self.recorded_frames.push(self.ca.to_rgb_bytes());
                }
                if self.recorded_frames.len() >= self.max_record_frames {
                    self.recording = false;
                }
            }
            ctx.request_repaint();
        }

        // Expire export message
        if let Some((_, expiry)) = &self.export_message {
            if now > *expiry {
                self.export_message = None;
            } else {
                ctx.request_repaint();
            }
        }

        if self.texture_dirty {
            self.update_texture(ctx);
            self.texture_dirty = false;
        }

        // ── Sidebar ──────────────────────────────────────────────────────────
        let sidebar_frame = egui::Frame {
            fill: crate::theme::sidebar_bg(self.dark_mode),
            inner_margin: egui::Margin::symmetric(12.0, 8.0),
            ..Default::default()
        };

        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(220.0)
            .frame(sidebar_frame)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    let rules_label =
                        if self.rules_open { "Rules \u{2611}" } else { "Rules \u{2610}" };
                    if ui.button(rules_label).clicked() {
                        self.rules_open = !self.rules_open;
                    }
                    let pre_label = if self.presets_open {
                        "Presets \u{2611}"
                    } else {
                        "Presets \u{2610}"
                    };
                    if ui.button(pre_label).clicked() {
                        self.presets_open = !self.presets_open;
                    }
                    let opt_label = if self.options_open {
                        "Options \u{2611}"
                    } else {
                        "Options \u{2610}"
                    };
                    if ui.button(opt_label).clicked() {
                        self.options_open = !self.options_open;
                    }
                    let stats_label = if self.histogram_open {
                        "Stats \u{2611}"
                    } else {
                        "Stats \u{2610}"
                    };
                    if ui.button(stats_label).clicked() {
                        self.histogram_open = !self.histogram_open;
                    }
                    if ui.button("About").clicked() {
                        self.about_open = !self.about_open;
                    }
                });
                ui.separator();
                ui.add_space(4.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui::render_grid_panel(self, ui);
                    ui.add_space(8.0);
                    ui::render_visual_panel(self, ui);
                    ui.add_space(8.0);
                    ui::render_patterns_panel(self, ui);
                    ui.add_space(8.0);
                    ui::render_simulation_panel(self, ui);
                });
            });

        // ── Central panel ────────────────────────────────────────────────────
        let content_frame = egui::Frame {
            fill: crate::theme::content_bg(self.dark_mode),
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(content_frame)
            .show(ctx, |ui| {
                if let Some(texture) = &self.texture {
                    let available_size = ui.available_size();
                    let bottom_space = 48.0;
                    let usable =
                        egui::vec2(available_size.x, available_size.y - bottom_space);
                    let aspect_ratio = self.ca.width as f32 / self.ca.height as f32;

                    let (display_width, display_height) = if usable.x / usable.y
                        > aspect_ratio
                    {
                        let h = usable.y;
                        (h * aspect_ratio, h)
                    } else {
                        let w = usable.x;
                        (w, w / aspect_ratio)
                    };

                    let size = egui::vec2(display_width, display_height);
                    let offset = egui::vec2(
                        (available_size.x - display_width) / 2.0,
                        (usable.y - display_height) / 2.0,
                    );
                    let image_rect =
                        egui::Rect::from_min_size(ui.min_rect().min + offset, size);

                    // Shadow & border
                    let painter = ui.painter();
                    for i in 1..=6u8 {
                        let spread = i as f32 * 2.0;
                        let alpha = 25u8.saturating_sub(i * 3);
                        painter.rect_filled(
                            image_rect
                                .translate(egui::vec2(spread, spread))
                                .expand(spread * 0.5),
                            2.0,
                            egui::Color32::from_black_alpha(alpha),
                        );
                    }
                    painter.rect_stroke(
                        image_rect,
                        0.0,
                        egui::Stroke::new(1.5, egui::Color32::from_gray(80)),
                    );

                    // Interaction
                    let response =
                        ui.allocate_rect(image_rect, egui::Sense::click_and_drag());

                    if self.paint_mode {
                        // no pan in paint mode
                    } else if response.dragged() {
                        self.pan += response.drag_delta();
                        let max_pan = egui::vec2(
                            size.x * (self.zoom - 1.0) / 2.0,
                            size.y * (self.zoom - 1.0) / 2.0,
                        );
                        self.pan.x = self.pan.x.clamp(-max_pan.x, max_pan.x);
                        self.pan.y = self.pan.y.clamp(-max_pan.y, max_pan.y);
                    }

                    // Zoom via scroll wheel (always)
                    let scroll_delta = ui.input(|i| {
                        if i.pointer
                            .hover_pos()
                            .map_or(false, |p| image_rect.contains(p))
                        {
                            i.raw_scroll_delta.y
                        } else {
                            0.0
                        }
                    });
                    if scroll_delta != 0.0 {
                        let factor: f32 =
                            if scroll_delta > 0.0 { 1.12 } else { 1.0 / 1.12 };
                        let new_zoom = (self.zoom * factor).clamp(1.0, 12.0);
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let center = image_rect.center();
                            let mouse_offset = mouse_pos - center;
                            let zoom_ratio = new_zoom / self.zoom;
                            self.pan =
                                mouse_offset * (1.0 - zoom_ratio) + self.pan * zoom_ratio;
                        }
                        self.zoom = new_zoom;
                        let max_pan = egui::vec2(
                            size.x * (self.zoom - 1.0) / 2.0,
                            size.y * (self.zoom - 1.0) / 2.0,
                        );
                        self.pan.x = self.pan.x.clamp(-max_pan.x, max_pan.x);
                        self.pan.y = self.pan.y.clamp(-max_pan.y, max_pan.y);
                    }

                    // Compute UV for zoom/pan
                    let uv_w = 1.0 / self.zoom;
                    let uv_cx = 0.5 - self.pan.x / (size.x * self.zoom);
                    let uv_cy = 0.5 - self.pan.y / (size.y * self.zoom);
                    let uv = egui::Rect::from_center_size(
                        egui::pos2(uv_cx, uv_cy),
                        egui::vec2(uv_w, uv_w),
                    );

                    // Cell painting
                    if self.paint_mode
                        && (response.clicked() || response.dragged())
                    {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let fx =
                                (pos.x - image_rect.left()) / image_rect.width();
                            let fy =
                                (pos.y - image_rect.top()) / image_rect.height();

                            let u = uv.min.x + fx * (uv.max.x - uv.min.x);
                            let v = uv.min.y + fy * (uv.max.y - uv.min.y);

                            let cx = (u * self.ca.width as f32) as isize;
                            let cy = (v * self.ca.height as f32) as isize;

                            let half = self.brush_size as isize / 2;
                            for dy in -half..=half {
                                for dx in -half..=half {
                                    let gx = cx + dx;
                                    let gy = cy + dy;
                                    if gx >= 0
                                        && gx < self.ca.width as isize
                                        && gy >= 0
                                        && gy < self.ca.height as isize
                                    {
                                        self.ca.set_cell(
                                            gx as usize,
                                            gy as usize,
                                            self.paint_type as u8,
                                        );
                                    }
                                }
                            }
                            self.texture_dirty = true;
                            ctx.request_repaint();
                        }
                    }

                    // Draw texture
                    ui.painter().image(
                        texture.id(),
                        image_rect,
                        uv,
                        egui::Color32::WHITE,
                    );

                    // Zoom indicator
                    if self.zoom > 1.01 {
                        let zoom_label = format!("{:.1}\u{00d7}", self.zoom);
                        let label_pos =
                            image_rect.right_bottom() + egui::vec2(-48.0, -20.0);
                        ui.painter().text(
                            label_pos,
                            egui::Align2::CENTER_CENTER,
                            &zoom_label,
                            egui::FontId::proportional(13.0),
                            egui::Color32::from_white_alpha(180),
                        );
                    }

                    // Paint mode indicator
                    if self.paint_mode {
                        let label_pos =
                            image_rect.left_top() + egui::vec2(8.0, 8.0);
                        ui.painter().text(
                            label_pos,
                            egui::Align2::LEFT_TOP,
                            "PAINT",
                            egui::FontId::proportional(11.0),
                            egui::Color32::from_white_alpha(200),
                        );
                    }

                    // Recording indicator
                    if self.recording {
                        let label_pos =
                            image_rect.right_top() + egui::vec2(-8.0, 8.0);
                        ui.painter().text(
                            label_pos,
                            egui::Align2::RIGHT_TOP,
                            &format!("REC {}", self.recorded_frames.len()),
                            egui::FontId::proportional(11.0),
                            egui::Color32::from_rgb(255, 60, 60),
                        );
                    }

                    // Status bar
                    let status_rect = egui::Rect::from_min_size(
                        ui.min_rect().min
                            + egui::vec2(
                                0.0,
                                available_size.y - bottom_space + 12.0,
                            ),
                        egui::vec2(available_size.x, bottom_space - 12.0),
                    );
                    ui.allocate_new_ui(
                        egui::UiBuilder::new().max_rect(status_rect),
                        |ui| {
                            ui.centered_and_justified(|ui| {
                                if let Some((msg, _)) = &self.export_message {
                                    ui.label(
                                        egui::RichText::new(msg).small().weak(),
                                    );
                                } else if self.paint_mode {
                                    ui.label(
                                        egui::RichText::new(
                                            "Click/drag to paint \u{00b7} Scroll to zoom",
                                        )
                                        .small()
                                        .weak(),
                                    );
                                } else if self.zoom > 1.01 {
                                    ui.label(
                                        egui::RichText::new(
                                            "Scroll to zoom \u{00b7} Drag to pan \u{00b7} Reset View in Options",
                                        )
                                        .small()
                                        .weak(),
                                    );
                                }
                            });
                        },
                    );
                }
            });

        // ── Floating windows ─────────────────────────────────────────────────
        ui::render_about_window(self, ctx);
        ui::render_rules_window(self, ctx);
        ui::render_options_window(self, ctx);
        ui::render_presets_window(self, ctx);
        ui::render_histogram_window(self, ctx);
    }
}
