<script lang="ts">
    import {onMount} from "svelte";

    let canvas: HTMLCanvasElement;

    function redraw() {
            let ctx = canvas.getContext("2d")!;
            ctx.canvas.width  = window.innerWidth;
            ctx.canvas.height = Math.max(window.innerHeight, document.body.scrollHeight);

            ctx.moveTo(0, 0);
            ctx.strokeStyle = "orange";
            ctx.lineTo(2000, 2000);
            ctx.stroke();
    }

    onMount(async () => {
        redraw();
        new ResizeObserver(redraw).observe(document.body);
    });

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