<script lang="ts">
    import { t } from "$lib/translations";
    import { onMount } from "svelte";
    import type { Stats } from "./types";

    let { key }: { key: string } = $props();

    type Entry = {
        stats: Stats;
        latest: boolean;
    };

    let entries: Entry[] = $state([]);

    type OrderCategory = {
        tkey: string;
        key: (a: Entry) => bigint;
    };

    const ORDER_CATEGORIES: OrderCategory[] = [
        { tkey: "score.score", key: (a: Entry) => a.stats.score },
        { tkey: "score.lines", key: (a: Entry) => a.stats.lines },
        { tkey: "score.level", key: (a: Entry) => a.stats.level },
        { tkey: "score.tetrises", key: (a: Entry) => a.stats.tetrises },
    ];

    let orderCategoryIdx = $state(0);

    $effect(() => {
        if (entries.length > 0) {
            localStorage.setItem(`scores-${key}`, JSON.stringify(entries));
        }
    });

    onMount(() => {
        const json = localStorage.getItem(`scores-${key}`);
        entries = json ? JSON.parse(json) : [];
    });

    function reorder() {
        const order = ORDER_CATEGORIES[orderCategoryIdx];
        entries.sort((a, b) => (order.key(b) < order.key(a) ? -1 : 1));
    }

    export const addEntry = (entry: Stats) => {
        if (entry.score == BigInt(0)) {
            return;
        }
        entries.forEach((e) => (e.latest = false));
        entries.push({ stats: entry, latest: true });
        reorder();
    };

    function setOrder(idx: number) {
        orderCategoryIdx = idx;
        reorder();
    }
</script>

<div>
    <h2>{$t("general.scoreboard")} ({key})</h2>

    <div>
        {#each ORDER_CATEGORIES as order, idx}
            <button
                onclick={() => setOrder(idx)}
                disabled={idx === orderCategoryIdx}
            >
                {$t(order.tkey)}
            </button>
        {/each}
    </div>
    <table>
        <thead>
            <tr>
                <th>#</th>
                <th>{$t("score.score")}</th>
                <th>{$t("score.lines")}</th>
                <th>{$t("score.level")}</th>
                <th>{$t("score.tetrises")}</th>
            </tr>
        </thead>
        <tbody>
            {#each entries.slice(0, 10) as entry, n}
                <tr class={{ latest: entry.latest }}>
                    <td>{n + 1}</td>
                    <td>{entry.stats.score.toLocaleString()}</td>
                    <td>{entry.stats.lines.toLocaleString()}</td>
                    <td>{entry.stats.level.toLocaleString()}</td>
                    <td>{entry.stats.tetrises.toLocaleString()}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<style>
    table {
        width: 100%;
        border-collapse: collapse;
        font-size: 0.9em;
    }

    th,
    td {
        padding: 4px 8px;
        border: 1px solid var(--fg1);
    }

    th {
        text-align: left;
    }

    .latest {
        background-color: var(--bg1);
    }
</style>
