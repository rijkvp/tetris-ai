<script lang="ts">
    import { t, locale, locales, setLocale } from "$lib/translations";
    import WeightsControl from "$lib/WeightsControl.svelte";
    import Tetris from "$lib/Tetris.svelte";
    import Scoreboard from "$lib/Scoreboard.svelte";
    import type { Stats } from "$lib/types";

    let tetris: Tetris;
    let scoreboard: Scoreboard;

    let speedIndex = $state(2);
    const SPEED_MUTIPLIER = [
        0.1, 0.5, 1, 2, 5, 10, 20, 50, 100, 1000, 10000, 100000, 1000000,
    ];
    let speedMultiplier = $derived(SPEED_MUTIPLIER[speedIndex]);
</script>

<h1>Tetris AI</h1>
<select
    onchange={(e) => {
        const vaule = (e.target as HTMLSelectElement).value;
        setLocale(vaule);
    }}
>
    {#each $locales as lc}
        <option value={lc} selected={lc === $locale}>{lc}</option>
    {/each}
</select>
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
        />
        <Scoreboard bind:this={scoreboard} />
    </div>
</div>

<style>
    .panels {
        display: flex;
        justify-content: space-between;
        gap: 20px;
    }
</style>
