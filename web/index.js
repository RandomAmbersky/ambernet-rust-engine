import module from '../src/amberskynet/Cargo.toml'

const FPS_THROTTLE = 1000.0 / 30.0 // milliseconds / frames

const canvas = document.getElementById('canvasGL')
const engine = new module.AmberSkyNetClient()

async function load (uri, responseType = "text") {
  return new Promise( function (resolve, reject){
    const xhr = new XMLHttpRequest()
    xhr.open("GET", uri, true)
    xhr.responseType = responseType;
    xhr.onload = function () {
      if (this.status >= 200 && this.status < 300) {
        resolve(xhr.response)
      } else {
        reject({
          status: xhr.status,
          statusText: xhr.statusText
        })
      }
    }
    xhr.onerror = function () {
      reject({
        status: xhr.status,
        statusText: xhr.statusText
      })
    }
    xhr.send()
  })
}

async function loadData () {
  const tiles_buf = await load('/map/tiles_many.png')
  const map_buf = await load('/map/laboratory3.json')

  const mapArray = new Uint8Array(map_buf.response)
  const tilesArray = new Uint8Array(tiles_buf.response)

  engine.upload_tiles(tilesArray)
  engine.upload_map(mapArray)
}

loadData()
  .then( _ => {
    renderLoop()
  })
  .catch(err => {
    alert(err)
  })

let lastDrawTime = Date.now();// In milliseconds

const update = (currTime) => {
  let elapsedTime = currTime - lastDrawTime
  lastDrawTime = currTime

  if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
    canvas.height = window.innerHeight
    canvas.width = window.innerWidth
    // console.log(canvas.width, canvas.height)
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
