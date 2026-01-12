import type { Plugin } from 'vite';
import init, { optimize } from 'svgtidy-wasm';
import fs from 'fs';
import path from 'path';

// Load WASM during build (simplified approach, ideally loaded asynchronously)
let wasmInitialized = false;

// Dirty hack: In node environment (Vite build), we might need to point to the .wasm file explicitly
// OR rely on the bundler to handle 'svgtidy-wasm' import correctly.
// For simplicity in this demo, let's assume standard Vite handling.

export default function svgtidyPlugin(options: any = {}): Plugin {
  return {
    name: 'vite-plugin-svgtidy',
    enforce: 'pre',
    async buildStart() {
        if (!wasmInitialized) {
            // For Node.js (Vite SSR/Build), we might need fs.readFileSync for the wasm file
            // But wasm-pack bundler output usually expects 'fetch' or async import.
            // In Node, we can import the wasm buffer directly if configured, or rely on 'init'.
            
            // NOTE: wasm-pack 'bundler' target is for webpack/vite to consume.
            // So simply calling init() usually works if the bundler handles the .wasm import.
            await init();  
            wasmInitialized = true;
        }
    },
    async transform(code: string, id: string) {
      if (id.endsWith('.svg')) {
        // Only optimize if it's imported as raw SVG or handled by another loader?
        // Typically Vite handles SVGs as URLs. 
        // If query param ?raw is present, Vite returns string.
        
        // Let's optimize content.
        // We need the raw content. 'code' might be the URL export from Vite default.
        // So we read the file.
        const cleanId = id.split('?')[0];
        const content = fs.readFileSync(cleanId, 'utf-8');
        
        try {
            const optimized = optimize(content);
            return {
                code: `export default ${JSON.stringify(optimized)}`,
                map: null
            }
        } catch (e) {
            console.error('svgtidy error:', e);
        }
      }
      return null;
    }
  };
}
