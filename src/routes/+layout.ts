export const ssr = false;
export const prerender = true;

import type { LayoutLoad } from './$types';
import init from "tetris-ai";

export const load: LayoutLoad = async () => {
    await init();
    console.info('Tetris AI initialized');
};
