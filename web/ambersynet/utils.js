import fetch from 'fetch'

async function loadFile (url, options) {
  return new Promise((resolve, reject) => {
    fetch.fetchUrl(url, options, (error, meta, body) => {
      if (error) {
        reject(error)
      }
      if (meta.status !== 200) {
        reject(meta)
      }
      resolve(body)
    })
  })
}

function loadImage (src) {
  return new Promise((resolve, reject) => {
    // eslint-disable-next-line no-undef
    const img = new Image()
    img.onload = () => resolve(img)
    img.onerror = reject
    img.src = src
  })
}

exports.loadImage = loadImage
exports.loadFile = loadFile
