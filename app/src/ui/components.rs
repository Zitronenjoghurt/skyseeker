pub mod toggle_button;

pub trait AppComponent: Sized {
    fn ui(self, ui: &mut bevy_egui::egui::Ui);
}
