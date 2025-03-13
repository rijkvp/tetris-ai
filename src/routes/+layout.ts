export const ssr = false;
export const prerender = false;

import { loadTranslations } from "$lib/translations";
import type { LayoutLoad } from "./$types";
import init from "tetris-ai";
import { redirect } from "@sveltejs/kit";
import { base } from "$app/paths";

export const load: LayoutLoad = async ({ url }) => {
    await init();
    console.info("Tetris AI initialized");

    const { pathname } = url;

    let appPath = pathname.substring(base.length);

    if (appPath.startsWith("/en")) {
        await loadTranslations("en", "");
    } else if (appPath.startsWith("/nl")) {
        await loadTranslations("nl", "");
    } else if (appPath === "/") {
        if (navigator.language.startsWith("nl")) {
            return redirect(302, `${base}/nl`);
        } else {
            return redirect(302, `${base}/en`);
        }
    } else {
        const lang = navigator.language.startsWith("nl") ? "nl" : "en";
        return redirect(302, `${base}/${lang}${pathname}`);
    }

    return {};
};
