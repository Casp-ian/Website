<script>
  import { onMount } from "svelte";

  export let layer = 1;
  export let width = '100%';
  export let height = 'fit-content';
  export let borderRadius = '5px';

  let top_element;
  let shadow;
  
  onMount(async () => {
    let box = top_element.getBoundingClientRect();
    move_shadow(box);
    set_shadow(box);
    console.log(box);
    
    addEventListener("resize", () => {set_shadow(top_element.getBoundingClientRect())});
    addEventListener("scroll", () => {move_shadow(top_element.getBoundingClientRect())});
  });

  function set_shadow(box) {
    shadow.style.width = box.width + 'px';
    shadow.style.height = box.height + 'px';
  }

  // TODO make sure what looks like the bottom of the element (the shadow) stays centered, to avoid it looking like its floating
  function move_shadow(box) {
    let screen_center_x = window.innerWidth / 2;
    let screen_center_y = window.innerHeight / 2;

    // TODO currently calculates from screen center to top left corner of element, do both corners and take shortest
    // TODO currently does not cover case of screen center being inside element
    // let screen_center_to_element_x = Math.min((box.x - screen_center_x), (box.x + box.width - screen_center_x));
    // let screen_center_to_element_y = Math.min((box.y - screen_center_y), (box.y + box.height - screen_center_y));
    let screen_center_to_element_x = (box.x - screen_center_x);
    let screen_center_to_element_y = (box.y - screen_center_y);
    console.log(screen_center_to_element_x, screen_center_to_element_y);

    let diff_x = screen_center_to_element_x / screen_center_x;
    let diff_y = screen_center_to_element_y / screen_center_y;

    shadow.style.marginTop = (-5 * diff_x) + 'px';
    shadow.style.marginLeft = (-5 * diff_y) + 'px';

    if (screen_center_x >= box.left && screen_center_x <= box.right) {
      shadow.style.marginLeft = '0';
    }
    if (screen_center_y >= box.top && screen_center_y <= box.bottom) {
      shadow.style.marginTop = '0';
    }
  }
</script>

<div id="wrapper">
  <div bind:this={top_element} style="z-index: {layer * 2}; width: {width}; height: {height}; border-radius: {borderRadius};" id="top_element">
    <slot></slot>
  </div>
  
  <div bind:this={shadow} style="z-index: {(layer * 2) - 1}; border-radius: {borderRadius};" id="shadow">
  </div>
</div>

<style>
  #wrapper {
    display: grid;
    padding: 10px;
  }
  
  #top_element {
    grid-column-start: 1;
    grid-row-start: 1;

    padding: 0.5rem;
    background-color: white;
  }

  #shadow {
    grid-column-start: 1;
    grid-row-start: 1;

    background-color: black;
  }
</style>
