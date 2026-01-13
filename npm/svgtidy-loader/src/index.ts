// Webpack loader context type
interface LoaderContext {
    async: () => (err: Error | null, result?: string) => void;
    resourcePath: string;
}

export default async function svgtidyLoader(this: LoaderContext, source: string) {
    const callback = this.async();
    
    try {
        // @ts-ignore
        const { optimize } = await import('svgtidy');
        const optimized = optimize(source);
        callback(null, optimized);
    } catch (err: any) {
        callback(err);
    }
}
