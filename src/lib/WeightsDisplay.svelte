<script lang="ts">
    import { t } from "$lib/translations";

    let {
        weightKeys,
        weightValues,
    }: {
        weightKeys: string[];
        weightValues: number[];
    } = $props();
</script>

<div class="weights-list">
    {#each weightValues as value, i}
        <span class="weight-value">{value.toFixed(2)}</span>
        <input
            class="weight-slider"
            type="range"
            {value}
            min="-10.0"
            max="10.0"
            disabled
        />
        <span class="weight-name">{$t(`feature.${weightKeys[i]}.name`)}</span>
    {/each}
</div>

<style>
    .weights-list {
        display: grid;
        grid-template-columns: 3rem 150px max-content;
        column-gap: 1rem;
        row-gap: 0.4rem;
        margin: 0.5rem 0;
        justify-items: stretch;
        align-items: center;
    }
</style>
