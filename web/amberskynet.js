const assert = require('assert')
const render = require('./render')


const VSHADER_SOURCE = `
  void main() {
   gl_PointSize = 10.0;
   gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
  }`

const FSHADER_SOURCE = `
  precision mediump float;
  void main() {
    gl_FragColor = vec4(1.0, 0.5, 0.0, 1.0);
  }`

class AmberSkyNet {
  constructor ({atlas, canvasName}) {
    assert(atlas, 'atlas not setup')
    assert(canvasName, 'canvasName not setup')
    this.__atlas = atlas
    this.__canvasName = canvasName
  }
  async load () {
    const gl = render.getContext(this.__canvasName)
    const prog = render.loadProgram(gl, VSHADER_SOURCE, FSHADER_SOURCE)
    gl.useProgram(prog)

    gl.clearColor(0.5, 0.5, 0.5, 1.0)
    gl.clear(gl.COLOR_BUFFER_BIT)
    gl.drawArrays(gl.POINTS, 0, 1)
    return true
  }
}

export default AmberSkyNet
