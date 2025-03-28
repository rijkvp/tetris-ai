<script lang="ts">
    import Level from "$lib/Level.svelte";
    import PlayableTetris from "$lib/PlayableTetris.svelte";
</script>

<Level title="1. Introduction to Tetris">
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
                once—especially four lines (called a <em>Tetris</em>)—to
                boost your score.
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
    {#snippet hints()}
        <h2>Controls</h2>
        <ul>
            <li>Left arrow: Move the tetromino to the left</li>
            <li>Right arrow: Move the tetromino to the right</li>
            <li>Down arrow: Move the tetromino down</li>
            <li>Up arrow: Rotate the tetromino</li>
            <li>Space: Drop the tetromino</li>
        </ul>
    {/snippet}
</Level>
