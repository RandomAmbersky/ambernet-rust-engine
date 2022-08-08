pub const TEXTURE: &'static [u8] = include_bytes!("./textures.png");

pub const TEXTURE_COORDINATES: [f32; 8] = [
	0., 1.,
	1., 1., // bottom right corner
	0., 0., // top left corner of texture
	1., 0.,
];

pub const VERTICES: [f32; 12] = [
	-0.5, -0.5, 0.,
	0.5, -0.5, 0.,
	-0.5, 0.5, 0.,
	0.5, 0.5, 0.,
];

pub const INDICES: [u16; 6] = [0, 1, 2, 1, 2, 3];

pub const VERTEX_SHADER: &str = r#"
attribute vec3 aCoordinates;
attribute vec2 aTextureCoord;

varying highp vec2 vTextureCoord;

void main(void) {
   gl_Position = vec4(aCoordinates, 1.0);
   vTextureCoord = aTextureCoord;
}
"#;

pub const FRAG_SHADER: &str = r#"
varying highp vec2 vTextureCoord;
uniform sampler2D uSampler;

void main(void) {
          gl_FragColor = texture2D(uSampler, vTextureCoord);
}"#;
