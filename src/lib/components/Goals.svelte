<script lang="ts">
    import { t } from "$lib/translations";
    import type { Stats } from "$lib/types";
    import type { Goals } from "$lib/levels";
    import { onMount } from "svelte";

    const { levelKey, goals }: { levelKey: string; goals: Goals } = $props();

    type GoalStat = "score" | "lines" | "level" | "tetrises";
    type GoalItem = {
        stat: GoalStat;
        progress: bigint;
        goal: bigint;
    };
    let completed = $state(false);
    let completedLevels: string[];

    let goalItems: GoalItem[] = $state(
        Object.entries(goals || {}).map(([stat, goal]) => ({
            stat: stat as GoalStat,
            progress: BigInt(0),
            goal: BigInt(goal),
        })),
    );

    let goalsComlete = $derived(
        goalItems.every((goal) => goal.progress >= goal.goal),
    );

    onMount(() => {
        const json = localStorage.getItem("completed_levels");
        completedLevels = json ? JSON.parse(json) : [];
    });

    function save() {
        localStorage.setItem(
            "completed_levels",
            JSON.stringify(completedLevels),
        );
    }

    export const updateGoals = (stats: Stats) => {
        for (const goal of goalItems) {
            goal.progress = stats[goal.stat];

            // mark level as completed
            if (goal.progress >= goal.goal) {
                completed = true;
                if (!completedLevels.includes(levelKey)) {
                    completedLevels.push(levelKey);
                    save();
                }
            }
        }
    };
</script>

<div class="goals">
    <h2>{$t("goals.goals")}</h2>
    <ul>
        {#each goalItems as item}
            {#if item.goal > 0}
                <li
                    class="goal-item"
                    class:goal-complete={item.progress >= item.goal}
                >
                    {$t(`score.${item.stat}`)}: {item.progress}/{item.goal}
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

<style>
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
