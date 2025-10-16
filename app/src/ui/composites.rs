pub mod toolbar;

pub trait AppComposite: Sized {
    fn show(self, ctx: &bevy_egui::egui::Context);
}
