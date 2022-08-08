pub const VERTICES: [f32; 12] = [
	0., 1.0,
	1.0, 0.,
	0., 0.,
	0., 1.0,
	1.0, 0.,
	1.0, 1.0
];

pub const VERTEX_SHADER: &str = r#"
attribute vec2 aPosition;
uniform mat4   uTransform;
varying vec4 worldCoord;
void main() {
	gl_Position = vec4(aPosition.x, aPosition.y, 0, 1);
	worldCoord = uTransform * gl_Position;
}
"#;

pub const FRAG_SHADER: &str = r#"
precision highp float;
uniform sampler2D uTileMap;
uniform sampler2D uTileSheet;
varying vec4 worldCoord;
void main() {
		vec2 mapSize   = vec2(32., 32.);
		vec2 sheetSize = vec2(256., 192.);
		vec2 tileSize  = vec2(16., 16.);

		vec2 tex_coord = worldCoord.xy / 4.0;
		// tex_coord.y = 1.0 - tex_coord.y;

		vec2 uv = tex_coord.xy; // / worldCoord.z;   // 0...1

		vec2 mapCoord = floor(uv * mapSize) / mapSize; // 0...mapSize => 0...1 разделенные на n частей
		vec2 tileOffset = (uv - mapCoord) * 2.0;

		vec2 tileId = floor(255.0 * texture2D(uTileMap, mapCoord).rg) / 255.0;
		tileId = texture2D(uTileMap, mapCoord).rg;
		tileId = vec2(2., 2.) / 256.0;

		// tileOffset.y = tileOffset.y * sheetSize.y / sheetSize.x;

    vec2 sheetCoord = tileId + tileOffset * 2.0;

    vec4 textureColor = texture2D(uTileSheet, sheetCoord) * 1. + vec4(sheetCoord.x, sheetCoord.y, 0, 1) * 0.;

    // textureColor = texture2D(uTileSheet, sheetCoord);
    gl_FragColor = vec4(textureColor.rgb,1.0);
}
"#;
