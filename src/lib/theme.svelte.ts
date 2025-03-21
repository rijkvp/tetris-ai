
let prefersDark = $state(
    window ?
        window.matchMedia('(prefers-color-scheme: dark)').matches : false,
);

let modeName = $derived(() => prefersDark ? "dark" : "light");

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    prefersDark = e.matches;
});

export const theme = {
    prefersDark: prefersDark,
    modeName: modeName,
}

