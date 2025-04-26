<script lang="ts">
    import { onMount } from "svelte";
    import type {
        TrainGeneration,
        TrainState,
        WorkerCommand,
        WorkerMessage,
    } from "./worker";
    import WeightsDisplay from "./WeightsDisplay.svelte";

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

    function startTrain() {
        worker.postMessage({ command: "start" } satisfies WorkerCommand);
    }

    function stopTrain() {
        worker.postMessage({ command: "stop" } satisfies WorkerCommand);
    }

    function resetTrain() {
        worker.postMessage({ command: "reset" } satisfies WorkerCommand);
        trainState = null;
        trainGeneration = null;
    }

    let worker: Worker;

    onMount(() => {
        worker = new Worker(new URL("./worker.ts", import.meta.url), {
            type: "module",
        });

        worker.onmessage = (event: MessageEvent<WorkerMessage>) => {
            switch (event.data.type) {
                case "train_state":
                    trainState = event.data.data;
                    if (trainState.generation) {
                        trainGeneration = trainState.generation;
                    }
                    break;
                case "status":
                    console.log("Received status:", event.data.status);
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

<div>
    <div class="controls">
        {#if isRunning}
            <button onclick={stopTrain}>Stop training</button>
        {:else}
            <button onclick={startTrain}>Start training</button>
        {/if}
        <button onclick={resetTrain}>Reset training</button>
    </div>
    <div>
        {#if trainState}
            <p>
                Generation {trainState.gen_index}, Model {trainState.model_index}
            </p>
        {/if}
    </div>
    <div>
        <h2>Current Weights</h2>
        {#if trainState}
            <WeightsDisplay
                weightKeys={FEATURE_KEYS}
                weightValues={trainState.eval_result.weights}
            />
        {/if}
    </div>
    <div>
        <h2>Best of last generation</h2>
        {#if trainGeneration}
            <p>
                Min {trainGeneration.min}, Max {trainGeneration.max}, Mean {trainGeneration.mean}
            </p>
            <WeightsDisplay
                weightKeys={FEATURE_KEYS}
                weightValues={trainGeneration.weights}
            />
        {/if}
    </div>
</div>
