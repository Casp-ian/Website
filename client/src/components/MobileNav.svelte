<script lang="ts">

  // TODO maybe only on move is needed, if onmove is on a specific object 
  // it just passes all move events when you press it and drag away, and stop when you release
  function onMove(event: Event) {
    console.log("move", event)
    test = "move";
  }
  function onDown(event: Event) {
    console.log("down", event)
    test = "down";
    held = true;
  }
  function onUp(event: Event) {
    console.log("up", event)
    test = "up";
    held = false;
  }

  const fields = [
    {
      text: "test",
      path: "/home"
    },
    {
      text: "wasd",
      path: "/projects"
    },
    {
      text: "dog",
      path: "/home"
    },
  ]
  
  let test = $state("");
  let held = $state(false);
</script>

<div id="wrapper">
  <div id="bar">
    <div class="item"
        ontouchstart={onDown} 
        ontouchend={onUp} 
        ontouchmove={onMove}
    >
      <p>&lt</p>
    </div>
    {#each fields as field}
      <!-- todo debate to use a element or not, it does cause issues on firefox and safari mobile at least -->
      <div class="item"
        ontouchstart={onDown} 
        ontouchend={onUp}
        ontouchmove={onMove}
      >
        <p>{field.text}</p>
      </div>
    {/each}
  </div>
</div>

<p>{test}</p>
<p>{held}</p>


<style>
  #wrapper {
    position: absolute;
    bottom: 33vh;
  }
  #bar {
    /* position to be right side, one third from the bottom */
    /* todo lefthanded version */
    
    display: flex;

    overflow: hidden;
  }

  .item {
    user-select: none;
    padding-left: .5rem;
    padding-right: .5rem;
  }
</style>
