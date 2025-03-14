import { readable } from 'svelte/store';

export const prefersDarkMode = readable(false, (set) => {
    if (typeof window === 'undefined') return;

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    set(mediaQuery.matches);

    const listener = (e: MediaQueryListEvent) => set(e.matches);
    mediaQuery.addEventListener('change', listener);

    return () => mediaQuery.removeEventListener('change', listener);
});

