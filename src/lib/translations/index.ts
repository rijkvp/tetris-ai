import type { Writable } from 'svelte/store';
import i18n from 'sveltekit-i18n';
import en from "./en.json";
import nl from "./nl.json";

const config = {
    translations: {
        en,
        nl,
    },
};

const stores = new i18n(config);
export const { t, locales, loading, loadTranslations } = stores;
export const locale = stores.locale as unknown as Writable<"en" | "nl">;
export const setLocale = stores.setLocale as (locale: "en" | "nl") => void;
