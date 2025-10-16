use crate::ui::windows::AppWindow;
use bevy_egui::egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DebugWindow {
    is_open: bool,
}

impl AppWindow for DebugWindow {
    fn id(&self) -> &'static str {
        "debug"
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Debug"
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.label("Hello, world!");
    }
}
