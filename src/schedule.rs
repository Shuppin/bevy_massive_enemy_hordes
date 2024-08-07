use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSystemSet {
    LoadingAssets,
    GameInit,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateSystemSet {
    UserInput,
    EntityUpdates,
    /// Useful for entity updates which rely on prior entity updates
    PostEntityUpdates,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (StartupSystemSet::LoadingAssets, StartupSystemSet::GameInit).chain(),
        )
        .configure_sets(
            Update,
            (
                UpdateSystemSet::UserInput,
                UpdateSystemSet::EntityUpdates,
                UpdateSystemSet::PostEntityUpdates,
            )
                .chain(),
        );
    }
}
