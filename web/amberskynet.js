import fs from 'fs'

const VSHADER_SOURCE = fs.readFileSync(__dirname + '/shaders/vert.glsl', 'utf8')
const FSHADER_SOURCE = fs.readFileSync(__dirname + '/shaders/frag.glsl', 'utf8')

const assert = require('assert')
const utils = require('./gl-utils')

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
    this.__imageReady = false
  }

  async load () {
    const canvas = document.getElementById(this.__canvasName)
    assert(canvas, 'canvas not found ' + this.__canvasName)

    const gl = canvas.getContext('webgl', {antialias: true})
    assert(gl, 'webgl not supported')

    this.__gl = gl
    this.__canvas = canvas

    gl.clearColor(0.5, 0.5, 0.5, 1.0)

    this.__prog = utils.loadProgram(gl, VSHADER_SOURCE, FSHADER_SOURCE)

    const meshArray = [
      -1.0, -1.0,
      1.0, -1.0,
      -1.0, 1.0,
      -1.0, 1.0,
      1.0, -1.0,
      1.0, 1.0]

    this.__mesh = utils.loadBuffer(gl, meshArray)

    const texCoord = [
      0.0, 0.0,
      1.0, 0.0,
      0.0, 1.0,
      0.0, 1.0,
      1.0, 0.0,
      1.0, 1.0]
    this.__tex_coord = utils.loadBuffer(gl, texCoord)

    const tileImage = await utils.loadImage(this.__atlas)

    this.__texture = gl.createTexture()
    gl.bindTexture(gl.TEXTURE_2D, this.__texture)

    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, tileImage)
    gl.bindTexture(gl.TEXTURE_2D, null)

    return true
  }

  renderLoop () {
    const gl = this.__gl
    const canvas = this.__canvas

    window.requestAnimationFrame(this.renderLoop.bind(this))
    const currTime = Date.now()
    if (currTime >= this.__lastDrawTime + FPS_THROTTLE) {
      this.__lastDrawTime = currTime

      if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
        canvas.height = window.innerHeight
        canvas.style.height = window.innerHeight.toString()

        canvas.width = window.innerWidth
        canvas.style.width = window.innerWidth.toString()
        // canvas.requestFullscreen()

        const devicePixelRatio = window.devicePixelRatio || 1
        console.log(devicePixelRatio)

        gl.viewport(window.innerWidth / 4, window.innerHeight / 4, window.innerWidth / 2, window.innerHeight / 2)
        // gl.viewport(0, 0, window.innerWidth, window.innerHeight)
      }

      gl.clear(gl.COLOR_BUFFER_BIT)

      const positionLocation = gl.getAttribLocation(this.__prog, 'a_position')
      gl.bindBuffer(gl.ARRAY_BUFFER, this.__mesh)
      gl.enableVertexAttribArray(positionLocation)
      gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0)

      const texCoordLocation = gl.getAttribLocation(this.__prog, 'a_texCoord')
      gl.bindBuffer(gl.ARRAY_BUFFER, this.__tex_coord)
      gl.enableVertexAttribArray(texCoordLocation)
      gl.vertexAttribPointer(texCoordLocation, 2, gl.FLOAT, false, 0, 0)

      gl.bindTexture(gl.TEXTURE_2D, this.__texture)

      gl.useProgram(this.__prog)
      gl.drawArrays(gl.TRIANGLES, 0, 6)
    }
  }
}

export default AmberSkyNet
