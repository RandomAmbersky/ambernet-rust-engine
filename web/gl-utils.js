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
  const res = gl.getShaderInfoLog(shader)
  console.log(res)

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
  gl.deleteShader(VS)
  gl.deleteShader(FS)
  return shaderProgram
}

function loadImage (src) {
  return new Promise((resolve, reject) => {
    // eslint-disable-next-line no-undef
    let img = new Image()
    img.onload = () => resolve(img)
    img.onerror = reject
    img.src = src
  })
}

function loadBuffer (gl, arr) {
  const buf = gl.createBuffer()
  gl.bindBuffer(gl.ARRAY_BUFFER, buf)
  gl.bufferData(
    gl.ARRAY_BUFFER,
    new Float32Array(arr),
    gl.STATIC_DRAW)
  gl.bindBuffer(gl.ARRAY_BUFFER, null)
  return buf
}

exports.loadProgram = loadProgram
exports.loadImage = loadImage
exports.loadBuffer = loadBuffer
