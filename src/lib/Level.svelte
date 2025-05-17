<script lang="ts">
    import { base } from "$app/paths";
    import { t, locale } from "$lib/translations";
    import { LEVEL_INFO, LEVELS } from "$lib/levels";
    import { goto } from "$app/navigation";
    import type { Snippet } from "svelte";

    let {
        key,
        content,
        side,
    }: {
        key: string;
        content: Snippet;
        side?: Snippet;
    } = $props();
    const levelInfo = $derived(LEVEL_INFO[key]);

    let inExplanation = $state(true);

    let currentIndex = $derived(LEVELS.findIndex((n) => n === key));
    let prev = $derived(currentIndex > 0 ? LEVELS[currentIndex - 1] : null);
    let next = $derived(
        currentIndex < LEVELS.length - 1 ? LEVELS[currentIndex + 1] : null,
    );
</script>

<nav>
    <button
        disabled={prev == null}
        onclick={() => {
            if (prev) {
                inExplanation = true;
                goto(`${base}/${$locale}/levels/${prev}`);
            }
        }}
    >
        {$t("general.previous")}</button
    >
    <h1>{levelInfo.name[$locale]}</h1>
    <button
        disabled={next == null}
        onclick={() => {
            if (next) {
                inExplanation = true;
                goto(`${base}/${$locale}/levels/${next}`);
            }
        }}
    >
        {$t("general.next")}</button
    >
</nav>

{#if inExplanation && levelInfo.explanation != null}
    <div class="explanation">
        {@html levelInfo.explanation[$locale]}
    </div>
    <button
        onclick={() => {
            inExplanation = false;
        }}
        class="btn-big btn-primary"
    >
        {$t("general.level_start")}</button
    >
{:else}
    <div class="panels">
        {@render content()}
        <div class="panel">
            {#if levelInfo.explanation != null}
                <button onclick={() => (inExplanation = true)}
                    >{$t("general.level_explanation")}</button
                >
            {/if}
            {#if levelInfo.sideText != null}
                <div>
                    {@html levelInfo.sideText[$locale]}
                </div>
            {/if}
            {#if side != null}
                {@render side()}
            {/if}
        </div>
    </div>
{/if}

<style>
    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }
    nav button {
        min-width: 4rem;
    }
    .explanation {
        margin-bottom: 1rem;
    }
    .panels {
        display: flex;
        flex-wrap: wrap;
        gap: 2rem;
    }
    .panel {
        /* fill up the remaining space */
        flex-grow: 1;
        flex-shrink: 1;
        flex-basis: 0;

        flex-direction: column;
        flex-basis: min-content;
        gap: 1rem;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
</style>
