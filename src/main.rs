mod asset_loader;
mod gun;
mod mouse;
mod movement;
mod player;
mod schedule;
mod state;

use asset_loader::AssetLoaderPlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use gun::GunPlugin;
use mouse::MousePlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::{SchedulePlugin, StartupSystemSet};
use state::StatePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::srgb(0.77, 0.80, 0.72)))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (500.0, 500.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            SchedulePlugin,
            StatePlugin,
            AssetLoaderPlugin,
            MovementPlugin,
            PlayerPlugin,
            GunPlugin,
            MousePlugin,
        ))
        .add_systems(Startup, setup.in_set(StartupSystemSet::GameInit))
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn close_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Bye bye!");
        exit.send(AppExit::Success);
    }
}
