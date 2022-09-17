pub const VERTICES: [f32; 12] = [
	0., 1.,
	1., 0.,
	0., 0.,
	0., 1.,
	1., 0.,
	1., 1.
];

#[allow(dead_code)]
pub const INDICES: [u16; 6] = [3, 2, 1, 3, 1, 0];
pub const VERTEX_SHADER: &str = include_str!("shaders/vert.glsl");
pub const FRAG_SHADER: &str = include_str!("shaders/frag.glsl");
