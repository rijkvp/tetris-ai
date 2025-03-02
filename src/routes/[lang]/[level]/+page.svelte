<script lang="ts">
    import type { PageProps } from "./$types";
    import Game from "$lib/Game.svelte";
    import type { Level } from "$lib/levels";

    let { data }: PageProps = $props();
    const level: Level = data.level!;
</script>

<Game {level} />
