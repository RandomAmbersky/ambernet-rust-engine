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

    //        gl_FragColor = vec4(v_texCoord, 1.0, 1.0);
//        gl_FragColor = texture2D(u_image[0], v_color);
//    gl_FragColor = texture2D(u_image, vec2(v_texCoord.s, v_texCoord.t));
    //    vec4 tile = texture2D(u_image[1], v_texCoord);
//    vec2 tileCoord = floor(tile.xy * 255.0);
//
//    tileCoord.x = (tileCoord.x * 16.0) / 16.0;
//    tileCoord.y = (tileCoord.y * 16.0) / 16.0;
//
//    vec4 color = texture2D(u_image[0], tileCoord);
//    gl_FragColor = vec4(color.rgb, color.a);
}
