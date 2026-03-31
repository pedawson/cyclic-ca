use crate::ca::{ColorScheme, CyclicCellularAutomata, Pattern};
use crate::ui;
use eframe::egui;

pub struct CyclicCAApp {
    pub ca: CyclicCellularAutomata,
    pub running: bool,
    pub texture: Option<egui::TextureHandle>,

    // Pending values for grid settings (applied on "Apply" button)
    pub pending_width: usize,
    pub pending_height: usize,
    pub pending_types: usize,

    // Selected pattern and color scheme
    pub selected_pattern: Pattern,
    pub selected_color_scheme: ColorScheme,

    // Speed control (updates per second)
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
}

impl eframe::App for CyclicCAApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Re-apply every frame so eframe can't override with the system theme
        crate::theme::apply_visuals(ctx);

        // Update simulation if running, respecting speed setting
        if self.running {
            let now = ctx.input(|i| i.time);
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

        // Update texture
        self.update_texture(ctx);

        // Sidebar panel — macOS blue-gray background
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
                        let label = if self.options_open { "Options ▲" } else { "Options ▼" };
                        if ui.button(label).clicked() {
                            self.options_open = !self.options_open;
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

        // Central panel — pure white
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
                }
            });

        // Options window (floats on top)
        ui::render_options_window(self, ctx);
    }
}
