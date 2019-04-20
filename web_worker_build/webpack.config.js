const path = require("path");

const dist = path.resolve(__dirname, "dist");
const exec = require('child_process').exec;

const workerConfig = {
  entry: "./worker/worker.ts" ,
    devtool: 'source-map',
  target: "webworker",

  resolve: {
    extensions: [".ts", ".js", ".wasm"]
  },
  module: {
        rules: [
            // all files with a '.ts' or '.tsx' extension will be handled by 'ts-loader'
            {
                test: /\.tsx?$/,
                use: {
                    loader: "ts-loader"
                }
            }
        ]
    },
  output: {
    path: dist,
    filename: "worker.js"
  },
    plugins: [
        {
            apply: (compiler) => {
                compiler.hooks.afterEmit.tap('AfterEmitPlugin', (compilation) => {
                    exec('E:\\git\\rgb_delivery\\copy-wasm-build-files.bat', (err, stdout, stderr) => {
                        if (stdout) process.stdout.write(stdout);
                        if (stderr) process.stderr.write(stderr);
                    });
                });
            }
        }
        ]
};

module.exports = [ workerConfig];
