use rs_amberskynet::gfx::Vertex;

pub const TEXTURE_SOURCE: &[u8] = include_bytes!("tiles_mod.png");
pub const SHADER_SOURCE: &str = include_str!("shader.wgsl");
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    }, // B
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    }, // E
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    }, // A
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    }, // A
];

pub const INDICES: &[u16] = &[0, 1, 2, 0, 3, 1];
