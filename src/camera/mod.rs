use bevy::prelude::*;

mod systems;
use systems::*;
pub mod components;

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}