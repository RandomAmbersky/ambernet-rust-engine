attribute vec2 a_texCoord;
attribute vec2 a_position;

void main() {
    vec2 pos = a_position;
    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}
