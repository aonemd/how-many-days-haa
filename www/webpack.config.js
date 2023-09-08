const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  experiments: { asyncWebAssembly: true },
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader', 'postcss-loader'],
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin(['index.html', 'static'])
  ],
  mode: "development",
  devServer: {
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    hot: true,
    compress: true,
  },
};
