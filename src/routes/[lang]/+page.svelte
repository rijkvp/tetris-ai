<script lang="ts">
    import { t, locale } from "$lib/translations";
    import { LEVELS, LEVEL_INFO } from "$lib/levels";

    function resetProgress() {
        localStorage.clear();
    }
</script>

<h2>{$t("level_select.title")}</h2>
<p>{$t("level_select.description")}</p>

<div class="level-select">
    {#each LEVELS as level}
        {@const info = LEVEL_INFO[level]}
        <a class="level" href="{$locale}/levels/{level}">
            <h3>
                {info.name[$locale]}
            </h3>
            <span>
                {info.summary[$locale]}
            </span>
        </a>
    {/each}
</div>

<button onclick={() => resetProgress()}>Reset progress</button>

<style>
    .level-select {
        display: flex;
        flex-direction: column;
    }
    .level {
        display: block;
        margin: 10px;
        padding: 20px;
        border: 1px solid var(--border);
    }
    .level:hover {
        background-color: var(--bg1);
        text-decoration: none;
    }
</style>
