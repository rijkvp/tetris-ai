<script lang="ts">
    import { t, locale } from "$lib/translations";
    import { NAVIGATION } from "$lib/levels";

    function resetProgress() {
        localStorage.clear();
    }
</script>

<h2>{$t("level_select.title")}</h2>
<p>{$t("level_select.description")}</p>

<ul>
    {#each NAVIGATION as level}
        <li>
            <a href="{$locale}/levels/{level}">{level}</a>
        </li>
    {/each}
</ul>

<button onclick={() => resetProgress()}>Reset progress</button>
