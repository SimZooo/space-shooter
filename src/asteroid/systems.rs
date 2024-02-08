use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const ASTEROID_SPEED: f32 = 300.;

use super::{components::*, resources::AsteroidTimer};
use crate::player::components::*;

pub fn spawn_asteroid(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>, asset_s: Res<AssetServer>, timer: Res<AsteroidTimer>) {
    let window = window.single();
    let side = rand::thread_rng().gen_range(0..4);
    let clos = {
        let transform = match side {
            0 => Transform::from_xyz(rand::random::<f32>() * window.width(), 0., 0.),  // BOTTOM
            1 => Transform::from_xyz(0., rand::random::<f32>() * window.height(), 0.), // LEFT
            2 => Transform::from_xyz(window.width(), rand::random::<f32>() * window.height(), 0.), // RIGHT 
            3 => Transform::from_xyz(rand::random::<f32>() * window.width(), window.height(), 0.), //TOP
            i32::MIN..=-1_i32 | 4_i32..=i32::MAX => todo!(),
        };

        let velocity = match side {
            0 => Vec2::new(rand::thread_rng().gen_range(0.0..2.0), rand::random::<f32>()),
            1 => Vec2::new(rand::thread_rng().gen_range(0.0..2.0), rand::thread_rng().gen_range(-1.0..2.0)),
            2 => Vec2::new(-rand::thread_rng().gen_range(0.0..2.0), rand::thread_rng().gen_range(-1.0..2.0)),
            3 => Vec2::new(rand::random::<f32>(), -rand::random::<f32>()),
            i32::MIN..=-1_i32 | 4_i32..=i32::MAX => todo!(),
        };

        (
            SpriteBundle {
            texture: asset_s.load(format!("sprites/asteroid{}.png", rand::thread_rng().gen_range(1..5) as u32)),
            transform: transform,
            ..default()
            },
            Asteroid {
                velocity
            }
        )
    };

    if timer.timer.finished() {
        commands.spawn(
            clos
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

pub fn check_hit_player(mut commands: Commands, player: Query<(Entity, &Transform), With<Player>>, asteroids: Query<&Transform, With<Asteroid>>) {
    if let Ok((entity, transform)) = player.get_single() {
        for asteroid in asteroids.iter() {
            if transform.translation.distance(asteroid.translation) < 64. {
                commands.entity(entity).despawn();
            }
        }
    }
}