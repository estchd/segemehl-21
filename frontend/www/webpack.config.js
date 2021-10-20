const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    entry: {
        bootstrap: "./bootstrap.js",
    },
    module: {
        rules: [
            {
                test: /\.worker\.js$/,
                use: { loader: "worker-loader" },
            }
        ]
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "[name].js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin([
            'index.html',
            'favicon.ico',
            './js/worker.js',
            {from: "./../pkg/segemehl_21_frontend.js", to: "pkg/segemehl_21_frontend.js"},
            //{from: "./../pkg/segemehl_21_frontend_bg.js", to: "pkg/segemehl_21_frontend_bg.js"},
            {from: "./../pkg/segemehl_21_frontend_bg.wasm", to: "pkg/segemehl_21_frontend_bg.wasm"}
        ])
    ],
};
