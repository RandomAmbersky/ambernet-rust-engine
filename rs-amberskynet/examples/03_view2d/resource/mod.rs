use crate::model_vertex::ModelVertex;

pub const TEXTURE_SOURCE: &[u8] = include_bytes!("tiles_mod.png");
pub const SHADER_SOURCE: &str = include_str!("shader.wgsl");
pub const VERTICES: &[ModelVertex] = &[
    ModelVertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    }, // B
    ModelVertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    }, // E
    ModelVertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    }, // A
    ModelVertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    }, // A
];

pub const INDICES: &[u16] = &[0, 1, 2, 0, 3, 1];
