const assert = require('assert')

class AmberSkyNet {
  constructor ({atlas}) {
    assert(atlas, 'atlas not setup')
    this.__atlas = atlas
  }
  async load () {
  }
}

export default AmberSkyNet
