use crate::ca::{ColorScheme, CyclicCellularAutomata, Neighborhood, Pattern};
use crate::ui;
use eframe::egui;

// ── Preset ───────────────────────────────────────────────────────────────────

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
}

// ── App ───────────────────────────────────────────────────────────────────────

pub struct CyclicCAApp {
    pub ca: CyclicCellularAutomata,
    pub running: bool,
    pub texture: Option<egui::TextureHandle>,

    // Pending grid settings (applied on "Apply")
    pub pending_width: usize,
    pub pending_height: usize,
    pub pending_types: usize,

    // Selected pattern and color scheme
    pub selected_pattern: Pattern,
    pub selected_color_scheme: ColorScheme,

    // Speed control
    pub speed: f32,
    pub last_update: f64,

    // Panel expansion state
    pub grid_panel_open: bool,
    pub visual_panel_open: bool,
    pub patterns_panel_open: bool,
    pub simulation_panel_open: bool,

    // Options window
    pub options_open: bool,
    pub steps_per_frame: usize,
    pub step_counter: u64,
    pub show_step_counter: bool,

    // Presets window
    pub presets_open: bool,
    pub presets: Vec<Preset>,
    pub preset_name_input: String,

    // Export feedback (message + expiry time)
    pub export_message: Option<(String, f64)>,
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
            options_open: false,
            steps_per_frame: 1,
            step_counter: 0,
            show_step_counter: true,
            presets_open: false,
            presets: Vec::new(),
            preset_name_input: String::new(),
            export_message: None,
        }
    }
}

impl CyclicCAApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::theme::load_fonts(&cc.egui_ctx);
        Self::default()
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        let image = self.ca.to_color_image();
        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.texture = Some(ctx.load_texture("ca_grid", image, egui::TextureOptions::NEAREST));
        }
    }

    pub fn export_png(&mut self, now: f64) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let desktop = std::path::Path::new(&home).join("Desktop");
        let filename = format!(
            "CyclicCA_{}x{}_t{}.png",
            self.ca.width,
            self.ca.height,
            self.step_counter
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

    pub fn save_preset(&mut self) {
        let name = self.preset_name_input.trim().to_string();
        if name.is_empty() {
            return;
        }
        // Replace existing preset with same name
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
            self.selected_color_scheme = p.color_scheme;
            self.pending_width = p.width;
            self.pending_height = p.height;
            self.pending_types = p.num_types;
            self.step_counter = 0;
        }
    }
}

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
                    self.step_counter += 1;
                }
                self.last_update = now;
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

        self.update_texture(ctx);

        // ── Sidebar ──────────────────────────────────────────────────────────
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
                        let opt_label = if self.options_open { "Options ▲" } else { "Options ▼" };
                        if ui.button(opt_label).clicked() {
                            self.options_open = !self.options_open;
                        }
                        let pre_label = if self.presets_open { "Presets ▲" } else { "Presets ▼" };
                        if ui.button(pre_label).clicked() {
                            self.presets_open = !self.presets_open;
                        }
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
        let content_frame = egui::Frame {
            fill: crate::theme::CONTENT_BG,
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(content_frame)
            .show(ctx, |ui| {
                if let Some(texture) = &self.texture {
                    let available_size = ui.available_size();
                    let bottom_space = 48.0;
                    let usable = egui::vec2(available_size.x, available_size.y - bottom_space);
                    let aspect_ratio = self.ca.width as f32 / self.ca.height as f32;

                    let (display_width, display_height) =
                        if usable.x / usable.y > aspect_ratio {
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
                    let image_rect = egui::Rect::from_min_size(
                        ui.min_rect().min + offset,
                        size,
                    );

                    // Shadow
                    let painter = ui.painter();
                    for i in 1..=6u8 {
                        let spread = i as f32 * 2.0;
                        let alpha = 25u8.saturating_sub(i * 3);
                        painter.rect_filled(
                            image_rect.translate(egui::vec2(spread, spread)).expand(spread * 0.5),
                            2.0,
                            egui::Color32::from_black_alpha(alpha),
                        );
                    }
                    // Border
                    painter.rect_stroke(
                        image_rect,
                        0.0,
                        egui::Stroke::new(1.5, egui::Color32::from_gray(80)),
                    );

                    ui.allocate_new_ui(
                        egui::UiBuilder::new().max_rect(image_rect),
                        |ui| {
                            ui.image(egui::load::SizedTexture::new(texture.id(), size));
                        },
                    );

                    // Export status message in bottom whitespace
                    if let Some((msg, _)) = &self.export_message {
                        let msg_rect = egui::Rect::from_min_size(
                            ui.min_rect().min + egui::vec2(0.0, available_size.y - bottom_space + 12.0),
                            egui::vec2(available_size.x, bottom_space - 12.0),
                        );
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(msg_rect), |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label(egui::RichText::new(msg).small().weak());
                            });
                        });
                    }
                }
            });

        // ── Floating windows ──────────────────────────────────────────────────
        ui::render_options_window(self, ctx);
        ui::render_presets_window(self, ctx);
    }
}
