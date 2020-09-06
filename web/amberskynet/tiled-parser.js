const assert = require('assert')

function parseLayer (layer) {
  return {
    w: layer.width,
    h: layer.height,
    data: layer.data
  }
}

function parseLayers (map) {
  assert(map.layers, 'map.layers not found')
  const layers = map.layers
  const layer = layers[0]
  return parseLayer(layer)
}

function parseMap (map) {
  return {
    layers: parseLayers(map)
  }
}

exports.parseMap = parseMap
