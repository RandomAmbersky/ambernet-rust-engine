pub const VERTICES: [f32; 12] = [
-1., 1., 0.,
-1., -1., 0.,
1., -1., 0.,
1., 1., 0.,
];

pub const INDICES: [u16; 6] = [3, 2, 1, 3, 1, 0];

pub const COLORS: [f32; 12] = [
0.0, 0.0, 0.0,
1.0, 0.0, 0.0,
0.0, 1.0, 0.0,
0.0, 0.0, 1.0
];

pub const VERTEX_SHADER: &str = r#"
attribute vec3 coordinates;
attribute vec3 color;
varying vec3 vColor;
void main(void) {
   gl_Position = vec4(coordinates, 1.0);
   vColor = color;
}
"#;

pub const FRAG_SHADER: &str = r#"
precision mediump float;
varying vec3 vColor;
void main(void) {
    gl_FragColor = vec4(vColor, 1.);
}"#;
