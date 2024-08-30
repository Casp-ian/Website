<script>
  import Cat from "../components/Cat.svelte";
</script>

<Cat startX={50} startY={50}/>

<div id="wrapper">
  <h1>haha funny kitty</h1>
  <p>meow meow</p>
  <p>you can pet it :3</p>
</div>


<style>
  #wrapper {
    background-color: var(--back-color);

    margin: .3rem;
    border-radius: .3rem;
    margin: 2rem 5rem 2rem 5rem;
  }
</style>
