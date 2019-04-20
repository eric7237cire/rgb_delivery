const path = require('path');
//const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {

  resolve: {
    alias: {
      RustRGBProject: path.resolve(__dirname, '../rgb-solver')
    },
    extensions: [ '*.d.ts', '.wasm', '.js', '.json', '.rs', '*', '.*']
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/experimental",

      },
    ]
  },
};
