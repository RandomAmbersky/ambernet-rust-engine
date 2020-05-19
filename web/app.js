import AmberSkyNet from './amberskynet'
import tileImageUrl from './map/tiles_many.png'

const config = {
  atlas: tileImageUrl,
  canvasName: 'canvasGL',
  mapName: document.baseURI + 'map/laboratory3.json'
}

async function load () {
  const a = new AmberSkyNet(config)
  await a.load()
  a.renderLoop()
  return true
}

load().then(
  res => console.log(res)
)
