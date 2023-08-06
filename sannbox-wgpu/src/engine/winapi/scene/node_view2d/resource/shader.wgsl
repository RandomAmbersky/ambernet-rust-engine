// Vertex shader

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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var isOk = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    var uMapSize = 32.0;
    var utileSize = 16.0;
    var uSheetSize = vec2<f32>(256.0, 192.0);
    var MAX_COLOR_VALUE_256 = 256.0; //  color 0..1 * 256.0 => 0..256

    var tex_coords = in.tex_coords; // 0..1

//  https://thebookofshaders.com/glossary/?search=floor
    var map_coord = floor( tex_coords * uMapSize ) / uMapSize; // координаты нарезаны на uMapSize кусков

    var tile_XY = floor( MAX_COLOR_VALUE_256 * textureSample(t_map, s_map, tex_coords).xy); // x and y on tile map in cells

    tile_XY = tile_XY * utileSize; // x and y on tile map in pixels
    tile_XY = tile_XY / uSheetSize; // 0..1 normalize

    var tile_Offset = fract(in.tex_coords * uMapSize); // 0..1 повторяемые uMapSize раз
    tile_Offset.y = tile_Offset.y * uSheetSize.x / uSheetSize.y;

//    tile_Offset.x = floor( tile_Offset.x * 255.0 ) / 255.0;
//    tile_Offset.y = floor( tile_Offset.y * 255.0 ) / 255.0;

    var sheet_Coord = tile_XY + tile_Offset / utileSize;

    return textureSample(t_texture, s_texture, sheet_Coord);
}
