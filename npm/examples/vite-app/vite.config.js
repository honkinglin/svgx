import { defineConfig } from 'vite';
import svgtidy from 'vite-plugin-svgtidy';

export default defineConfig({
  plugins: [svgtidy()],
  build: {
    minify: false // Keep output readable for verification
  }
});
