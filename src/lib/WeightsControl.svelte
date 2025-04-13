<script lang="ts">
    import { t } from "$lib/translations";
    import { Weights } from "$lib/weights.svelte";
    import { localState } from "$lib/stores.svelte";
    import ExampleBoard from "./ExampleBoard.svelte";
    import DynamicIcon from "./DynamicIcon.svelte";

    let { weights }: { weights: Weights } = $props();

    let selectedFeature: string = $state("col_trans");
    let infoDialog: HTMLDialogElement;
</script>

<div>
    <h2>{$t("general.features")}</h2>
    <div class="buttons">
        <button
            onclick={() => {
                weights.reset();
            }}
        >
            <DynamicIcon icon="reset" alt="Reset" />
            {$t("feature_control.reset")}</button
        >
        <button
            onclick={() => {
                weights.randomize();
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
                weights.loadPreset("infinite");
            }}>Preset (infinite)</button
        >
        <button
            onclick={() => {
                weights.loadPreset("score");
            }}>Preset (score)</button
        >
    </div>
    <div class="weights-list">
        {#each weights.entries() as [key, entry]}
            <div class="weight-item">
                <input type="checkbox" bind:checked={entry.enabled} />
                <input
                    type="range"
                    bind:value={entry.value}
                    min="-10.0"
                    max="10.0"
                    step="0.1"
                />
                <span class="weight-value">{entry.value}</span>
                <span class="weight-name">{$t(`feature.${key}.name`)}</span>
                <button
                    onclick={() => {
                        selectedFeature = key;
                        infoDialog.showModal();
                    }}>?</button
                >
            </div>
        {/each}
    </div>
</div>
<dialog bind:this={infoDialog}>
    <h1>{$t(`feature.${selectedFeature}.name`)}</h1>
    <p>{$t(`feature.${selectedFeature}.description`)}</p>
    <div class="example">
        <h2>{$t("general.example")}</h2>
        <p>{$t(`feature.${selectedFeature}.example`)}</p>
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
        padding: 1rem;
        border: 1px solid var(--border);
        background: var(--bg0);
        color: var(--fg0);
        box-shadow:
            0 4px 8px 0 rgba(0, 0, 0, 0.2),
            0 6px 20px 0 rgba(0, 0, 0, 0.19);
    }
    @media (min-width: 600px) {
        dialog {
            min-width: 550px;
        }
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
        margin-top: 0.5rem;
    }
    .example-board {
        margin-top: 0.5rem;
        display: flex;
        justify-content: center;
    }
    .close-button {
        position: absolute;
        top: 0;
        right: 0;
    }
    dialog::backdrop {
        backdrop-filter: blur(6px);
        background: rgba(0, 0, 0, 0.3);
    }
</style>
