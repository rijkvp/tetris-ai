<script lang="ts">
    import { t } from "$lib/translations";
    import { Weights } from "tetris-ai";
    import { localState } from "$lib/stores.svelte";

    let weightValues = $state([...Weights.defaults().values()]);
    let weightsInfo = Weights.info();

    let {
        onWeightsChange,
        availableFeatures,
    }: {
        onWeightsChange: (weights: Weights) => void;
        availableFeatures: string[] | undefined;
    } = $props();

    function updateWeights() {
        const weights = Weights.from_values(new Float64Array(weightValues));
        onWeightsChange(weights);
    }

    let selectedWeight: string = $state("");
    let infoDialog: HTMLDialogElement;
</script>

<div>
    <h2>{$t("weights")}</h2>
    <button
        onclick={() => {
            weightValues = [...Weights.defaults().values()];
            updateWeights();
        }}>Reset</button
    >
    <div style:display={localState.cheatMode ? "inline" : "none"}>
        <h2>Cheat mode</h2>
        <button
            onclick={() => {
                weightValues = [...Weights.preset().values()];
                updateWeights();
            }}>Preset 1</button
        >
        <button
            onclick={() => {
                weightValues = [...Weights.preset2().values()];
                updateWeights();
            }}>Preset 2</button
        >
    </div>
    <div class="weights-grid">
        {#each weightValues as weight, n}
            {#if !availableFeatures || availableFeatures.includes(weightsInfo[n].name)}
                <input type="checkbox" />
                <input
                    type="range"
                    bind:value={weightValues[n]}
                    min="-10.0"
                    max="10.0"
                    step="0.1"
                    onchange={() => updateWeights()}
                />
                <span>{$t(`feature.${weightsInfo[n].name}.name`)}</span>
                <span>{weight}</span>
                <button
                    onclick={() => {
                        selectedWeight = weightsInfo[n].name;
                        infoDialog.showModal();
                    }}>?</button
                >
            {/if}
        {/each}
    </div>
</div>
<dialog bind:this={infoDialog}>
    <h2>{$t(`feature.${selectedWeight}.name`)}</h2>
    <p>{$t(`feature.${selectedWeight}.description`)}</p>
    <button onclick={() => infoDialog.close()}>Close</button>
</dialog>

<style>
    .weights-grid {
        display: grid;
        grid-template-columns: 1rem 200px auto 20px 20px;
        gap: 1rem;
    }
    dialog {
        inset: 50%;
        transform: translate(-50%, -50%);
        width: 500px;
        padding: 1rem;
        border: none;
        background: var(--bg0);
        color: var(--fg0);
    }
    dialog::backdrop {
        backdrop-filter: blur(6px);
        background: rgba(0, 0, 0, 0.3);
    }
</style>
