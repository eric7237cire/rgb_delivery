const path = require('path');

module.exports = {

  module: {
    rules: [

      {
        test: /rgb-solver/,
        use: ["null-loader"],

      }
    ]
  }
};
