<script lang="ts">
    import { t, locale } from "$lib/translations";
    import { levels } from "$lib/levels";
</script>

<h2>{$t("level_select.title")}</h2>
<p>{$t("level_select.description")}</p>

<ul>
    {#each levels as level}
        <li>
            <a href="{$locale}/{level.key}">{level.name[$locale]}</a>
        </li>
    {/each}
</ul>
