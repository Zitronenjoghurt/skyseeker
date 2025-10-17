use bevy::app::{PluginGroupBuilder, TaskPoolThreadAssignmentPolicy};
use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::tasks::available_parallelism;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use skyseeker_core::codec::decode;
use skyseeker_core::math::angle_format_to_radians;
use skyseeker_core::position::earth_orientation::EarthOrientation;
use skyseeker_core::position::observer::Observer;
use skyseeker_core::position::time::Time;

mod observation;
mod ui;

pub const VERSION_STRING: &str = "v0.1.0";

pub struct AppPlugins;
impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ui::UiPlugin)
            .add(observation::ObservationPlugin)
    }
}

fn main() {
    #[cfg(feature = "tracy")]
    init_tracing();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Skyseeker ({VERSION_STRING})"),
                        ..default()
                    }),
                    ..default()
                })
                // No parallelized I/O threads, using more threads for computing, potentially less idling but potential performance implications on I/O operations
                // https://bevy-cheatbook.github.io/setup/perf.html#overprovisioning
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        compute: TaskPoolThreadAssignmentPolicy {
                            min_threads: available_parallelism(),
                            max_threads: usize::MAX,
                            percent: 1.0,
                            on_thread_spawn: None,
                            on_thread_destroy: None,
                        },
                        ..default()
                    },
                }),
        )
        .add_plugins(EguiPlugin::default())
        .add_plugins(AppPlugins)
        .run();
}

#[cfg(feature = "tracy")]
fn init_tracing() {
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry()
        .with(tracing_tracy::TracyLayer::default())
        .init();
}

fn example() {
    let mut skyseeker = skyseeker_core::Skyseeker::new();
    skyseeker.load_standard_bodies();
    skyseeker.load_bodies(decode(include_bytes!("../../data/bsc5-stars.bin")).unwrap());

    let observer = Observer {
        longitude: angle_format_to_radians('+', 14, 0, 0.0).unwrap(),
        latitude: angle_format_to_radians('+', 51, 0, 0.0).unwrap(),
        height: Some(300.0),
        pressure: None,
        temperature: None,
        humidity: None,
        wavelength: None,
    };

    let body = skyseeker.get_body("HR 5340").unwrap();
    let position = body
        .position(&observer, &Time::now(), &EarthOrientation::default())
        .unwrap();

    println!("{:?}", position);
}
