import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
  mode: 'production',
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  module: {
    rules: [
      {
        test: /\.svg$/,
        use: [
          {
            loader: 'svgtidy-loader', // Use the local loader
          },
          'raw-loader' // Read file as string first (optional, svgtidy-loader handles raw string)
          // Actually svgtidy-loader expects string content. 
          // If we use 'type: asset/source' in webpack 5 it might be easier, 
          // but loader chaining gives us more control to test the loader specifically.
          // Let's assume svgtidy-loader handles the file reading or is passed the source?
          // Webpack passes the file content to the loader.
        ],
        type: 'javascript/auto' // Prevent default JSON/asset behavior if conflict
      },
    ],
  },
  optimization: {
    minimize: false
  }
};
