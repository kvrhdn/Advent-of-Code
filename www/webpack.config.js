const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

const days = [
  '01', '02', '03', '04', '05',
  '06', '07', '08', '09',
  '11', '12', '13',
];

module.exports = {
  // generate an entry for each day, this is an object with:
  //  "01": "./src/day_01.ts"
  //  "02": "./src/day_02.ts"
  //  ...
  entry: days.reduce((acc, day) => {
    acc[`d${day}`] = `./src/day_${day}.ts`;
    return acc;
  }, {}),
  output: {
    filename: '[name]/script.js',
    chunkFilename: '[name].[chunkhash].js',
    publicPath: "/",
    path: path.resolve(__dirname, 'dist'),
  },
  plugins: [
    new CleanWebpackPlugin(),
    new CopyWebpackPlugin(['src/index.html', 'src/style.css']),
    // generate a HtmlWebpackPlugin for each day, this generates a static html
    // page with the correct entry script
    ...days.map(day =>
      new HtmlWebpackPlugin({
        pageName: day,
        inject: true,
        chunks: [`d${day}`],
        template: 'src/template.html',
        filename: `d${day}/index.html`,
      })
    ),
  ],
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  mode: "development",
};
