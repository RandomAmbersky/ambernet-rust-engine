pub const VERTICES: [f32; 12] = [
	-1.0, 1.0,
	1.0, -1.0,
	-1.0, -1.0,
	-1.0, 1.0,
	1.0, -1.0,
	1.0, 1.0
];

pub const VERTEX_SHADER: &str = r#"
attribute vec2 aPosition;
void main() {
	vec2 pos = aPosition;
	gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}
"#;

pub const FRAG_SHADER: &str = r#"
precision mediump float;
uniform vec2 uImageSize; // tiles image width x height in pixels
uniform vec2 uTileSize; // one tile width x height in pixels
uniform vec2 uResolution; // screen width x height in pixels
uniform vec2 uMapSize; // map width x height in tiles

uniform sampler2D u_image0;
uniform sampler2D u_image1;

void main() {
    vec2 u_image_size = vec2( 256.,192.);
    vec2 u_image_scale = u_image_size / uTileSize; // width x height in tiles

    vec2 st = gl_FragCoord.xy / uResolution.xy; // [0..1]
    vec2 mulSt = st * uMapSize; // [0..map_size-1] float
    vec2 floorMulSt = floor(mulSt); // [0..map_size-1] int

		vec2 textureOffset = mulSt - floorMulSt; // [0..1]
    textureOffset = textureOffset / u_image_scale - 0.00001;

    vec2 cellXY = mulSt / uMapSize; // [0..1]//

    vec4 cell = texture2D(u_image0, cellXY);
    vec2 textureCoord = vec2(cell.x, cell.y);

    textureCoord = ceil(textureCoord * 255.);

    textureCoord = textureCoord / u_image_scale;
    textureCoord.x = textureCoord.x + textureOffset.x;
    textureCoord.y = textureCoord.y + 1./u_image_scale.y  - textureOffset.y;

	  // vec2 textureCoordConst = vec2(9., 9.) / u_image_scale;
    // vec2 textureCoord = textureCoordConst + textureOffset;

    vec4 textureColor = texture2D(u_image1, textureCoord);
    gl_FragColor = vec4(textureColor.rgb,1.0);
}
"#;
