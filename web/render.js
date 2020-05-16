function getShader (gl, id, str) {
  let shader
  if (id === 'vs') {
    shader = gl.createShader(gl.VERTEX_SHADER)
  } else if (id === 'fs') {
    shader = gl.createShader(gl.FRAGMENT_SHADER)
  } else {
    return null
  }

  gl.shaderSource(shader, str)
  gl.compileShader(shader)

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    console.log(gl.getShaderInfoLog(shader))
    return null
  }
  return shader
}

function loadProgram (gl, vs, fs) {
  const VS = getShader(gl, 'vs', vs)
  const FS = getShader(gl, 'fs', fs)

  const shaderProgram = gl.createProgram()
  gl.attachShader(shaderProgram, VS)
  gl.attachShader(shaderProgram, FS)
  gl.linkProgram(shaderProgram)
  return shaderProgram
}

exports.loadProgram = loadProgram
