<script lang="ts">
    import { t } from "$lib/translations";

    let {
        isRunning = $bindable(),
        gameOver = $bindable(),
        onPauseToggle,
        onNewGame,
    } = $props();
</script>

<div>
    <button
        class:btn-primary={!isRunning && !gameOver}
        onclick={() => onPauseToggle()}
        disabled={gameOver}
        title={isRunning ? $t("controls.pause") : $t("controls.play")}
    >
        {#if isRunning}
            <svg inline-src="pause" />
            {$t("controls.pause")}
        {:else}
            <svg inline-src="play" />
            {$t("controls.play")}
        {/if}
    </button>
    <button
        class:btn-primary={gameOver}
        onclick={() => onNewGame()}
        title={$t("controls.new_game")}
    >
        <svg inline-src="arrow-clockwise" />
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

        min-width: 5.5rem;
    }
</style>
