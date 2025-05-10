import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { inlineSvg } from '@svelte-put/inline-svg/vite';

export default defineConfig(() => {
    return {
        plugins: [
            inlineSvg([
                {
                    directories: 'assets/icons',
                    attributes: {
                        class: 'icon',
                        width: '20',
                        height: '20',
                    },
                },
            ]),
            sveltekit(),
            wasm(),
            topLevelAwait()
        ],
        server: {
            fs: {
                allow: ["./tetris-ai"]
            }
        }
    };
});
