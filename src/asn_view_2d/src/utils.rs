pub const VERTICES: [f32; 12] = [
	0., 1.0,
	1.0, 0.,
	0., 0.,
	0., 1.0,
	1.0, 0.,
	1.0, 1.0
];

pub const VERTEX_SHADER: &str = include_str!("shaders/vert.glsl");
pub const FRAG_SHADER: &str = include_str!("shaders/frag.glsl");
