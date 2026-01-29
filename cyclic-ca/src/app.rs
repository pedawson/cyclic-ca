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
        }
    }
}

impl CyclicCAApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
        // Update simulation if running, respecting speed setting
        if self.running {
            let now = ctx.input(|i| i.time);
            let interval = 1.0 / self.speed as f64;

            if now - self.last_update >= interval {
                self.ca.update();
                self.last_update = now;
            }
            ctx.request_repaint();
        }

        // Update texture
        self.update_texture(ctx);

        // Left side panel with controls
        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Cyclic CA");
                ui.separator();

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

        // Central panel with CA visualization
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let available_size = ui.available_size();
                let aspect_ratio = self.ca.width as f32 / self.ca.height as f32;

                let (display_width, display_height) = if available_size.x / available_size.y > aspect_ratio {
                    let h = available_size.y;
                    (h * aspect_ratio, h)
                } else {
                    let w = available_size.x;
                    (w, w / aspect_ratio)
                };

                let size = egui::vec2(display_width, display_height);
                let offset = egui::vec2(
                    (available_size.x - display_width) / 2.0,
                    (available_size.y - display_height) / 2.0,
                );

                ui.allocate_new_ui(
                    egui::UiBuilder::new().max_rect(
                        egui::Rect::from_min_size(ui.min_rect().min + offset, size)
                    ),
                    |ui| {
                        ui.image(egui::load::SizedTexture::new(texture.id(), size));
                    },
                );
            }
        });
    }
}
