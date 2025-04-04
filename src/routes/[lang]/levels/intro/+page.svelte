<script lang="ts">
    import Level from "$lib/Level.svelte";
    import PlayableTetris from "$lib/PlayableTetris.svelte";
</script>

<Level key="intro" title="Introduction to Tetris">
    {#snippet content()}
        <PlayableTetris />
    {/snippet}
    {#snippet explanation()}
        <h2>Welcome!</h2>
        <p>
            This 'game' lets you explore how an artificial intelligence (AI)
            works that plays the game Tetris. You’ll learn how an AI plays
            Tetris and how the AI is 'trained' to play better.
        </p>

        <h3>New to Tetris? Here’s what you need to know:</h3>
        <ul>
            <li>
                <strong>What is Tetris?</strong> It’s a classic video game where
                you fit puzzle pieces, called <em>tetrominos</em>, into a grid.
            </li>
            <li>
                <strong>The goal:</strong> Arrange the tetrominos to fill horizontal
                lines. When a line is full, it disappears, and you earn points!
            </li>
            <li>
                <strong>Earn more points:</strong> Clear multiple lines at
                once—especially four lines (called a <em>Tetris</em>)—to boost
                your score.
            </li>
            <li>
                <strong>Game over:</strong> If the tetrominos stack up to the top
                of the board, the game ends.
            </li>
        </ul>
        <p>
            First, let's start by playing Tetris yourself to get a feel for the
            game.
        </p>
    {/snippet}
    {#snippet side()}
        <h2>Controls</h2>
        <ul>
            <li>
                <strong>Left/Right arrow keys</strong>: move the tetromino to
                the left/right
            </li>
            <li>Up arrow/Z: Rotate the tetromino</li>
            <li>Down arrow: Move the tetromino down (soft drop)</li>
            <li>Space: Drop the tetromino (hard drop)</li>
        </ul>
    {/snippet}
</Level>
