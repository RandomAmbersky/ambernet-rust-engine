// const webpack = require("webpack")
const CopyPlugin = require("copy-webpack-plugin")
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

const path = require('path')

const copyPlugin = new CopyPlugin({
  patterns: [
    "index.html",
    {
      from: "map",
      to: "map"
    }
  ],
  options: {
    concurrency: 100,
  },
})

const wasmPackPlugin = new WasmPackPlugin({
  crateDirectory: path.resolve(__dirname, '../src/amberskynet'),
  watchDirectories: [
    path.resolve(__dirname, "../src/renders/tests/textured_quad"),
    path.resolve(__dirname, "../src/asn_render_webgl"),
    path.resolve(__dirname, "../src/asn_view_2d"),
    path.resolve(__dirname, "../src/asn_tiled")
  ],
})

// const providePlugin = new webpack.ProvidePlugin({
//   Buffer: ['buffer', 'Buffer'],
// })

module.exports = {
  entry: "./bootstrap.js",
  watchOptions: {
    aggregateTimeout: 500
    // poll: 200, is not necessary as long as you remove pkg/* before building your wasm files
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  },
  performance: {
    hints: false,
    maxEntrypointSize: 512000,
    maxAssetSize: 512000
  },
  // mode: "development",
  mode: 'production',
  plugins: [
    copyPlugin,
    wasmPackPlugin,
    // providePlugin
  ],
};
