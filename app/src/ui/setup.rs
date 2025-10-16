use bevy::prelude::Query;
use bevy_egui::{EguiContextSettings, EguiContexts};

pub fn setup_ui(mut contexts: EguiContexts, mut egui_settings: Query<&mut EguiContextSettings>) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    if let Ok(mut settings) = egui_settings.single_mut() {
        settings.scale_factor = 1.5;
    }

    let mut fonts = bevy_egui::egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}
