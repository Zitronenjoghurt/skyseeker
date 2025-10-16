use bevy::prelude::Resource;
use bevy_egui::egui;
use bevy_egui::egui::{Id, WidgetText};
use serde::{Deserialize, Serialize};

mod debug;

#[derive(Default, Serialize, Deserialize, Resource)]
pub struct WindowManager {
    pub debug: debug::DebugWindow,
}

impl WindowManager {
    pub fn render(&mut self, ctx: &egui::Context) {
        self.debug.show(ctx);
    }
}

pub trait AppWindow: Sized {
    fn id(&self) -> &'static str;
    fn title(&self) -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn render_content(&mut self, ui: &mut egui::Ui);

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        true
    }

    fn show(&mut self, ctx: &egui::Context) {
        if !self.is_open() {
            return;
        }

        let mut is_open = self.is_open();
        let open_before = is_open;
        egui::Window::new(self.title())
            .id(Id::new(self.id()))
            .open(&mut is_open)
            .resizable(self.resizable())
            .collapsible(self.collapsible())
            .movable(self.movable())
            .show(ctx, |ui| {
                self.render_content(ui);
            });

        if is_open != open_before {
            self.set_open(is_open);
        }
    }
}
