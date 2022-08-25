precision highp float;
uniform vec2 uMapSize;
uniform vec2 uSheetSize;
uniform vec2 utileSize;

uniform sampler2D uTileMap;
uniform sampler2D uTileSheet;

varying vec4 worldCoord;

void main() {
    float MAX_COLOR_VALUE_256 = 256.0; //  color 0..1 * 256.0 => 0..256

    float isOk = 0.;

    vec2 tex_coord = worldCoord.xy / 2.0; // 0..1
//    if( tex_coord.x > 0.1 ) {
//        isOk = 1.0;
//    }

//    if( gl_PointCoord.x < 0. ) {
//        isOk = 1.0;
//    }

    tex_coord.y = 1.0 - tex_coord.y;

    vec2 mapCoord = floor(tex_coord * uMapSize) / uMapSize;// 0...1 разделенные на uMapSize частей
//     if( mapCoord.x > 0.5 ) {
//     	isOk = 1.0;
//     }

//    mapCoord = vec2 (0., 0.) / uMapSize;
//    vec2 check_tileXY = texture2D(uTileMap, mapCoord).xy;
//    if( check_tileXY.x > 0.05 ) {
//        isOk = 1.0;
//    }

//  256 - max value of color(0..1) * 256.0
    vec2 tileXY = floor( MAX_COLOR_VALUE_256 * texture2D(uTileMap, mapCoord).xy ); // x and y on tile map in cells

//     проверили что по координатам 0,0 действительно находятся данные [11,9]
//     vec2 tileXY_checking = tileXY;
//     if (tileXY_checking.y * uSheetSize.x / utileSize.x + tileXY_checking.x == 155.0) {
//     	isOk = 1.0;
//     } else {
//     	isOk = 0.;
//     };

    tileXY = tileXY * utileSize; // x and y on tile map in pixels
    tileXY = tileXY / uSheetSize; // 0..1 normalize

    vec2 tileOffset = fract(tex_coord * uMapSize); // 0..1 повторяемые uMapSize раз
    tileOffset.y = tileOffset.y * uSheetSize.x / uSheetSize.y;

    tileOffset.x = floor( tileOffset.x * 255.0 ) / 255.0;
    tileOffset.y = floor( tileOffset.y * 255.0 ) / 255.0;
//    if( tileOffset.x > 0.9 ) {
//        isOk = 1.0;
//    }

    vec2 sheetCoord = tileXY + tileOffset / utileSize;

//    if (sheetCoord.x > 1.0) {
//        isOk = 1.0;
//    }
//    if (sheetCoord.y > 1.0) {
//     	isOk = 1.0;
//    }

    vec4 textureColor = texture2D(uTileSheet, sheetCoord.xy) * 1. + vec4(isOk, 0, 0, 1) * 1.;

    gl_FragColor = vec4(textureColor.rgb, 1.0);
}
