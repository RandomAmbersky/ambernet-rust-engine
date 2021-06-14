import module  from '../crate/Cargo.toml'

import VSHADER_SOURCE from './amberskynet/shaders/vert.glsl'
import FSHADER_SOURCE from './amberskynet/shaders/frag.glsl'
const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames

const canvas = document.getElementById('canvasGL');
const engine = new module.AmberSkyNetClient()

engine.upload_program(VSHADER_SOURCE, FSHADER_SOURCE)

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
