<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import { browser } from '$app/environment';

    export let src;
    export let alt;
    export let margin: string = "0px";

    let scale: number = 1;
    let center = "center";

    let bigImage: HTMLImageElement;

    let zooming = true;
    let dialog: HTMLDialogElement;

    function zoom(e: WheelEvent) {
        if (zooming) {
            return;
        }
        let rect = bigImage.getBoundingClientRect();
        let x = (e.clientX - rect.x) / rect.width * 100;
        let y = (e.clientY - rect.y) / rect.height * 100;


        scale += e.deltaY * 0.001;
        center = x + "% " + y + "%";

        if (scale < 1) {
            scale = 1;
        }

        // bigImage.style.transform = "scale(" + scale + " " + scale + ")";
        // bigImage.style.transformOrigin = x + "% " + y + "%";
    }

    onMount(async () => {
        addEventListener("wheel", zoom);
        dialog.addEventListener("close", close)
    });

    onDestroy(async () =>
    {
        if (browser) {
            removeEventListener("wheel", zoom);
        }
    });

    function close() {
        document.body.style.overflow = "auto";
        zooming = true;
    }

    function open() {
        dialog.showModal()
        document.body.style.overflow = "hidden";
        zooming = false;
        scale = 1;
    }
</script>


<!-- the normal image, as if pretty image was never even used -->
<img style="--margin: {margin}" onclick={open} {src} {alt}/>

<!--NOTE might want to disable this on mobile, since they can already zoom in easier anyway-->
<dialog
    onclick={() => dialog.close()}
    bind:this={dialog}
>
    <img
            bind:this={bigImage}
            class="big" style="--scale: {scale}; --center: {center}" {src} {alt}/>
</dialog>

<style>
    img {
        display: block;
        width: calc(100% - var(--margin, 0) * 2);
        margin: var(--margin, 0);
        cursor: pointer;
    }

    dialog {
        padding: 0;
        overflow: hidden;

        background: none;
    }

    img.big {
        margin: 0;
        max-width: 95vw;
        max-height: 95vh;

        /* TODO we scale those */
        transform: scale(var(--scale, 1), var(--scale, 1));
        transform-origin: var(--center, center);
    }
</style>
