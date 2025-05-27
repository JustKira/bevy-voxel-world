mod mesher;

use bevy::{
    color::palettes::{css, tailwind},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use mesher::mesh_data::create_chunk;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin::default()))
        .add_systems(Startup, setup)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: tailwind::GRAY_700.into(),
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    // asset_server: Res<AssetServer>,
) {
    // Setup Camera

    // let uv_texture = asset_server.load("textures/uv_checker.png");

    commands.spawn((
        Mesh3d(meshes.add(create_chunk(32, 32, 32))),
        // MeshMaterial3d(materials.add(StandardMaterial {
        //     base_color_texture: Some(uv_texture.clone()),
        //     ..default()
        // })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: css::WHITE.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color_texture: Some(uv_texture),
    //         ..default()
    //     })),
    //     Transform::from_xyz(0.0, 3.5, 0.0),
    // ));

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: DirectionalLight::default().illuminance * 0.05,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 2.0, 0.0).looking_at(Vec3::NEG_X * 1.5, Vec3::Y),
    ));
}
