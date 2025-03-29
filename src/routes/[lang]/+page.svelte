<script lang="ts">
    import { t, locale } from "$lib/translations";
    import { levels } from "$lib/levels";

    function resetProgress() {
        localStorage.clear();
    }
</script>

<h2>{$t("level_select.title")}</h2>
<p>{$t("level_select.description")}</p>

<ul>
    <li>
        <a href="{$locale}/levels/intro"
            >1. Intoduction to Tetris (START HERE)</a
        >
    </li>
    {#each levels as level}
        <li>
            <a href="{$locale}/levels/{level.key}">{level.name[$locale]}</a>
        </li>
    {/each}
    <li>
        <a href="{$locale}/sandbox">Sandbox</a>
    </li>
</ul>

<button onclick={() => resetProgress()}>Reset progress</button>
