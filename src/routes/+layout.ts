export const ssr = false;
export const prerender = false;

import { loadTranslations } from "$lib/translations";
import type { LayoutLoad } from "./$types";
import init from "tetris-ai";
import { redirect } from "@sveltejs/kit";

export const load: LayoutLoad = async ({ url }) => {
    await init();
    console.info("Tetris AI initialized");

    const { pathname } = url;
    if (pathname.startsWith("/en")) {
        await loadTranslations("en", "");
    } else if (pathname.startsWith("/nl")) {
        await loadTranslations("nl", "");
    } else if (pathname === "/") {
        if (navigator.language.startsWith("nl")) {
            return redirect(302, "/nl");
        } else {
            return redirect(302, "/en");
        }
    } else {
        const lang = navigator.language.startsWith("nl") ? "nl" : "en";
        return redirect(302, `/${lang}${pathname}`);
    }

    return {};
};
