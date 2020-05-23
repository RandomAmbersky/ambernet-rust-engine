precision mediump float;

varying vec2 v_texCoord;
varying vec3 v_color;

uniform sampler2D u_image0;
uniform sampler2D u_image1;

void main() {
    vec4 color0 = texture2D(u_image0, v_texCoord);
    vec3 color0_r = vec3(color0.r, color0.r, color0.r);
    vec3 color0_g = vec3(color0.g, color0.g, color0.g);
    vec3 color0_b = vec3(color0.b, color0.b, color0.b);

    vec4 color1 = texture2D(u_image1, v_texCoord);

    gl_FragColor = vec4(color0_r, 1.0);
}
