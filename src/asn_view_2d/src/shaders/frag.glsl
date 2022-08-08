precision highp float;
uniform sampler2D uTileMap;
uniform sampler2D uTileSheet;
varying vec4 worldCoord;
void main() {
    vec2 mapSize   = vec2(32., 32.);
    vec2 sheetSize = vec2(256., 192.);
    vec2 tileSize  = vec2(16., 16.);
    float isOk = 0.;

    vec2 tex_coord = worldCoord.xy;// 0..1
    // tex_coord = gl_PointCoord;
    tex_coord.y = 1.0 - tex_coord.y;

    vec2 uv = tex_coord;// 0..1

    vec2 mapCoord = floor(uv * mapSize) / mapSize;// 0...mapSize => 0...1 разделенные на n частей

    vec2 tileOffset = fract(tex_coord * mapSize);
    tileOffset.y = tileOffset.y * 256.0 / 192.0;

//    tileOffset.x = floor( 32.0 * tileOffset.x ) / 32.0;
//    tileOffset.y = floor( 32.0 * tileOffset.y ) / 32.0;
    // if( mapCoord.x > 0.5 ) {
    // 	isOk = 1.0;
    // }


    vec2 tileXY = floor( 256.0 * texture2D(uTileMap, mapCoord).xy ) / 256.0;

//    tileXY = vec2(3., 11.) / 256.0;
    tileXY = tileXY * 256.0;

    // проверили что по координатам 0,0 действительно находятся данные [11,9]
    // vec2 tileXY_checking = tileXY * 255.0;
    // if (tileXY_checking.y * 16.0 + tileXY_checking.x == 155.0) {
    // 	isOk = 1.0;
    // } else {
    // 	isOk = 0.;
    // };

//     if (tileOffset.x > 0.5) {
//     	isOk = 1.0;
//     }

    vec2 sheetCoord = tileXY * tileSize / sheetSize + tileOffset / tileSize;

//    if (sheetCoord.x > 1.0) {
//        isOk = 1.0;
//    }
//    if (sheetCoord.y > 1.0) {
//     	isOk = 1.0;
//    }

    vec4 textureColor = texture2D(uTileSheet, sheetCoord.xy) * 1. + vec4(isOk, 0, 0, 1) * 1.;
    gl_FragColor = vec4(textureColor.rgb, 1.0);
}
