const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

const appConfig = {
  entry: "./app/main.js",
  devServer: {
    contentBase: dist
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html"
    })
  ],
  resolve: {
    extensions: [".js"]
  },
  output: {
    path: dist,
    filename: "app.js"
  }
};

const workerConfig = {
  entry: "./worker/worker.js",
  target: "webworker",
  plugins: [

  ],
  resolve: {
    extensions: [".js", ".wasm"]
  },
  output: {
    path: dist,
    filename: "worker.js"
  }
};

module.exports = [appConfig, workerConfig];
