import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type PluginOption } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig(({ mode }) => {
    let wasmPlugins: PluginOption[] = [];
    if (mode === 'development') {
        wasmPlugins = [wasmPack('./tetris-ai')];
    } else if (mode === 'production') {
        wasmPlugins = [wasm(), topLevelAwait()];
    }

    return {
        plugins: [
            sveltekit(),
            ...wasmPlugins,
        ],
        server: {
            fs: {
                allow: ["./tetris-ai"]
            }
        }
    };
});
