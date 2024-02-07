use bevy::prelude::*;

#[derive(Event)]
pub struct ShootEvent(pub Transform);