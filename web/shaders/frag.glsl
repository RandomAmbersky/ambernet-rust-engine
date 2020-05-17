precision mediump float;

varying vec2 v_texCoord;
varying vec3 v_color;

uniform sampler2D u_image;

void main() {
    gl_FragColor = vec4(vec3(v_color.x, v_color.y, v_color.z), 1.0);
//    gl_FragColor = texture2D(u_image, vec2(v_texCoord.s, v_texCoord.t));
}
