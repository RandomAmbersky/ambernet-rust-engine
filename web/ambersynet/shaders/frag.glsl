precision mediump float;

varying vec2 v_texCoord;
varying vec3 v_color;

uniform vec2 u_resolution; // screen width x height in pixels
uniform vec2 u_tile_size;  // one tile width x height in pixels
uniform vec2 u_map_size; // map width x height in tiles

uniform sampler2D u_image0;
uniform sampler2D u_image1;

void main() {

    vec2 u_image_size = vec2( 256.,192.);
    vec2 u_image_scale = u_image_size / u_tile_size; // width x height in tiles

    vec2 st = gl_FragCoord.xy/u_resolution.xy; // [0..1]
    vec2 mulSt = st*u_map_size; // [0..map_size-1] float
    vec2 floorMulSt = floor(mulSt); // [0..map_size-1] int

    vec2 textureOffset = mulSt - floorMulSt; // [0..1]
    textureOffset = textureOffset / u_image_scale;

    vec2 cellXY = mulSt / u_map_size; // [0..1]//

    vec4 cell = texture2D(u_image0, cellXY);
    vec2 textureCoord = vec2(cell.x, cell.y);

    textureCoord = ceil(textureCoord * 255.);

    textureCoord = textureCoord / u_image_scale;
    textureCoord = textureCoord + textureOffset;

    // ------------
// some test
//    vec2 textureCoordConst = vec2(5., 5.) / u_image_scale;
//    vec2 textureCoord = textureCoordConst + textureOffset;
// ------------

    vec4 textureColor = texture2D(u_image1, textureCoord);
    gl_FragColor = vec4(textureColor.rgb,1.0);
}
