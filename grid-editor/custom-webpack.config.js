const path = require('path');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
   stats: {
   errorDetails: true
   }
   ,
  resolve: {
    alias: {
        Rust: path.resolve(__dirname, '../rgb-solver/src/'),
    },
    extensions: ['.wasm', '.js', '.json', '.rs', '*', '.*']
  },
  module: {
    rules: [
    //https://github.com/webpack/webpack/issues/7352
      {
        test: /\.wasm$/,
        type: 'javascript/auto',
        loader: 'file-loader',
      },
     {
      test: /\.rs$/,
      use: [{
        loader: 'wasm-loader'
      },
      {
        loader: 'rust-native-wasm-loader',
        options: {
          release: true,
          wasmBindgen: {
            typescript: true
          }
        }
      }]
    }]


  },

  plugins: [
    //new webpack.IgnorePlugin(/rgb_solver_bg\.wasm/),
    /*new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../rgb-solver"),
      //https://github.com/rustwasm/rust-webpack-template/issues/43
      withTypeScript: true // this is new
      // WasmPackPlugin defaults to compiling in "dev" profile. To change that, use forceMode: 'release':
      // forceMode: 'release'
    }),*/
  ]
};
