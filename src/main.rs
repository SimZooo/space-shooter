use bevy::prelude::*;
mod camera;
use camera::*;
mod player;
use player::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CamPlugin, PlayerPlugin))
        .run();
}
