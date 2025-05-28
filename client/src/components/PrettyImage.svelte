<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import { browser } from '$app/environment';

    export let src;
    export let alt;
    export let margin: string = "0px";

    let scale: number = 1;

    let invisible = true;
    let dialog: HTMLDialogElement;

    function zoom(e: WheelEvent) {
        if (invisible) {
            return;
        }
        scale += e.deltaY * 0.001;
    }

    onMount(async () => {
        addEventListener("wheel", zoom);
    });

    onDestroy(async () => {
        if (browser) {
            removeEventListener("wheel", zoom);
        }
    });

    function close() {
        dialog.close();
        document.body.style.overflow = "auto";
        invisible = true;
    }

    function open() {
        dialog.showModal();
        document.body.style.overflow = "hidden";
        invisible = false;
        scale = 1;
    }
</script>


<!-- the normal image, as if pretty image was never even used -->
<img style="--margin: {margin}" onclick={open} {src} {alt}/>

<!--NOTE might want to disable this on mobile, since they can already zoom in easier anyway-->
<dialog
    onclick={close}
    bind:this={dialog}
>
    <img class="big" style="--scale: {scale}" {src} {alt}/>
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
        width: 100%;

        /* TODO we scale those */
        transform: scale(var(--scale, 1), var(--scale, 1));
        transform-origin: center;
    }
</style>
