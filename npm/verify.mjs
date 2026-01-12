
// Mock Vite Context and testing the plugin logic
// import svgtidyPlugin from './vite-plugin-svgtidy/src/index.ts'; 

// Let's create a `verify.sh` script that installs deps, builds, and runs these tests.
// And I will use `npm/test-plugin-mock.mjs` that mimics the plugin logic for verification 
// mainly to prove the WASM works in a "plugin-like" environment.

// Correction: The user wants test cases. I should provide standard test files.
// Let's put a simple test in `npm/run-tests.mjs` that automates everything.

import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';

console.log("🚀 Starting Verification Suite");

try {
    // 1. WASM Core Test
    console.log("\n--- 1. Testing WASM Core ---");
    execSync('node test-wasm.mjs', { stdio: 'inherit', cwd: import.meta.dirname });

    // 2. Build Plugins
    console.log("\n--- 2. Building Plugins ---");
    const plugins = ['vite-plugin-svgtidy', 'svgtidy-loader'];
    
    for (const plugin of plugins) {
        const pluginDir = path.resolve(import.meta.dirname, plugin);
        console.log(`📦 Building ${plugin}...`);
        execSync('npm install && npm run build', { stdio: 'inherit', cwd: pluginDir });
    }

    // 3. Test Vite Example
    console.log("\n--- 3. Testing Vite Plugin ---");
    const viteAppDir = path.resolve(import.meta.dirname, 'examples/vite-app');
    console.log("🔨 Building Vite App...");
    execSync('npm install && npm run build', { stdio: 'inherit', cwd: viteAppDir });
    
    // Verify Vite Output
    const viteOutput = fs.readFileSync(path.join(viteAppDir, 'dist/assets/index.js'), 'utf-8'); // Vite hashes assets usually... wait.
    // In library mode or simple build, it might be different. 
    // Vite bundles module scripts into assets dir with hash.
    // Let's find the .js file.
    const assetsDir = path.join(viteAppDir, 'dist/assets');
    const jsFile = fs.readdirSync(assetsDir).find(f => f.endsWith('.js'));
    const viteContent = fs.readFileSync(path.join(assetsDir, jsFile), 'utf-8');
    
    if (viteContent.includes('<path') && !viteContent.includes('<rect')) {
         console.log("✅ Vite Plugin Output Verified");
    } else {
         throw new Error("❌ Vite Plugin Failed: Output does not contain optimized SVG");
    }

    // 4. Test Webpack Example
    console.log("\n--- 4. Testing Webpack Loader ---");
    const webpackAppDir = path.resolve(import.meta.dirname, 'examples/webpack-app');
    console.log("🔨 Building Webpack App...");
    execSync('npm install && npm run build', { stdio: 'inherit', cwd: webpackAppDir });
    
    // Verify Webpack Output
    const webpackContent = fs.readFileSync(path.join(webpackAppDir, 'dist/bundle.js'), 'utf-8');
    if (webpackContent.includes('<path') && !webpackContent.includes('<rect')) {
         console.log("✅ Webpack Loader Output Verified");
    } else {
         // Note: Webpack output might escape strings differently, but basic check should hold
         if (webpackContent.includes('M10 10H90V90H10z')) {
             console.log("✅ Webpack Loader Output Verified (Content Match)");
         } else {
             throw new Error("❌ Webpack Loader Failed: Output does not contain optimized SVG");
         }
    }

    console.log("\n🎉 All Tests Passed!");
    
} catch (e) {
    console.error("\n❌ Verification Failed");
    console.error(e);
    process.exit(1);
}
