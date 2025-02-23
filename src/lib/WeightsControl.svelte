<script lang="ts">
    import { t } from "$lib/translations";
    import { onMount, onDestroy } from "svelte";
    import { Weights } from "tetris-ai";

    let weightValues = $state([...Weights.defaults().values()]);
    let weightsInfo = Weights.info();
    let cheatActive = $state(false);

    let { onWeightsChange }: { onWeightsChange: (weights: Weights) => void } =
        $props();

    function updateWeights() {
        const weights = Weights.from_values(new Float64Array(weightValues));
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
    <button
        style:display={cheatActive ? "inline" : "none"}
        onclick={() => {
            weightValues = [...Weights.preset().values()];
            updateWeights();
        }}>Preset</button
    >
    <div class="weights-grid">
        {#each weightValues as weight, n}
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
        grid-template-columns: 200px auto 20px 20px;
        gap: 1rem;
    }
    dialog {
        inset: 50%;
        transform: translate(-50%, -50%);
        width: 500px;
        padding: 1rem;
        border: none;
    }
    dialog::backdrop {
        backdrop-filter: blur(6px);
        background: rgba(0, 0, 0, 0.3);
    }
</style>
