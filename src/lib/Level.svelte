<script lang="ts">
    let { title, content, explanation, hints } = $props();

    let inExplanation = $state(true);
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
            {@render hints()}
        </div>
    </div>
{/if}

<style>
    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
    }
    nav button {
        min-width: 4rem;
    }
</style>
