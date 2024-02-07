use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::components::*;

pub fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    commands.spawn(
        (
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        MainCamera
    )
    );
}