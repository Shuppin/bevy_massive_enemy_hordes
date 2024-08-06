use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSystemSet {
    LoadingAssets,
    GameInit,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (StartupSystemSet::LoadingAssets, StartupSystemSet::GameInit).chain(),
        );
    }
}
