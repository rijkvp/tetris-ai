<script lang="ts">
    import { Weights } from "tetris-ai";

    const weightCount: number = Weights.defaults().values().length;
    let weightValues: Float64Array = new Float64Array(weightCount);
    let weightsInfo = Weights.info();

    export let onWeightsChange: (weights: Weights) => void;

    function updateWeights() {
        const weights = Weights.from_values(weightValues);
        onWeightsChange(weights);
    }
</script>

<div>
    <h2>Weights</h2>
    <button
        on:click={() => {
            weightValues = new Float64Array(weightCount);
            updateWeights();
        }}>Reset to zero</button
    >
    <button
        on:click={() => {
            weightValues = Weights.defaults().values();
            updateWeights();
        }}>Default weights</button
    >
    <div class="weights-grid">
        {#each weightValues as weight, n}
            <input
                type="range"
                bind:value={weight}
                min="-10.0"
                max="10.0"
                step="0.1"
                on:change={() => {
                    updateWeights();
                }}
            />
            <span>{weightsInfo[n].name}</span>
            <span>{weight}</span>
            <!-- <span>{weightsInfo[n].default_value}</span> -->
        {/each}
    </div>
</div>

<style>
    .weights-grid {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        gap: 1rem;
    }
</style>
