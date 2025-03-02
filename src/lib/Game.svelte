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
</script>

<h1>{level.name}</h1>
<p>{level.description}</p>

{#if level.goals}
    <div>
        <h2>Goals</h2>
        <ul>
            {#each Object.entries(level.goals) as [name, amount]}
                {#if amount > 0}
                    <li>{name}: {amount}</li>
                {/if}
            {/each}
        </ul>
    </div>
{/if}

<div class="panels">
    <Tetris
        bind:this={tetris}
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
            enabledFeatures={level.features}
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
</style>
