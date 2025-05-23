<script lang="ts">
    import { t } from "$lib/translations";
    import { Weights } from "$lib/weights.svelte";
    import { localState } from "$lib/stores.svelte";
    import ExampleBoard from "$lib/components/ExampleBoard.svelte";

    let { weights }: { weights: Weights } = $props();

    let selectedFeature: string = $state("col_trans");
    let infoDialog: HTMLDialogElement;
</script>

<div>
    <h2>{$t("general.features")}</h2>
    <div class="buttons">
        <button
            onclick={() => {
                weights.randomize();
            }}
        >
            <svg inline-src="shuffle" />
            {$t("feature_control.randomize")}</button
        >
        <button
            onclick={() => {
                weights.reset();
            }}
        >
            <svg inline-src="arrow-clockwise" />
            {$t("feature_control.reset")}</button
        >
    </div>
    <div style:display={localState.cheatMode ? "inline" : "none"}>
        <h2>Cheat mode</h2>
        <button
            onclick={() => {
                weights.loadPreset("score");
            }}>Preset (score)</button
        >
        <button
            onclick={() => {
                weights.loadPreset("levels");
            }}>Preset (levels)</button
        >
    </div>
    <div class="weights-list">
        {#each weights.entries() as [key, entry]}
            <input
                type="checkbox"
                bind:checked={entry.enabled}
                disabled={entry.locked}
            />
            <span class="weight-value">{entry.value}</span>
            <input
                type="range"
                bind:value={entry.value}
                disabled={entry.locked}
                min="-10.0"
                max="10.0"
                step="0.1"
            />
            <span class="weight-name">
                {$t(`feature.${key}.name`) === `feature.${key}.name`
                    ? key
                    : $t(`feature.${key}.name`)}
            </span>
            <button
                disabled={$t(`feature.${key}.description`) ===
                    `feature.${key}.description`}
                onclick={() => {
                    selectedFeature = key;
                    infoDialog.showModal();
                }}>?</button
            >
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
    <button class="close-button" onclick={() => infoDialog.close()}>
        <svg inline-src="x" />
        {$t("general.close")}
    </button>
</dialog>

<style>
    .weights-list {
        display: grid;
        grid-template-columns: max-content 1.8rem 150px max-content min-content;
        column-gap: 1rem;
        row-gap: 0.4rem;
        margin: 0.5rem 0;
        justify-items: stretch;
        align-items: center;
        max-height: 300px;
        overflow-y: auto;
    }
    .weight-value {
        font-weight: bold;
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
    .buttons button {
        min-width: 5.5rem;
    }
    .example {
        margin-top: 0.5rem;
        overflow: hidden;
    }
    .example-board {
        margin-top: 0.5rem;
        display: flex;
        justify-content: center;
        transform: scale(0.9);
        transform-origin: top center;
        overflow: hidden;
        height: 576px;
    }
    .close-button {
        position: absolute;
        top: 0;
        right: 0;
        display: flex;
        align-items: center;
        font-size: 1rem;
        gap: 0.2rem;
    }
    .close-button svg {
        width: 0.9rem;
        height: 0.9rem;
    }
    dialog::backdrop {
        backdrop-filter: blur(6px);
        background: rgba(0, 0, 0, 0.3);
    }
</style>
