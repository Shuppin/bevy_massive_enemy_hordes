use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    mouse::MouseInWorld,
    player::Player,
    schedule::{StartupSystemSet, UpdateSystemSet},
};

#[derive(Component, Debug)]
pub struct Gun;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gun.in_set(StartupSystemSet::GameInit))
            .add_systems(
                Update,
                update_gun_transform.in_set(UpdateSystemSet::PostEntityUpdates),
            );
    }
}

fn spawn_gun(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let first_sprite = scene_assets
        .texture_atlas_data
        .get("weapon_knife")
        .expect("player weapon not found");
    let no_sprite_rows = 1;
    let no_sprite_cols = 1;

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
        Gun,
        SpriteBundle {
            transform: Transform::from_scale(Vec3::new(6.0, 6.0, 7.0)),
            texture: scene_assets.texture_atlas.clone(),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
    ));
}

#[derive(Default)]
struct UpdateGunTransformLocal {
    previous_angle: Option<f32>,
}

fn update_gun_transform(
    mut local: Local<UpdateGunTransformLocal>,
    mouse: Res<MouseInWorld>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        warn!("No player entity found, but gun system is still running");
        return;
    };

    let Ok(mut gun_transform) = gun_query.get_single_mut() else {
        warn!("No gun entity found, but gun system is still running");
        return;
    };

    let p_pos = player_transform.translation;
    let angle = if mouse.mouse_in_window {
        let angle = (p_pos.y - mouse.pos_in_world.y).atan2(p_pos.x - mouse.pos_in_world.x) + PI;
        local.previous_angle = Some(angle);
        gun_transform.rotation = Quat::from_rotation_z(angle);
        gun_transform.rotate_local_z(-FRAC_PI_2);
        angle
    } else {
        local.previous_angle.unwrap_or_else(|| {
            let angle = (p_pos.y - mouse.pos_in_world.y).atan2(p_pos.x - mouse.pos_in_world.x) + PI;
            local.previous_angle = Some(angle);
            gun_transform.rotation = Quat::from_rotation_z(angle);
            gun_transform.rotate_local_z(-FRAC_PI_2);
            angle
        })
    };
    let offset = 95.0;
    let offset_gun_pos = Vec2::new(
        p_pos.x + offset * angle.cos(),
        p_pos.y + offset * angle.sin() - 20.0,
    );

    gun_transform.translation = Vec3::new(
        offset_gun_pos.x,
        offset_gun_pos.y,
        gun_transform.translation.z,
    );
}
