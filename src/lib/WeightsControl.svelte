<script lang="ts">
    import { Weights } from "tetris-ai";

    // const weights = Weights.defaults();
    let weightValues = Weights.defaults().values();
    let weightsInfo = Weights.info();
    export let onWeightsChange: (weights: Weights) => void;
</script>

<div>
    <h2>Weights</h2>
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
        <label>{weightsInfo[n].name} {weight} (best: {weightsInfo[n].default_value})</label>
    {/each}
</div>
