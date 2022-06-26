const CopyPlugin = require("copy-webpack-plugin");
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

const path = require('path');

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
  // mode: "development",
  mode: 'production',
  plugins: [
    copyPlugin
  ],
};
