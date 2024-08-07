use bevy::{prelude::*, window::PrimaryWindow};

use crate::schedule::UpdateSystemSet;

#[derive(Resource, Debug)]
pub struct MouseInWorld {
    pub pos_in_world: Vec2,
    pub mouse_in_window: bool,
}

impl Default for MouseInWorld {
    fn default() -> Self {
        Self {
            pos_in_world: Vec2::new(100.0, 100.0),
            mouse_in_window: false,
        }
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseInWorld>().add_systems(
            Update,
            update_mouse_position_in_world.in_set(UpdateSystemSet::UserInput),
        );
    }
}

fn update_mouse_position_in_world(
    mut mouse_pos: ResMut<MouseInWorld>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.get_single() else {
        mouse_pos.mouse_in_window = false;
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        mouse_pos.mouse_in_window = false;
        return;
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_pos.pos_in_world = world_position;
        mouse_pos.mouse_in_window = true;
    } else {
        mouse_pos.mouse_in_window = false;
    }
}
