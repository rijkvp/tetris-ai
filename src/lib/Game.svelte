<script lang="ts">
    import { t, locale } from "$lib/translations";
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
            goal.progress = stats[goal.key]!;
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
            <h1>{level.name[$locale]}</h1>
            <p>{level.description[$locale]}</p>
            {#if level.goals}
                <div class="goals">
                    <h2>{$t("goals.goals")}</h2>
                    <ul>
                        {#each goals as item}
                            {#if item.goal > 0}
                                <li
                                    class="goal-item"
                                    class:goal-complete={item.progress >=
                                        item.goal}
                                >
                                    {$t(`score.${item.key}`)}: {item.progress}/{item.goal}
                                </li>
                            {/if}
                        {/each}
                    </ul>
                    {#if goalsComlete}
                        <div class="goals-complete-text">
                            {$t("goals.complete")}
                        </div>
                    {:else}
                        <div class="goals-info-text">
                            {$t("goals.info")}
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
        flex-wrap: wrap;
        justify-content: space-between;
        gap: 2rem;
    }
    .panel {
        /* fill up the remaining space */
        flex-grow: 1;
        flex-shrink: 1;
        flex-basis: 0;

        display: flex;
        flex-direction: column;
        flex-basis: min-content;
        gap: 1rem;
    }
    .goals {
        margin-top: 1rem;
    }
    .goal-item {
        font-size: 1.1rem;
        margin-bottom: 5px;
    }
    .goal-item:not(.goal-complete) {
        font-weight: bold;
        text-decoration: underline;
    }
    .goal-complete {
        text-decoration: line-through;
        opacity: 0.8;
    }
    .goals-info-text {
        font-weight: bold;
        color: red;
    }
    .goals-complete-text {
        font-weight: bold;
        color: green;
    }
</style>
