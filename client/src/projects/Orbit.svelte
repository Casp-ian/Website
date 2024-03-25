<script>
    import { onMount } from "svelte";

    import wasm from '../../../wasm/orbit/pkg';
    import {Universe} from '../../../wasm/orbit/pkg';

    import PrettyLink from "../components/PrettyLink.svelte";
    import Code from "../components/Code.svelte";

    let universe;

    function update() {
        universe.step(1);
        x = universe.get_x();
        y = universe.get_y();
        radius = universe.get_radius();

        console.log(x[0], y[0], radius[0]);
    }

    let x;
    let y;
    let radius;

    onMount(async () => {
        await wasm();
        universe = Universe.new();
        setInterval(update, 50);
    });

    let codeSnippet = "fn testMethod(&mut self, test: Option<uint32>) {\n    test.unwrap()\n}"
</script>


<div id="text">
    <h1>Smooth orbiting balls</h1>
    <p>In trying to learn more about web assembly and rust i was looking for fun projects to make.</p>
    <PrettyLink url="https://github.com/johnBuffer/NoCol">Eventually i found this!</PrettyLink>
    <p>This is my first try</p>
    <Code language='rust' text={codeSnippet}></Code>
</div>

{#if !radius}
    <p>web assembly loading...</p>
{:else}
    {#each radius as r, i(i)}
        <div class="ball" style="width: {r * 2}px; height: {r * 2}px; left: calc(50% - {x[i]}px); top: calc(50% - {y[i]}px);"></div>
    {/each}
{/if}


<style>
    #text {
        margin: 10rem;
    }

    .ball {
        position: absolute;
        background-color: #ff3e00;

        width: 100px;
        height: 100px;

    }
</style>
