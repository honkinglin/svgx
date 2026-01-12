declare module 'svgtidy' {
  export function init(): Promise<void>;
  export function optimize(svg: string): string;
}
