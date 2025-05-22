<script>
    import { t } from "$lib/translations";
</script>

<svelte:head>
    <title>{$t("general.title")} - {$t("credits.title")}</title>
</svelte:head>

<div>
    <h1>{$t("credits.title")}</h1>
    <p>{@html $t("credits.content")}</p>
</div>
