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

    vec2 tileOffset = tex_coord;
    tileOffset.y = tileOffset.y / 192.0 * 255.0;

    // if( mapCoord.x > 0.5 ) {
    // 	isOk = 1.0;
    // }

    vec2 tileXY = texture2D(uTileMap, mapCoord).xy;
    tileXY = vec2(11., 9.) * 16.0 / 255.0;

    // проверили что по координатам 0,0 действительно находятся данные [11,9]
    // vec2 tileXY_checking = tileXY * 255.0;
    // if (tileXY_checking.y * 16.0 + tileXY_checking.x == 155.0) {
    // 	isOk = 1.0;
    // } else {
    // 	isOk = 0.;
    // };

    // if (tileOffset.x > 0.5) {
    // 	isOk = 1.0;
    // }

    vec2 sheetCoord = vec2(1., 0.) * 16.0 / 255.0 + tileOffset / 16.0;

    // if (sheetCoord.x > 1.0) {
    // 	isOk = 1.0;
    // }
    // if (sheetCoord.y > 1.0) {
    // 	isOk = 1.0;
    // }

    vec4 textureColor = texture2D(uTileSheet, sheetCoord.xy) * 1. + vec4(isOk, 0, 0, 1) * 1.;

    // textureColor = texture2D(uTileSheet, sheetCoord);
    gl_FragColor = vec4(textureColor.rgb, 1.0);
}
