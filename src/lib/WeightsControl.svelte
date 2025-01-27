<script lang="ts">
    import { Weights } from "tetris-ai";

    // const weights = Weights.defaults();
    let weightValues = Weights.defaults().values();
    let weightsInfo = Weights.info();
    export let onWeightsChange: (weights: Weights) => void;
</script>

<div>
    <h2>Weights</h2>
    <div class="weights-grid">
        {#each weightValues as weight, n}
            <input
                type="range"
                bind:value={weight}
                min="-10.0"
                max="10.0"
                step="0.001"
                on:change={() => {
                    const weights = Weights.from_values(weightValues);
                    onWeightsChange(weights);
                }}
            />
            <span>{weightsInfo[n].name}</span>
            <span>{weight}</span>
            <span>{weightsInfo[n].default_value}</span>
        {/each}
    </div>
</div>

<style>
    .weights-grid {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr;
        gap: 1rem;
    }
</style>
