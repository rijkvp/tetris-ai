<script lang="ts">
    import "../app.css"; // global styles
    import { t, locale } from "$lib/translations";
    import LangSwitcher from "$lib/LangSwitcher.svelte";
    import { onMount, onDestroy } from "svelte";
    import { base } from "$app/paths";
    import { localState } from "$lib/stores.svelte";

    let { children } = $props();

    function handleKeydown(event: KeyboardEvent) {
        if (event.ctrlKey && event.key === "y") {
            event.preventDefault();
            localState.cheatMode = !localState.cheatMode;
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });
</script>

<header>
    <h1>
        <a href="{base}/{$locale}"> Tetris AI </a>
    </h1>
    <nav>
        <a href="{base}/{$locale}">{$t("nav.levels")}</a>
        <a href="{base}/{$locale}/sandbox">{$t("nav.sandbox")}</a>
        <a href="{base}/{$locale}/guide">{$t("nav.guide")}</a>
    </nav>
    <LangSwitcher />
</header>
{@render children()}

<style>
    header {
        display: flex;
        align-items: stretch;
        gap: 2rem;
        margin-bottom: 1.2rem;
    }
    nav {
        display: flex;
        flex-grow: 1;
        align-items: center;
        gap: 1rem;
    }
    header > h1 {
        margin: 0;
        font-size: 1.6rem;
    }
</style>
