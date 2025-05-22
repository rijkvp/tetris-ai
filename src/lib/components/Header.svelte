<script lang="ts">
    import { theme } from "$lib/theme.svelte";
    import { t, locale } from "$lib/translations";
    import { base } from "$app/paths";
    import { goto } from "$app/navigation";
</script>

<header>
    <div class="header-section">
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
            <a href="{base}/{$locale}">{$t("general.title")}</a>
        </h1>
    </div>
    <div class="header-section">
        <button onclick={() => goto(`${base}/${$locale}`)}>
            <svg inline-src="list" />
            {$t("general.level_menu")}
        </button>
    </div>
</header>

<style>
    @media (min-width: 700px) {
        header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 2rem;
            margin-bottom: 1rem;
        }
    }
    header img {
        height: 3rem;
    }
    @media (max-width: 900px) {
        header img {
            height: 2rem;
        }
    }
    .header-section {
        display: flex;
        gap: 1.3rem;
        align-items: center;
    }
    .header-section > h1 {
        display: inline-block;
        margin: 0;
        font-size: 1.6rem;
    }
</style>
