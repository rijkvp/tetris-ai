let prefersDark = $state(
    window ?
        window.matchMedia('(prefers-color-scheme: dark)').matches : false,
);

let modeName = $derived(() => prefersDark ? "dark" : "light");

if (window) {
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
        prefersDark = e.matches;
    });
}

export const theme = {
    get prefersDark() { return prefersDark },
    get modeName() { return modeName }
};
