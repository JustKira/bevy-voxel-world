use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(FlyCameraPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), FlyCamera::default()));
}
