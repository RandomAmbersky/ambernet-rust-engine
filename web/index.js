import * as wasm from 'amberskynet'

const FPS_THROTTLE = 1000.0 / 30.0 // milliseconds / frames

const canvas = document.getElementById('canvasGL')


async function loadData () {
  console.log('wasm', wasm)
}

loadData()
  .then( _ => {
    console.log('ok')
  })
  .catch(err => {
    alert(err)
  })
