import init, { optimize } from 'svgtidy-wasm';

// Webpack loader context type
interface LoaderContext {
    async: () => (err: Error | null, result?: string) => void;
    resourcePath: string;
}

// Global WASM initialization state
let wasmInitialized = false;

export default async function svgtidyLoader(this: LoaderContext, source: string) {
    const callback = this.async();
    
    try {
        if (!wasmInitialized) {
            await init();
            wasmInitialized = true;
        }

        const optimized = optimize(source);
        callback(null, optimized);
    } catch (err: any) {
        callback(err);
    }
}
