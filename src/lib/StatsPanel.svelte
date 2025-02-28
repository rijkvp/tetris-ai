<script lang="ts">
    import { t } from "$lib/translations";
    import type { Stats } from "./types";

    let stats: Stats = $state({
        score: BigInt(0),
        lines: BigInt(0),
        level: BigInt(0),
        steps: BigInt(0),
        tetrises: BigInt(0),
    });

    export const update = (state: Stats) => {
        stats = state;
    };
</script>

<div class="stats">
    <div class="item">
        <div class="label">{$t("score.score")}</div>
        <div class="value">{stats.score}</div>
    </div>
    <div class="item">
        <div class="label">{$t("score.lines")}</div>
        <div class="value">{stats.lines}</div>
    </div>
    <div class="item">
        <div class="label">{$t("score.level")}</div>
        <div class="value">{stats.level}</div>
    </div>
    <div class="item">
        <div class="label">{$t("score.tetrises")}</div>
        <div class="value">{stats.tetrises}</div>
    </div>
</div>

<style>
    .stats {
        display: flex;
        flex-direction: column;
        width: 100px;

        padding: 0.8rem 0.4rem;
        border: 2px solid #fff;
        border-radius: 0px;
        background-color: #070707;
        box-shadow:
            0 4px 8px 0 rgba(0, 0, 0, 0.2),
            0 6px 20px 0 rgba(0, 0, 0, 0.19);
    }
    .item {
        margin: 0.2rem 0;
    }
    .label,
    .value {
        text-align: center;
    }
    .label {
        font-weight: bold;
        font-size: 0.8em;
    }
    .value {
        font-size: 1.2em;
    }
</style>
