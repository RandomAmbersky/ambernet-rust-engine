import AmberSkyNet from './amberskynet'

const config = {
  atlas: 'image.png',
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
