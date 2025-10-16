use bevy::app::App;
use bevy::prelude::*;

mod camera;

pub struct ObservationPlugin;

impl Plugin for ObservationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    commands.spawn(camera::CustomCamera::default());
    clear_color.0 = Color::BLACK;
}
