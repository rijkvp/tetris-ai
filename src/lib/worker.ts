import init, { Trainer } from "tetris-ai";

export type TrainCriterion = "score" | "level" | "tetrises";

export type WorkerCommand =
    | { command: 'restart', featureNames: string[], criterion: TrainCriterion }
    | { command: 'stop' };

export type EvalResult = {
    weights: number[];
    score: number;
}

export type TrainGeneration = {
    weights: number[];
    std_dev: number[];
    max: number;
    min: number;
    mean: number
};

export type TrainState = {
    gen_index: number;
    model_index: number;
    eval_result: EvalResult;
    generation: TrainGeneration;
}

export type WorkerMessage =
    | { type: 'train_state'; data: TrainState }
    | { type: 'status'; status: 'stopped' | 'started', message?: string };

let trainer: Trainer;

let isRunning: boolean = false;
let isStopRequested: boolean = false;

function stepTrainer(): boolean {
    if (trainer.is_stable()) {
        self.postMessage({ type: 'status', status: 'stopped' } satisfies WorkerMessage);
        return true;
    }

    const trainState = trainer.step();
    self.postMessage({ type: 'train_state', data: trainState.into_js() } satisfies WorkerMessage);
    return false;
}

async function runTrainingLoop(): Promise<void> {
    if (isRunning) {
        console.warn("Attempted to start loop but it's already running.");
        return;
    }
    isRunning = true;
    isStopRequested = false;
    self.postMessage({ type: 'status', status: 'started' } satisfies WorkerMessage);

    while (!isStopRequested) {
        if (stepTrainer()) break;

        await new Promise(resolve => setTimeout(resolve, 0)); // yield to the event loop
    }

    isRunning = false;
    isStopRequested = false;
    self.postMessage({ type: 'status', status: 'stopped' } satisfies WorkerMessage);
}


self.onmessage = (event: MessageEvent<WorkerCommand>): void => {
    switch (event.data.command) {
        case 'restart':
            if (!isRunning) {
                trainer = Trainer.from_feature_names(event.data.featureNames, event.data.criterion);
                runTrainingLoop();
            }
            break;
        case 'stop':
            if (isRunning) {
                isStopRequested = true;
            }
            break;
        default:
            console.error("Worker: Unknown message command received:", event.data);
    }
};

async function initWorker() {
    await init(); // initialize the WASM module
}

initWorker();
