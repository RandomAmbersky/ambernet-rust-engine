import AmberSkyNet from './amberskynet'

const config = {
  atlas: 'image.png'
}

async function load () {
  const ambernet = new AmberSkyNet(config)
  return true
}

load().then(res => console.log(res))
