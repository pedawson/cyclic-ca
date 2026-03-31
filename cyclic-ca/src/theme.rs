use eframe::egui;

/// Load system fonts once at startup.
pub fn load_fonts(ctx: &egui::Context) {
    // Try macOS Helvetica Neue — a clean, Apple-feel proportional font
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

/// Apply macOS-inspired visuals. Call every frame to prevent eframe
/// from overriding with the system dark/light theme.
pub fn apply_visuals(ctx: &egui::Context) {
    let accent    = egui::Color32::from_rgb(0, 122, 255);   // macOS Blue
    let separator = egui::Color32::from_rgb(198, 198, 200);
    let text      = egui::Color32::from_rgb(20,  20,  20);
    let rounding  = egui::Rounding::same(6.0);

    // Button look: white fill with a thin border (classic macOS)
    let btn_bg    = egui::Color32::WHITE;
    let btn_hover = egui::Color32::from_rgb(242, 242, 247);
    let btn_press = egui::Color32::from_rgb(228, 228, 235);

    let mut v = egui::Visuals::light();

    // ── Window ───────────────────────────────────────────────────────────
    v.window_rounding = egui::Rounding::same(10.0);
    v.window_fill     = egui::Color32::WHITE;
    v.window_stroke   = egui::Stroke::new(0.5, separator);
    v.window_shadow   = egui::Shadow {
        offset: egui::vec2(0.0, 8.0),
        blur:   24.0,
        spread: 0.0,
        color:  egui::Color32::from_black_alpha(32),
    };
    v.menu_rounding = egui::Rounding::same(8.0);
    v.popup_shadow  = egui::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur:   12.0,
        spread: 0.0,
        color:  egui::Color32::from_black_alpha(20),
    };

    // ── Panel default (overridden per-panel in app.rs) ───────────────────
    v.panel_fill = egui::Color32::from_rgb(236, 236, 246);

    // ── Selection & links ────────────────────────────────────────────────
    v.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(0, 122, 255, 50);
    v.selection.stroke  = egui::Stroke::new(1.0, accent);
    v.hyperlink_color   = accent;

    // ── Misc ─────────────────────────────────────────────────────────────
    v.extreme_bg_color       = egui::Color32::WHITE;
    v.faint_bg_color         = egui::Color32::from_rgb(250, 250, 252);
    v.slider_trailing_fill   = true;
    v.indent_has_left_vline  = false;
    v.collapsing_header_frame = false;

    // ── Widget states ────────────────────────────────────────────────────
    // noninteractive
    v.widgets.noninteractive.bg_fill      = egui::Color32::TRANSPARENT;
    v.widgets.noninteractive.weak_bg_fill = egui::Color32::TRANSPARENT;
    v.widgets.noninteractive.bg_stroke    = egui::Stroke::new(0.5, separator);
    v.widgets.noninteractive.fg_stroke    = egui::Stroke::new(1.0, text);
    v.widgets.noninteractive.rounding     = rounding;

    // inactive (idle)
    v.widgets.inactive.bg_fill      = btn_bg;
    v.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(244, 244, 248);
    v.widgets.inactive.bg_stroke    = egui::Stroke::new(0.8, separator);
    v.widgets.inactive.fg_stroke    = egui::Stroke::new(1.0, text);
    v.widgets.inactive.rounding     = rounding;

    // hovered
    v.widgets.hovered.bg_fill      = btn_hover;
    v.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(235, 235, 242);
    v.widgets.hovered.bg_stroke    = egui::Stroke::new(0.8, egui::Color32::from_rgb(170, 170, 180));
    v.widgets.hovered.fg_stroke    = egui::Stroke::new(1.0, text);
    v.widgets.hovered.rounding     = rounding;
    v.widgets.hovered.expansion    = 0.0;

    // active / selected  →  accent blue
    v.widgets.active.bg_fill      = accent;
    v.widgets.active.weak_bg_fill = accent;
    v.widgets.active.bg_stroke    = egui::Stroke::NONE;
    v.widgets.active.fg_stroke    = egui::Stroke::new(1.5, egui::Color32::WHITE);
    v.widgets.active.rounding     = rounding;
    v.widgets.active.expansion    = 0.0;

    // open (combo boxes, menus)
    v.widgets.open.bg_fill      = btn_press;
    v.widgets.open.weak_bg_fill = egui::Color32::from_rgb(220, 220, 228);
    v.widgets.open.bg_stroke    = egui::Stroke::new(0.5, separator);
    v.widgets.open.fg_stroke    = egui::Stroke::new(1.0, text);
    v.widgets.open.rounding     = rounding;

    ctx.set_visuals(v);

    // ── Spacing & type scale (macOS HIG) ─────────────────────────────────
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing   = egui::vec2(6.0, 5.0);
    style.spacing.button_padding = egui::vec2(12.0, 5.0);
    style.spacing.indent         = 16.0;
    style.spacing.interact_size  = egui::vec2(40.0, 22.0);
    style.spacing.slider_width   = 130.0;
    style.spacing.combo_width    = 130.0;

    use egui::{FontFamily, FontId, TextStyle};
    style.text_styles = [
        (TextStyle::Small,     FontId::new(11.0, FontFamily::Proportional)),
        (TextStyle::Body,      FontId::new(13.0, FontFamily::Proportional)),
        (TextStyle::Button,    FontId::new(13.0, FontFamily::Proportional)),
        (TextStyle::Heading,   FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, FontFamily::Monospace)),
    ]
    .into();

    ctx.set_style(style);
}

/// Colour constants for explicit panel frames (used in app.rs).
pub const SIDEBAR_BG: egui::Color32 = egui::Color32::from_rgb(236, 236, 246);
pub const CONTENT_BG: egui::Color32 = egui::Color32::WHITE;
