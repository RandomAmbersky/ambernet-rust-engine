const assert = require('assert')

export default class AmberSkyNet {
  constructor (config) {
    assert(config.atlas, 'atlas not setup')
    this.__atlas = config.atlas
  }

  async load () {

  }
}
