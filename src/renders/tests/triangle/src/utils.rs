pub const VERTICES: [f32; 9] = [
	-0.5, 0.5, 0.0,
	-0.5, -0.5, 0.0,
	0.5, -0.5, 0.0,
];

pub const INDICES: [u16; 3] = [0, 1, 2];

pub const VERTEX_SHADER: &str = r#"
attribute vec3 aCoordinates;
void main(void) {
   gl_Position = vec4(aCoordinates, 1.0);
}
"#;

pub const FRAG_SHADER: &str = r#"
void main(void) {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 0.5);
}"#;
