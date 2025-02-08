<script lang="ts">
    import type { Stats } from "./types";

    type Entry = {
        stats: Stats;
        latest: boolean;
    };

    let entries: Entry[] = $state([]);

    export const addEntry = (entry: Stats) => {
        console.log("Adding entry", entry);
        entries.forEach((e) => (e.latest = false));
        entries.push({ stats: entry, latest: true });
        entries.sort((a, b) => (b.stats.score < a.stats.score ? -1 : 1));
        entries = entries.slice(0, 10);
    };
</script>

<div>
    <h2>Scoreboard</h2>
    <table>
        <thead>
            <tr>
                <th>#</th>
                <th>Score</th>
                <th>Lines</th>
                <th>Level</th>
            </tr>
        </thead>
        <tbody>
            {#each entries as entry, n}
                <tr class={{ latest: entry.latest }}>
                    <td>{n + 1}</td>
                    <td>{entry.stats.score}</td>
                    <td>{entry.stats.lines}</td>
                    <td>{entry.stats.level}</td>
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
