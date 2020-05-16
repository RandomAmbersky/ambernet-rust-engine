const VSHADER_SOURCE =
  'void main() {\n' +
  ' gl_PointSize = 10.0;\n' +
  ' gl_Position = vec4(0.0, 0.0, 0.0, 1.0);\n' +
  ' }\n'

const FSHADER_SOURCE =
  ' precision mediump float;\n' +
  ' void main() {\n' +
  ' gl_FragColor = vec4(1.0, 0.5, 0.0, 1.0);\n' +
  ' }\n'

function getShader(gl, id, str){
  let shader
  if(id === 'vs'){
    shader = gl.createShader(gl.VERTEX_SHADER)
  }else if(id==='fs'){
    shader = gl.createShader(gl.FRAGMENT_SHADER)
  }else {
    return null
  }

  gl.shaderSource(shader, str)
  gl.compileShader(shader)

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    alert(gl.getShaderInfoLog(shader))
    return null
  }
  return shader
}

function initShaders(gl){
  const VS = getShader(gl, 'vs', VSHADER_SOURCE)
  const FS = getShader(gl, 'fs', FSHADER_SOURCE)

  shaderProgram = gl.createProgram()
  gl.attachShader(shaderProgram,VS)
  gl.attachShader(shaderProgram,FS)
  gl.linkProgram(shaderProgram)
  gl.useProgram(shaderProgram)
}

function webGLStart() {

  const canvas = document.getElementById('canvasGL')
  if (!canvas){
    console.log('failed')
    return
  }
  canvas.width =400
  canvas.height=400

  let gl
  try {
    gl = canvas.getContext("webgl", {antialias: false})
  } catch (e) {
    alert("You are not webgl compatible :(")
    return false
  }

  initShaders(gl)

  gl.clearColor(0.5,0.5,0.5,1.0)
  gl.clear(gl.COLOR_BUFFER_BIT)
  gl.drawArrays(gl.POINTS, 0, 1)
}

webGLStart()
