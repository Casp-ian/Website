<script lang="ts">
    import {onDestroy, onMount} from "svelte";

    import wasm from "$wasm/orbit/pkg";
    import {Universe, Ball} from '$wasm/orbit/pkg';
    import {browser} from "$app/environment";

    let universe: any;

    function update() {
        universe.step(1);

        clear();

        for (let i = 0; i < universe.get_ball_count(); i++) {
            let ball: Ball = universe.get_ball(i);
            drawBall(ball);
        }
    }

    function animationLoop() {
        update();
        requestAnimationFrame(animationLoop);
    }

    onMount(async () => {
        await wasm();
        universe = Universe.new();
        universe.set_center(400, 400);

        resize();
        new ResizeObserver(resize).observe(document.body);
        addEventListener("wheel", resize);

        requestAnimationFrame(animationLoop);
    });

    onDestroy(async () => {
        if (browser) {
            removeEventListener("wheel", resize);
        }
    })

    let canvas: HTMLCanvasElement;

    function drawBall(ball: Ball) {
        let ctx = canvas.getContext("2d")!;

        ctx.beginPath();
        ctx.arc(ball.x, ball.y, ball.radius, 0, 2 * Math.PI);
        ctx.fillStyle = "red";
        ctx.fill();
    }

    function clear() {
        let ctx = canvas.getContext("2d")!;
        ctx.clearRect(0, 0, canvas.width, canvas.height);
    }

    function resize() {
        let ctx = canvas.getContext("2d")!;

        ctx.canvas.width  = window.innerWidth;
        ctx.canvas.height = Math.max(window.innerHeight, document.body.scrollHeight);

        universe.set_center(window.innerWidth / 2, window.innerHeight / 2 + window.scrollY);
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