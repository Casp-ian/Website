<script>
    import Weather from '$components/big/Weather.svelte';
    import Navigation from "$components/big/Navigation.svelte";

    let {children} = $props();
</script>

<!-- Global stuffs-->
<Weather/>
<!--<Notifications/>-->

<Navigation/>
<!--    <MobileNavigation/>-->

<main>
    {@render children()}
</main>


<style lang="scss">
  /*
    Having everything absolute or relative like this will cause everything to layer on top of each other,
    Also this means that every child will need to set pointer-events back to auto
  */
  div, main {
    position: absolute;

    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    pointer-events: none;
  }

  main {
    position: relative;
  }

  :global(body) {
    background: #8338ec;
  }


  /*:global(*) {*/
  /*    --main-color: #f7fff7;*/
  /*    --back-color: #240021;*/
  /*    --back-color2: #3d003a;*/
  /*    --interactable-color: #ed455c;*/
  /*    --accent-color: #8338ec;*/
  /*    transition: background-color 0.2s;*/
  /*}*/

  /*:global(p, h1, h2, h3, button, pre) {*/
  /*    color: var(--main-color);*/
  /*    overflow: hidden;*/
  /*    overflow-wrap: break-word;*/
  /*}*/

  /*:global(a) {*/
  /*    color: var(--interactable-color);*/
  /*}*/

  /*:root {*/
  /*    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,*/
  /*    'Open Sans', 'Helvetica Neue', sans-serif;*/
  /*}*/

</style>
