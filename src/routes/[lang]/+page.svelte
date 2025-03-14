<script lang="ts">
    import { locale } from "$lib/translations";
    import { levels } from "$lib/levels";
</script>

<h2>Level selection</h2>

<ul>
    {#each levels as level}
        <li>
            <a href="{$locale}/{level.key}">{level.name[$locale]}</a>
        </li>
    {/each}
</ul>
