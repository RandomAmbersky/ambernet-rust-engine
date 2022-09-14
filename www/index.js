import {loadBinary} from './utils'

import { AmberSkyNetClient } from '../src/amberskynet/pkg'

const FPS_THROTTLE = 1000.0 / 30.0 // milliseconds / frames

const canvas = document.getElementById('canvasGL')
const engine = new AmberSkyNetClient()

canvas.setAttribute('tabindex','0');
canvas.focus();

canvas.onclick = (e) => {
  console.log(e)
}

canvas.onkeydown = (e) => {
  console.log(e)
}

canvas.onkeyup = (e) => {
  console.log(e)
}

canvas.onmousedown = (e) => {
  console.log(e)
}

canvas.onmouseup = (e) => {
  console.log(e)
}

canvas.onmousemove = (e) => {
  console.log(e)
}

canvas.ondrag = (e) => {
  console.log(e)
}

canvas.onwheel = (e) => {
  console.log(e)
}

const data1 = {
  map: {
    cellSizeX: 32,
    cellSizeY: 32
  },
  sheet: {
    pixelSizeX: 256,
    pixelSizeY: 192
  },
  tiles: {
    pixelSizeX: 16,
    pixelSizeY: 16
  },
  mapName: '/map/cell.tmx',
  tileName: '/map/tiles_mod.png'
}

const data2 = {
  map: {
    cellSizeX: 32,
    cellSizeY: 32
  },
  sheet: {
    pixelSizeX: 256,
    pixelSizeY: 192
  },
  tiles: {
    pixelSizeX: 16,
    pixelSizeY: 16
  },
  mapName: '/map/laboratory3.tmx',
  tileName: '/map/tiles_many.png'
}

async function loadData (data) {

  const mapArray = await loadBinary(data.mapName)
  const tilesArray = await loadBinary(data.tileName)
  engine.upload_tiles(tilesArray)
  engine.upload_map(mapArray)
}

loadData(data1)
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
