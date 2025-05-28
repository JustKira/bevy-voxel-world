mod mesher;

use bevy::{
    color::palettes::{css, tailwind},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    winit::get_selected_videomode,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use fastnoise_lite::{
    CellularDistanceFunction, CellularReturnType, DomainWarpType, FastNoiseLite, FractalType,
};
use mesher::{mesh_data::create_chunk, noise::NoiseMesher};

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

    let mut fastnoise_lite = FastNoiseLite::new();
    fastnoise_lite.set_frequency(Some(0.03));
    fastnoise_lite.set_fractal_type(Some(FractalType::Ridged));
    fastnoise_lite.set_fractal_octaves(Some(3));
    fastnoise_lite.set_fractal_lacunarity(Some(2.0));
    fastnoise_lite.set_fractal_gain(Some(0.5));
    fastnoise_lite.set_fractal_weighted_strength(Some(0.0));
    fastnoise_lite.set_fractal_ping_pong_strength(Some(2.0));
    fastnoise_lite.set_cellular_distance_function(Some(CellularDistanceFunction::EuclideanSq));
    fastnoise_lite.set_cellular_return_type(Some(CellularReturnType::Distance));
    fastnoise_lite.set_cellular_jitter(Some(1.0));
    fastnoise_lite.set_domain_warp_type(Some(DomainWarpType::OpenSimplex2));
    fastnoise_lite.set_domain_warp_amp(Some(1.0));

    let noise_mesher = NoiseMesher::new(fastnoise_lite);

    commands.spawn((
        Mesh3d(meshes.add(create_chunk(&noise_mesher, 16, 256, 16))),
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
