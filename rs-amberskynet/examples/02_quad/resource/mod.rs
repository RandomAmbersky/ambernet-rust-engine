use rs_amberskynet::gfx::Vertex;

pub const TEXTURE_SOURCE: &[u8] = include_bytes!("tiles_mod.png");
pub const SHADER_SOURCE: &str = include_str!("shader.wgsl");
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    }, // A
    Vertex {
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    }, // B
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    }, // E
];

pub const INDICES: &[u16] = &[0, 1, 2];

// pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, 0];
