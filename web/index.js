import 'babel-polyfill'
import app from './app'


// const AmberSkyNet = await import('./amberskynet')
//
// const render = require('./render')
//
// const VSHADER_SOURCE = `
//   void main() {
//    gl_PointSize = 10.0;
//    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
//   }`
//
// const FSHADER_SOURCE = `
//   precision mediump float;
//   void main() {
//     gl_FragColor = vec4(1.0, 0.5, 0.0, 1.0);
//   }`
////
// async function webGLStart () {
//   console.log(ambernet)
//
//   const canvas = document.getElementById('canvasGL')
//   if (!canvas) {
//     console.log('failed')
//     return false
//   }
//   canvas.width = 400
//   canvas.height = 400
//
//   let gl
//   try {
//     gl = canvas.getContext('webgl', {antialias: false})
//   } catch (e) {
//     console.log('You are not webgl compatible :(')
//     return false
//   }
//
//   const prog = render.loadProgram(gl, VSHADER_SOURCE, FSHADER_SOURCE)
//   gl.useProgram(prog)
//
//   gl.clearColor(0.5, 0.5, 0.5, 1.0)
//   gl.clear(gl.COLOR_BUFFER_BIT)
//   gl.drawArrays(gl.POINTS, 0, 1)
//   return true
// }
//
// webGLStart()
//   .then(res => {
//     console.log(res)
//   })
//   .catch(err => {
//     console.log(err)
//   })
