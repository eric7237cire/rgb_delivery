const path = require('path');
var webpack = require('webpack');

module.exports = {


  module: {
    rules: [


    ]
  },

  plugins: [new webpack.IgnorePlugin(/rgb_solver_bg/)]
};
