mod asset_loader;
mod player;

use asset_loader::AssetLoaderPlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use player::PlayerPlugin;

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
            AssetLoaderPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
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
