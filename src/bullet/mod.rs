use bevy::{prelude::*};

mod systems;
use systems::*;
mod components;
use components::*;

#[derive(Bundle)]
pub struct BulletBundle {
    pub sprite: SpriteBundle,
    pub bullet: Bullet
}

impl Default for BulletBundle {
    fn default() -> Self {
        BulletBundle {
            sprite: SpriteBundle {
                ..default()
            },
            bullet: Bullet {
                rotation: Vec2::ZERO
            }
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_for_shoot, update_bullets, check_hit));
    }
}
