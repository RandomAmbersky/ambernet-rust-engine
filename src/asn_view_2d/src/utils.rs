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
		vec2 mapSize   = vec2(32, 32);
		vec2 sheetSize = vec2(256, 192);
		vec2 tileSize  = vec2(16, 16);

		vec2 uv = worldCoord.xy / worldCoord.z;
		vec2 mapCoord = floor(uv);
		vec2 tileOffset = fract(uv);
		vec2 tileId = floor(255.0 * texture2D(uTileMap, mapCoord / mapSize).ra);
    vec2 sheetCoord = (tileId + tileOffset) * (tileSize / sheetSize);
    gl_FragColor = texture2D(uTileSheet, sheetCoord);
}
"#;
