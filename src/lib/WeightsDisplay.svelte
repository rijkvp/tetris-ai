<script lang="ts">
    import { t } from "$lib/translations";
    import { onMount } from "svelte";

    let {
        weightKeys,
        weightValues,
    }: {
        weightKeys: string[];
        weightValues: number[];
    } = $props();
</script>

<div>
    <h2>{$t("general.features")}</h2>
    <div class="weights-list">
        {#each weightValues as value, i}
            <div class="weight-item">
                <input type="range" {value} min="-10.0" max="10.0" disabled />
                <span class="weight-value">{value.toFixed(2)}</span>
                <span class="weight-name"
                    >{$t(`feature.${weightKeys[i]}.name`)}</span
                >
            </div>
        {/each}
    </div>
</div>

<style>
    .weights-list {
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
        margin: 0.5rem 0;
    }
    .weight-item {
        display: flex;
        gap: 1rem;
        justify-content: center;
    }
    .weight-item input[type="range"] {
        width: 150px;
    }
    .weight-value {
        width: 1rem;
    }
    .weight-name {
        flex-grow: 1;
    }
</style>
