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

exports.loadFile = loadFile
