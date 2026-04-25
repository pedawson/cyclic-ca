use eframe::egui;

pub fn load_fonts(ctx: &egui::Context) {
    let font_paths = [
        "/System/Library/Fonts/HelveticaNeue.ttc",
        "/System/Library/Fonts/Helvetica.ttc",
        "/Library/Fonts/Arial.ttf",
    ];

    for path in font_paths {
        if let Ok(bytes) = std::fs::read(path) {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "system_font".to_owned(),
                egui::FontData {
                    font: std::borrow::Cow::Owned(bytes),
                    index: 0,
                    tweak: Default::default(),
                },
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "system_font".to_owned());
            ctx.set_fonts(fonts);
            break;
        }
    }
}

pub fn sidebar_bg(dark: bool) -> egui::Color32 {
    if dark {
        egui::Color32::from_rgb(38, 38, 42)
    } else {
        egui::Color32::from_rgb(236, 236, 246)
    }
}

pub fn content_bg(dark: bool) -> egui::Color32 {
    if dark {
        egui::Color32::from_rgb(28, 28, 30)
    } else {
        egui::Color32::WHITE
    }
}

pub fn apply_visuals(ctx: &egui::Context, dark_mode: bool) {
    let accent = egui::Color32::from_rgb(0, 122, 255);
    let rounding = egui::Rounding::same(6.0);

    if dark_mode {
        let separator = egui::Color32::from_rgb(68, 68, 72);
        let text = egui::Color32::from_rgb(230, 230, 235);
        let btn_bg = egui::Color32::from_rgb(50, 50, 55);
        let btn_hover = egui::Color32::from_rgb(60, 60, 65);
        let btn_press = egui::Color32::from_rgb(70, 70, 75);

        let mut v = egui::Visuals::dark();

        v.window_rounding = egui::Rounding::same(10.0);
        v.window_fill = egui::Color32::from_rgb(44, 44, 48);
        v.window_stroke = egui::Stroke::new(0.5, separator);
        v.window_shadow = egui::Shadow {
            offset: egui::vec2(0.0, 8.0),
            blur: 24.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(64),
        };
        v.menu_rounding = egui::Rounding::same(8.0);
        v.popup_shadow = egui::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 12.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(40),
        };

        v.panel_fill = egui::Color32::from_rgb(38, 38, 42);

        v.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(0, 122, 255, 80);
        v.selection.stroke = egui::Stroke::new(1.0, accent);
        v.hyperlink_color = accent;

        v.extreme_bg_color = egui::Color32::from_rgb(20, 20, 22);
        v.faint_bg_color = egui::Color32::from_rgb(35, 35, 38);
        v.slider_trailing_fill = true;
        v.indent_has_left_vline = false;
        v.collapsing_header_frame = false;

        v.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
        v.widgets.noninteractive.weak_bg_fill = egui::Color32::TRANSPARENT;
        v.widgets.noninteractive.bg_stroke = egui::Stroke::new(0.5, separator);
        v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.noninteractive.rounding = rounding;

        v.widgets.inactive.bg_fill = btn_bg;
        v.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(45, 45, 50);
        v.widgets.inactive.bg_stroke = egui::Stroke::new(0.8, separator);
        v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.inactive.rounding = rounding;

        v.widgets.hovered.bg_fill = btn_hover;
        v.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(55, 55, 60);
        v.widgets.hovered.bg_stroke =
            egui::Stroke::new(0.8, egui::Color32::from_rgb(85, 85, 90));
        v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.hovered.rounding = rounding;
        v.widgets.hovered.expansion = 0.0;

        v.widgets.active.bg_fill = accent;
        v.widgets.active.weak_bg_fill = accent;
        v.widgets.active.bg_stroke = egui::Stroke::NONE;
        v.widgets.active.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);
        v.widgets.active.rounding = rounding;
        v.widgets.active.expansion = 0.0;

        v.widgets.open.bg_fill = btn_press;
        v.widgets.open.weak_bg_fill = egui::Color32::from_rgb(65, 65, 70);
        v.widgets.open.bg_stroke = egui::Stroke::new(0.5, separator);
        v.widgets.open.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.open.rounding = rounding;

        ctx.set_visuals(v);
    } else {
        let separator = egui::Color32::from_rgb(198, 198, 200);
        let text = egui::Color32::from_rgb(20, 20, 20);
        let btn_bg = egui::Color32::WHITE;
        let btn_hover = egui::Color32::from_rgb(242, 242, 247);
        let btn_press = egui::Color32::from_rgb(228, 228, 235);

        let mut v = egui::Visuals::light();

        v.window_rounding = egui::Rounding::same(10.0);
        v.window_fill = egui::Color32::WHITE;
        v.window_stroke = egui::Stroke::new(0.5, separator);
        v.window_shadow = egui::Shadow {
            offset: egui::vec2(0.0, 8.0),
            blur: 24.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(32),
        };
        v.menu_rounding = egui::Rounding::same(8.0);
        v.popup_shadow = egui::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 12.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(20),
        };

        v.panel_fill = egui::Color32::from_rgb(236, 236, 246);

        v.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(0, 122, 255, 50);
        v.selection.stroke = egui::Stroke::new(1.0, accent);
        v.hyperlink_color = accent;

        v.extreme_bg_color = egui::Color32::WHITE;
        v.faint_bg_color = egui::Color32::from_rgb(250, 250, 252);
        v.slider_trailing_fill = true;
        v.indent_has_left_vline = false;
        v.collapsing_header_frame = false;

        v.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
        v.widgets.noninteractive.weak_bg_fill = egui::Color32::TRANSPARENT;
        v.widgets.noninteractive.bg_stroke = egui::Stroke::new(0.5, separator);
        v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.noninteractive.rounding = rounding;

        v.widgets.inactive.bg_fill = btn_bg;
        v.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(244, 244, 248);
        v.widgets.inactive.bg_stroke = egui::Stroke::new(0.8, separator);
        v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.inactive.rounding = rounding;

        v.widgets.hovered.bg_fill = btn_hover;
        v.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(235, 235, 242);
        v.widgets.hovered.bg_stroke =
            egui::Stroke::new(0.8, egui::Color32::from_rgb(170, 170, 180));
        v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.hovered.rounding = rounding;
        v.widgets.hovered.expansion = 0.0;

        v.widgets.active.bg_fill = accent;
        v.widgets.active.weak_bg_fill = accent;
        v.widgets.active.bg_stroke = egui::Stroke::NONE;
        v.widgets.active.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);
        v.widgets.active.rounding = rounding;
        v.widgets.active.expansion = 0.0;

        v.widgets.open.bg_fill = btn_press;
        v.widgets.open.weak_bg_fill = egui::Color32::from_rgb(220, 220, 228);
        v.widgets.open.bg_stroke = egui::Stroke::new(0.5, separator);
        v.widgets.open.fg_stroke = egui::Stroke::new(1.0, text);
        v.widgets.open.rounding = rounding;

        ctx.set_visuals(v);
    }

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(6.0, 5.0);
    style.spacing.button_padding = egui::vec2(12.0, 5.0);
    style.spacing.indent = 16.0;
    style.spacing.interact_size = egui::vec2(40.0, 22.0);
    style.spacing.slider_width = 130.0;
    style.spacing.combo_width = 130.0;

    use egui::{FontFamily, FontId, TextStyle};
    style.text_styles = [
        (TextStyle::Small, FontId::new(11.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(13.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(13.0, FontFamily::Proportional)),
        (TextStyle::Heading, FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, FontFamily::Monospace)),
    ]
    .into();

    ctx.set_style(style);
}
