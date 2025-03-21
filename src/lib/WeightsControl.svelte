<script lang="ts">
    import { t } from "$lib/translations";
    import { WeightsMap } from "tetris-ai";
    import { localState } from "$lib/stores.svelte";
    import ExampleBoard from "./ExampleBoard.svelte";
    import DynamicIcon from "./DynamicIcon.svelte";

    let {
        onWeightsChange,
        availableFeatures,
    }: {
        onWeightsChange: (weights: [string, number][]) => void;
        availableFeatures: string[] | undefined;
    } = $props();

    let weights: [string, number][] = $state()!;
    let enabledWeights: boolean[] = $state()!;

    function reset() {
        weights = WeightsMap.defaults().into_js();
        enabledWeights = WeightsMap.defaults()
            .into_js()
            .map(
                ([key, _]: [string, number]) =>
                    availableFeatures?.includes(key) ?? true,
            );
    }

    reset();

    function updateWeights() {
        const actualWeights = WeightsMap.defaults().into_js();
        for (let i = 0; i < weights.length; i++) {
            if (enabledWeights[i]) {
                actualWeights[i][1] = weights[i][1];
            }
        }
        onWeightsChange(actualWeights);
    }

    let selectedFeature: string = $state("col_trans");
    let infoDialog: HTMLDialogElement;
</script>

<div>
    <h2>{$t("general.features")}</h2>
    <div class="buttons">
        <button
            onclick={() => {
                reset();
                updateWeights();
            }}
        >
            <DynamicIcon icon="reset" alt="Reset" />
            {$t("feature_control.reset")}</button
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
            }}
        >
            <DynamicIcon icon="shuffle" alt="Shuffle" />
            {$t("feature_control.randomize")}</button
        >
    </div>
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
    <div class="weights-list">
        {#each weights as [key, value], i}
            {#if !availableFeatures || availableFeatures.includes(key)}
                <div class="weight-item">
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
                    <span class="weight-value">{value}</span>
                    <span class="weight-name">{$t(`feature.${key}.name`)}</span>
                    <button
                        onclick={() => {
                            selectedFeature = key;
                            infoDialog.showModal();
                        }}>?</button
                    >
                </div>
            {/if}
        {/each}
    </div>
</div>
<dialog bind:this={infoDialog}>
    <h1>{$t(`feature.${selectedFeature}.name`)}</h1>
    <p>{$t(`feature.${selectedFeature}.description`)}</p>
    <div class="example">
        <h2>{$t("general.example")}:</h2>
        <div class="example-board">
            <ExampleBoard feature={selectedFeature} />
        </div>
    </div>
    <button class="close-button" onclick={() => infoDialog.close()}
        >{$t("general.close")}</button
    >
</dialog>

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
    dialog {
        inset: 50%;
        transform: translate(-50%, -50%);
        width: 500px;
        padding: 1rem;
        border: none;
        background: var(--bg0);
        color: var(--fg0);
    }
    .buttons {
        display: flex;
        gap: 0.5rem;
    }
    .buttons > button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.2rem;
    }
    .example {
        margin: 1rem 0;
    }
    .example-board {
        margin: 1rem 0;
        display: flex;
        justify-content: center;
    }
    .close-button {
        float: right;
    }
    dialog::backdrop {
        backdrop-filter: blur(6px);
        background: rgba(0, 0, 0, 0.3);
    }
</style>
