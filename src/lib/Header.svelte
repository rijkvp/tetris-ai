<script lang="ts">
    import { theme } from "$lib/theme.svelte";
    import { locale } from "$lib/translations";
    import LangSwitcher from "$lib/LangSwitcher.svelte";
    import { base } from "$app/paths";
</script>

<header>
    <div class="logo">
        <a
            href="https://www.rug.nl/jantina-tammes-school/"
            target="_blank"
            rel="noopener"
        >
            {#if theme.prefersDark}
                <img src="{base}/logos/jts-white.png" alt="JTS Logo" />
            {:else}
                <img src="{base}/logos/jts-red.png" alt="JTS Logo" />
            {/if}
        </a>
        <h1>
            <a href="{base}/{$locale}">Tetris AI</a>
        </h1>
    </div>
    <LangSwitcher />
</header>

<style>
    header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 2rem;
        margin-bottom: 1rem;
    }
    header img {
        height: 2rem;
    }
    .logo {
        display: flex;
        gap: 1.3rem;
        align-items: center;
    }
    .logo > h1 {
        display: inline-block;
        margin: 0;
        font-size: 1.6rem;
    }
</style>
