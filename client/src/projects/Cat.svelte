<script>
  import Cat from "../components/Cat.svelte";
</script>

<Cat startX={500} startY={500}/>

<div id="wrapper">
  <h1>haha funny kitty</h1>
  <p>meow meow</p>
</div>


<style>
  #wrapper {
    background-color: var(--back-color);

    margin: .3rem;
    border-radius: .3rem;
    padding: 1rem 2rem 1rem 2rem;
  }
</style>
