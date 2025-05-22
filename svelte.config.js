import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

let basePath = '';
if (process.env.NODE_ENV === 'production') {
    basePath = process.env.BASE_PATH;
    console.log(`Base path: '${basePath}'`);
}

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        // adapter-static
        adapter: adapter({
            pages: 'build',
            assets: 'build',
            fallback: '404.html',
            precompress: true,
            strict: true
        }),
        paths: {
            base: basePath
        }
    }
};

export default config;
