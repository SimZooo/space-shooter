use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const ASTEROID_SPEED: f32 = 100.;

use super::{components::*, resources::AsteroidTimer};

pub fn spawn_asteroid(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>, asset_s: Res<AssetServer>, timer: Res<AsteroidTimer>) {
    let window = window.single();

    if timer.timer.finished() {
        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_s.load(format!("sprites/asteroid{}.png", rand::thread_rng().gen_range(1..5) as u32)),
                    transform: Transform::from_xyz(rand::thread_rng().gen_range(0..2) as f32 * window.width(), rand::thread_rng().gen_range(0..2) as f32 * window.height(), 0.),
                    ..default()
                },
                Asteroid {
                    velocity: Vec2::new(rand::thread_rng().gen_range(-1.0..2.0) as f32, rand::thread_rng().gen_range(-1.0..2.0) as f32)
                }
            )
        );
    }
}

pub fn tick_timer(mut timer: ResMut<AsteroidTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn update_asteroids(mut asteroids: Query<(&Asteroid, &mut Transform), With<Asteroid>>, time: Res<Time>) {
    for (asteroid, mut transform) in asteroids.iter_mut() {
        let direction = asteroid.velocity;
        transform.translation += Vec3::new(direction.x, direction.y, 0.) * ASTEROID_SPEED * time.delta_seconds();
    }
}
