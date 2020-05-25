import * as utils from './utils'

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
  gl.deleteShader(VS)
  gl.deleteShader(FS)
  return shaderProgram
}

async function loadTexture (gl, src) {
  const img = await utils.loadImage(src)

  const texture = gl.createTexture()

  gl.bindTexture(gl.TEXTURE_2D, texture)
  gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, img)
  gl.bindTexture(gl.TEXTURE_2D, null)
  img.src = null
  return texture
}

function createTexture (gl, arr, width, height) {
  const texture = gl.createTexture()

  gl.bindTexture(gl.TEXTURE_2D, texture)
  gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
  gl.texImage2D(
    gl.TEXTURE_2D, // target
    0, // mip level
    gl.RGBA, // internal format
    width, height, // width and height
    0, // border
    gl.RGBA, // format
    gl.UNSIGNED_BYTE, // type
    arr // texture data
  )
  gl.bindTexture(gl.TEXTURE_2D, null)
  return texture
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
exports.loadBuffer = loadBuffer
exports.loadTexture = loadTexture
exports.createTexture = createTexture
