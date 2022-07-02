pub const VERTICES: [f32; 12] = [
-1., 1., 0.,
-1., -1., 0.,
1., -1., 0.,
1., 1., 0.,
];

pub const INDICES: [u16; 6] = [3, 2, 1, 3, 1, 0];

pub const VERTEX_SHADER: &str = r#"
attribute vec3 coordinates;
void main(void) {
   gl_Position = vec4(coordinates, 1.0);
}
"#;

pub const FRAG_SHADER: &str = r#"
void main(void) {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 0.5);
}"#;
