const path = require("path");

module.exports = {
  mode: 'production',
  entry: {
    main: "./src/main"
  },
  devServer: {
    port: 9000,
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  devtool: "source-map",
  resolve: {
    extensions: ['.js', '.ts']
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: ['/node_modules/']
      },
      { test: /\.tsx?$/, loader: "ts-loader" },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"]
      }
    ]
  }
};
