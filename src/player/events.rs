use bevy::prelude::*;

#[derive(Event)]
pub struct ShootEvent {
    pub transform: Transform,
    pub angle: f32
}
