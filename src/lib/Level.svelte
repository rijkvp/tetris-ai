<script lang="ts">
    import { base } from "$app/paths";
    import { locale } from "$lib/translations";
    import { NAVIGATION } from "$lib/levels";
    import { goto } from "$app/navigation";
    let { title, key, content, explanation = null, side } = $props();

    let inExplanation = $state(true);
    let showExplanation = $derived(inExplanation && explanation != null);

    let currentIndex = $derived(NAVIGATION.findIndex((n) => n === key));
    let prev = $derived(currentIndex > 0 ? NAVIGATION[currentIndex - 1] : null);
    let next = $derived(
        currentIndex < NAVIGATION.length - 1
            ? NAVIGATION[currentIndex + 1]
            : null,
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
        Previous</button
    >
    <h1>{title}</h1>
    <button
        disabled={next == null}
        onclick={() => {
            if (next) {
                inExplanation = true;
                goto(`${base}/${$locale}/levels/${next}`);
            }
        }}
    >
        Next</button
    >
</nav>

{#if showExplanation}
    <div>
        {@render explanation()}
    </div>
    <button onclick={() => (inExplanation = false)} class="btn-big btn-primary">
        Start level</button
    >
{:else}
    <div class="panels">
        {@render content()}
        <div class="panel">
            {@render side()}
        </div>
    </div>
{/if}

<style>
    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem 0rem;
        margin-bottom: 0.5rem;
    }
    nav button {
        min-width: 4rem;
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
    }
</style>
