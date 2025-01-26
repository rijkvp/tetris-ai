import type { LayoutLoad } from './$types';
import init from "tetris-ai";

export const ssr = false;

export const load: LayoutLoad = async () => {
    await init();
    console.log('Tetris AI initialized');
};
