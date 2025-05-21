<script lang="ts">
    import Navigation from "$components/big/Navigation.svelte";
    import BackgroundCanvas from "$components/big/BackgroundCanvas.svelte";
    import Notifications from "$components/big/Notifications.svelte";

    let {children} = $props();
</script>

<!-- Global stuffs go here! -->

<!--<MobileNavigation/>-->
<BackgroundCanvas/>

<main>
    <div id="left">
        <Navigation/>
    </div>
    <div id="center">
        {@render children()}
    </div>
    <div id="right">
        <Notifications/>
    </div>
</main>


<style lang="scss">


  /*
    Having everything absolute or relative like this will cause everything to layer on top of each other,
    Also this means that every child will need to set pointer-events back to auto
  */
  main {
    display: flex;
    flex-direction: row;
  }

  #left, #right {
    width: 15vw;
  }

  #center {
    width: 70vw;
  }

  :global(body) {
    margin: 0;
    padding: 0;

    background-image: url('$assets/sigma.png');
  }


  /*:global(*) {*/
  /*    --main-color: #f7fff7;*/
  /*    --back-color: #240021;*/
  /*    --back-color2: #3d003a;*/
  /*    --interactable-color: #ed455c;*/
  /*    --accent-color: #8338ec;*/
  /*    transition: background-color 0.2s;*/
  /*}*/


  :global(p, h1, h2, h3, button, pre) {
      color: white;
      //overflow: hidden;
      //overflow-wrap: break-word;
  }

  /*:global(a) {*/
  /*    color: var(--interactable-color);*/
  /*}*/

  /*:root {*/
  /*    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,*/
  /*    'Open Sans', 'Helvetica Neue', sans-serif;*/
  /*}*/

</style>
