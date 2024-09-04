<script>
  import { onMount } from "svelte";

  export let startX = 0;
  export let startY = 0;
  
  // -99-: stunned, -1: running, 0-9: sitting, 10-19: idle, 20-29: petting, 30-39: sleeping,
  let catState = -1;
  let runDirection = 0;
  // run direction laid out like
  // 704
  // 3X1
  // 625
  let imageOffset = {x: -32, y: -96};
  let even = false;

  let mouseMoveCount = 0;

  let mouse = {x: startX, y: startY};
  let cat = {x: startX, y: startY};

  function update() {

    // cat movement logic
    let diff = {x: mouse.x - cat.x, y: mouse.y - cat.y};
    const distance = Math.sqrt(diff.x * diff.x+ diff.y * diff.y);
    let speed = even ? 13 : 12; //differ speed to make movement more "natural"
    let unit = {x: diff.x / distance, y: diff.y / distance};

    setRunDirection(unit);

    // stunned
    if (catState < -1) {
      catState += 1;
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


      cat.x += unit.x * speed;
      cat.y += unit.y * speed;
      
      return;
    }

    // sitting
    if (0 <= catState && catState < 10) {
      // check if should run
      if (distance > 50) {
        catState = -5;
        return;
      }
      
      if (checkIfPet()) {
        catState = 20;
      }
      
      return;
    }

    // petting
    if (20 <= catState && catState < 30) {
      catState++;
      if (catState == 30) {
        catState = 0;
      }

      return;
    }

    // self assign to make svelte realize it needs to update
    cat = cat;
  }

  function animate() {
    even = !even;
    
    let y = even ? 0 : -32;

    // stunning
    if (catState < -1) {
      imageOffset = {x: -288, y};
    }

    // running
    if (catState == -1) {
      switch (runDirection) {
        case 0:
          imageOffset = {x: -32, y};  
          break;
        case 1:
          imageOffset = {x: -64, y};
          break;
        case 2:
          imageOffset = {x: -96, y};
          break;
        case 3:
          imageOffset = {x: -128, y};
          break;
        case 4:
          imageOffset = {x: -160, y};
          break;
        case 5:
          imageOffset = {x: -192, y};
          break;
        case 6:
          imageOffset = {x: -224, y};
          break;
        case 7:
          imageOffset = {x: -256, y};
          break;
      }
    }

    // sitting
    if (catState == 0) {
      imageOffset = {x: 0, y: -32};
    }

    // petting
    if (20 <= catState && catState < 30) {
      imageOffset = {x: -352, y};
    }
  }

  function setRunDirection(unit) {
    // cardinal directions
    if (unit.y < -0.88) {
      runDirection = 0;
      // imageOffset = {x: -32, y: yOffset};  
      return;
    }
    if (unit.x > 0.88) {
      runDirection = 1;
      // imageOffset = {x: -64, y: yOffset};
      return;
    }
    if (unit.y > 0.88) {
      runDirection = 2;
      // imageOffset = {x: -96, y: yOffset};
      return;
    }
    if (unit.x < -0.88) {
      runDirection = 3;
      // imageOffset = {x: -128, y: yOffset};
      return;
    }

    // diagonals
    if (unit.x > 0 && unit.y < 0) {
      runDirection = 4;
      // imageOffset = {x: -160, y: yOffset};
    }
    if (unit.x > 0 && unit.y > 0) {
      runDirection = 5;
      // imageOffset = {x: -192, y: yOffset};
    }
    if (unit.x < 0 && unit.y > 0) {
      runDirection = 6;
      // imageOffset = {x: -224, y: yOffset};
    }
    if (unit.x < 0 && unit.y < 0) {
      runDirection = 7;
      // imageOffset = {x: -256, y: yOffset};
    }
  }

  function checkIfPet() {
    if (mouseMoveCount > 50) {
      mouseMoveCount = 0;
      console.log("meow");
      return true;
    }
    return false;
  }

  function updateMouse(event) {
    mouseMoveCount++;
    // currently does not update on scroll
    mouse.x = event.clientX;
    mouse.y = event.clientY + window.scrollY;
  }

  onMount(async () => {
    setInterval(update, 110);
    setInterval(animate, 110);
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
    background-image: url("../assets/oneko2.png");
    image-rendering: pixelated;
    transform: translate(-50%, -50%);
  }
</style>
