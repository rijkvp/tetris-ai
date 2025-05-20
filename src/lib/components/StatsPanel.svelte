<script lang="ts">
    import { t } from "$lib/translations";
    import type { Stats } from "$lib/types";
    import { localState } from "$lib/stores.svelte";

    let {
        stats = $bindable(),
        moveRate = $bindable(),
    }: { stats: Stats; moveRate?: number } = $props();
</script>

<div class="stats border">
    <div class="item">
        <div class="label">{$t("score.score")}</div>
        <div
            class="value"
            style:font-size={stats.score > 99999 ? "0.9rem" : "1.1rem"}
        >
            {stats.score.toLocaleString()}
        </div>
    </div>
    <div class="item">
        <div class="label">{$t("score.lines")}</div>
        <div class="value">{stats.lines.toLocaleString()}</div>
    </div>
    <div class="item">
        <div class="label">{$t("score.level")}</div>
        <div class="value">{stats.level.toLocaleString()}</div>
    </div>
    <div class="item">
        <div class="label">{$t("score.tetrises")}</div>
        <div class="value">{stats.tetrises.toLocaleString()}</div>
    </div>
    {#if localState.cheatMode}
        <div class="item">
            <div class="label">Moves/Sec</div>
            <div class="value">{moveRate}</div>
        </div>
    {/if}
</div>

<style>
    .stats {
        display: flex;
        flex-direction: column;
        width: 100px;

        padding: 0.8rem 0.4rem;
        box-shadow:
            0 4px 8px 0 rgba(0, 0, 0, 0.2),
            0 6px 20px 0 rgba(0, 0, 0, 0.19);
        gap: 0.8rem;
        background: var(--bg0);
        border: 3px solid var(--border);
    }
    .label,
    .value {
        text-align: center;
    }
    .label {
        font-weight: bold;
        margin: 0.4rem 0;
        font-size: 0.8rem;
        color: var(--fg1);
    }
    .value {
        font-size: 1.1rem;
        font-weight: bold;
    }
</style>
