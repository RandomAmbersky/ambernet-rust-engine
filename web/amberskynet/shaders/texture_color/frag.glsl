precision mediump float;

//varying vec2 v_texCoord;
varying vec3 v_color;

//uniform sampler2D u_image[2];

void main() {
        gl_FragColor = vec4(v_color, 1.0);
    //    gl_FragColor = texture2D(u_image[0], v_color);
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
