<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import type {
        TrainGeneration,
        TrainState,
        WorkerCommand,
        WorkerMessage,
    } from "./worker";
    import WeightsDisplay from "./WeightsDisplay.svelte";
    import { Weights } from "./weights.svelte";
    import TetrisBoard from "./TetrisBoard.svelte";
    import { TetrisSimulator } from "./simulator.svelte";
    import { TetrisAnimator } from "./animator.svelte";

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

    let tetrisBoard: TetrisBoard;
    let sim = new TetrisSimulator();
    let animator = new TetrisAnimator(
        sim,
        () => tetrisBoard.display(),
        () => {
            console.log("Game over");
        },
    );

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
        startSim();
    }

    function startSim(weightValues?: number[]) {
        let weights: Weights;
        if (weightValues) {
            weights = Weights.fromValues(FEATURE_KEYS, weightValues);
        } else {
            weights = new Weights(FEATURE_KEYS.map((key) => [key, 0]));
        }
        sim.reset();
        sim.updateWeights(weights.getWeightsMap());
        tetrisBoard.clear();
        animator.restart();
        animator.setSpeed(80);
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
                        startSim(trainGeneration.weights);
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

        startSim();

        return () => {
            worker.terminate();
        };
    });

    onDestroy(() => {
        animator.stop();
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
    <div class="board">
        <TetrisBoard
            bind:this={tetrisBoard}
            bind:state={sim.state}
            bind:currentMove={animator.currentMove}
        />
    </div>
    <div class="weights">
        <div class="weights-item">
            <h2>Current Weights</h2>
            {#if trainState}
                <WeightsDisplay
                    weightKeys={FEATURE_KEYS}
                    weightValues={trainState.eval_result.weights}
                />
            {/if}
        </div>
        <div class="weights-item">
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
</div>

<style>
    .weights {
        display: flex;
        justify-content: space-between;
        gap: 20px;
    }
    .weights-item {
        flex: 1;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
    }
</style>
