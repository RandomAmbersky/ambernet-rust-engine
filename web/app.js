import AmberSkyNet from './amberskynet'

const config = {
  atlas: 'image.png',
  canvasName: 'canvasGL'
}

async function load () {
  const ambernet = new AmberSkyNet(config)
  await ambernet.load()
  return true
}

load().then(res => console.log(res))
