import {loadBinary} from './utils'

import { AmberSkyNetClient } from '../src/amberskynet/pkg'

const FPS_THROTTLE = 1000.0 / 30.0 // milliseconds / frames

const canvas = document.getElementById('canvasGL')
const engine = new AmberSkyNetClient()

canvas.setAttribute('tabindex','0');
canvas.focus();

canvas.onclick = (e) => {
  engine.on_event(e)
}

canvas.onkeydown = (e) => {
  engine.on_event(e)
}

canvas.onkeyup = (_e) => {
  // engine.on_event(e)
}

canvas.onmousedown = (e) => {
  engine.on_event(e)
}

canvas.onmouseup = (e) => {
  engine.on_event(e)
}

canvas.onmousemove = (e) => {
  engine.on_event(e)
}

canvas.ondrag = (e) => {
  engine.on_event(e)
}

canvas.onwheel = (e) => {
  engine.on_event(e)
}

const data1 = {
  mapName: '/map/cell.tmx',
  tileName: '/map/tiles_mod.png'
}

const data2 = {
  mapName: '/map/laboratory3.tmx',
  tileName: '/map/tiles_many.png'
}

async function loadData (data) {
  const mapArray = await loadBinary(data.mapName)
  const tilesArray = await loadBinary(data.tileName)

  engine.upload_tiles(tilesArray)
  engine.upload_map(mapArray)
}

loadData(data2)
  .then( _ => {
    renderLoop()
  })
  .catch(err => {
    console.error(err)
  })

let lastDrawTime = Date.now();// In milliseconds

const update = (currTime) => {
  let elapsedTime = currTime - lastDrawTime
  lastDrawTime = currTime

  if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
    canvas.height = window.innerHeight
    canvas.width = window.innerWidth
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
