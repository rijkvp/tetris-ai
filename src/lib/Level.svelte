<script lang="ts">
    let { title, content, explanation = null, side } = $props();

    let inExplanation = $state(explanation != null);
</script>

<nav>
    <button>Previous</button>
    <h1>{title}</h1>
    <button>Next</button>
</nav>

{#if inExplanation}
    <div>
        {@render explanation()}
    </div>
    <button onclick={() => (inExplanation = false)}>Start level</button>
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
