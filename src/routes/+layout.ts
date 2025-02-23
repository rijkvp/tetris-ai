export const ssr = false;
export const prerender = true;

import { loadTranslations } from '$lib/translations';
import type { LayoutLoad } from './$types';
import init from "tetris-ai";

export const load: LayoutLoad = async ({ url }) => {
    await init();
    console.info('Tetris AI initialized');

    const { pathname } = url;

    await loadTranslations('en', pathname);

    return {};
};
