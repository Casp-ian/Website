<script>
    import Notifications from '$components/big/Notifications.svelte';
    import Weather from '$components/big/Weather.svelte';
    import Navigation from "$components/big/Navigation.svelte";
    import MobileNavigation from "$components/big/MobileNavigation.svelte";

    let {children} = $props();
</script>

<!-- Global stuffs-->
<Notifications/>
<Weather/>

<main>
    <div class="left desktop">
        <Navigation/>
    </div>
    <div class="left mobile">
        <MobileNavigation/>
    </div>

    <div id="center">
        {@render children()}
    </div>
</main>

<style lang="scss">
  $test: 600px;

  @media not (min-width: $test) {
    .desktop {
      width: 0;
      visibility: hidden;
    }
    .mobile {
      width: 20vw;
    }
  }

  @media (min-width: $test) {
    .mobile {
      width: 0;
      visibility: hidden;
    }
    .desktop {
      width: 20vw;
    }
  }

  main {
    //@media (min-width: $test) {
    //  flex-direction: row;
    //}
    //@media not (min-width: $test) {
    //  flex-direction: column;
    //}

    flex-direction: row;

    display: flex;
    flex-wrap: nowrap;

    position: absolute;
    top: 0;
    left: 0;
    min-height: 100vh;
    width: 100vw;

    /* TODO does this even do anything? it is supposed to stop shit from adding scroll if they go off screen */
    //clip: rect(0, auto, auto, 0);
  }

  #center {
    width: 60vw;
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
