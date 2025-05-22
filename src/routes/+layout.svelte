<script lang="ts">
    import "../app.css"; // global styles
    import { t } from "$lib/translations";
    import { localState } from "$lib/stores.svelte";
    import { onMount, onDestroy } from "svelte";
    import Header from "$lib/components/Header.svelte";
    import Footer from "$lib/components/Footer.svelte";

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

<svelte:head>
    <title>
        {$t("general.title")}
    </title>
</svelte:head>
<Header />
{@render children()}
<Footer />
