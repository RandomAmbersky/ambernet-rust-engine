attribute vec2 aPosition;
uniform mat4   uTransform;
varying vec4 worldCoord;

void main() {
    gl_Position = uTransform * vec4(aPosition.x, aPosition.y, 0, 1);
    worldCoord =  gl_Position + 1.0;
}
