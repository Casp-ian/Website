<script lang="ts">
    import wasm, {Universe, Ball} from '$wasm/orbit/pkg';
    import {browser} from "$app/environment";
    import {onMount} from "svelte";

    let universe: any;

    // in loop

    // for (let i = 0; i < universe.get_ball_count(); i++) {
    //     let ball: Ball = universe.get_ball(i);
    //     drawBall(ball);
    // }

    onMount(async () => {
        await wasm();
        universe = Universe.new();
        universe.set_center(400, 400);
    });

    function drawBall(ball: Ball) {
        // let ctx = canvas.getContext("2d")!;
        //
        // ctx.beginPath();
        // ctx.arc(ball.x, ball.y, ball.radius, 0, 2 * Math.PI);
        // ctx.fillStyle = "red";
        // ctx.fill();
    }

</script>
