use bevy::prelude::*;

const ASTEROID_SPAWN_TIME: f32 = 1.;

#[derive(Resource)]
pub struct AsteroidTimer {
    pub timer: Timer
}

impl Default for AsteroidTimer {
    fn default() -> Self {
        AsteroidTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWN_TIME, TimerMode::Repeating)
        }
    }
}
