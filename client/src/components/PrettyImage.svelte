<script lang="ts">
    import {onDestroy, onMount} from "svelte";

    export let src;
    export let alt;
    export let margin: string = "0px";

    let invisible = true;
    let dialog: HTMLDialogElement;

    function zoom(e: WheelEvent) {
        if (invisible) {
            return;
        }
        // TODO zoom
        console.log(e.deltaY);
    }

    onMount(async () => {
        addEventListener("wheel", zoom);
    });

    onDestroy(async () => {
        removeEventListener("wheel", zoom);
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
    }
</script>


<!-- the normal image, as if pretty image was never even used -->
<img style="--margin: {margin}" onclick={open} {src} {alt}/>

<dialog
    onclick={close}
    bind:this={dialog}
>
    <img class="big" {src} {alt}/>
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

        /*NOTE
         There is a small white line at the bottom of the dialog, why?
         No clue, but we hide it with this, but a small empty line remains
        */
        background: none;
    }

    img.big {
        margin: 0;

        max-width: 95vw;
        max-height: 95vh;

        width: 100%;
    }

</style>
