use bevy::prelude::*;

mod systems;
pub mod components;
mod resources;

use systems::*;
use resources::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidTimer>()
           .add_systems(Update, (spawn_asteroid, tick_timer, update_asteroids));
    }
}
