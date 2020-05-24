precision mediump float;

varying vec2 v_texCoord;
varying vec3 v_color;

uniform vec2 u_resolution; // screen width x height in pixels
uniform vec2 u_tile_size;  // tile width x height in pixels
uniform vec2 u_map_size; // map width x height in tiles

uniform sampler2D u_image0;
uniform sampler2D u_image1;

void main() {
    vec2 st = gl_FragCoord.xy/u_resolution.xy; // [0..1]

    vec2 mulSt = st*u_map_size;
    vec2 cellCoordStart = floor(st*u_map_size); // [0..map_size-1]
    vec2 textureOffset = (mulSt - cellCoordStart) / u_map_size;

    vec3 color = vec3(cellCoordStart.x/u_map_size.x,cellCoordStart.y/u_map_size.y,0.); // [0..1]

    vec2 textureCoord = vec2(color.xy)+textureOffset.xy;

    vec4 textureColor = texture2D(u_image1, textureCoord);

    gl_FragColor = vec4(textureColor.rgb,1.0);
}
