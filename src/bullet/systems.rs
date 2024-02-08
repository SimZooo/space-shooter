use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::bullet::*;
use crate::player::events::*;

use crate::asteroid::components::*;

const BULLET_SPEED: f32 = 800.;

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

pub fn check_asteroid_hit(commands: Commands, asteroids: Query<(Entity, &Transform), With<Asteroid>>) {
}
