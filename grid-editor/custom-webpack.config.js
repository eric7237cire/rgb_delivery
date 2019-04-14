const path = require('path');
//const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  stats: {warnings:false},
  resolve: {
    alias: {
      RustRGBProject: path.resolve(__dirname, '../rgb-solver')
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


    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../rgb-solver"),

      // Check https://rustwasm.github.io/wasm-pack/book/commands/build.html for
      // the available set of arguments.
      //
      // Default arguments are `--typescript --target browser --mode normal`.


      // Optional array of absolute paths to directories, changes to which
      // will trigger the build.
      // watchDirectories: [
      //   path.resolve(__dirname, "another-crate/src")
      // ],

      // If defined, `forceWatch` will force activate/deactivate watch mode for
      // `.rs` files.
      //
      // The default (not set) aligns watch mode for `.rs` files to Webpack's
      // watch mode.
      // forceWatch: true,

      // If defined, `forceMode` will force the compilation mode for `wasm-pack`
      //
      // Possible values are `development` and `production`.
      //
      // the mode `development` makes `wasm-pack` build in `debug` mode.
      // the mode `production` makes `wasm-pack` build in `release` mode.
      // forceMode: "development",
    }),


  ]
};
