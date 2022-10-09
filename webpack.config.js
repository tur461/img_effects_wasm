const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmWebpackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: './public/main.js',
  output: {
    path: path.resolve(__dirname, 'bundles'),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './public/index.html',
    },
    new WasmWebpackPlugin({
      crateDirectory: path.resolve(__dirname, '.'),
    })
)
  ]
}
