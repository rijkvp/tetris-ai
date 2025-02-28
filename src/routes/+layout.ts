export const ssr = false;
export const prerender = true;

import { loadTranslations } from "$lib/translations";
import type { LayoutLoad } from "./$types";
import init from "tetris-ai";
import { redirect } from "@sveltejs/kit";

export const load: LayoutLoad = async ({ url }) => {
    await init();
    console.info("Tetris AI initialized");

    const { pathname } = url;
    if (pathname === "/") {
        if (navigator.language.startsWith("nl")) {
            return redirect(302, "/nl");
        } else {
            return redirect(302, "/en");
        }
    }
    if (pathname.startsWith("/en")) {
        await loadTranslations("en", "");
    } else if (pathname.startsWith("/nl")) {
        await loadTranslations("nl", "");
    }

    return {};
};
