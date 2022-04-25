
pub const FRAG_SHADER: &str = "
        precision mediump float;

        uniform vec4 uColor;
        uniform float uOpacity;

        void main() {
            gl_FragColor = vec4(uColor.r, uColor.g, uColor.b, uColor.a * uOpacity);
        }";

pub const VERT_SHADER: &str = r#"
        attribute vec4 aPosition;
        uniform mat4 uTransform;

        void main() {
            gl_Position = uTransform * aPosition;
        }"#;
