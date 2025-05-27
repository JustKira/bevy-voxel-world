use bevy::{
    asset::RenderAssetUsages,
    math::{Vec3, vec3},
    render::mesh::{Indices, Mesh, PrimitiveTopology},
};

// Check Bevy Coordinate System
// https://bevy-cheatbook.github.io/fundamentals/coords.html
//
// The X axis goes from left to right (+X points right)
// The Y axis goes from bottom to top (+Y points up).
// The Z axis goes from far to near (+Z points towards you, out of the screen).
const VOXEL_VERTICES: [Vec3; 8] = [
    vec3(0.0, 0.0, 0.0), // Vert 0
    vec3(1.0, 0.0, 0.0), // Vert 1
    vec3(1.0, 0.0, 1.0), // Vert 2
    vec3(0.0, 0.0, 1.0), // Vert 3
    vec3(0.0, 1.0, 0.0), // Vert 4
    vec3(1.0, 1.0, 0.0), // Vert 5
    vec3(1.0, 1.0, 1.0), // Vert 6
    vec3(0.0, 1.0, 1.0), // Vert 7
];

// quad order
// down, up, right, left, front, back
const QUAD_VERTICES: [[u8; 4]; 6] = [
    // Note the Diagonal are always shared we can Group them together
    [3, 2, 1, 0], // -> DOWN
    [4, 5, 6, 7], // -> UP
    [2, 6, 5, 1], // -> RIGHT
    [0, 4, 7, 3], // -> LEFT
    [3, 7, 6, 2], // -> FRONT
    [1, 5, 4, 0], // -> BACK
];

const VOXEL_NORMALS: [[f32; 3]; 6] = [
    [0.0, -1.0, 0.0], // DOWN
    [0.0, 1.0, 0.0],  // UP
    [1.0, 0.0, 0.0],  // RIGHT
    [-1.0, 0.0, 0.0], // LEFT
    [0.0, 0.0, 1.0],  // FRONT
    [0.0, 0.0, -1.0], // BACK
];

// We are created Seperated Quad for each Face with there own Vertices
// Beacuse we need Flat faces and We will use **Greedy mesher**!
fn create_quad(
    indices: &mut Vec<u32>,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    side: usize,
    offset: Vec3,
) {
    let vertices_count = vertices.len() as u32;

    indices.push(vertices_count + 1);
    indices.push(vertices_count + 0);
    indices.push(vertices_count + 2);
    indices.push(vertices_count + 2);
    indices.push(vertices_count + 0);
    indices.push(vertices_count + 3);

    for vertex in 0..4 as usize {
        let position = VOXEL_VERTICES[QUAD_VERTICES[side][vertex] as usize] + offset;
        vertices.push(position.to_array());
        normals.push(VOXEL_NORMALS[side]);
    }
}

pub fn create_mesh() -> Mesh {
    let mut indices: Vec<u32> = Vec::new();
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();

    // loop to create Each Quad
    for side in 0..6 {
        create_quad(&mut indices, &mut vertices, &mut normals, side, Vec3::ZERO);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
}

pub fn create_chunk(x_size: u32, y_size: u32, z_size: u32) -> Mesh {
    let mut indices: Vec<u32> = Vec::new();
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    for x in 0..x_size {
        for y in 0..y_size {
            for z in 0..z_size {
                for side in 0..6 {
                    create_quad(
                        &mut indices,
                        &mut vertices,
                        &mut normals,
                        side,
                        Vec3::new(x as f32, y as f32, z as f32),
                    );
                }
            }
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
}
