<script>

  export let src;
  export let alt;

  let img;

  let invisible = true;

  function close() {
    invisible = true;
  }

  function open() {
    invisible = false;
  }

  function onKeyDown(e) {
    if (e.keyCode == 27) {
      close();
    }
  }

  
</script>

<!-- the normal image, as if pretty image was never even used -->
<img on:click={open} src={src} alt={alt}/>

<!-- big image, hidden untill image is clicked -->
<img class="big" class:invisible={invisible} bind:this={img} on:click={close} src={src} alt={alt}/>

<svelte:window on:keydown={onKeyDown} />


<style>
  img {
    width: 100%;
  }

  img.big {
    position: fixed;
    top: 50%;
    left: 50%;

    transform: translate(-50%, -50%);

    max-width: 95vw;
    max-height: 95vh;

    width: auto;
    height: auto;
  }

  img.invisible {
    visibility: hidden;
  }


</style>
