use bevy::prelude::*;

use crate::schedule::UpdateSystemSet;

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub const ZERO: Self = Velocity(Vec3::ZERO);
}

#[derive(Component, Debug)]
pub struct Acceleration(pub Vec3);

impl Acceleration {
    pub const ZERO: Self = Acceleration(Vec3::ZERO);
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
                .in_set(UpdateSystemSet::EntityUpdates),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
