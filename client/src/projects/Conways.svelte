<script>
    import { onMount } from "svelte";

    import wasm from '../../../wasm/conway/pkg';
    import {Universe} from '../../../wasm/conway/pkg';
    import PrettyLink from "../components/PrettyLink.svelte";

    let universe;

    let output = "";

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


<p>conway conway pvp webassembly</p>
<PrettyLink url="https://rustwasm.github.io/docs/book/game-of-life/implementing.html">mostly learned from this great tutorial</PrettyLink>
<p>i still plan to expand this before making this portfolio public</p>

{#if !universe}
    <p>web assembly loading...</p>
{:else}
    <pre on:click={() => rerender()}>{output}</pre>
{/if}


<style>
    pre {
        font-size: 20px;
    }
</style>
