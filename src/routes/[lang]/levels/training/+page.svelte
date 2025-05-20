<script lang="ts">
    import { t } from "$lib/translations";
    import { onMount } from "svelte";
    import type {
        TrainCriterion,
        TrainGeneration,
        TrainState,
        WorkerCommand,
        WorkerMessage,
    } from "$lib/worker.ts";
    import WeightsDisplay from "$lib/WeightsDisplay.svelte";
    import { Weights } from "$lib/weights.svelte";
    import TrainTetris from "$lib/TrainTetris.svelte";
    import Level from "$lib/Level.svelte";

    let trainCriterion: TrainCriterion = $state("score");
    let isRunning: boolean = $state(false);
    let trainState: TrainState | null = $state(null);
    let trainGeneration: TrainGeneration | null = $state(null);

    const FEATURE_KEYS: string[] = [
        "row_trans",
        "col_trans",
        "pits",
        "landing_height",
        "eroded_cells",
        "cuml_wells",
    ];
    let weights = $state(new Weights(FEATURE_KEYS.map((key) => [key, 0])));

    function restartTrain() {
        trainState = null;
        trainGeneration = null;
        weights = new Weights(FEATURE_KEYS.map((key) => [key, 0]));
        worker.postMessage({
            command: "restart",
            criterion: trainCriterion,
        } satisfies WorkerCommand);
    }

    function stopTrain() {
        worker.postMessage({ command: "stop" } satisfies WorkerCommand);
    }

    let worker: Worker;

    onMount(() => {
        worker = new Worker(new URL("$lib/worker.ts", import.meta.url), {
            type: "module",
        });

        worker.onmessage = (event: MessageEvent<WorkerMessage>) => {
            switch (event.data.type) {
                case "train_state":
                    trainState = event.data.data;
                    if (trainState.generation) {
                        trainGeneration = trainState.generation;
                        weights = Weights.fromValues(
                            FEATURE_KEYS,
                            trainGeneration.weights,
                        );
                    }
                    break;
                case "status":
                    if (event.data.status === "started") {
                        isRunning = true;
                    } else if (event.data.status === "stopped") {
                        isRunning = false;
                    }
                    break;
                default:
                    console.error("Unknown message from worker:", event.data);
            }
        };

        return () => {
            worker.terminate();
        };
    });
</script>

<Level key="training">
    {#snippet content()}
        <div class="left">
            <div class="controls">
                {#if isRunning}
                    <button onclick={stopTrain}>
                        <svg inline-src="stop" alt="Stop" />
                        {$t("training.stop")}</button
                    >
                {:else}
                    <button onclick={restartTrain}>
                        <svg inline-src="arrow-clockwise" alt="Reset" />

                        {$t("training.restart")}</button
                    >
                {/if}
                <span class="criterion-label">{$t("training.criterion")}:</span>
                <select bind:value={trainCriterion}>
                    <option value="score">Score</option>
                    <option value="level">Level</option>
                    <option value="tetrises">Tetrises</option>
                </select>
            </div>
            <TrainTetris bind:weights bind:isRunning />
        </div>
    {/snippet}
    {#snippet side()}
        <div class="weights">
            {#if trainState != null}
                <div class="weights-item">
                    <h2>
                        {$t("training.generation")}
                        {trainState.gen_index}, {$t("training.model")}
                        {trainState.model_index}
                    </h2>
                    {#if trainState}
                        <WeightsDisplay
                            weightKeys={FEATURE_KEYS}
                            weightValues={trainState.eval_result.weights}
                        />
                    {/if}
                </div>
                {#if trainState.gen_index > 1}
                    <div class="weights-item">
                        <h2>
                            {$t("training.current_results")} ({$t(
                                "training.generation",
                            )}
                            {trainState.gen_index - 1})
                        </h2>
                        {#if trainGeneration}
                            <p>
                                {$t("training.min")}:
                                <strong>{trainGeneration.min}</strong>,
                                {$t("training.mean")}:
                                <strong>{trainGeneration.mean}</strong>,
                                {$t("training.max")}:
                                <strong>{trainGeneration.max}</strong>
                            </p>
                            <WeightsDisplay
                                weightKeys={FEATURE_KEYS}
                                weightValues={trainGeneration.weights}
                            />
                        {/if}
                    </div>
                {/if}
            {/if}
        </div>
    {/snippet}
</Level>

<style>
    .controls {
        display: flex;
        gap: 0.5rem;
        align-items: stretch;
        margin-bottom: 1rem;
        justify-content: space-between;
    }
    .controls button {
        min-width: 5rem;
    }
    .criterion-label {
        align-self: center;
        font-weight: bold;
    }
    .left {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
    .weights {
        display: flex;
        justify-content: space-between;
        flex-direction: column;
        gap: 1rem;
    }
</style>
