<script lang="ts">
    import {onMount} from "svelte";
    import {updateFunction} from "$stores/backCanvas";

    let canvas: HTMLCanvasElement;

    function update() {
        $updateFunction(canvas);
    }

    function animationLoop() {
        update();
        requestAnimationFrame(animationLoop);
    }

    onMount(async () => {
        resize();

        // TODO debounce
        new ResizeObserver(resize).observe(document.body);

        requestAnimationFrame(animationLoop);
    });


    function resize() {
        let ctx = canvas.getContext("2d")!;

        let previousImage = ctx.getImageData(0,0, canvas.width, canvas.height);

        ctx.canvas.width  = window.innerWidth;
        ctx.canvas.height = Math.max(window.innerHeight, document.body.scrollHeight);

        ctx.putImageData(previousImage, 0, 0);
    }
</script>

<canvas
        bind:this={canvas}
>
</canvas>

<style>
    canvas {
        position: absolute;
        top: 0;
        left: 0;
        z-index: -10;
    }
</style>