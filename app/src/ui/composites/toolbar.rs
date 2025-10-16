use crate::ui::components::toggle_button::ToggleButton;
use crate::ui::components::AppComponent;
use crate::ui::composites::AppComposite;
use crate::ui::windows::{AppWindow, WindowManager};
use bevy_egui::egui::TopBottomPanel;
use egui_phosphor::regular;

pub struct ToolBar<'a> {
    window_manager: &'a mut WindowManager,
}

impl<'a> ToolBar<'a> {
    pub fn new(window_manager: &'a mut WindowManager) -> Self {
        Self { window_manager }
    }
}

impl AppComposite for ToolBar<'_> {
    fn show(self, ctx: &bevy_egui::egui::Context) {
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let mut debug_open = self.window_manager.debug.is_open();
            ToggleButton::new(&mut debug_open, regular::BUG).ui(ui);
            self.window_manager.debug.set_open(debug_open);
        });
    }
}
