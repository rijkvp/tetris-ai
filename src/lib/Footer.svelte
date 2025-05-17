<script lang="ts">
    import { t, locale } from "$lib/translations";
    import { base } from "$app/paths";
</script>

<footer>
    <a href="{base}/{$locale}/credits"> {$t("footer.credits")} </a>

    &nbsp;|&nbsp;

    <a
        href="https://github.com/rijkvp/tetris-ai"
        target="_blank"
        rel="noopener noreferrer"
    >
        {$t("footer.source_code")}
    </a>
</footer>

<style>
    footer {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        margin-top: 3rem;
        margin-bottom: 1rem;
        color: var(--fg1);
        font-size: 0.8rem;
    }
</style>
