<script lang="ts">
    import { theme } from "$lib/theme.svelte";
    import { base } from "$app/paths";

    let { icon, alt }: { icon: string; alt: string } = $props();
</script>

{#if theme.prefersDark}
    <img src={`${base}/icons/${icon}-light.png`} {alt} />
{:else}
    <img src={`${base}/icons/${icon}-dark.png`} {alt} />
{/if}

<style>
    img {
        image-rendering: optimizeSpeed;
        image-rendering: -o-crisp-edges;
        image-rendering: -webkit-optimize-contrast;
        image-rendering: -moz-crisp-edges;
        image-rendering: crisp-edges;
        image-rendering: pixelated;
    }
</style>
