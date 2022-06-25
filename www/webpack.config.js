const CopyPlugin = require("copy-webpack-plugin");
const webpack = new CopyPlugin({
  patterns: [
    "index.html",
  ],
  options: {
    concurrency: 100,
  },
})

const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
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
    webpack
  ],
};
