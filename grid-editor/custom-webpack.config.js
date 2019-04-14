//const path = require('path');
//const webpack = require('webpack');
//const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
   stats: {
   errorDetails: true
   }
   ,
  resolve: {
    alias: {

    },
    extensions: ['.wasm', '.js', '.json', '.rs', '*', '.*']
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/experimental"

      },
    ]


  },

  plugins: [

  ]
};
