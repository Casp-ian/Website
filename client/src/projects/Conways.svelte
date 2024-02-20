<script>
    import { onMount } from "svelte";
    import wasm from '../../../wasm/conway/pkg';

    let gameOfLife;

    onMount(async () => {
        gameOfLife = await wasm();
    });
</script>


<p>conway conway pvp webassembly</p>

{#if !gameOfLife}
    <p>web assembly loading...</p>
{:else}
    <p>{gameOfLife.number(1)}</p>
{/if}