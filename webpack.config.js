//https://www.webpackjs.com/loaders/html-loader/

const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const getHtmlConfig = ({name, title="index"})=>({
        template    : './www/view/' + name + '.html',
        filename    :  name + '.html',
       // favicon     : './www/favicon.ico',
        title       : name,
        inject      : true,
        hash        : true,
        chunks      : ['common', name]
})

const PAGES=[
    {name:"index",tilte:"index"},
    {name:"login",tilte:"login"},
].map(getHtmlConfig)

module.exports = {
    resolve: {
      alias: {
       // "@": ".",
      },
    },
    entry: {
      //'common': ['./src/page/common/index.js'],
        'login': ['./www/js/1.js'],
        'index': ['./www/js/index.js'],
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'js/[name].js'
    },
    optimization: {
         splitChunks: {
           name: 'common',
         },
    },
    plugins: [
        ...PAGES.map(x=>new HtmlWebpackPlugin(x)),
       new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
       new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        }),
    ],
    module: {
      rules: [
        // { test: /\.json$/, use: 'raw-loader' },
        // { test: /\.html$/, use: 'raw-loader',},
        { test: /\.jpg$/, use: [ "file-loader" ] },
        { test: /\.png$/, use: [ "url-loader?mimetype=image/png" ] },
        {
          test: /\.(html)$/,
//        use: [ 'file-loader?name=[path][name].[ext]!extract-loader!html-loader'],
          use: {
            loader: 'html-loader',
            options: {
                  attrs: [':data-src'],
                  minimize: true,
                  removeComments: false,
                  collapseWhitespace: false
            }
          }
        },
      ]
    },
    mode: 'development'
};
