import AmberSkyNet from './amberskynet'
import tileImageUrl from './images/tiles_many.png'

const config = {
  atlas: tileImageUrl,
  canvasName: 'canvasGL'
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
