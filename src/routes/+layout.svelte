<script lang="ts">
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

<h1>Tetris AI</h1>
<header>
    <nav>
        <a href="{base}/{$locale}">{$t("nav.levels")}</a>
        <a href="{base}/{$locale}/sandbox">{$t("nav.sandbox")}</a>
    </nav>
    <LangSwitcher />
</header>
{@render children()}

<style>
    header,
    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 20px;
    }
</style>
