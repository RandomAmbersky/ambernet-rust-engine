attribute vec2 a_texCoord;
attribute vec2 a_position;

varying vec2 v_texCoord;

void main() {
    gl_PointSize = 30.0;
    gl_Position = vec4(a_position.x, a_position.y, 0.0, 1.0);
    v_texCoord = a_texCoord;
}
