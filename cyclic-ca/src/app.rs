use crate::ca::{ColorScheme, CyclicCellularAutomata, Neighborhood, Pattern, Symmetry};
use crate::ui;
use eframe::egui;

// ── Preset ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
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

// ── App ───────────────────────────────────────────────────────────────────────

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

    pub options_open: bool,
    pub steps_per_frame: usize,
    pub step_counter: u64,
    pub show_step_counter: bool,
    pub symmetry: Symmetry,

    pub presets_open: bool,
    pub presets: Vec<Preset>,
    pub preset_name_input: String,

    // Status message (export / save / load feedback)
    pub status_message: Option<(String, f64)>,

    // Zoom / pan
    pub zoom: f32,
    pub pan: egui::Vec2,

    // Auto-stop
    pub auto_stop_enabled: bool,
    pub auto_stop_steps: u64,

    // GIF recording
    pub recording: bool,
    pub record_frame_target: usize,
    pub record_capture_every: usize,
    pub record_since_last: usize,
    pub record_frames: Vec<Vec<u8>>,

    // Custom palette editor
    pub palette_open: bool,
    pub custom_palette: [[u8; 3]; 6],
    pub palette_selected: usize,   // 0-5
    pub palette_hex_input: String,
}

/// The palette shown when the app first runs (and restored by "Reset to defaults").
pub const DEFAULT_PALETTE: [[u8; 3]; 6] = [
    [220,  50,  50],   // red
    [240, 140,   0],   // amber
    [200, 220,   0],   // yellow-green
    [  0, 180, 100],   // teal
    [  0, 100, 220],   // blue
    [150,   0, 200],   // violet
];

impl Default for CyclicCAApp {
    fn default() -> Self {
        let width = 200;
        let height = 200;
        let num_types = 12;
        let initial_hex = Self::rgb_to_hex(DEFAULT_PALETTE[0]);

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
            options_open: false,
            steps_per_frame: 1,
            step_counter: 0,
            show_step_counter: true,
            symmetry: Symmetry::None,
            presets_open: false,
            presets: Vec::new(),
            preset_name_input: String::new(),
            status_message: None,
            zoom: 1.0,
            pan: egui::Vec2::ZERO,
            auto_stop_enabled: false,
            auto_stop_steps: 500,
            recording: false,
            record_frame_target: 60,
            record_capture_every: 1,
            record_since_last: 0,
            record_frames: Vec::new(),
            palette_open: false,
            custom_palette: DEFAULT_PALETTE,
            palette_selected: 0,
            palette_hex_input: initial_hex,
        }
    }
}

impl CyclicCAApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::theme::load_fonts(&cc.egui_ctx);
        Self::default()
    }

    // ── Colour math helpers (static) ──────────────────────────────────────────

    /// h: 0–360, s/v: 0–1  →  [r, g, b] each 0–255
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
        let c = v * s;
        let h6 = h / 60.0;
        let x = c * (1.0 - ((h6 % 2.0) - 1.0).abs());
        let m = v - c;
        let (r, g, b) = match h6 as u32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        [
            ((r + m) * 255.0).round().clamp(0.0, 255.0) as u8,
            ((g + m) * 255.0).round().clamp(0.0, 255.0) as u8,
            ((b + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ]
    }

    /// [r, g, b] each 0–255  →  [h 0–360, s 0–1, v 0–1]
    pub fn rgb_to_hsv(rgb: [u8; 3]) -> [f32; 3] {
        let r = rgb[0] as f32 / 255.0;
        let g = rgb[1] as f32 / 255.0;
        let b = rgb[2] as f32 / 255.0;
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;
        let v = max;
        let s = if max > 1e-6 { delta / max } else { 0.0 };
        let h = if delta < 1e-6 {
            0.0
        } else if (max - r).abs() < 1e-6 {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if (max - g).abs() < 1e-6 {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        [h, s, v]
    }

    pub fn rgb_to_hex(rgb: [u8; 3]) -> String {
        format!("{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
    }

    pub fn parse_hex(s: &str) -> Option<[u8; 3]> {
        let s = s.trim().trim_start_matches('#');
        if s.len() != 6 { return None; }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some([r, g, b])
    }

    // ── Custom palette operations ─────────────────────────────────────────────

    /// Push custom_palette into the CA colours.
    pub fn apply_custom_palette(&mut self) {
        self.ca.set_custom_colors(&self.custom_palette);
    }

    /// Select a palette slot and sync the hex field.
    pub fn select_palette_slot(&mut self, slot: usize) {
        self.palette_selected = slot.min(5);
        self.palette_hex_input = Self::rgb_to_hex(self.custom_palette[self.palette_selected]);
    }

    /// Called by the UI whenever an RGB slider changes — syncs hex and re-applies to CA.
    pub fn update_palette_from_rgb(&mut self, rgb: [u8; 3]) {
        self.custom_palette[self.palette_selected] = rgb;
        self.palette_hex_input = Self::rgb_to_hex(rgb);
        if self.selected_color_scheme == ColorScheme::Custom {
            self.apply_custom_palette();
        }
    }

    /// Parse the hex field and apply it to the selected slot.
    pub fn apply_hex_input(&mut self) {
        if let Some(rgb) = Self::parse_hex(&self.palette_hex_input) {
            self.custom_palette[self.palette_selected] = rgb;
            self.palette_hex_input = Self::rgb_to_hex(rgb); // normalise case
            if self.selected_color_scheme == ColorScheme::Custom {
                self.apply_custom_palette();
            }
        }
    }

    /// Restore the six slots to the original startup colours.
    pub fn reset_palette(&mut self) {
        self.custom_palette = DEFAULT_PALETTE;
        self.palette_hex_input = Self::rgb_to_hex(DEFAULT_PALETTE[self.palette_selected]);
        if self.selected_color_scheme == ColorScheme::Custom {
            self.apply_custom_palette();
        }
    }

    /// Fill slots 1-4 by interpolating (in HSV, shortest-path hue) between slot 0 and slot 5.
    pub fn interpolate_palette(&mut self) {
        let s0 = Self::rgb_to_hsv(self.custom_palette[0]);
        let s5 = Self::rgb_to_hsv(self.custom_palette[5]);
        for i in 1usize..5 {
            let t = i as f32 / 5.0;
            // Shortest angular path for hue
            let mut dh = s5[0] - s0[0];
            if dh > 180.0  { dh -= 360.0; }
            if dh < -180.0 { dh += 360.0; }
            let h = (s0[0] + dh * t).rem_euclid(360.0);
            let s = s0[1] + (s5[1] - s0[1]) * t;
            let v = s0[2] + (s5[2] - s0[2]) * t;
            self.custom_palette[i] = Self::hsv_to_rgb(h, s, v);
        }
        // Refresh hex field for the currently selected slot
        self.palette_hex_input = Self::rgb_to_hex(self.custom_palette[self.palette_selected]);
        if self.selected_color_scheme == ColorScheme::Custom {
            self.apply_custom_palette();
        }
    }

    /// Switch to Custom scheme, clamping num_types to ≤ 6.
    pub fn activate_custom_scheme(&mut self) {
        self.selected_color_scheme = ColorScheme::Custom;
        if self.ca.num_types > 6 {
            let (w, h) = (self.ca.width, self.ca.height);
            self.ca.resize(w, h, 6);
            self.pending_types = 6;
        }
        self.apply_custom_palette();
    }

    /// Resize the CA and re-apply the colour scheme (handles Custom correctly).
    pub fn apply_size_change(&mut self) {
        self.ca.resize(self.pending_width, self.pending_height, self.pending_types);
        if self.selected_color_scheme == ColorScheme::Custom {
            self.apply_custom_palette();
        } else {
            self.ca.set_color_scheme(self.selected_color_scheme);
        }
        self.step_counter = 0;
    }

    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan = egui::Vec2::ZERO;
    }

    fn set_status(&mut self, msg: impl Into<String>, now: f64) {
        self.status_message = Some((msg.into(), now + 4.0));
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        let image = self.ca.to_color_image();
        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.texture = Some(ctx.load_texture("ca_grid", image, egui::TextureOptions::NEAREST));
        }
    }

    // ── Save / Load ───────────────────────────────────────────────────────────

    pub fn save_state(&mut self, now: f64) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&home).join("Desktop").join("CyclicCA_state.ccas");

        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(b"CCAS");          // magic
        data.push(1u8);                            // version
        data.extend_from_slice(&(self.ca.width as u32).to_le_bytes());
        data.extend_from_slice(&(self.ca.height as u32).to_le_bytes());
        data.extend_from_slice(&(self.ca.num_types as u32).to_le_bytes());
        data.push(self.ca.color_scheme.as_u8());
        data.push(self.ca.neighborhood.as_u8());
        data.extend_from_slice(&(self.ca.threshold as u32).to_le_bytes());
        data.push(self.symmetry.as_u8());
        data.extend_from_slice(&self.step_counter.to_le_bytes());
        data.extend_from_slice(&self.speed.to_le_bytes());
        for row in &self.ca.grid {
            for &cell in row {
                data.push(cell as u8);
            }
        }

        match std::fs::write(&path, &data) {
            Ok(_) => self.set_status("State saved: CyclicCA_state.ccas", now),
            Err(e) => self.set_status(format!("Save failed: {}", e), now),
        }
    }

    pub fn load_state(&mut self, now: f64) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&home).join("Desktop").join("CyclicCA_state.ccas");

        let data = match std::fs::read(&path) {
            Ok(d) => d,
            Err(e) => { self.set_status(format!("Load failed: {}", e), now); return; }
        };

        if data.len() < 9 || &data[0..4] != b"CCAS" || data[4] != 1 {
            self.set_status("Invalid or unsupported save file", now);
            return;
        }

        let mut p = 5usize;
        let read_u32 = |data: &[u8], p: &mut usize| -> usize {
            let v = u32::from_le_bytes(data[*p..*p+4].try_into().unwrap_or([0;4])) as usize;
            *p += 4; v
        };
        let read_f32 = |data: &[u8], p: &mut usize| -> f32 {
            let v = f32::from_le_bytes(data[*p..*p+4].try_into().unwrap_or([0;4]));
            *p += 4; v
        };
        let read_u64 = |data: &[u8], p: &mut usize| -> u64 {
            let v = u64::from_le_bytes(data[*p..*p+8].try_into().unwrap_or([0;8]));
            *p += 8; v
        };

        let width      = read_u32(&data, &mut p);
        let height     = read_u32(&data, &mut p);
        let num_types  = read_u32(&data, &mut p);
        let scheme     = ColorScheme::from_u8(data[p]);  p += 1;
        let nb         = Neighborhood::from_u8(data[p]); p += 1;
        let threshold  = read_u32(&data, &mut p);
        let sym        = Symmetry::from_u8(data[p]);     p += 1;
        let steps      = read_u64(&data, &mut p);
        let speed      = read_f32(&data, &mut p);

        let expected = p + width * height;
        if data.len() < expected || width == 0 || height == 0 || num_types == 0 {
            self.set_status("Save file is corrupt or incomplete", now);
            return;
        }

        self.ca.resize(width, height, num_types);
        self.ca.set_color_scheme(scheme);
        self.ca.neighborhood = nb;
        self.ca.threshold = threshold;
        self.symmetry = sym;
        self.step_counter = steps;
        self.speed = speed;
        self.selected_color_scheme = scheme;
        self.pending_width = width;
        self.pending_height = height;
        self.pending_types = num_types;

        for y in 0..height {
            for x in 0..width {
                self.ca.grid[y][x] = (data[p] as usize).min(num_types - 1);
                p += 1;
            }
        }

        self.reset_view();
        self.set_status("State loaded: CyclicCA_state.ccas", now);
    }

    // ── Export PNG ────────────────────────────────────────────────────────────

    pub fn export_png(&mut self, now: f64) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let filename = format!("CyclicCA_{}x{}_t{}.png", self.ca.width, self.ca.height, self.step_counter);
        let path = std::path::Path::new(&home).join("Desktop").join(&filename);
        let bytes = self.ca.to_rgb_bytes();
        match image::save_buffer(&path, &bytes, self.ca.width as u32, self.ca.height as u32, image::ColorType::Rgb8) {
            Ok(_) => self.set_status(format!("Saved: {}", filename), now),
            Err(e) => self.set_status(format!("Export failed: {}", e), now),
        }
    }

    // ── GIF recording ─────────────────────────────────────────────────────────

    pub fn start_recording(&mut self) {
        self.record_frames.clear();
        self.record_since_last = 0;
        self.recording = true;
    }

    pub fn finish_recording(&mut self, now: f64) {
        self.recording = false;
        if self.record_frames.is_empty() {
            self.set_status("No frames recorded", now);
            return;
        }

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let filename = format!("CyclicCA_{}x{}_t{}.gif", self.ca.width, self.ca.height, self.step_counter);
        let path = std::path::Path::new(&home).join("Desktop").join(&filename);

        let w = self.ca.width as u32;
        let h = self.ca.height as u32;
        let fps = (self.speed / self.record_capture_every as f32).max(1.0);
        let delay = image::Delay::from_saturating_duration(
            std::time::Duration::from_millis((1000.0 / fps) as u64)
        );

        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let file = std::fs::File::create(&path)?;
            let mut encoder = image::codecs::gif::GifEncoder::new(file);
            encoder.set_repeat(image::codecs::gif::Repeat::Infinite)?;
            for rgb in &self.record_frames {
                let rgba: Vec<u8> = rgb.chunks(3)
                    .flat_map(|c| [c[0], c[1], c[2], 255u8])
                    .collect();
                let img = image::RgbaImage::from_raw(w, h, rgba)
                    .ok_or("Failed to create frame")?;
                encoder.encode_frame(image::Frame::from_parts(img, 0, 0, delay))?;
            }
            Ok(())
        })();

        let n = self.record_frames.len();
        self.record_frames.clear();
        match result {
            Ok(_) => self.set_status(format!("GIF saved: {} ({} frames)", filename, n), now),
            Err(e) => self.set_status(format!("GIF failed: {}", e), now),
        }
    }

    // ── Presets ───────────────────────────────────────────────────────────────

    pub fn save_preset(&mut self) {
        let name = self.preset_name_input.trim().to_string();
        if name.is_empty() { return; }
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
            self.reset_view();
        }
    }
}

// ── eframe App ────────────────────────────────────────────────────────────────

impl eframe::App for CyclicCAApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::theme::apply_visuals(ctx);
        let now = ctx.input(|i| i.time);

        // Simulation tick
        if self.running {
            let interval = 1.0 / self.speed as f64;
            if now - self.last_update >= interval {
                for _ in 0..self.steps_per_frame {
                    self.ca.update();
                    self.ca.apply_symmetry(self.symmetry);
                    self.step_counter += 1;

                    // GIF frame capture
                    if self.recording {
                        self.record_since_last += 1;
                        if self.record_since_last >= self.record_capture_every {
                            self.record_frames.push(self.ca.to_rgb_bytes());
                            self.record_since_last = 0;
                        }
                        if self.record_frames.len() >= self.record_frame_target {
                            let n = now;
                            self.finish_recording(n);
                        }
                    }

                    // Auto-stop
                    if self.auto_stop_enabled && self.step_counter >= self.auto_stop_steps {
                        self.running = false;
                        break;
                    }
                }
                self.last_update = now;
            }
            ctx.request_repaint();
        }

        // Expire status message
        if let Some((_, expiry)) = &self.status_message {
            if now > *expiry { self.status_message = None; }
            else { ctx.request_repaint(); }
        }

        self.update_texture(ctx);

        // ── Sidebar ───────────────────────────────────────────────────────────
        let sidebar_frame = egui::Frame {
            fill: crate::theme::SIDEBAR_BG,
            inner_margin: egui::Margin::symmetric(12.0, 8.0),
            ..Default::default()
        };
        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(220.0)
            .frame(sidebar_frame)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.heading("Cyclic CA");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let ol = if self.options_open { "Options ▲" } else { "Options ▼" };
                        if ui.button(ol).clicked() { self.options_open = !self.options_open; }
                        let pl = if self.presets_open { "Presets ▲" } else { "Presets ▼" };
                        if ui.button(pl).clicked() { self.presets_open = !self.presets_open; }
                    });
                });
                ui.add_space(2.0);
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

        // ── Central panel ─────────────────────────────────────────────────────
        let content_frame = egui::Frame { fill: crate::theme::CONTENT_BG, ..Default::default() };
        egui::CentralPanel::default().frame(content_frame).show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let available_size = ui.available_size();
                let bottom_space = 48.0;
                let usable = egui::vec2(available_size.x, available_size.y - bottom_space);
                let aspect_ratio = self.ca.width as f32 / self.ca.height as f32;
                let (display_width, display_height) =
                    if usable.x / usable.y > aspect_ratio { let h = usable.y; (h * aspect_ratio, h) }
                    else { let w = usable.x; (w, w / aspect_ratio) };

                let size = egui::vec2(display_width, display_height);
                let offset = egui::vec2(
                    (available_size.x - display_width) / 2.0,
                    (usable.y - display_height) / 2.0,
                );
                let image_rect = egui::Rect::from_min_size(ui.min_rect().min + offset, size);

                // Shadow & border
                let painter = ui.painter();
                for i in 1..=6u8 {
                    let spread = i as f32 * 2.0;
                    let alpha = 25u8.saturating_sub(i * 3);
                    painter.rect_filled(
                        image_rect.translate(egui::vec2(spread, spread)).expand(spread * 0.5),
                        2.0, egui::Color32::from_black_alpha(alpha),
                    );
                }
                painter.rect_stroke(image_rect, 0.0, egui::Stroke::new(1.5, egui::Color32::from_gray(80)));

                // Zoom / pan
                let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());
                if response.dragged() {
                    self.pan += response.drag_delta();
                    let max_pan = egui::vec2(size.x * (self.zoom - 1.0) / 2.0, size.y * (self.zoom - 1.0) / 2.0);
                    self.pan.x = self.pan.x.clamp(-max_pan.x, max_pan.x);
                    self.pan.y = self.pan.y.clamp(-max_pan.y, max_pan.y);
                }
                let scroll_delta = ui.input(|i| {
                    if i.pointer.hover_pos().map_or(false, |p| image_rect.contains(p)) { i.raw_scroll_delta.y } else { 0.0 }
                });
                if scroll_delta != 0.0 {
                    let factor: f32 = if scroll_delta > 0.0 { 1.12 } else { 1.0 / 1.12 };
                    let new_zoom = (self.zoom * factor).clamp(1.0, 12.0);
                    if let Some(mp) = ui.input(|i| i.pointer.hover_pos()) {
                        let mo = mp - image_rect.center();
                        let zr = new_zoom / self.zoom;
                        self.pan = mo * (1.0 - zr) + self.pan * zr;
                    }
                    self.zoom = new_zoom;
                    let max_pan = egui::vec2(size.x * (self.zoom - 1.0) / 2.0, size.y * (self.zoom - 1.0) / 2.0);
                    self.pan.x = self.pan.x.clamp(-max_pan.x, max_pan.x);
                    self.pan.y = self.pan.y.clamp(-max_pan.y, max_pan.y);
                }

                // Draw with UV for zoom/pan
                let uv_w = 1.0 / self.zoom;
                let uv = egui::Rect::from_center_size(
                    egui::pos2(0.5 - self.pan.x / (size.x * self.zoom), 0.5 - self.pan.y / (size.y * self.zoom)),
                    egui::vec2(uv_w, uv_w),
                );
                ui.painter().image(texture.id(), image_rect, uv, egui::Color32::WHITE);

                // Zoom indicator
                if self.zoom > 1.01 {
                    ui.painter().text(
                        image_rect.right_bottom() + egui::vec2(-48.0, -20.0),
                        egui::Align2::CENTER_CENTER,
                        format!("{:.1}×", self.zoom),
                        egui::FontId::proportional(13.0),
                        egui::Color32::from_white_alpha(180),
                    );
                }

                // Status bar
                let status_rect = egui::Rect::from_min_size(
                    ui.min_rect().min + egui::vec2(0.0, available_size.y - bottom_space + 12.0),
                    egui::vec2(available_size.x, bottom_space - 12.0),
                );
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(status_rect), |ui| {
                    ui.centered_and_justified(|ui| {
                        let msg = if let Some((m, _)) = &self.status_message {
                            m.clone()
                        } else if self.recording {
                            format!("⏺ Recording: {}/{} frames", self.record_frames.len(), self.record_frame_target)
                        } else if self.zoom > 1.01 {
                            "Scroll to zoom · Drag to pan · Reset View in Options".to_string()
                        } else {
                            String::new()
                        };
                        if !msg.is_empty() {
                            ui.label(egui::RichText::new(&msg).small().weak());
                        }
                    });
                });
            }
        });

        ui::render_options_window(self, ctx);
        ui::render_presets_window(self, ctx);
        ui::render_palette_window(self, ctx);
    }
}
