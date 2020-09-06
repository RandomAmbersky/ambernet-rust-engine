//attribute vec2 a_texCoord;
attribute vec2 a_position;
attribute vec3 a_color;

//varying vec2 v_texCoord;
varying vec3 v_color;

void main() {
    gl_Position = vec4(a_position.x, a_position.y, 0.0, 1.0);
//    v_texCoord = a_texCoord * 256.0 / 16.0;
    v_color = a_color;
}
