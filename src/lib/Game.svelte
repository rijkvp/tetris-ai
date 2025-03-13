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
            goal.progress = stats[goal.key];
        }
    }
</script>

<div class="panels">
    <Tetris
        bind:this={tetris}
        onNewStats={(stats: Stats) => checkGoals(stats)}
        onGameOver={(stats: Stats) => scoreboard.addEntry(stats)}
    />
    <div class="panel">
        <div>
            <h1>{level.name}</h1>
            <p>{level.description}</p>
            {#if level.goals}
                <div class="goals">
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
                        <div class="goals-complete-text">
                            All goals complete!
                        </div>
                    {/if}
                </div>
            {/if}
        </div>
        <div>
            <WeightsControl
                onWeightsChange={(weights) => tetris.setWeights(weights)}
                availableFeatures={level.features}
            />
        </div>
        <div>
            <Scoreboard key={level.key} bind:this={scoreboard} />
        </div>
    </div>
</div>

<style>
    .panels {
        display: flex;
        justify-content: space-between;
        gap: 2rem;
    }
    .panel {
        /* fill up the remaining space */
        flex-grow: 1;
        flex-shrink: 1;
        flex-basis: 0;

        margin-top: 2rem;
        display: flex;
        flex-direction: column;
        flex-basis: min-content;
        gap: 1rem;
    }
    .goals {
        margin-top: 1rem;
    }
    .goal-item {
        margin-bottom: 5px;
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
