use bevy::prelude::*;
mod camera;
use camera::*;
mod player;
use player::*;
mod bullet;
use bullet::*;
mod asteroid;
use asteroid::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CamPlugin, PlayerPlugin, BulletPlugin, AsteroidPlugin))
        .run();
}
