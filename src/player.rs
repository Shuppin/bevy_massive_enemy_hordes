use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, Velocity},
    schedule::{StartupSystemSet, UpdateSystemSet},
};

const MOVEMENT_SPEED: f32 = 150.0;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player.in_set(StartupSystemSet::GameInit))
            .add_systems(
                Update,
                handle_player_movement.in_set(UpdateSystemSet::UserInput),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let first_sprite = scene_assets
        .texture_atlas_data
        .get("doc_idle_anim_f0")
        .expect("player sprite not found");
    let no_sprite_rows = 1;
    let no_sprite_cols = 4;

    let tile_size = UVec2::new(first_sprite.width, first_sprite.height);
    let offset = UVec2::new(first_sprite.x, first_sprite.y);

    let layout = TextureAtlasLayout::from_grid(
        tile_size,
        no_sprite_cols,
        no_sprite_rows,
        None,
        Some(offset),
    );

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Player,
        Velocity::ZERO,
        Acceleration::ZERO,
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0)),
            texture: scene_assets.texture_atlas.clone(),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
    ));
}

fn handle_player_movement(
    mut query: Query<&mut Velocity, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut velocity) = query.get_single_mut() else {
        warn!("No player entity found, but movement system is still running");
        return;
    };

    let mut delta = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        delta.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        delta.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    delta = delta.normalize_or_zero() * MOVEMENT_SPEED;

    velocity.0 = Vec3::new(delta.x, delta.y, 0.0);
}
