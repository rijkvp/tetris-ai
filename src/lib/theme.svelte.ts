
export const theme = $state({
    prefersDark:
        window ?
            window.matchMedia('(prefers-color-scheme: dark)').matches : false,
});

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    theme.prefersDark = e.matches;
});
