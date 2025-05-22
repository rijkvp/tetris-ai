<script lang="ts">
    import { t, locale } from "$lib/translations";
    import { LEVELS, LEVEL_INFO } from "$lib/levels";
    import { onMount } from "svelte";

    let completedLevels: string[] = $state([]);

    function resetProgress() {
        localStorage.clear();
        completedLevels = [];
    }

    onMount(() => {
        const json = localStorage.getItem("completed_levels");
        completedLevels = json ? JSON.parse(json) : [];
    });
</script>

<svelte:head>
    <title>{$t("general.title")} - {$t("level_select.title")}</title>
</svelte:head>

<h2>{$t("level_select.title")}</h2>
<p>{$t("level_select.description")}</p>

<div class="level-select">
    {#each LEVELS as level, i}
        {@const info = LEVEL_INFO[level]}
        <a class="level" href="{$locale}/levels/{level}">
            <h3>
                {i + 1}: {info.name[$locale]}
                {#if completedLevels.includes(level)}
                    <span class="completed">&#10004;</span>
                {/if}
            </h3>
            <span>
                {info.summary[$locale]}
            </span>
        </a>
    {/each}
</div>

<button class="reset-progress" onclick={() => resetProgress()}>
    <svg inline-src="trash" />
    {$t("general.reset_progress")}
</button>

<style>
    h3 {
        margin-bottom: 0.5rem;
    }
    a:link,
    a:visited,
    a:hover,
    a:active {
        font: inherit;
        font-weight: normal;
    }
    .level-select {
        display: flex;
        flex-direction: column;
    }
    .level {
        display: block;
        margin: 0.5rem 0;
        padding: 0.5rem;
        border: 1px solid var(--border);
    }
    .level:hover {
        background-color: var(--bg1);
        text-decoration: none;
    }
    .reset-progress {
        margin-top: 0.5rem;
        float: right;
    }
    .completed {
        color: var(--green);
        font-weight: bold;
    }
</style>
