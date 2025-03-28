<script lang="ts">
    import "../app.css"; // global styles
    import Header from "$lib/Header.svelte";
    import { base } from "$app/paths";
    import { localState } from "$lib/stores.svelte";
    import { onMount, onDestroy } from "svelte";
    import { t, locale } from "$lib/translations";

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

<Header />
{@render children()}
