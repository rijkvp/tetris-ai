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
    import { Weights } from "$lib/weights.svelte";
    import WeightsDisplay from "$lib/components/WeightsDisplay.svelte";
    import TrainTetris from "$lib/components/TrainTetris.svelte";
    import Level from "$lib/components/Level.svelte";

    // This are all the features available in the training
    // TODO: Expensibility: should be defined elsewhere
    const FEATURE_NAMES: string[] = [
        "row_trans",
        "col_trans",
        "pits",
        "landing_height",
        "eroded_cells",
        "cuml_wells",
    ];

    let trainCriterion: TrainCriterion = $state("score");
    let currentFeatures: string[] = $state(FEATURE_NAMES);
    let featureNames: string[] = $state(FEATURE_NAMES);

    let isRunning: boolean = $state(false);
    let isTetrisRunning: boolean = $state(false);
    let trainState: TrainState | null = $state(null);
    let trainGeneration: TrainGeneration | null = $state(null);
    let trainTetris: TrainTetris;

    function restartTrain() {
        trainState = null;
        trainGeneration = null;
        if (trainTetris != null) {
            const weights = new Weights(FEATURE_NAMES.map((key) => [key, 0]));
            trainTetris.updateWeights(weights, true);
        }
        currentFeatures = featureNames;
        worker.postMessage({
            command: "restart",
            featureNames: $state.snapshot(featureNames),
            criterion: $state.snapshot(trainCriterion),
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
                        const weights = Weights.fromValues(
                            FEATURE_NAMES,
                            trainGeneration.weights,
                        );
                        trainTetris.updateWeights(weights, false);
                    }
                    break;
                case "status":
                    if (event.data.status === "started") {
                        isRunning = true;
                        isTetrisRunning = true;
                    } else if (event.data.status === "stopped") {
                        isRunning = false;
                        isTetrisRunning = false;
                    } else if (event.data.status === "finished") {
                        isRunning = false;
                    }
                    break;
                default:
                    console.error("Unknown message from worker:", event.data);
            }
        };

        return () => {
            stopTrain();
            worker.terminate();
        };
    });
</script>

<Level
    key="training"
    onStart={() => {
        restartTrain();
    }}
>
    {#snippet content()}
        <div class="left">
            <div class="controls">
                {#if isRunning}
                    <button onclick={stopTrain}>
                        <svg inline-src="stop" />
                        {$t("training.stop")}</button
                    >
                {:else}
                    <button onclick={restartTrain}>
                        <svg inline-src="arrow-clockwise" />

                        {$t("training.restart")}</button
                    >
                {/if}
            </div>
            <TrainTetris
                bind:this={trainTetris}
                bind:isRunning={isTetrisRunning}
            />
        </div>
    {/snippet}
    {#snippet side()}
        <div>
            <h2>
                {$t("training.settings")}
            </h2>
            <div class="setting">
                <label for="criterion">
                    {$t("training.criterion")}:
                </label>
                <select
                    name="criterion"
                    bind:value={trainCriterion}
                    disabled={isRunning}
                >
                    <option value="score">Score</option>
                    <option value="level">Level</option>
                    <option value="tetrises">Tetrises</option>
                </select>
            </div>
            <div class="setting">
                <label for="available_features">
                    {$t("training.available_features")}:
                </label>
                <select
                    name="available_features"
                    bind:value={featureNames}
                    disabled={isRunning}
                    multiple
                >
                    {#each FEATURE_NAMES as featureName}
                        <option value={featureName}>
                            {$t(`feature.${featureName}.name`)}
                        </option>
                    {/each}
                </select>
            </div>
        </div>
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
                            weightKeys={currentFeatures}
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
                                weightKeys={currentFeatures}
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
        justify-content: space-between;
        width: 100%;
    }
    .controls button {
        min-width: 8rem;
    }
    .setting {
        display: flex;
        gap: 0.5rem;
        margin: 0.5rem 0;
        align-items: center;
        justify-content: space-between;
    }
    .left {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        align-items: start;
    }
    .weights {
        display: flex;
        justify-content: space-between;
        flex-direction: column;
        gap: 1rem;
    }
</style>
