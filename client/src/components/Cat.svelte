<script>
  import { onMount } from "svelte";

  // -99-: stunned, -1: running, 0: sitting, 9: sleeping, 10-99: petting,
  let catState = -1;
  let imageOffset = {x: -32, y: -96};
  let even = false;

  let mouseMoveCount = 0;

  let mouse = {x: 0, y: 0};
  let cat = {x: 0, y: 0};

  function update() {
    even = !even;

  
    let diff = {x: mouse.x - cat.x, y: mouse.y - cat.y};
    const distance = Math.sqrt(diff.x * diff.x+ diff.y * diff.y);
    let speed = even ? 12 : 10;
    let unit = {x: diff.x / distance, y: diff.y / distance};

    // stunned
    if (catState < -1) {
      catState += 1;
      imageOffset = {x: -288, y: 0};
      return;
    }

    // running
    if (catState == -1) {

      // check if should sit
      if (distance < 50) {
        catState = 0;
        mouseMoveCount = 0;
        return;
      }
  
      if (even) {
        setRunAnimation(unit, -32);
      } else {
        setRunAnimation(unit, 0);
      }
          
      cat.x += unit.x * speed;
      cat.y += unit.y * speed;
      
      return;
    }

    // sitting
    if (catState == 0) {
      // check if should run
      if (distance > 50) {
        catState = -9;
        return;
      }
      
      checkIfPet();
      imageOffset = {x: 0, y: 0};
      
      return;
    }

    cat = cat;
  }

  function setRunAnimation(unit, yOffset) {
    // cardinal directions
    if (unit.y < -0.88) {
      imageOffset = {x: -32, y: yOffset};  
      return;
    }
    if (unit.x > 0.88) {
      imageOffset = {x: -64, y: yOffset};
      return;
    }
    if (unit.y > 0.88) {
      imageOffset = {x: -96, y: yOffset};
      return;
    }
    if (unit.x < -0.88) {
      imageOffset = {x: -128, y: yOffset};
      return;
    }

    // diagonals
    if (unit.x > 0 && unit.y < 0) {
      imageOffset = {x: -160, y: yOffset};
    }
    if (unit.x > 0 && unit.y > 0) {
      imageOffset = {x: -192, y: yOffset};
    }
    if (unit.x < 0 && unit.y > 0) {
      imageOffset = {x: -224, y: yOffset};
    }
    if (unit.x < 0 && unit.y < 0) {
      imageOffset = {x: -256, y: yOffset};
    }
  }

  function checkIfPet() {
    if (mouseMoveCount > 100) {
      mouseMoveCount = 0;
      console.log("meow");
    }
  }

  function updateMouse(event) {
    mouseMoveCount++;
    // currently does not update on scroll
    mouse.x = event.clientX;
    mouse.y = event.clientY + window.scrollY;
  }

  onMount(async () => {
    setInterval(update, 100);
    document.onmousemove = updateMouse;
  })

</script>


<div id="cat" style="top: {cat.y}px; left: {cat.x}px; background-position-x: {imageOffset.x}px; background-position-y: {imageOffset.y}px;" />


<style>
  #cat {
    pointer-events: none;
    position: absolute;
    width: 32px;
    height: 32px;
    background-image: url("../assets/oneko.png");
    image-rendering: pixelated;
    transform: translate(-50%, -50%);
  }
</style>
