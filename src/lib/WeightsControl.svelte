<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Weights } from "tetris-ai";

    let weightValues: Float64Array = Weights.defaults().values();
    let weightsInfo = Weights.info();
    let cheatActive = false;

    export let onWeightsChange: (weights: Weights) => void;

    function updateWeights() {
        const weights = Weights.from_values(weightValues);
        onWeightsChange(weights);
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.ctrlKey && event.key === "y") {
            event.preventDefault();
            cheatActive = !cheatActive;
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });
</script>

<div>
    <h2>Weights</h2>
    <button
        on:click={() => {
            weightValues = Weights.defaults().values();
            updateWeights();
        }}>Reset</button
    >
    <button
        style:display={cheatActive ? "inline" : "none"}
        on:click={() => {
            weightValues = Weights.preset().values();
            updateWeights();
        }}>Preset</button
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
