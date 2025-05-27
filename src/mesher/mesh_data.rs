use bevy::{
    asset::RenderAssetUsages,
    render::mesh::{Indices, Mesh, PrimitiveTopology},
};

// Check Bevy Coordinate System
// https://bevy-cheatbook.github.io/fundamentals/coords.html
//
// The X axis goes from left to right (+X points right)
// The Y axis goes from bottom to top (+Y points up).
// The Z axis goes from far to near (+Z points towards you, out of the screen).

const VOXEL_VERTICES: [[f32; 3]; 8] = [
    [0.0, 0.0, 0.0], // Vert 0
    [1.0, 0.0, 0.0], // Vert 1
    [1.0, 0.0, 1.0], // Vert 2
    [0.0, 0.0, 1.0], // Vert 3
    [0.0, 1.0, 0.0], // Vert 4
    [1.0, 1.0, 0.0], // Vert 5
    [1.0, 1.0, 1.0], // Vert 6
    [0.0, 1.0, 1.0], // Vert 7
];

// quad order
// down, up, right, left, front, back
const QUAD_VERTICES: [[u8; 4]; 6] = [
    // Note the Diagonal are always shared we can Group them together
    [3, 2, 1, 0], // -> DOWN
    [4, 5, 6, 7], // -> UP
    [0, 4, 7, 3], // -> RIGHT
    [2, 6, 5, 1], // -> LEFT
    [3, 7, 6, 2], // -> FRONT
    [1, 5, 4, 0], // -> BACK
];

// We are created Seperated Quad for each Face with there own Vertices
// Beacuse we need Flat faces and We will use **Greedy mesher**!
fn create_quad(indices: &mut Vec<u32>, vertices: &mut Vec<[f32; 3]>, side: usize) {
    let vertices_count = vertices.len() as u32;

    indices.push(vertices_count + 1);
    indices.push(vertices_count + 0);
    indices.push(vertices_count + 2);
    indices.push(vertices_count + 2);
    indices.push(vertices_count + 0);
    indices.push(vertices_count + 3);

    for vertex in 0..4 as usize {
        let index = QUAD_VERTICES[side][vertex];
        let position = VOXEL_VERTICES[QUAD_VERTICES[side][vertex] as usize];
        vertices.push(position);
    }
}

// We are creating a Voxel Mesh with 6 Quads
fn create_voxel() -> (Vec<u32>, Vec<[f32; 3]>) {
    let mut indices: Vec<u32> = Vec::new();
    let mut vertices: Vec<[f32; 3]> = Vec::new();

    // loop to create Each Quad
    for side in 0..6 {
        create_quad(&mut indices, &mut vertices, side);
    }

    (indices, vertices)
}

pub fn create_mesh() -> Mesh {
    let (indices, vertices) = create_voxel();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(indices))
    .with_computed_normals()
}
