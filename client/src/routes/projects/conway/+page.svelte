<script lang="ts">
    import {onMount} from 'svelte';

    import wasm, {Universe} from '../../../../../wasm/conway/pkg';
    import PrettyLink from '$components/PrettyLink.svelte';
    import Pane from "$components/Pane.svelte";

    let universe: any; // ohno!!! :0 any type!!!!

    let output = '';

    function rerender() {
        universe.tick();
        output = universe.render();
    }

    onMount(async () => {
        await wasm();
        universe = Universe.new();
        output = universe.render();
    });
</script>

<Pane>

    <p>conway conway pvp webassembly</p>
    <PrettyLink url="https://rustwasm.github.io/docs/book/game-of-life/implementing.html"
    >mostly learned from this great tutorial
    </PrettyLink
    >
    <p>i still plan to expand this before making this portfolio public</p>

    {#if !universe}
        <p>web assembly loading...</p>
    {:else}
        <pre on:click={() => rerender()}>{output}</pre>
    {/if}


</Pane>

<style>
    pre, p, h1, h2, h3 {
        padding: 1rem;
        margin: 0;
    }

    pre {
        font-size: 20px;
    }
</style>