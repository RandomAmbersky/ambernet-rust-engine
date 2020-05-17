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
    gl_FragColor = vec4(1.0, 0.5, 1.0, 1.0);
  }`

const FPS_DEFAULT = 10.0
const FPS_THROTTLE = 1000.0 / FPS_DEFAULT // milliseconds / frames

class AmberSkyNet {
  constructor ({atlas, canvasName}) {
    assert(atlas, 'atlas not setup')
    assert(canvasName, 'canvasName not setup')
    this.__atlas = atlas
    this.__canvasName = canvasName
    this.__initialTime = Date.now()
    this.__lastDrawTime = -1
  }

  async load () {
    const canvas = document.getElementById(this.__canvasName)
    assert(canvas, 'canvas not found' + this.__canvasName)

    const gl = canvas.getContext('webgl', {antialias: true})
    assert(canvas, 'gl not supported ')

    this.__gl = gl
    this.__canvas = canvas

    this.__prog = render.loadProgram(gl, VSHADER_SOURCE, FSHADER_SOURCE)

    // gl.useProgram(this.__prog)
    // gl.clearColor(0.5, 0.5, 0.5, 1.0)
    // gl.clear(gl.COLOR_BUFFER_BIT)
    // gl.drawArrays(gl.POINTS, 0, 1)

    return true
  }

  renderLoop () {
    const gl = this.__gl
    const canvas = this.__canvas

    window.requestAnimationFrame(this.renderLoop.bind(this))
    const currTime = Date.now()
    if (currTime >= this.__lastDrawTime + FPS_THROTTLE) {
      console.log(currTime)
      this.__lastDrawTime = currTime

      if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
        canvas.height = window.innerHeight
        canvas.style.height = window.innerHeight.toString()

        canvas.width = window.innerWidth
        canvas.style.width = window.innerWidth.toString()

        gl.viewport(0, 0, window.innerWidth, window.innerHeight)
      }

      gl.useProgram(this.__prog)
      gl.clearColor(0.5, 0.5, 0.5, 1.0)
      gl.clear(gl.COLOR_BUFFER_BIT)
      gl.drawArrays(gl.POINTS, 0, 1)
    }
  }
}

export default AmberSkyNet
