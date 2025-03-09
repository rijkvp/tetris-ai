<script lang="ts">
    import { t } from "$lib/translations";
    import { WeightsMap } from "tetris-ai";
    import { localState } from "$lib/stores.svelte";

    let weights: [string, number][] = $state(WeightsMap.defaults().into_js());
    let enabledWeights: boolean[] = $state(
        Array(WeightsMap.defaults().into_js().length).fill(true),
    );

    let {
        onWeightsChange,
        availableFeatures,
    }: {
        onWeightsChange: (weights: [string, number][]) => void;
        availableFeatures: string[] | undefined;
    } = $props();

    function updateWeights() {
        const actualWeights = WeightsMap.defaults().into_js();
        for (let i = 0; i < weights.length; i++) {
            if (enabledWeights[i]) {
                actualWeights[i][1] = weights[i][1];
            }
        }
        onWeightsChange(actualWeights);
    }

    let selectedWeight: string = $state("");
    let infoDialog: HTMLDialogElement;
</script>

<div>
    <h2>{$t("weights")}</h2>
    <button
        onclick={() => {
            weights = WeightsMap.defaults().into_js();
            enabledWeights = Array(weights.length).fill(true);
            updateWeights();
        }}>Reset</button
    >
    <button
        onclick={() => {
            weights.forEach((_, i) => {
                if (enabledWeights[i]) {
                    weights[i][1] = parseFloat(
                        (Math.random() * 20 - 10).toFixed(1),
                    );
                }
            });
            updateWeights();
        }}>Randomize</button
    >
    <div style:display={localState.cheatMode ? "inline" : "none"}>
        <h2>Cheat mode</h2>
        <button
            onclick={() => {
                weights = WeightsMap.preset("infinite").into_js();
                updateWeights();
            }}>Preset (infinite)</button
        >
        <button
            onclick={() => {
                weights = WeightsMap.preset("score").into_js();
                updateWeights();
            }}>Preset (score)</button
        >
    </div>
    <div class="weights-grid">
        {#each weights as [key, value], i}
            {#if !availableFeatures || availableFeatures.includes(key)}
                <input
                    type="checkbox"
                    bind:checked={enabledWeights[i]}
                    onchange={() => updateWeights()}
                />
                <input
                    type="range"
                    bind:value={weights[i][1]}
                    disabled={!enabledWeights[i]}
                    min="-10.0"
                    max="10.0"
                    step="0.1"
                    oninput={() => updateWeights()}
                />
                <span>{value}</span>
                <span>{$t(`feature.${key}.name`)}</span>
                <button
                    onclick={() => {
                        selectedWeight = key;
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
        grid-template-columns: 1rem 200px 3rem 180px 2rem;
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
