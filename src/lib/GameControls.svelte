<script lang="ts">
    import { t } from "$lib/translations";
    import DynamicIcon from "./DynamicIcon.svelte";

    let {
        isRunning = $bindable(),
        gameOver = $bindable(),
        onPauseToggle,
        onNewGame,
    } = $props();
</script>

<div>
    <button
        onclick={() => onPauseToggle()}
        disabled={gameOver}
        title={isRunning ? $t("controls.pause") : $t("controls.play")}
    >
        {#if isRunning}
            <DynamicIcon icon="pause" alt="Pause" />
            {$t("controls.pause")}
        {:else}
            <DynamicIcon icon="play" alt="Play" />
            {$t("controls.play")}
        {/if}
    </button>
    <button onclick={() => onNewGame()} title={$t("controls.new_game")}>
        <DynamicIcon icon="reset" alt="Reset" />
        {$t("controls.new_game")}
    </button>
</div>

<style>
    div {
        width: 100%;
        justify-content: center;
        display: flex;
        gap: 12px;
        height: fit-content;
    }

    button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.2rem;

        min-width: 4rem;
        height: 1.5rem;
        font-size: 1.1rem;
    }
</style>
