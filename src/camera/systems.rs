use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};

use crate::camera::components::*;

pub fn spawn_camera(mut commands: Commands, mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Confined;
    window.cursor.visible = false;
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