// Vertex shader
struct MapSetupUniform {
    uMapSize: vec2<f32>,
    uTileSize: vec2<f32>,
    uSheetSize: vec2<f32>,
    MAX_COLOR_VALUE: f32
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_map: texture_2d<f32>;
@group(0)@binding(1)
var s_map: sampler;
@group(0) @binding(2)
var t_texture: texture_2d<f32>;
@group(0)@binding(3)
var s_texture: sampler;
@group(1)@binding(0)
var<uniform> map_setup: MapSetupUniform;
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var uMapSize = map_setup.uMapSize;     //  vec2<f32>(32.0, 32.0);
    var uTileSize = map_setup.uTileSize;   //  vec2<f32>(16.0, 16.0);
    var uSheetSize = map_setup.uSheetSize; //  vec2<f32>(256.0, 192.0);
//    var MAX_COLOR_VALUE = map_setup.MAX_COLOR_VALUE; //256.0 * 2.0 * 2.0 * 2.0 * 2.0; //  color 0..1 * MAX_COLOR_VALUE => 0..MAX_COLOR_VALUE
    var MAX_COLOR_VALUE = 256.0;

    var isOk = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    var tex_coords = in.tex_coords; // 0..1

//  https://thebookofshaders.com/glossary/?search=floor
    var map_coord = floor( tex_coords * uMapSize ) / uMapSize; // координаты нарезаны на uMapSize кусков

    var tile_XY = floor( MAX_COLOR_VALUE * textureSample(t_map, s_map, tex_coords).xy); // x and y on tile map in cells
//    isOk.x = tile_XY.x;
//    isOk.y = tile_XY.y;

    tile_XY = tile_XY * uTileSize; // x and y on tile map in pixels
    tile_XY = tile_XY / uSheetSize; // 0..1 normalize

    var tile_Offset = fract(in.tex_coords * uMapSize); // 0..1 повторяемые uMapSize раз
    tile_Offset.y = tile_Offset.y * uSheetSize.x / uSheetSize.y;

    tile_Offset.x = floor( tile_Offset.x * 255.0 ) / 255.0;
    tile_Offset.y = floor( tile_Offset.y * 255.0 ) / 255.0;

    var sheet_Coord = tile_XY + tile_Offset / uTileSize;

    return textureSample(t_texture, s_texture, sheet_Coord);
//    return isOk;
}

