use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    player::Player,
    schedule::{StartupSystemSet, UpdateSystemSet},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera.in_set(StartupSystemSet::GameInit))
            .add_systems(
                Update,
                (
                    attach_camera_to_player.in_set(UpdateSystemSet::PostEntityUpdates),
                    camera_zoom_contols.in_set(UpdateSystemSet::UserInput),
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn attach_camera_to_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Ok(mut cam_transform) = camera_query.get_single_mut() else {
        return;
    };

    cam_transform.translation.x = player_transform.translation.x;
    cam_transform.translation.y = player_transform.translation.y;
}

fn camera_zoom_contols(
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let Ok(mut ortho) = camera_query.get_single_mut() else {
        return;
    };

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let sens = 0.2;
                let scale = if ev.y < 0.0 {
                    1.0 - (ev.y * sens)
                } else {
                    1.0 / (1.0 + (ev.y * sens))
                };
                ortho.scale = (ortho.scale * scale).clamp(0.5, 5.0);
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
        }
    }
}
