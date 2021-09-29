import module  from '../crate/Cargo.toml'

import VSHADER_SOURCE from './amberskynet/shaders/test_2d/vert.glsl'
import FSHADER_SOURCE from './amberskynet/shaders/test_2d/frag.glsl'
const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames

import myFont from './amberskynet/fonts/LiberationMono-Regular.ttf'

const canvas = document.getElementById('canvasGL');
const engine = new module.AmberApi()

engine.upload_render_program(VSHADER_SOURCE, FSHADER_SOURCE)

const binaryFontLoader = new XMLHttpRequest();
binaryFontLoader.open("GET", myFont, true);
binaryFontLoader.responseType = "arraybuffer";
binaryFontLoader.send()
binaryFontLoader.onload = function (oEvent) {
  const arrayBuffer = binaryFontLoader.response;
  if (arrayBuffer) {
    const byteArray = new Uint8Array(arrayBuffer)
    engine.upload_font(byteArray)
  }
}

let lastDrawTime = Date.now();// In milliseconds

const update = (currTime) => {
  let elapsedTime = currTime - lastDrawTime;
  lastDrawTime = currTime;

  if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
    canvas.height = window.innerHeight
    canvas.width = window.innerWidth
    console.log(canvas.width, canvas.height)
    engine.resize(canvas.width, canvas.height)
  }
  engine.update(elapsedTime)
  engine.render()
}

function renderLoop() {
  const currTime = Date.now();
  if (currTime >= lastDrawTime + FPS_THROTTLE) {
    update(currTime)
  }
  window.requestAnimationFrame(renderLoop);
}

renderLoop()
