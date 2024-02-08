use bevy::prelude::*;

mod systems;
pub mod components;
pub mod events;

use systems::*;
use events::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
           .add_systems(Startup, spawn_player)
           .add_systems(Update, (player_movement, check_shoot));
    }
}
