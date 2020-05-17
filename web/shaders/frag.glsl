precision mediump float;

uniform sampler2D u_image;
varying vec2 v_texCoord;

void main() {
//    gl_FragColor = vec4(vec2(v_texCoord.s, v_texCoord.t), 0.0, 1.0);
    gl_FragColor = texture2D(u_image, vec2(v_texCoord.s, v_texCoord.t));
}
