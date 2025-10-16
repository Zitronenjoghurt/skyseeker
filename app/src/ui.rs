use bevy::app::App;
use bevy::prelude::{run_once, IntoScheduleConfigs, Plugin, Update};
use bevy_egui::EguiPrimaryContextPass;

mod components;
mod composites;
mod render;
mod setup;
mod windows;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(windows::WindowManager::default())
            .add_systems(Update, setup::setup_ui.run_if(run_once))
            .add_systems(EguiPrimaryContextPass, render::render_ui);
    }
}
