<script lang="ts">
    import "../app.css"; // global styles
    import Header from "$lib/Header.svelte";
    import { localState } from "$lib/stores.svelte";
    import { onMount, onDestroy } from "svelte";
    import Footer from "$lib/Footer.svelte";

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
<Footer />
