use bevy::{
    color::palettes::{css::WHITE, tailwind::RED_500},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin::default()))
        .add_systems(Startup, setup)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: RED_500.into(),
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // Setup Camera

    let mesh_handle: Handle<Mesh> = meshes.add(create_mesh_handle());
    let uv_texture = asset_server.load("textures/uv_checker.png");

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(uv_texture),
            ..default()
        })),
        // MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(1.8, 1.8, 1.8)));
}

fn create_mesh_handle() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Back face (Z = 0)
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0], // 1: bottom-right-back
            [1.0, 0.0, 1.0], // 2: top-right-back
            [0.0, 0.0, 1.0], // 3: top-left-back
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Front face - corrected UV (flipped vertically)
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
            [0.0, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 2, 3, 0]))
    .with_duplicated_vertices()
    .with_computed_flat_normals()
}
