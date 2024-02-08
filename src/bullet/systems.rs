use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{asteroid, bullet::*};
use crate::player::events::*;

use crate::asteroid::components::*;
use crate::asteroid::*;
use crate::player::components::*;

const BULLET_SPEED: f32 = 1000.;

pub fn check_for_shoot(mut commands: Commands, mut evr: EventReader<ShootEvent>, asset_s: Res<AssetServer>) {
    for ev in evr.read() {
        commands.spawn(
            BulletBundle {
                sprite: SpriteBundle {
                    texture: asset_s.load("sprites/missile.png"),
                    transform: ev.transform,
                    ..default()
                },
                bullet: Bullet {
                    rotation: Vec2::new(-ev.angle.sin(), ev.angle.cos())
                },
                ..default()
            }
        );
    }
}

pub fn update_bullets(mut commands: Commands, mut bullets: Query<(Entity, &Bullet, &mut Transform), With<Bullet>>, time: Res<Time>, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
     for (entity, bullet, mut transform) in bullets.iter_mut() {
        if transform.translation.x > window.width() || transform.translation.x < 0. || transform.translation.y > window.height() || transform.translation.y < 0. {
            commands.entity(entity).despawn();
            println!("DEEEEESPAWN")
        }
        let rot = Vec3::new(bullet.rotation.x, bullet.rotation.y, 0.);
        transform.translation += rot * BULLET_SPEED * time.delta_seconds();
     }
}

pub fn check_hit(mut commands: Commands, player: Query<(Entity, &Transform), With<Player>>, asteroids: Query<(Entity, &Transform), With<Asteroid>>, bullets: Query<(Entity, &Transform), (With<Bullet>, Without<Asteroid>)>) {
    let half_size = 64.;
    let half_size_player = 64.;
    if let Ok((player_en, transform)) = player.get_single() {
        for (bullet_en, bullet_transform) in bullets.iter() {
            for (asteroid_en, asteroid_transform) in asteroids.iter() {
                if bullet_transform.translation.distance(asteroid_transform.translation) < half_size {
                    commands.entity(bullet_en).despawn();
                    commands.entity(asteroid_en).despawn();
                }
                println!("{} {}", transform.translation, asteroid_transform.translation);
            }
        }
    }
}
