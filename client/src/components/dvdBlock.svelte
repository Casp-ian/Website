<script lang="ts">
    import {browser} from '$app/environment';
    import {onDestroy, onMount} from 'svelte';

    export let text = 'Hello :3';

    let block: HTMLElement;

    let color = 'yellow';

    // TODO randomise these >:)
    let speedY = 2;
    let speedX = 2;

    let x = 0;
    let y = 0;

    let maxX = 0;
    let maxY = 0;

    let width = 0;
    let height = 0;

    function setup() {
        width = block.getBoundingClientRect().width;
        height = block.getBoundingClientRect().height;
        maxX = block.parentElement!.getBoundingClientRect().width - width;
        maxY = block.parentElement!.getBoundingClientRect().height - height;
    }

    let interval: number;
    onMount(async () => {
        setup();
        tick();
        interval = setInterval(tick, 10);

        window.addEventListener('resize', setup);
    });


    onDestroy(async () => {
        // onMount only gets called in browser, so we should only run this on browser too...
        if (browser) {
            clearInterval(interval);
            window.removeEventListener('resize', setup);
        }
    });

    function tick() {

        y += speedY;
        x += speedX;

        if (y > maxY) {
            speedY = -Math.abs(speedY);
            if (maxY === 0) {
                y = 0;
            } else {
                y = y - (y % maxY);
            }
            color = 'red';
        } else if (y <= 0) {
            speedY = Math.abs(speedY);
            y = Math.abs(y);
            color = 'green';
        }

        if (x > maxX) {
            speedX = -Math.abs(speedX);
            if (maxX === 0) {
                x = 0;
            } else {
                x = x - (x % maxX);
            }
            color = 'blue';
        } else if (x <= 0) {
            speedX = Math.abs(speedX);
            x = Math.abs(x);
            color = 'yellow';
        }

        block.style.color = color;
        block.style.left = x + 'px';
        block.style.top = y + 'px';
    }
</script>

<strong id="block" bind:this={block}>
    {text}
</strong>

<style>
    #block {
        width: fit-content;
        height: fit-content;
        position: relative;

        font-size: 40px;
        line-height: 40px;
        margin: 0;
        padding: 0;
    }
</style>
