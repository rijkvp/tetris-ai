<script lang="ts">
    import { onMount } from "svelte";
    import type { Stats } from "./types";

    type Entry = {
        stats: Stats;
        latest: boolean;
    };

    let entries: Entry[] = $state([]);

    type OrderCategory = {
        label: string;
        key: (a: Entry) => bigint;
    };

    const ORDER_CATEGORIES: OrderCategory[] = [
        { label: "Score", key: (a: Entry) => a.stats.score },
        { label: "Lines", key: (a: Entry) => a.stats.lines },
        { label: "Level", key: (a: Entry) => a.stats.level },
        { label: "Tetrises", key: (a: Entry) => a.stats.tetrises },
    ];

    let orderCategoryIdx = $state(0);

    $effect(() => {
        if (entries.length > 0) {
            localStorage.setItem("scores", JSON.stringify(entries));
        }
    });

    onMount(() => {
        const json = localStorage.getItem("scores");
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
    <h2>Scoreboard</h2>

    <div>
        {#each ORDER_CATEGORIES as order, idx}
            <button
                onclick={() => setOrder(idx)}
                disabled={idx === orderCategoryIdx}
            >
                {order.label}
            </button>
        {/each}
    </div>
    <table>
        <thead>
            <tr>
                <th>#</th>
                <th>Score</th>
                <th>Lines</th>
                <th>Level</th>
                <th>Tetrises</th>
            </tr>
        </thead>
        <tbody>
            {#each entries as entry, n}
                <tr class={{ latest: entry.latest }}>
                    <td>{n + 1}</td>
                    <td>{entry.stats.score}</td>
                    <td>{entry.stats.lines}</td>
                    <td>{entry.stats.level}</td>
                    <td>{entry.stats.tetrises}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<style>
    table {
        width: 100%;
        border-collapse: collapse;
    }

    th,
    td {
        padding: 4px 8px;
        border: 1px solid #ddd;
    }

    th {
        text-align: left;
    }

    .latest {
        background-color: #ddd;
    }
</style>
