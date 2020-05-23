import VSHADER_SOURCE from './shaders/vert.glsl'
import FSHADER_SOURCE from './shaders/frag.glsl'

import assert from 'assert'
import * as glUtils from './utils-gl'
import * as utils from './utils'

const FPS_DEFAULT = 10.0
const FPS_THROTTLE = 1000.0 / FPS_DEFAULT // milliseconds / frames

class AmberSkyNet {
  constructor ({atlas, canvasName, mapName}) {
    assert(atlas, 'atlas not setup')
    assert(canvasName, 'canvasName not setup')
    assert(mapName, 'mapName not setup')
    this.__atlas = atlas
    this.__canvasName = canvasName
    this.__mapName = mapName
    this.__initialTime = Date.now()
    this.__lastDrawTime = -1
    this.__imageReady = false
  }

  async load () {
    // тут надо еще сделать преобразование карты с тайлами в текстуру и биндинг её в WebGL как текстуру
    const rawMap = await utils.loadFile(this.__mapName)
    const tiled = JSON.parse(rawMap.toString())

    const canvas = document.getElementById(this.__canvasName)
    assert(canvas, 'canvas not found ' + this.__canvasName)

    const gl = canvas.getContext('webgl', {antialias: true})
    assert(gl, 'webgl not supported')

    const layer = tiled.layers[0]
    const layerArray = []
    for (let i = 0; i < layer.width * layer.height; i++) {
      const g = Math.floor(layer.data[i] / layer.height)
      const r = layer.data[i] - g * layer.width
      const b = 0
      const a = 0
      layerArray.push(r)
      layerArray.push(g)
      layerArray.push(b)
      layerArray.push(a)
    }

    const uLayerArray = new Uint8Array(layerArray)

    this.__gl = gl
    this.__canvas = canvas

    this.__layerArray = glUtils.createTexture(gl, uLayerArray, layer.width, layer.height)

    gl.clearColor(0.5, 0.5, 0.5, 1.0)

    this.__prog = glUtils.loadProgram(gl, VSHADER_SOURCE, FSHADER_SOURCE)

    const meshArray = [
      -1.0, -1.0,
      1.0, -1.0,
      -1.0, 1.0,
      -1.0, 1.0,
      1.0, -1.0,
      1.0, 1.0]

    this.__mesh = glUtils.loadBuffer(gl, meshArray)

    const texCoord = [
      0.0, 0.0,
      1.0, 0.0,
      0.0, 1.0,
      0.0, 1.0,
      1.0, 0.0,
      1.0, 1.0]
    this.__tex_coord = glUtils.loadBuffer(gl, texCoord)

    this.__texture = await glUtils.loadTexture(gl, this.__atlas)
    // console.log(this.__texture)

    const colorArray = [
      0.0, 0.0, 0.0,
      1.0, 0.0, 0.0,
      0.0, 1.0, 0.0,
      0.0, 0.0, 1.0,
      1.0, 1.0, 0.0,
      1.0, 1.0, 1.0]
    this.__color_array = glUtils.loadBuffer(gl, colorArray)

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

        // gl.viewport(window.innerWidth / 4, window.innerHeight / 4, window.innerWidth / 2, window.innerHeight / 2)
        gl.viewport(0, 0, 256 * 3, 192 * 3)
      }

      gl.clear(gl.COLOR_BUFFER_BIT)

      gl.useProgram(this.__prog)

      // const colorLocation = gl.getAttribLocation(this.__prog, 'a_color')
      // gl.bindBuffer(gl.ARRAY_BUFFER, this.__color_array)
      // gl.enableVertexAttribArray(colorLocation)
      // gl.vertexAttribPointer(colorLocation, 3, gl.FLOAT, false, 0, 0)

      const positionLocation = gl.getAttribLocation(this.__prog, 'a_position')
      gl.bindBuffer(gl.ARRAY_BUFFER, this.__mesh)
      gl.enableVertexAttribArray(positionLocation)
      gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0)

      const texCoordLocation = gl.getAttribLocation(this.__prog, 'a_texCoord')
      gl.bindBuffer(gl.ARRAY_BUFFER, this.__tex_coord)
      gl.enableVertexAttribArray(texCoordLocation)
      gl.vertexAttribPointer(texCoordLocation, 2, gl.FLOAT, false, 0, 0)

      const uImage0Location = gl.getUniformLocation(this.__prog, 'u_image0')
      const uImage1Location = gl.getUniformLocation(this.__prog, 'u_image1')
      gl.uniform1i(uImage0Location, 0)
      gl.uniform1i(uImage1Location, 1)

      gl.activeTexture(gl.TEXTURE0)
      gl.bindTexture(gl.TEXTURE_2D, this.__layerArray)
      gl.activeTexture(gl.TEXTURE1)
      gl.bindTexture(gl.TEXTURE_2D, this.__texture)

      gl.drawArrays(gl.TRIANGLES, 0, 6)
    }
  }
}

export default AmberSkyNet
