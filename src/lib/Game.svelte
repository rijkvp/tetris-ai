<script lang="ts">
    import { t } from "$lib/translations";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import type { Stats } from "$lib/types";
    import type { Level } from "./levels";

    const { level }: { level: Level } = $props();

    let tetris: Tetris;
    let scoreboard: Scoreboard;

    let speedIndex = $state(2);
    const SPEED_MUTIPLIER = [
        0.1, 0.5, 1, 2, 5, 10, 20, 50, 100, 1000, 10000, 100000, 1000000,
    ];
    let speedMultiplier = $derived(SPEED_MUTIPLIER[speedIndex]);

    type GoalItem = {
        key: string;
        progress: number;
        goal: number;
    };

    let goals: GoalItem[] = $state(
        Object.entries(level.goals || {}).map(([key, goal]) => ({
            key,
            progress: 0,
            goal,
        })),
    );

    let goalsComlete = $derived(
        goals.every((goal) => goal.progress >= goal.goal),
    );

    function checkGoals(stats: Stats) {
        for (const goal of goals) {
            goal.progress = stats[goal.key]!;
        }
    }
</script>

<h1>{level.name}</h1>
<p>{level.description}</p>

{#if level.goals}
    <div>
        <h2>{$t("goals.goals")}</h2>
        {#each goals as item}
            {#if item.goal > 0}
                <div
                    class="goal-item"
                    class:goal-complete={item.progress >= item.goal}
                >
                    {$t(`score.${item.key}`)}: {item.progress}/{item.goal}
                </div>
            {/if}
        {/each}
        {#if goalsComlete}
            <div class="goals-complete-text">All goals complete!</div>
        {/if}
    </div>
{/if}

<div class="panels">
    <Tetris
        bind:this={tetris}
        onNewStats={(stats: Stats) => checkGoals(stats)}
        onGameOver={(stats: Stats) => scoreboard.addEntry(stats)}
    />
    <div>
        <h2>{$t("speed")}</h2>
        <input
            id="speed-input"
            type="range"
            min="0"
            max={SPEED_MUTIPLIER.length - 1}
            bind:value={speedIndex}
            oninput={() => tetris.setSpeed(speedMultiplier)}
        />
        <label for="speed-input">{speedMultiplier.toLocaleString()}x</label>
        <WeightsControl
            onWeightsChange={(weights) => tetris.setWeights(weights)}
            availableFeatures={level.features}
        />
        <Scoreboard key={level.key} bind:this={scoreboard} />
    </div>
</div>

<style>
    .panels {
        display: flex;
        justify-content: space-between;
        gap: 20px;
    }
    .goal-complete {
        text-decoration: line-through;
        opacity: 0.8;
    }
    .goals-complete-text {
        font-weight: bold;
        font-size: 1.2em;
        color: green;
    }
</style>
