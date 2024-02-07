use std::f32::consts::PI;

use bevy::{math::Quat, prelude::*, window::PrimaryWindow};

use crate::player::components::*;
use crate::player::events::*;
use crate::camera::components::*;

const DEFAULT_SPEED: f32 = 200.;

pub fn spawn_player(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>, asset_s: Res<AssetServer>) {
    let window = window.single();
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
                texture: asset_s.load("sprites/spaceShips_008.png"),
                ..default()
            },
            Player {
                speed: DEFAULT_SPEED
            }
        )
    );
}

pub fn player_movement(mut commands: Commands, mut player: Query<(&Player, &mut Transform), With<Player>>, kb_in: Res<Input<KeyCode>>, time: Res<Time>, window: Query<&Window, With<PrimaryWindow>>, camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>, asset_s: Res<AssetServer>) {
    let window = window.single();
    if let Ok((player, mut transform)) = player.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if kb_in.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if kb_in.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        }
        if kb_in.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.);
        }
        if kb_in.pressed(KeyCode::S) {
            direction += Vec3::new(0.,-1., 0.);
        }
        transform.translation += direction * player.speed * time.delta_seconds();

        let (camera, camera_transform) = camera.single();

        if let Some(cursor) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            
            let pos = transform.translation.truncate();
            let direction =Vec2::new(cursor.x - window.width() / 2., cursor.y - window.height() / 2.);
            let mut angle = direction.y.atan2(direction.x) - PI / 2.;

            if angle < 0.0 {
                angle += 2. * PI;
            }
            transform.rotation = Quat::from_rotation_z(angle);
        }

    }
}

pub fn check_shoot(mut evw: EventWriter<ShootEvent>, player: Query<&Transform, With<Player>>, kb_in: Res<Input<KeyCode>>) {
    if let Ok(transform) = player.get_single() {
        if kb_in.just_released(KeyCode::Space) {
            //let trans = Transform::default().with_rotation(transform.rotation).with_translation(transform.translation);
            evw.send(ShootEvent(*transform));
        }
    }
}
