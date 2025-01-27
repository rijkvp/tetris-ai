<script lang="ts">
    import { Weights } from "tetris-ai";

    let rowTrans = 0.0;
    export let onWeightsChange: (weights: Weights) => void;
</script>

<div>
    <h2>Weights</h2>
    <input
        id="row-trans-input"
        type="range"
        bind:value={rowTrans}
        on:input={() => {
            const weights = new Weights();
            weights.row_trans = rowTrans;
            onWeightsChange(weights);
        }}
        min="-10.0"
        max="10.0"
    />
    <label for="row-trans-input">Row transitions: TODO</label>
</div>
