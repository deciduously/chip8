const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: process.env.NODE_ENV === 'production' ? 'production' : 'development',
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                { 
                    from: path.resolve(__dirname, "index.html"),
                    to: path.resolve(__dirname, "dist")
                }
            ]
        }
        )
    ]
};
