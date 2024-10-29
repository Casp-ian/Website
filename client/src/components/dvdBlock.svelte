<script>
  import { onMount } from 'svelte';

  let test = {x: 0, y: 0, color: "red"}

  let up = true;
  let left = true;

  // TODO randomise these >:)
  const speedUp = 50;
  const speedLeft = 27;

  const max = 5000

  onMount(async () => {
    setInterval(tick, 50)
  })
  
  function tick() {
    if (up) {
      test.y += speedUp;
    } else {
      test.y -= speedUp;
    }

    if (left) {
      test.x += speedLeft;
    } else {
      test.x -= speedLeft;
    }

    if (test.y >= max) {
      up = false;
      test.color = "red"
    } else if (test.y <= 0) {
      up = true
      test.color = "green"
    }
    
    if (test.x >= max) {
      left = false;
      test.color = "blue"
    } else if (test.x <= 0) {
      left = true
      test.color = "yellow"
    }

    test = test;
  }

</script>


<div id="block" style="--top: {test.y / max}; --left: {test.x / max}; background-color: {test.color}">
  <p>hello!</p>
</div>


<style>
  #block {
    width: 100px;
    height: 41px;
    position: relative;
    top: calc(var(--top) * (100% - 41px));
    left: calc(var(--left) * (100% - 100px));

    background-color: var(--accent-color);

    text-align: center;
    vertical-align: middle;
    line-height: 41px;


  }

  p {
    color: black;
  }

</style>
