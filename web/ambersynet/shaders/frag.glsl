precision mediump float;

varying vec2 v_texCoord;
varying vec3 v_color;

uniform vec2 u_resolution; // screen width x height in pixels
uniform sampler2D u_image0;
uniform sampler2D u_image1;

void main() {

    vec2 st = gl_FragCoord.xy/u_resolution.xy; // [0..1]

    vec2 cellCoordStart = floor(st*16.); // [0..16]

    vec3 color = vec3(cellCoordStart.x/16.,cellCoordStart.y/16.,0.);

    gl_FragColor = vec4(color,1.0);

//    vec4 color0 = texture2D(u_image0, v_texCoord);

//    vec4 tile = texture2D(u_image0, v_texCoord);
//    vec2 tileCoord = floor(tile.xy) * 256.0;

//    tileCoord.x = (tileCoord.x * 16.0) / 16.0;
//    tileCoord.y = (tileCoord.y * 16.0) / 16.0;

//    vec4 color = texture2D(u_image1, tileCoord) * 1000000.;
//    gl_FragColor = vec4(color.rgb, color.a);

//    vec2 tex_coord = vec2(color0.r, color0.g);
//    vec4 color1 = texture2D(u_image1, tex_coord);
//
//    gl_FragColor = vec4(color1.rgb, 1.0);
}

/*
    #ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
    vec2 st = gl_FragCoord.xy/u_resolution.xy; // [0..1]

    vec2 cellCoordStart = floor(st*16.); // [0..16]

    vec3 color = vec3(cellCoordStart.x/16.,cellCoordStart.y/16.,0.);

    gl_FragColor = vec4(color,1.0);
}
*/