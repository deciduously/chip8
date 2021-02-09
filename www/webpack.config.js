const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: __dirname + "/src/bootstrap.js",
    output: {
        path: __dirname + "/dist",
        filename: "bootstrap.js",
    },
    mode: process.env.NODE_ENV === 'production' ? 'production' : 'development',
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                {
                    from: __dirname + "/src/index.html",
                    to: __dirname + "/dist"
                }
            ]
        }
        )
    ],
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: [
                    "style-loader",
                    "css-loader",
                    {
                        loader: "postcss-loader",
                        options: {
                            postcssOptions: {
                                plugins: [
                                    [
                                        "postcss-preset-env",
                                        {
                                            // Options
                                        },
                                    ],
                                ],
                            },
                        },
                    },
                ],
            },
        ],
    }
};
